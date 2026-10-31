#!/usr/bin/env python3
"""Static validation of the T7 expansion; does not approve or execute tasks."""
import argparse
import collections
import hashlib
import json
import pathlib
import re

parser = argparse.ArgumentParser()
parser.add_argument('--map', default='quality/remediation-map.json')
parser.add_argument('--plan', default='engineering-foundation-plan.md')
parser.add_argument('--require-integrated', action='store_true')
args = parser.parse_args()
data = json.loads(pathlib.Path(args.map).read_text())
invpath = pathlib.Path('quality/findings.json')
inventory = json.loads(invpath.read_text())
assert data['inventory_sha256'] == hashlib.sha256(invpath.read_bytes()).hexdigest(), 'inventory changed; reconcile map'
items = inventory['findings'] + inventory['buckets']
expected = {item['id'] for item in items}
mapped = collections.Counter(row['finding_id'] for row in data['finding_map'])
assert all(mapped[fid] == 1 for fid in expected), 'missing/duplicated inventory finding'
assert set(mapped) - expected == {f'F-T11-{i:02}' for i in range(1, 18)}, 'candidate coverage mismatch'
assert all(n == 1 for n in mapped.values()), 'duplicate finding mappings'
assert not any(row['approved_exception'] for row in data['finding_map']), 'planning cannot approve an exception'
tasks = {task['id']: task for task in data['tasks']}
assert len(tasks) == len(data['tasks']), 'duplicate task ID'
assert set(tasks) == {f'T7.{i}' for i in range(1, len(tasks) + 1)}, 'unstable task numbering'
text = pathlib.Path(args.plan).read_text()
records = re.findall(r'^### (T[\d.]+):[^\n]*\n- \*\*depends_on\*\*: \[([^\]]*)\]', text, re.M)
assert len(records) == len({r[0] for r in records}), 'duplicate plan IDs'
graph = {name: [v.strip() for v in deps.split(',') if v.strip()] for name, deps in records}
integrated = any(name.startswith('T7.') for name in graph)
if args.require_integrated:
    assert integrated, 'remediation records not inserted into plan'
if integrated:
    assert set(tasks).issubset(graph), 'partially inserted remediation expansion'
    for name, task in tasks.items():
        assert set(graph[name]) == set(task['depends_on']), ('plan/map dependency mismatch', name)
    for name, dependencies in data['gate_dependency_additions'].items():
        assert set(dependencies).issubset(graph[name]), ('gate additions not inserted', name)
for task in tasks.values():
    graph[task['id']] = task['depends_on'][:]
    assert len(task['source_paths']) <= 6, (task['id'], 'implementation cluster too large')
    assert task['validation'] and task['acceptance_criteria'], (task['id'], 'empty contract')
    assert task['evidence'] in task['ownership'], (task['id'], 'evidence lacks owner')
    assert not any('**' in p or p in ('src/', 'web/') for p in task['ownership']), 'broad ownership'
    for path in task['source_paths']:
        assert pathlib.Path(path).exists() or path.startswith('/usr/'), (task['id'], 'unknown source', path)
    for fid in task['finding_ids']:
        assert any(r['finding_id'] == fid and r['task_id'] == task['id'] for r in data['finding_map'])
for name, deps in data['gate_dependency_additions'].items():
    assert name in graph, ('missing gate', name)
    graph[name] = list(dict.fromkeys(graph[name] + deps))
active = []
ancestors = {}
def visit(name):
    assert name in graph, ('unknown dependency', name)
    assert name not in active, ('cycle', active + [name])
    if name in ancestors:
        return ancestors[name]
    active.append(name)
    parents = set(graph[name])
    for dependency in graph[name]:
        parents.update(visit(dependency))
    active.pop()
    ancestors[name] = parents
    return parents
for name in graph:
    visit(name)
# All same-file remediation writers must be ordered; source references are read-only.
writers = collections.defaultdict(list)
for task in tasks.values():
    for path in task['ownership']:
        writers[path].append(task['id'])
for path, owners in writers.items():
    for i, left in enumerate(owners):
        for right in owners[i + 1:]:
            assert left in ancestors[right] or right in ancestors[left], ('unordered writers', path, left, right)
assert set(tasks).issubset(ancestors['T33']), 'closure gate misses remediation task'
assert data['shared_resource_contracts']['native_test_registration'], 'missing registration producer'
for row in data['finding_map']:
    assert row['task_id'] in graph, ('unknown mapped task', row)
assert {g['id'] for g in data['coverage_gaps']} == {g['id'] for g in inventory['coverage_gaps']}
assert all(not gap['approved_exception'] for gap in data['coverage_gaps'])
assert set(data['discrepancy_map']) == {d['id'] for d in json.load(open('quality/api-inventory.json'))['discrepancies']}
for targets in data['discrepancy_map'].values():
    assert all(target in graph for target in targets)
print(f'PASS: {len(expected)} inventory findings, 17 candidates, {len(tasks)} remediation tasks, '
      f'{len(graph)} acyclic graph nodes; coverage, ownership ordering, source paths and T33 fence verified')
