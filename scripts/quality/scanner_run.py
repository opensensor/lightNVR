#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
"""Run one scanner with real exit/report/file provenance (never a policy verdict).

Usage: scanner_run.py --spec scan.json --root . --reports build/scan \
  --expected-revision <full SHA>
The JSON spec supplies argv arrays (no shell), a version_command, report paths,
and explicit scope evidence. Existing reports/scanner records are never replaced.
"""
import argparse
import datetime as dt
import hashlib
import fnmatch
import json
import os
from pathlib import Path, PurePosixPath
import re
import signal
import shlex
import shutil
import subprocess
import tempfile
import zipfile


def safe_path(base, relative):
    """Reject traversal and symlinks before reads, writes, or hashes."""
    base = Path(base).resolve()
    if not isinstance(relative, str) or not relative or '\\' in relative:
        raise ValueError('Expected a nonempty portable relative path')
    parsed = PurePosixPath(relative)
    if parsed.is_absolute() or any(part in ('..', '.') for part in relative.split('/')):
        raise ValueError(f'Unsafe relative path: {relative}')
    target = base
    for part in parsed.parts:
        target = target / part
        if target.is_symlink():
            raise ValueError(f'Symlink is not scan evidence: {relative}')
    if not target.resolve().is_relative_to(base):
        raise ValueError(f'Path escapes evidence root: {relative}')
    return target


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def timestamp():
    return dt.datetime.now(dt.timezone.utc).isoformat()


def execute(argv, cwd, timeout, stdout=None, stderr=None):
    if not isinstance(argv, list) or not argv or any(not isinstance(x, str) for x in argv):
        raise ValueError('Scanner commands must be nonempty argv arrays')
    process = subprocess.Popen(argv, cwd=cwd, stdout=stdout or subprocess.PIPE,
                               stderr=stderr or subprocess.PIPE, start_new_session=True)
    timed_out = False
    try:
        out, err = process.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        timed_out = True
        os.killpg(process.pid, signal.SIGKILL)
        out, err = process.communicate()
    return {'exit_code': process.returncode, 'timed_out': timed_out}, out or b'', err or b''


def source_records(root, names):
    files = []
    hashes = {}
    for name in names:
        if not isinstance(name, str):
            raise ValueError('Scanned-file names must be strings')
        candidate = Path(name)
        if candidate.is_absolute():
            try:
                name = candidate.relative_to(root).as_posix()
            except ValueError:
                raise ValueError('Reported source file is outside the checkout') from None
        file = safe_path(root, name)
        if not file.is_file():
            raise ValueError(f'Scanned source does not exist: {name}')
        files.append(name)
        hashes[name] = sha(file)
    return sorted(set(files)), hashes


def checkout_relative(root, path):
    candidate = Path(path)
    if candidate.is_absolute():
        try:
            path = candidate.relative_to(root).as_posix()
        except ValueError:
            raise ValueError('FAAC evidence is outside checkout') from None
    else:
        path = candidate.as_posix()
    safe_path(root, path)
    return path


def load_faac_provenance(root, options):
    """Verify copied bytes and actual compiler inputs against tracked authority."""
    root = Path(root).resolve()
    if not isinstance(options, dict):
        raise ValueError('Missing required compiled FAAC provenance')
    static_path = safe_path(root, 'patches/faac/manifest.json')
    static = json.loads(static_path.read_text())
    patch_path = safe_path(root, 'patches/faac/0001-bound-input-pointer-arithmetic.patch')
    prov_path = safe_path(root, options['provenance'])
    compdb_path = safe_path(root, options['compile_commands'])
    prov = json.loads(prov_path.read_text())
    if static.get('schema_version') != 1 or prov.get('schema_version') != 1:
        raise ValueError('Unsupported FAAC provenance version')
    for key in ('upstream_commit', 'patch_sha256'):
        if prov.get(key) != static.get(key) or not prov.get(key):
            raise ValueError('FAAC provenance identity mismatch: ' + key)
    if prov.get('manifest_sha256') != sha(static_path) or static['patch_sha256'] != sha(patch_path):
        raise ValueError('FAAC tracked manifest/patch hash mismatch')
    source_root = safe_path(root, 'third_party/faac')
    head, stdout, stderr = execute(['git','-C',str(source_root),'rev-parse','HEAD'], root, 30)
    link, linkout, linkerr = execute(['git','-C',str(root),'ls-files','--stage','--','third_party/faac'], root, 30)
    expected_link = '160000 ' + static['upstream_commit'] + ' 0\tthird_party/faac'
    if head['exit_code'] != 0 or head['timed_out'] or stdout.decode().strip() != static['upstream_commit']:
        raise ValueError('FAAC actual upstream pin mismatch')
    if link['exit_code'] != 0 or link['timed_out'] or linkout.decode().strip() != expected_link:
        raise ValueError('FAAC actual gitlink mismatch')
    copy = safe_path(root, checkout_relative(root, prov['build_root']))
    if prov.get('source_root') != str(source_root) or not copy.as_posix().endswith('/_deps/lightnvr-faac'):
        raise ValueError('FAAC source/copy root mismatch')
    if prov_path != copy / 'provenance.json' or compdb_path != copy.parent.parent / 'compile_commands.json':
        raise ValueError('FAAC provenance/compile database mixed build roots')
    rows = prov.get('files')
    if not isinstance(rows, list) or len(rows) != len(static['input_files']):
        raise ValueError('FAAC copied-file inventory mismatch')
    seen = set()
    for row in rows:
        name = row['upstream_path']
        if name in seen or name not in static['input_files'] or name not in static['output_files']:
            raise ValueError('FAAC unexpected/duplicate mapped source')
        seen.add(name)
        original = safe_path(source_root, name)
        copied = safe_path(copy, name)
        if row['compiled_path'] != str(copied):
            raise ValueError('FAAC compiled path disguises original identity')
        if row['input_sha256'] != static['input_files'][name] or sha(original) != row['input_sha256']:
            raise ValueError('FAAC original source hash mismatch: ' + name)
        if row['output_sha256'] != static['output_files'][name] or sha(copied) != row['output_sha256']:
            raise ValueError('FAAC compiled source hash mismatch: ' + name)
    if seen != set(static['output_files']):
        raise ValueError('FAAC input/output inventory mismatch')
    expected_units = sorted(str(safe_path(copy, p)) for p in static['translation_units'])
    if not expected_units or prov.get('translation_units') != expected_units:
        raise ValueError('FAAC translation-unit inventory mismatch')
    commands = json.loads(compdb_path.read_text())
    if not isinstance(commands, list):
        raise ValueError('Malformed compilation database')
    actual_units = []
    for row in commands:
        file = Path(row['file'])
        if not file.is_absolute():
            file = Path(row['directory']) / file
        if file.is_relative_to(source_root):
            raise ValueError('Original FAAC compile root mixed with patched source')
        if not file.is_relative_to(copy):
            continue
        safe_path(root, checkout_relative(root, file))
        if str(file) not in expected_units:
            raise ValueError('Unexpected compiled FAAC translation unit')
        argv = row.get('arguments') or shlex.split(row['command'])
        if not isinstance(argv, list) or not all(isinstance(x, str) for x in argv):
            raise ValueError('Malformed FAAC compiler argv')
        if any(str(source_root) in arg for arg in argv):
            raise ValueError('Original FAAC include root mixed with patched headers')
        joined = ' '.join(argv)
        if str(copy / 'include') not in joined or str(copy / 'libfaac') not in joined:
            raise ValueError('Missing copied FAAC include roots')
        actual_units.append(str(file))
    if sorted(actual_units) != expected_units:
        raise ValueError('FAAC compile database omits/duplicates translation units')
    artifacts = {'provenance.json':prov_path, 'compile_commands.json':compdb_path,
        'manifest.json':static_path, 'manifest.cmake':safe_path(root,'patches/faac/manifest.cmake'),
        'input-pointer.patch':patch_path}
    if options.get('analysis_compile_commands'):
        analysis_path = safe_path(root, options['analysis_compile_commands'])
        selected = json.loads(analysis_path.read_text())
        if not isinstance(selected, list) or not selected or any(row not in commands for row in selected):
            raise ValueError('Filtered analysis compilation database differs from actual compiler inputs')
        artifacts['analysis_compile_commands.json'] = analysis_path
    return prov, commands, artifacts


def native_analysis_inputs(root, options, scanner):
    """Union the old scanner-specific source selection with compiled FAAC."""
    root = Path(root).resolve()
    prov, commands, _ = load_faac_provenance(root, options)
    selected = {checkout_relative(root, p) for p in prov['translation_units']}
    for row in commands:
        path = Path(row['file'])
        if not path.is_absolute():
            path = Path(row['directory']) / path
        if not path.is_relative_to(root):
            continue
        if scanner == 'clang-tidy':
            keep = path.is_relative_to(root / 'src') and not any(p in ('sod','ezxml') for p in path.parts)
        elif scanner == 'cppcheck':
            keep = fnmatch.fnmatch(path.as_posix(), '*/src/*')
        else:
            raise ValueError('Unknown native scanner selection')
        if keep:
            selected.add(checkout_relative(root, path))
    return sorted(selected)


def attach_faac_evidence(spec, root, reports, scope):
    required = spec['scanner_id'] in ('clang-tidy','cppcheck','codeql-cpp') and (root/'patches/faac/manifest.json').exists()
    if not required and 'faac' not in spec:
        return
    prov, _, artifacts = load_faac_provenance(root, spec.get('faac'))
    needed = {checkout_relative(root, p) for p in prov['translation_units']}
    if scope.get('kind') == 'codeql-source-archive':
        needed.add(checkout_relative(root, Path(prov['build_root'])/'libfaac/faac_internal.h'))
        artifacts['codeql-src.zip'] = safe_path(root, spec['scope']['archive'])
    if spec['scanner_id'] in ('clang-tidy','cppcheck'):
        needed.update(native_analysis_inputs(root, spec['faac'], spec['scanner_id']))
    missing = needed - set(scope.get('files', []))
    if missing:
        raise ValueError('Required compiled/native sources were not scanned: ' + ', '.join(sorted(missing)))
    archived = {}
    for name, original in artifacts.items():
        relative = 'provenance/faac/' + name
        target = safe_path(reports, relative)
        target.parent.mkdir(parents=True, exist_ok=True)
        with original.open('rb') as source, target.open('xb') as dest:
            shutil.copyfileobj(source, dest)
        archived[relative] = sha(target)
    scope['faac_provenance'] = {'upstream_commit':prov['upstream_commit'],
        'patch_sha256':prov['patch_sha256'], 'manifest_sha256':prov['manifest_sha256'],
        'files':prov['files'], 'translation_units':prov['translation_units'],
        'evidence_sha256':archived}


def scope_evidence(spec, root, reports):
    scope = spec.get('scope') or {}
    kind = scope.get('kind')
    evidence = {}
    if kind == 'eslint-json':
        report = safe_path(reports, scope['report'])
        rows = json.loads(report.read_text())
        if not isinstance(rows, list):
            raise ValueError('ESLint scope requires its JSON file report')
        names = [row['filePath'] for row in rows]
        evidence[scope['report']] = sha(report)
    elif kind == 'cppcheck-progress':
        # Cppcheck --project cannot accept positional source filenames. Its
        # actual progress stream, not compile_commands candidates, is scope.
        if scope.get('report') != f"logs/{spec['scanner_id']}-0.stdout.log":
            raise ValueError('Cppcheck scope must use the captured command progress')
        report = safe_path(reports, scope['report'])
        names = re.findall(r'^Checking (.+?)(?: \.\.\.|:)', report.read_text(), re.MULTILINE)
        evidence[scope['report']] = sha(report)
    elif kind == 'invocation-files':
        names = scope.get('files', [])
        if not all(c.get('append_scope_files') for c in spec['commands']):
            raise ValueError('Invocation scope files must be passed to every scanner command')
    elif kind == 'npm-lockfiles':
        names = []
        for command in spec['commands']:
            argv = command['argv']
            if Path(argv[0]).name not in ('npm', 'npm.cmd') or 'audit' not in argv:
                raise ValueError('npm-lockfiles scope requires npm audit commands')
            names.append((PurePosixPath(command.get('cwd', '')) / 'package-lock.json').as_posix())
    elif kind == 'file-list-report':
        report = safe_path(reports, scope['report'])
        names = json.loads(report.read_text())['files']
        evidence[scope['report']] = sha(report)
    elif kind == 'codeql-source-archive':
        archive = safe_path(root, scope['archive'])
        names = []
        prefix = root.as_posix().lstrip('/') + '/'
        excluded = 0
        with zipfile.ZipFile(archive) as zipped:
            for item in zipped.infolist():
                if item.is_dir():
                    continue
                name = item.filename.lstrip('/')
                if name.startswith(prefix):
                    name = name[len(prefix):]
                elif not (root / name).is_file():
                    excluded += 1
                    continue
                file = safe_path(root, name)
                if hashlib.sha256(zipped.read(item)).hexdigest() != sha(file):
                    raise ValueError(f'CodeQL archive differs from checkout: {name}')
                names.append(name)
        files, hashes = source_records(root, names)
        if not files:
            raise ValueError('CodeQL archive contains no checkout sources')
        return {'kind': kind, 'files': files, 'file_sha256': hashes,
                'archive_sha256': sha(archive), 'external_entries_omitted': excluded}
    elif kind == 'image':
        report = safe_path(reports, scope['report'])
        runs = json.loads(report.read_text())['runs']
        targets = []
        for run in runs:
            props = run.get('properties') or {}
            if not props.get('imageName') or not props.get('imageID'):
                raise ValueError('Image scope requires tool-reported imageName and imageID')
            targets.append({'name': props['imageName'], 'digest': props['imageID']})
        if not targets:
            raise ValueError('Image scanner reported no targets')
        return {'kind': 'image', 'targets': targets, 'evidence_sha256': {scope['report']: sha(report)}}
    else:
        raise ValueError(f'Unsupported or candidate-only scope evidence: {kind}')
    files, hashes = source_records(root, names)
    if not files:
        raise ValueError('No actual scanned files recorded')
    return {'kind': kind, 'files': files, 'file_sha256': hashes, 'evidence_sha256': evidence}


def run_scanner(spec, *, root, reports, revision):
    root = Path(root).resolve()
    reports = Path(reports)
    if not reports.is_absolute():
        reports = root / reports
    try:
        reports = safe_path(root, reports.absolute().relative_to(root).as_posix())
    except ValueError:
        raise ValueError('Report directory must be a nonsymlink path inside checkout root') from None
    if not re.fullmatch(r'(?:[0-9a-f]{40}|[0-9a-f]{64})', revision or ''):
        raise ValueError('Expected a full immutable commit revision')
    sid = spec.get('scanner_id')
    if not isinstance(sid, str) or not re.fullmatch(r'[a-z0-9][a-z0-9-]*', sid):
        raise ValueError('Invalid scanner ID')
    commands = spec.get('commands')
    if not isinstance(commands, list) or not commands:
        raise ValueError('No scanner commands supplied')
    declared = []
    evidence_outputs = []
    for command in commands:
        command_reports = command.get('reports') or []
        if not command_reports:
            raise ValueError('Each scanner command must declare its reports')
        declared.extend(command_reports)
        evidence_outputs.extend(command.get('evidence_outputs', []))
        if command.get('stdout_report') and command['stdout_report'] not in command_reports:
            raise ValueError('stdout report must be among declared reports')
    outputs = declared + evidence_outputs
    scope = spec.get('scope') or {}
    if scope.get('kind') in ('eslint-json', 'file-list-report', 'image') and scope.get('report') not in outputs:
        raise ValueError('Tool scope must come from a declared fresh output')
    if len(set(outputs)) != len(outputs):
        raise ValueError('Report output paths must be unique')
    for relative in outputs:
        if safe_path(reports, relative).exists():
            raise ValueError(f'Refusing preexisting/stale report: {relative}')
    manifest_path = safe_path(reports, 'run-manifest.json')
    manifest = {'schema_version': 1, 'revision': revision, 'scanners': {}}
    if manifest_path.exists():
        manifest = json.loads(manifest_path.read_text())
        if manifest.get('revision') != revision or sid in manifest.get('scanners', {}):
            raise ValueError('Manifest belongs to another revision or already records this scanner')
    reports.mkdir(parents=True, exist_ok=True)
    for relative in outputs:
        safe_path(reports, relative).parent.mkdir(parents=True, exist_ok=True)
    entry = {'status': 'failure', 'exit_code': None, 'tool_version': None, 'revision': revision,
             'started_at': timestamp(), 'reports': declared, 'report_sha256': {}, 'commands': [], 'errors': []}
    def capture_version():
        try:
            version, out, err = execute(spec['version_command'], root, 30)
            if version['exit_code'] != 0 or version['timed_out'] or not (out + err).strip():
                raise ValueError('Tool-version command did not return a version')
            entry['tool_version'] = (out + err).decode('utf8', errors='replace').strip()[:4096]
            if spec.get('version_format') == 'trivy-json':
                metadata = json.loads(out)
                database = metadata.get('VulnerabilityDB')
                if not isinstance(database, dict) or not database.get('Version') or not database.get('UpdatedAt'):
                    raise ValueError('Trivy version lacks vulnerability database metadata')
                if not metadata.get('Version'):
                    raise ValueError('Trivy version lacks tool version')
                entry['tool_version'] = metadata['Version']
                entry['db_version'] = database
                entry['database_metadata'] = {k:v for k,v in metadata.items() if k != 'Version'}
        except (OSError, ValueError, KeyError) as error:
            entry['errors'].append(str(error))
    capture_version()
    for index, command in enumerate(commands):
        argv = list(command.get('argv') or [])
        record = {'argv': argv, 'exit_code': None, 'timed_out': False}
        try:
            cwd = root if not command.get('cwd') else safe_path(root, command['cwd'])
            if command.get('append_scope_files'):
                names, _ = source_records(root, spec.get('scope', {}).get('files', []))
                argv += [str(root / name) for name in names]
                record['argv'] = argv
            stdout_name = command.get('stdout_report') or f'logs/{sid}-{index}.stdout.log'
            stdout = safe_path(reports, stdout_name)
            stderr = safe_path(reports, f'logs/{sid}-{index}.stderr.log')
            stdout.parent.mkdir(parents=True, exist_ok=True)
            stderr.parent.mkdir(parents=True, exist_ok=True)
            with stdout.open('xb') as out, stderr.open('xb') as err:
                result, _, _ = execute(argv, cwd, command.get('timeout_seconds', 1800), out,
                                      subprocess.STDOUT if command.get('stderr_to_stdout') else err)
            record.update(result)
            accepted = command.get('accepted_exit_codes', [0])
            record['accepted'] = record['exit_code'] in accepted and not record['timed_out']
            if not record['accepted']:
                entry['errors'].append(f'Command {index} failed with exit {record["exit_code"]}')
        except (OSError, ValueError, KeyError) as error:
            record['accepted'] = False
            entry['errors'].append(str(error))
        entry['commands'].append(record)
    if spec.get('version_format') == 'trivy-json':
        # Include supplementary databases fetched during the actual scan.
        capture_version()
    codes = [c['exit_code'] for c in entry['commands'] if isinstance(c['exit_code'], int)]
    failed = [c['exit_code'] for c in entry['commands'] if not c['accepted'] and isinstance(c['exit_code'], int)]
    entry['exit_code'] = failed[0] if failed else max(codes, default=None)
    for relative in declared:
        try:
            entry['report_sha256'][relative] = sha(safe_path(reports, relative))
        except (OSError, ValueError) as error:
            entry['errors'].append(str(error))
    try:
        entry['scope'] = scope_evidence(spec, root, reports)
        attach_faac_evidence(spec, root, reports, entry['scope'])
        if 'files' in entry['scope']:
            entry['files_scanned'] = len(entry['scope']['files'])
    except (OSError, ValueError, KeyError, TypeError, zipfile.BadZipFile) as error:
        entry['errors'].append(str(error))
    entry['completed_at'] = timestamp()
    if not entry['errors']:
        entry['status'] = 'success'
    manifest['scanners'][sid] = entry
    # One output directory per CI job; independent jobs merge only after finish.
    # Atomic replacement avoids a partial JSON manifest after process failure.
    with tempfile.NamedTemporaryFile(mode='w', dir=reports, prefix='.manifest-', delete=False) as temp:
        json.dump(manifest, temp, indent=2)
        temp.write('\n')
        temporary = temp.name
    os.replace(temporary, manifest_path)
    return entry


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--spec', required=True)
    parser.add_argument('--root', default='.')
    parser.add_argument('--reports', required=True)
    parser.add_argument('--expected-revision', required=True)
    args = parser.parse_args(argv)
    try:
        entry = run_scanner(json.loads(Path(args.spec).read_text()), root=Path(args.root), reports=Path(args.reports), revision=args.expected_revision)
        print(f'SCANNER {entry["status"].upper()}: this is execution provenance, not a findings-policy verdict')
        for error in entry['errors']:
            print(error)
        return 0 if entry['status'] == 'success' else 1
    except (OSError, ValueError) as error:
        print(f'SCANNER ERROR: {error}')
        return 2


if __name__ == '__main__':
    raise SystemExit(main())
