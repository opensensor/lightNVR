#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
"""Explicitly PARTIAL per-scanner findings policy; never a global coverage gate."""
import argparse
import copy
import datetime as dt
import json
from pathlib import Path
import findings


def evaluate_lane(*, reports, inventory, scanners, policy, scanner_ids, expected_revision, today=None):
    expected = {item['id'] for item in scanners.get('scanners', [])}
    selected = set(scanner_ids)
    if not selected or selected - expected:
        raise ValueError('Select one or more known scanner identities')
    if not expected_revision:
        raise ValueError('Lane checks require an explicit expected revision')
    projected_inventory = copy.deepcopy(inventory)
    for collection in ('findings', 'buckets'):
        projected_inventory[collection] = [item for item in inventory.get(collection, []) if item.get('scanner_id') in selected]
    projected_policy = copy.deepcopy(policy)
    projected_policy['require_run_manifest'] = True
    projected_policy['accepted_gaps'] = [item for item in policy.get('accepted_gaps', []) if item.get('scanner') in selected]
    projected_scanners = dict(scanners, scanners=[item for item in scanners['scanners'] if item['id'] in selected])
    result = findings.evaluate(reports, projected_inventory, projected_scanners, projected_policy,
                               today or dt.date.today(), expected_revision=expected_revision)
    return {'schema_version': 1, 'status': 'partial-' + result['status'], 'global_verdict': None,
            'selected_scanners': sorted(selected), 'omitted_scanners': sorted(expected-selected),
            'result': {k: v for k, v in result.items() if not k.startswith('_')}}


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--reports', required=True)
    parser.add_argument('--scanner', action='append', required=True)
    parser.add_argument('--expected-revision', required=True)
    parser.add_argument('--inventory', default='quality/findings.json')
    parser.add_argument('--scanners', default='quality/scanners.json')
    parser.add_argument('--policy', default='quality/findings-policy.json')
    parser.add_argument('--json-out')
    args = parser.parse_args(argv)
    try:
        result = evaluate_lane(reports=Path(args.reports), scanner_ids=args.scanner,
            expected_revision=args.expected_revision,
            **{key: json.loads(Path(getattr(args,key)).read_text()) for key in ('inventory','scanners','policy')})
        print(f'PARTIAL LANE: {result["status"]}; GLOBAL COVERAGE NOT EVALUATED')
        print('Selected: ' + ', '.join(result['selected_scanners']))
        print('Omitted expected scanners: ' + (', '.join(result['omitted_scanners']) or '(none; this command still does not issue a global verdict)'))
        for failure in result['result']['failures']:
            print(f'{failure["kind"]}: {failure["detail"]}')
        if args.json_out:
            output = Path(args.json_out)
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_text(json.dumps(result,indent=2)+'\n')
        return 0 if result['status'] == 'partial-pass' else 1
    except (OSError, ValueError) as error:
        print(f'PARTIAL LANE ERROR: {error}; no global verdict')
        return 2


if __name__ == '__main__':
    raise SystemExit(main())
