#!/usr/bin/env python3
"""Prepare a release, wait for generated Rust and CI, then publish one tag."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent.parent
# These workflows run on every main push (Rust also runs for version/rust changes).
REQUIRED_WORKFLOWS = {
    'lightnvr-rs.yml', 'integration-test.yml', 'sanitizer.yml',
    'static-analysis.yml', 'codeql-analysis.yml', 'container-scan.yml',
}
FRESHNESS_STEP = 'Verify checked-in Rust is current'


class ReleaseError(Exception):
    pass


def command(*args, capture=True, **kwargs):
    result = subprocess.run(args, cwd=ROOT, text=True, check=True,
                            stdout=subprocess.PIPE if capture else None, **kwargs)
    return result.stdout.strip() if capture else None


def git(*args):
    return command('git', *args)


def api(repo, endpoint, **params):
    args = ['gh', 'api', '-X', 'GET', '--paginate', '--slurp',
            f'repos/{repo}/{endpoint}']
    for key, value in params.items():
        args.extend(['-f', f'{key}={value}'])
    return json.loads(command(*args))


def versions(ref):
    cmake = git('show', f'{ref}:CMakeLists.txt')
    match = re.search(r'project\(\s*LightNVR\s+VERSION\s+([0-9.]+)', cmake)
    if not match:
        raise ReleaseError(f'Cannot read the CMake version at {ref}.')
    web = json.loads(git('show', f'{ref}:web/package.json'))['version']
    return match.group(1), web


def check_versions(ref, version, rust=False):
    if versions(ref) != (version, version):
        raise ReleaseError(f'CMake/web versions at {ref} do not both match {version}.')
    if rust:
        cargo = git('show', f'{ref}:rust/Cargo.toml')
        match = re.search(r'^version\s*=\s*"([^"]+)"', cargo, re.MULTILINE)
        if not match or match.group(1) != version:
            raise ReleaseError('Generated Rust version is stale; do not tag this commit.')


def fetch_main():
    git('fetch', '--no-tags', 'origin',
        '+refs/heads/main:refs/remotes/origin/main')
    return git('rev-parse', 'refs/remotes/origin/main')


def check_descendants(anchor, head):
    """Follow only generated changes, never silently include new product changes."""
    try:
        git('merge-base', '--is-ancestor', anchor, head)
    except subprocess.CalledProcessError as exc:
        raise ReleaseError('main no longer descends from the prepared release.') from exc
    for commit in git('rev-list', f'{anchor}..{head}').splitlines():
        parents = git('rev-list', '--parents', '-n', '1', commit).split()
        paths = git('diff-tree', '--no-commit-id', '--name-only', '-r', commit).splitlines()
        if len(parents) != 2 or not paths or any(not p.startswith('rust/') for p in paths):
            raise ReleaseError(
                f'main changed outside generated rust/ at {commit[:12]}. '
                'Review and pull those changes, then rerun with --publish-only.')


def ci_state(runs, sha):
    """Use the latest push run of each workflow for this exact main commit."""
    latest = {}
    for run in runs:
        if (run['head_sha'] != sha or run['event'] != 'push'
                or run['head_branch'] != 'main'):
            continue
        workflow = run['path'].split('@', 1)[0].rsplit('/', 1)[-1]
        if workflow not in latest or run['id'] > latest[workflow]['id']:
            latest[workflow] = run
    waiting = sorted(REQUIRED_WORKFLOWS - latest.keys())
    failed = []
    for workflow, run in sorted(latest.items()):
        if run['status'] != 'completed':
            waiting.append(f"{workflow} ({run['status']})")
        elif run['conclusion'] != 'success':
            failed.append(f"{workflow}: {run['conclusion']} — {run['html_url']}")
    return waiting, failed


def wait_for_ci(repo, anchor, version, timeout, interval=30, follow_main=True):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        head = fetch_main() if follow_main else anchor
        if follow_main:
            check_descendants(anchor, head)
        check_versions(head, version)
        pages = api(repo, 'actions/runs', head_sha=head, branch='main',
                    event='push', per_page=100)
        runs = [run for page in pages for run in page['workflow_runs']]
        waiting, failed = ci_state(runs, head)
        # Regeneration may have pushed a new head (and cancelled the old runs)
        # while we queried CI. Always switch to that commit before deciding.
        if follow_main and fetch_main() != head:
            continue
        if failed:
            raise ReleaseError('CI did not pass:\n' + '\n'.join(failed)
                               + '\nFix/rerun CI, then resume with --publish-only.')
        if not waiting:
            rust_run = max((run for run in runs
                            if run['head_sha'] == head and run['event'] == 'push'
                            and run['head_branch'] == 'main'
                            and run['path'] == '.github/workflows/lightnvr-rs.yml'),
                           key=lambda run: run['id'])
            jobs = api(repo, f"actions/runs/{rust_run['id']}/jobs", per_page=100)
            fresh = any(step['name'] == FRESHNESS_STEP and step['conclusion'] == 'success'
                        for page in jobs for job in page['jobs']
                        for step in job.get('steps', []))
            if follow_main and fetch_main() != head:
                continue
            if not fresh:
                raise ReleaseError('Rust CI did not verify the checked-in Rust as current. '
                                   'Check the regeneration push before resuming; no tag created.')
            check_versions(head, version, rust=True)
            return head
        print(f"Waiting on {head[:12]}: {', '.join(waiting)}", flush=True)
        time.sleep(min(interval, max(0, deadline - time.monotonic())))
    raise ReleaseError('Timed out waiting for CI. No tag was created. '
                       'Resume with --publish-only after CI completes.')


def check_tag_absent(version):
    # Accept existing historical naming, but never release the same version twice.
    for tag in (version, f'v{version}'):
        if git('tag', '--list', tag) or git('ls-remote', '--tags', 'origin', f'refs/tags/{tag}'):
            raise ReleaseError(f'Tag {tag} already exists; no release changes made.')


def publish(repo, anchor, version, timeout):
    head = wait_for_ci(repo, anchor, version, timeout)
    check_tag_absent(version)
    if fetch_main() != head:
        raise ReleaseError('main moved before tagging. Resume with --publish-only.')
    if git('rev-parse', 'HEAD') != anchor or git('status', '--porcelain'):
        raise ReleaseError('Local checkout changed while waiting; no tag was created.')
    git('merge', '--ff-only', head)
    tag = f'v{version}'
    git('tag', '-a', tag, head, '-m', f'Release {tag}')
    try:
        # Push only this tag. The main lease also rejects a concurrent branch
        # update instead of overwriting it. Never force an existing release tag.
        command('git', 'push', '--atomic', f'--force-with-lease=refs/heads/main:{head}',
                'origin', f'{head}:refs/heads/main', f'refs/tags/{tag}:refs/tags/{tag}',
                capture=False)
    except subprocess.CalledProcessError:
        # Preserve the local tag: a transport error can occur after remote success.
        raise ReleaseError(f'Push failed or its outcome is uncertain. Inspect remote {tag} '
                           'before retrying; the local tag was retained.') from None
    print(f'Published {tag} at {head}. Release builds are starting, not yet complete.')
    print(f'https://github.com/{repo}/actions')


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('version', help='MAJOR.MINOR.PATCH')
    modes = parser.add_mutually_exclusive_group()
    modes.add_argument('--prepare-only', action='store_true',
                       help='bump, commit and push; do not wait or tag')
    modes.add_argument('--publish-only', action='store_true',
                       help='resume a prepared version; wait for CI and tag')
    modes.add_argument('--no-push', action='store_true',
                       help='bump and commit locally; do not push or create a tag')
    modes.add_argument('--verify-tag', metavar='TAG',
                       help='CI: verify an existing tag and wait for its exact commit; never write')
    parser.add_argument('--yes', action='store_true', help='skip the confirmation prompt')
    parser.add_argument('--timeout', type=int, default=14400,
                        help='maximum CI wait in seconds (default: 14400)')
    args = parser.parse_args(argv)
    if not re.fullmatch(r'[0-9]+\.[0-9]+\.[0-9]+', args.version) or args.timeout <= 0:
        raise ReleaseError('Use a MAJOR.MINOR.PATCH version and a positive timeout.')
    if args.verify_tag:
        if args.verify_tag not in (args.version, f'v{args.version}'):
            raise ReleaseError('Tag name does not match the requested version.')
        head = git('rev-parse', 'HEAD')
        if git('rev-parse', '--verify', f'refs/tags/{args.verify_tag}^{{commit}}') != head:
            raise ReleaseError('Checkout does not match the release tag.')
        check_versions(head, args.version, rust=True)
        repo = json.loads(command('gh', 'repo', 'view', git('remote', 'get-url', 'origin'),
                                  '--json', 'nameWithOwner'))['nameWithOwner']
        wait_for_ci(repo, head, args.version, args.timeout, follow_main=False)
        print(f'{args.verify_tag} passed release preflight at {head}.')
        return
    if git('branch', '--show-current') != 'main':
        raise ReleaseError('Releases must be prepared from main.')
    if git('status', '--porcelain'):
        raise ReleaseError('Working directory is not clean. Commit or stash changes first.')
    check_tag_absent(args.version)
    # gh resolves the repository explicitly from origin, not a configured fork default.
    repo = None
    if not args.no_push:
        origin = git('remote', 'get-url', 'origin')
        repo = json.loads(command('gh', 'repo', 'view', origin,
                                  '--json', 'nameWithOwner'))['nameWithOwner']
        command('gh', 'api', f'repos/{repo}/actions/workflows/lightnvr-rs.yml')
    remote = fetch_main()
    # Do not silently pull new product changes into a release the operator reviewed.
    try:
        git('merge-base', '--is-ancestor', remote, 'HEAD')
    except subprocess.CalledProcessError:
        # An interrupted preparation may already have its generated follow-up.
        # Only those changes are safe to pick up without a new review.
        check_descendants(git('rev-parse', 'HEAD'), remote)
        git('merge', '--ff-only', remote)
    current, web = versions('HEAD')
    if current != web:
        raise ReleaseError('CMake and web versions disagree; fix them before releasing.')
    if args.publish_only and current != args.version:
        raise ReleaseError('--publish-only requires the prepared version in this checkout.')
    if tuple(map(int, args.version.split('.'))) < tuple(map(int, current.split('.'))):
        raise ReleaseError('Refusing to lower the release version.')
    print(f'Release v{args.version}: prepare main, wait for Rust regeneration and CI, then tag.')
    if not args.yes and input('Proceed? [y/N] ').strip().lower() != 'y':
        print('Release cancelled.')
        return
    if current != args.version:
        command('bash', str(ROOT / 'scripts/bump-version.sh'), args.version,
                input='y\n', capture=False)
        git('add', '--', 'CMakeLists.txt', 'web/package.json')
        command('git', 'diff', '--cached', '--', 'CMakeLists.txt', 'web/package.json', capture=False)
        git('commit', '-m', f'Bump version to {args.version}')
    anchor = git('rev-parse', 'HEAD')
    if args.no_push:
        print(f'Prepared locally; no tag created. Resume: scripts/release.sh {args.version} --publish-only')
        return
    command('git', 'push', 'origin', f'{anchor}:refs/heads/main', capture=False)
    if args.prepare_only:
        print(f'Preparation pushed; no tag created. Resume: scripts/release.sh {args.version} --publish-only')
        return
    publish(repo, anchor, args.version, args.timeout)


if __name__ == '__main__':
    try:
        main()
    except (ReleaseError, subprocess.CalledProcessError, OSError) as error:
        sys.exit(f'Release stopped: {error}')
    except KeyboardInterrupt:
        sys.exit('Release interrupted. Resume the prepared version with --publish-only.')
