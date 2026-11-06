#!/usr/bin/env python3
"""Prepare pinned historical binaries and validate sanitized recovery evidence.

No database is created here. upgrade.py seeds the genuine prior binary's schema.
Build logs and full reports are private; only explicit summaries are publishable.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import platform
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
REVISION = 'c82d6ed17c7e8ec9038bf4ebcdaa96d3b8cd8163'
TAG = '0.45.3'
PINS = {'go2rtc': '6986a1b3f122651b318a957534da44bb4c65e271',
        'third_party/faac': 'eb4928a76fd075f9bab20e8195df84f937da289b'}
FLAGS = ['CMAKE_BUILD_TYPE', 'CMAKE_C_FLAGS', 'CMAKE_CXX_FLAGS', 'BUILD_TESTS',
         'ENABLE_SOD', 'ENABLE_LITERT', 'ENABLE_GO2RTC', 'ENABLE_MQTT', 'ENABLE_SSL',
         'GO2RTC_BINARY_PATH', 'YAML_VERSION']


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha(file):
    return hashlib.sha256(Path(file).read_bytes()).hexdigest()


def run(command, cwd=ROOT, log=None, timeout=900):
    command = list(map(str, command))
    if log:
        with Path(log).open('w') as output:
            subprocess.run(command, cwd=cwd, stdout=output, stderr=subprocess.STDOUT,
                           check=True, timeout=timeout)
        return ''
    return subprocess.check_output(command, cwd=cwd, stderr=subprocess.PIPE,
                                   text=True, timeout=timeout).strip()


def git(checkout, *args):
    return run(['git', '-C', checkout, *args], timeout=180)


def resolve_manifest_path(value):
    path = Path(value)
    return path.resolve() if path.is_absolute() else (ROOT / path).resolve()


def write_json(path, data):
    Path(path).write_text(json.dumps(data, indent=2) + '\n')


def cache_flags(binary):
    cache = Path(binary).resolve().parents[1] / 'CMakeCache.txt'
    result = {}
    for line in cache.read_text().splitlines():
        if ':' in line and '=' in line:
            key = line.split(':', 1)[0]
            if key in FLAGS:
                result[key] = line.split('=', 1)[1]
    require(result.get('ENABLE_GO2RTC') == 'ON', 'Managed go2rtc build required')
    require(result.get('CMAKE_BUILD_TYPE') == 'Debug', 'Debug sanitizer historical build required')
    return result


def source_manifest(checkout):
    names = git(checkout, 'ls-files', '--', 'CMakeLists.txt', 'cmake', 'src', 'include', 'db/migrations').splitlines()
    digest = hashlib.sha256()
    files = []
    for name in sorted(set(names)):
        file = checkout / name
        if not file.is_file():
            continue
        data = file.read_bytes()
        digest.update(name.encode() + b'\0' + data + b'\0')
        files.append({'path': name, 'sha256': hashlib.sha256(data).hexdigest()})
    require(files, 'Historical source manifest is empty')
    return {'revision': REVISION, 'source_sha256': digest.hexdigest(), 'files': files}


def verify_checkout(checkout, allowed_build=None):
    require(checkout != ROOT, 'Historical checkout must be isolated from current source')
    require(Path(git(checkout, 'rev-parse', '--show-toplevel')).resolve() == checkout,
            'Expected a checkout root, not a source subdirectory')
    require(git(checkout, 'rev-parse', 'HEAD') == REVISION, 'Historical HEAD does not match pin')
    require(git(checkout, 'rev-parse', f'{TAG}^{{commit}}') == REVISION, 'Historical tag does not match pin')
    changed = set(git(checkout, 'diff', 'HEAD', '--name-only', '--ignore-submodules=none').splitlines())
    require(changed <= {'web/js/version.js'}, 'Historical tracked source or dependency is dirty')
    for name in git(checkout, 'ls-files', '--others', '--exclude-standard').splitlines():
        file = (checkout / name).resolve()
        require(allowed_build is not None and file.is_relative_to(allowed_build),
                'Unexpected untracked historical source')
    for name, revision in PINS.items():
        tree = git(checkout, 'ls-tree', REVISION, '--', name).split()
        require(len(tree) >= 3 and tree[0] == '160000' and tree[2] == revision,
                f'Historical gitlink mismatch: {name}')
        dependency = checkout / name
        require(git(dependency, 'rev-parse', 'HEAD') == revision, f'Historical dependency HEAD mismatch: {name}')
        require(not git(dependency, 'status', '--porcelain', '--untracked-files=all'),
                f'Historical dependency is dirty: {name}; current patches must never be applied here')


def verify_old_manifest(prior, checkout):
    require(prior['revision'] == REVISION and prior['submodules'] == PINS, 'Reuse provenance pins differ')
    require(resolve_manifest_path(prior['checkout']) == checkout, 'Reuse checkout differs from recorded provenance')
    binary, go = (resolve_manifest_path(prior[key]) for key in ('binary', 'go2rtc'))
    require(binary.is_relative_to(checkout) and go.is_relative_to(checkout), 'Reuse executables must belong to historical checkout')
    require(sha(binary) == prior['binary_sha256'] and sha(go) == prior['go2rtc_sha256'], 'Reuse executable hash mismatch')
    recorded = json.loads(resolve_manifest_path(prior['source_manifest']).read_text())
    require(recorded['revision'] == REVISION, 'Reuse source revision mismatch')
    digest = hashlib.sha256()
    for item in recorded['files']:
        file = (checkout / item['path']).resolve()
        require(file.is_relative_to(checkout) and sha(file) == item['sha256'], 'Reuse source file mismatch')
        digest.update(item['path'].encode() + b'\0' + file.read_bytes() + b'\0')
    require(digest.hexdigest() == recorded['source_sha256'] == prior['source_sha256'], 'Reuse source digest mismatch')
    verify_checkout(checkout, binary.parents[1])
    return binary, go


def prepare(args):
    output = args.output.resolve()
    output.mkdir(parents=False, mode=0o700, exist_ok=False)
    try:
        current_binary = args.current_binary.resolve(strict=True)
        current_go = args.current_go2rtc.resolve(strict=True)
        current_manifest = args.current_build_manifest.resolve(strict=True)
        current = json.loads(current_manifest.read_text())
        require(current['binary_sha256'] == sha(current_binary), 'Current native provenance mismatch')
        require(current['go2rtc_sha256'] == sha(current_go), 'Current go2rtc provenance mismatch')
        require(current['build_flags']['ENABLE_GO2RTC'] == 'ON', 'Current upgrade binary must manage go2rtc')
        commands = {}
        if args.reuse_checkout:
            require(args.reuse_manifest is not None, '--reuse-checkout requires --reuse-manifest')
            checkout = args.reuse_checkout.resolve(strict=True)
            old = json.loads(args.reuse_manifest.read_text())['prior']
            binary, go = verify_old_manifest(old, checkout)
            mode = 'verified-existing-build'
            commands = {key: old[key] for key in ('configure_command', 'build_command', 'go_build_command')}
        else:
            require(args.reuse_manifest is None, '--reuse-manifest requires --reuse-checkout')
            checkout = output / 'prior'
            commands['clone_command'] = ['git', 'clone', '--no-hardlinks', '--no-checkout', str(ROOT), str(checkout)]
            run(commands['clone_command'], log=output / 'clone.log')
            git(checkout, 'checkout', '--detach', REVISION)
            # The old release supplies the URLs and immutable gitlinks. Do not
            # replace its go2rtc revision with the current parent's pin.
            run(['git', '-C', checkout, 'submodule', 'update', '--init', '--depth', '1', *PINS],
                log=output / 'submodules.log')
            verify_checkout(checkout)
            go = checkout / 'go2rtc/go2rtc'
            commands['go_build_command'] = ['go', 'build', '-trimpath', '-ldflags', '-s -w', '-o', str(go), '.']
            run(commands['go_build_command'], cwd=checkout / 'go2rtc', log=output / 'go-build.log')
            build = checkout / 'native'
            commands['configure_command'] = ['cmake', '--fresh', '-S', str(checkout), '-B', str(build),
                '-DCMAKE_BUILD_TYPE=Debug', '-DBUILD_TESTS=OFF', '-DENABLE_SOD=OFF', '-DENABLE_LITERT=OFF',
                '-DENABLE_GO2RTC=ON', f'-DGO2RTC_BINARY_PATH={go}', '-DENABLE_MQTT=OFF', '-DENABLE_SSL=ON',
                *args.cmake_arg]
            run(commands['configure_command'], log=output / 'configure.log')
            commands['build_command'] = ['cmake', '--build', str(build), '--target', 'lightnvr', '-j', str(args.jobs)]
            run(commands['build_command'], log=output / 'native-build.log')
            binary = build / 'bin/lightnvr'
            verify_checkout(checkout, build)
            mode = 'fresh-build'
        source = source_manifest(checkout)
        source_file = output / 'prior-source.json'
        write_json(source_file, source)
        execution_head = git(ROOT, 'rev-parse', 'HEAD')
        git(ROOT, 'merge-base', '--is-ancestor', REVISION, execution_head)
        tools = {'python': platform.python_version(), 'platform': platform.platform(),
                 **{tool: run(command).splitlines()[0] for tool, command in {
                     'cmake': ['cmake', '--version'], 'cc': ['cc', '--version'], 'go': ['go', 'version'],
                     'ffmpeg': ['ffmpeg', '-version']}.items()}}
        flags = cache_flags(binary)
        write_json(output / 'historical-go-buildinfo.txt.json', {'go_version_m': run(['go', 'version', '-m', go])})
        manifest = {'format_version': 1, 'prior': {'tag': TAG, 'revision': REVISION,
            'commit_date': git(checkout, 'show', '-s', '--format=%cI', 'HEAD'),
            'ancestor_of_execution_head': True, 'execution_head': execution_head,
            'checkout': str(checkout), 'binary': str(binary), 'binary_sha256': sha(binary),
            'go2rtc': str(go), 'go2rtc_sha256': sha(go), 'last_migration': '0089',
            'source_sha256': source['source_sha256'], 'source_manifest': str(source_file), 'submodules': PINS,
            'preparation_mode': mode, 'build_flags': flags, 'tools_at_preparation': tools,
            'build_environment': old.get('build_environment', {}) if args.reuse_checkout else {key: os.environ.get(key, '') for key in ('PKG_CONFIG_LIBDIR', 'PKG_CONFIG_PATH')},
            'preparation_environment': {key: os.environ.get(key, '') for key in ('PKG_CONFIG_LIBDIR', 'PKG_CONFIG_PATH')},
            'environment_note': 'Observed at preparation; reuse commands/flags describe the previously recorded build, not a new compilation.',
            **commands}, 'current': {'go2rtc': str(current_go), 'build_manifest': str(current_manifest),
                                    'last_migration': args.current_last_migration or max(file.name[:4] for file in (ROOT / 'db/migrations').glob('[0-9][0-9][0-9][0-9]_*.sql'))},
            'fixture': {'preserve_original': True, 'current_starts': 2,
                        'claim': 'Actual prior binary creates synthetic installation; forward upgrade only, no fabricated schema.'}}
        write_json(output / 'upgrade-manifest.json', manifest)
        write_json(output / 'preparation-status.json', {'status': 'passed', 'mode': mode,
            'prior_revision': REVISION, 'prior_binary_sha256': sha(binary), 'prior_go2rtc_sha256': sha(go),
            'prior_source_sha256': source['source_sha256']})
        print(json.dumps({'status': 'passed', 'manifest': str(output / 'upgrade-manifest.json'), 'mode': mode}))
    except Exception as error:
        (output / 'failure.txt').write_text(f'{type(error).__name__}: {error}\n')
        write_json(output / 'preparation-status.json', {'status': 'failed', 'failure_kind': type(error).__name__})
        print(f'Historical preparation failed; private diagnostics retained in {output}', file=sys.stderr)
        return 1
    return 0


def capture(args):
    spec = importlib.util.spec_from_file_location('reliability_source', ROOT / 'tests/reliability/interrupted-recording.py')
    fixture = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(fixture)
    require(not args.output.exists(), 'Current build evidence must use a new path')
    evidence = fixture.source_evidence(args.binary.resolve(), args.go2rtc.resolve())
    require(evidence['build_flags']['ENABLE_GO2RTC'] == 'ON', 'Managed build required')
    evidence['go2rtc_diff_sha256'] = hashlib.sha256(git(ROOT / 'go2rtc', 'diff', 'HEAD').encode()).hexdigest()
    write_json(args.output, evidence)
    print('Current build provenance captured')
    return 0


def validated_report(path, kind, expected_binary=None):
    report = json.loads(Path(path).read_text())
    require(report['status'] == 'passed', 'Qualification status is not passed')
    require('error' not in report and 'cleanup_error' not in report, 'Report contains a failure')
    if kind == 'storage':
        require(report['format_version'] == 1 and report['task'] == 'T26' and report['uid'] != 0,
                'Invalid or privileged storage result')
        modes = [case['mode'] for case in report['cases']]
        require(sorted(modes) == ['full', 'permission', 'readonly', 'unavailable'], 'Storage matrix incomplete')
        require(all(case['status'] == 'passed' for case in report['cases']), 'Storage case failed')
        boundary = report['self_tests']
        require(sorted(case['mode'] for case in boundary) == ['full', 'kernel-permission', 'none', 'readonly'],
                'Storage boundary self-tests missing')
        expected_errno = {'none': 0, 'full': 28, 'readonly': 30, 'kernel-permission': 13}
        require(all(case['result']['errno'] == expected_errno[case['mode']] for case in boundary),
                'Storage boundary self-test failed')
        source, binary = report['source']['sha256'], report['binary_sha256']
        count = len(report['cases'])
    else:
        expected = {'interrupted': 'interrupted-recording', 'lifecycle': 'reconnect-and-lifecycle', 'upgrade': 'historical-upgrade'}[kind]
        require(report['scenario'] == expected, 'Unexpected scenario')
        # Use the repository-pinned Ajv 2020 compiler, not a home-grown schema interpreter.
        javascript = """const fs=require('fs');const Ajv=require('ajv/dist/2020').default;
const validate=new Ajv({strict:false,allErrors:true}).compile(JSON.parse(fs.readFileSync(process.argv[1],'utf8')));
if(!validate(JSON.parse(fs.readFileSync(process.argv[2],'utf8'))))process.exit(1);"""
        run(['node', '-e', javascript, ROOT / 'tests/reliability/results-schema.json', Path(path).resolve()], timeout=30)
        source, binary = report['evidence']['source_sha256'], report['evidence']['binary_sha256']
        count = len(report['observations'])
    require(re.fullmatch('[0-9a-f]{64}', source) and re.fullmatch('[0-9a-f]{64}', binary), 'Invalid provenance digest')
    if expected_binary:
        require(binary == expected_binary, 'Report does not belong to the declared current build')
    # Allowlisted only: never echo API bodies, configs, identifiers, paths, free-form errors or media.
    summary = {'scenario': kind, 'status': 'passed', 'source_sha256': source,
               'binary_sha256': binary, 'case_or_observation_group_count': count}
    if kind == 'upgrade':
        prior = report['manifest']['prior']
        require(prior['revision'] == REVISION and prior['submodules'] == PINS, 'Historical report pins differ')
        hashes = {key: prior[key] for key in ('source_sha256', 'binary_sha256', 'go2rtc_sha256')}
        require(all(re.fullmatch('[0-9a-f]{64}', value) for value in hashes.values()), 'Invalid historical digest')
        summary['prior'] = {'revision': REVISION, 'submodules': PINS, **hashes}
    return summary


def summarize(args):
    directory = args.reports_root.resolve()
    try:
        current = json.loads(args.build_manifest.read_text())
        expected_binary = current['binary_sha256']
        require(re.fullmatch('[0-9a-f]{64}', expected_binary), 'Invalid current build digest')
    except Exception:
        expected_binary = None
    reports = {kind: directory / kind / 'result.json' for kind in ('interrupted', 'lifecycle', 'upgrade')}
    storage = list((directory / 'tmp').glob('lightnvr-storage-fault-*/result.json'))
    reports['storage'] = storage[0] if len(storage) == 1 else None
    results = []
    for kind, file in reports.items():
        try:
            require(file is not None, 'Missing or ambiguous storage report')
            require(expected_binary is not None, 'Missing valid current build evidence')
            results.append(validated_report(file, kind, expected_binary))
        except Exception:
            results.append({'scenario': kind, 'status': 'failed'})
    passed = all(item['status'] == 'passed' for item in results)
    write_json(args.output, {'status': 'passed' if passed else 'failed', 'reports': results,
                            'passed_count': sum(item['status'] == 'passed' for item in results), 'required_count': 4})
    print(json.dumps({'status': 'passed' if passed else 'failed', 'required_reports': 4}))
    return 0 if passed else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    p = commands.add_parser('prepare', help='Build pinned prior source, or explicitly verify an existing build')
    p.add_argument('--output', type=Path, required=True, help='New directory; parent must already exist')
    p.add_argument('--current-binary', type=Path, required=True)
    p.add_argument('--current-go2rtc', type=Path, required=True)
    p.add_argument('--current-build-manifest', type=Path, required=True)
    p.add_argument('--current-last-migration', help='Override compiled migration endpoint; default is latest current source migration')
    p.add_argument('--reuse-checkout', type=Path)
    p.add_argument('--reuse-manifest', type=Path)
    p.add_argument('--jobs', type=int, default=2)
    p.add_argument('--cmake-arg', action='append', default=[], help='Explicit host/toolchain override; use --cmake-arg=-D...')
    p.set_defaults(handler=prepare)
    p = commands.add_parser('capture-current', help='Record current managed build immediately after compilation')
    p.add_argument('--binary', type=Path, required=True); p.add_argument('--go2rtc', type=Path, required=True)
    p.add_argument('--output', type=Path, required=True); p.set_defaults(handler=capture)
    p = commands.add_parser('validate-report', help='Validate one actual report and emit only allowlisted fields')
    p.add_argument('--report', type=Path, required=True); p.add_argument('--kind', choices=['interrupted','storage','lifecycle','upgrade'], required=True)
    p.set_defaults(handler=lambda args: print(json.dumps(validated_report(args.report, args.kind))) or 0)
    p = commands.add_parser('summarize', help='Fail on missing/failed reports; write only safe status/provenance/counts')
    p.add_argument('--reports-root', type=Path, required=True); p.add_argument('--build-manifest', type=Path, required=True)
    p.add_argument('--output', type=Path, required=True); p.set_defaults(handler=summarize)
    args = parser.parse_args()
    if hasattr(args, 'jobs'): require(1 <= args.jobs <= 64, 'jobs must be 1..64')
    return args.handler(args)


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except (OSError, ValueError, KeyError, TypeError, IndexError, subprocess.SubprocessError):
        # Detailed diagnostics remain in private output; do not leak report values.
        print('Recovery preparation or evidence validation failed', file=sys.stderr)
        raise SystemExit(1)
