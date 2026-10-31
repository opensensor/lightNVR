#!/usr/bin/env python3
"""Build and run named native tests with isolated outputs and a shared runtime lease."""
import argparse
from contextlib import contextmanager
import json
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys
import uuid

ROOT = Path(__file__).resolve().parents[2]
LOCK = ROOT / 'build/eng-artifacts/.runtime-lock'


def validate_task(task):
    if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_.-]{0,79}', task) or '..' in task:
        raise ValueError('task must be a simple task ID (for example T31.3)')
    return task


@contextmanager
def runtime_lock(lock=LOCK):
    lock.parent.mkdir(parents=True, exist_ok=True)
    lock.mkdir()  # Never break a lock or reuse another run's directory.
    token = str(uuid.uuid4())
    (lock / 'owner').write_text(token)
    try:
        yield
    finally:
        if (lock / 'owner').read_text() != token:
            raise RuntimeError('Runtime lock ownership changed; refusing removal')
        (lock / 'owner').unlink()
        lock.rmdir()


def build_env(source=None):
    env = dict(os.environ if source is None else source)
    for key in list(env):
        if key.startswith('LIGHTNVR_') or key in {'CFLAGS', 'CXXFLAGS', 'CPPFLAGS', 'LDFLAGS', 'CPATH', 'C_INCLUDE_PATH', 'CPLUS_INCLUDE_PATH', 'LIBRARY_PATH', 'LD_LIBRARY_PATH', 'PKG_CONFIG_SYSROOT_DIR'}:
            env.pop(key, None)
    arch = subprocess.check_output(['/usr/bin/cc', '-print-multiarch'], text=True).strip()
    env['PKG_CONFIG_PATH'] = ''
    env['PKG_CONFIG_LIBDIR'] = f'/usr/lib/{arch}/pkgconfig:/usr/lib/pkgconfig:/usr/share/pkgconfig'
    return env


def run_command(command, log, env=None):
    print('+ ' + shlex.join(str(x) for x in command), flush=True)
    with Path(log).open('w') as output:
        result = subprocess.run(command, cwd=ROOT, env=env, stdout=output, stderr=subprocess.STDOUT)
    if result.returncode:
        print(Path(log).read_text(errors='replace')[-12000:], file=sys.stderr)
        raise subprocess.CalledProcessError(result.returncode, command)


def test_names(selection):
    tests = selection.get('tests')
    if not isinstance(tests, list) or not tests:
        raise ValueError('No native tests selected; refusing an empty success')
    names = []
    for test in tests:
        name = test.get('name') if isinstance(test, dict) else None
        if not isinstance(name, str) or not re.fullmatch(r'[A-Za-z0-9_-]+', name):
            raise ValueError('Invalid native test name')
        names.append(name)
    return names


def verify_ffmpeg(env, artifacts):
    packages = ['libavcodec', 'libavformat', 'libavutil', 'libswscale', 'libswresample', 'libavfilter']
    includes = sorted(set(subprocess.check_output(['pkg-config', '--variable=includedir', p], env=env, text=True).strip() for p in packages))
    if any(not p.startswith('/usr/include') for p in includes):
        raise ValueError('System FFmpeg include paths are required; refusing /usr/local ABI ambiguity')
    flags = sum((['-isystem', p] for p in includes), [])
    checks = [('avcodec', 'libavcodec/avcodec.h', 'LIBAVCODEC_VERSION_MAJOR'), ('avformat', 'libavformat/avformat.h', 'LIBAVFORMAT_VERSION_MAJOR'), ('avutil', 'libavutil/avutil.h', 'LIBAVUTIL_VERSION_MAJOR'), ('swscale', 'libswscale/swscale.h', 'LIBSWSCALE_VERSION_MAJOR'), ('swresample', 'libswresample/swresample.h', 'LIBSWRESAMPLE_VERSION_MAJOR'), ('avfilter', 'libavfilter/avfilter.h', 'LIBAVFILTER_VERSION_MAJOR')]
    code = '#include <stdio.h>\n' + ''.join(f'#include <{header}>\n' for _, header, _ in checks) + 'int main(void) { int failed=0;\n'
    for name, _, macro in checks:
        code += f'printf("{name} header=%u runtime=%u\\n", (unsigned){macro}, {name}_version() >> 16); failed |= ({macro} != ({name}_version() >> 16));\n'
    code += 'return failed;}\n'
    source = artifacts / 'ffmpeg-probe.c'
    source.write_text(code)
    pkgflags = shlex.split(subprocess.check_output(['pkg-config', '--cflags', '--libs', *packages], env=env, text=True))
    probe = artifacts / 'ffmpeg-probe'
    run_command(['/usr/bin/cc', *flags, str(source), '-o', str(probe), *pkgflags], artifacts / 'ffmpeg-probe-build.log', env)
    run_command([str(probe)], artifacts / 'ffmpeg-probe.log', env)
    return shlex.join(flags)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--task', required=True)
    parser.add_argument('--regex')
    parser.add_argument('--profile', choices=['lean', 'integration', 'full'], default='lean')
    parser.add_argument('--sanitizer', choices=['none', 'asan-ubsan'], default='asan-ubsan')
    parser.add_argument('--build-only', action='store_true', help='Build the server, explicitly without claiming test execution')
    parser.add_argument('--jobs', type=int, default=2)
    args = parser.parse_args(argv)
    validate_task(args.task)
    if args.jobs < 1 or (not args.regex and not args.build_only):
        parser.error('--regex is required unless --build-only; jobs must be positive')
    artifacts = ROOT / 'build/eng-artifacts' / args.task / ('native-' + uuid.uuid4().hex[:12])
    artifacts.mkdir(parents=True)
    build = ROOT / 'build/eng-native' / args.task / f'{args.profile}-{args.sanitizer}'
    env = build_env()
    with runtime_lock():
        flags = verify_ffmpeg(env, artifacts)
        full = args.profile == 'full'
        go2rtc = args.profile != 'lean'
        configure = ['cmake', '-S', str(ROOT), '-B', str(build), '-DBUILD_TESTS=ON', '-DCMAKE_C_COMPILER=/usr/bin/cc', '-DCMAKE_CXX_COMPILER=/usr/bin/c++', '-DCMAKE_BUILD_TYPE=' + ('Debug' if args.sanitizer == 'asan-ubsan' else 'Release'), '-DCMAKE_C_FLAGS=' + flags, '-DCMAKE_CXX_FLAGS=' + flags]
        for feature, enabled in [('SOD', full), ('LITERT', full), ('GO2RTC', go2rtc), ('MQTT', full)]:
            configure.append(f'-DENABLE_{feature}={"ON" if enabled else "OFF"}')
        configure.append('-DSOD_DYNAMIC_LINK=' + ('ON' if full else 'OFF'))
        run_command(configure, artifacts / 'configure.log', env)
        if args.build_only:
            targets = ['lightnvr']
        else:
            selection = subprocess.check_output(['ctest', '--test-dir', str(build), '--show-only=json-v1', '-R', args.regex], env=env, text=True)
            (artifacts / 'selection.json').write_text(selection)
            targets = test_names(json.loads(selection))
        run_command(['cmake', '--build', str(build), '--parallel', str(args.jobs), '--target', *targets], artifacts / 'build.log', env)
        env['LD_LIBRARY_PATH'] = str(build / 'bin')
        metadata = {'task': args.task, 'revision': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(), 'profile': args.profile, 'sanitizer': args.sanitizer, 'build_directory': str(build), 'binary': str(build / 'bin/lightnvr') if args.build_only else None, 'artifacts': str(artifacts), 'test_names': [] if args.build_only else targets, 'tests_executed': not args.build_only, 'configure': configure}
        (artifacts / 'run.json').write_text(json.dumps(metadata, indent=2) + '\n')
        if not args.build_only:
            run_command(['ctest', '--test-dir', str(build), '--output-on-failure', '--no-tests=error', '-R', args.regex, '--output-junit', str(artifacts / 'ctest.xml')], artifacts / 'ctest.log', env)
        print(json.dumps(metadata), flush=True)
    return 0


if __name__ == '__main__':
    try:
        sys.exit(main())
    except subprocess.CalledProcessError as exc:
        sys.exit(exc.returncode if 0 < exc.returncode < 126 else 1)
    except (ValueError, OSError, RuntimeError) as exc:
        print(f'Runner error: {exc}', file=sys.stderr)
        sys.exit(2)
