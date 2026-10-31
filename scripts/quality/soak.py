#!/usr/bin/env python3
"""Bounded host composition. Exit 0 qualified, 1 functional failure, 2 unqualified.

Each existing scenario acquires its own runtime lock. This supervisor never
acquires/nests/breaks that lock and never signals a process by name or bare PID.
"""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import re
import select
import shutil
import signal
import statistics
import subprocess
import sys
import time
import uuid

ROOT = Path(__file__).resolve().parents[2]
LOCK = ROOT / 'build/eng-artifacts/.runtime-lock'
SCENARIOS = {'media-video', 'media-aac', 'lifecycle'}
GROWTH = {'rss_bytes', 'fds', 'threads'}


def finite(value):
    return isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value)


def validate_profile(profile):
    if not isinstance(profile, dict): raise ValueError('Profile must be an object')
    for key, low, high in [('repetitions', 1, 10), ('sample_interval_seconds', .05, 10), ('scenario_timeout_seconds', 1, 1800)]:
        value = profile.get(key)
        if not finite(value) or not low <= value <= high: raise ValueError('Invalid ' + key)
    if not isinstance(profile['repetitions'], int): raise ValueError('Integer repetitions required')
    cases = profile.get('scenarios')
    if not isinstance(cases, list) or not cases or any(c not in SCENARIOS for c in cases) or len(set(cases)) != len(cases):
        raise ValueError('Invalid scenarios')
    limits = profile.get('growth_limits')
    if not isinstance(limits, dict) or not limits or set(limits) - GROWTH: raise ValueError('Invalid growth limits')
    for value in limits.values():
        if value is not None and (not finite(value) or value < 0): raise ValueError('Invalid growth limit')
    return profile


def qualify(budgets, observations):
    rows = []
    for metric in budgets:
        value, limit = observations.get(metric['id']), metric.get('limit')
        status = 'unqualified' if not finite(limit) else 'unavailable' if not finite(value) else 'passed' if value <= limit else 'failed'
        rows.append({'id': metric['id'], 'observed': value, 'limit': limit, 'status': status})
    return {'passed': bool(rows) and all(r['status'] == 'passed' for r in rows), 'metrics': rows}


def distribution(values):
    values = sorted(v for v in values if finite(v))
    return None if not values else {'samples':len(values), 'min':values[0], 'median':statistics.median(values),
        'p95':values[max(0, math.ceil(len(values)*.95)-1)], 'max':values[-1], 'last_minus_first': None}


def resource_summary(samples, limits):
    rows, failures = {}, []
    for name, limit in limits.items():
        values = [s['nvr'][name] for s in samples if isinstance(s.get('nvr'),dict) and finite(s['nvr'].get(name))]
        observed = max(0, values[-1]-values[0]) if len(values) >= 2 else None
        status = 'unqualified' if limit is None else 'unavailable' if observed is None else 'failed' if observed > limit else 'passed'
        rows[name] = {'first':values[0] if values else None, 'last':values[-1] if values else None, 'growth':observed,'limit':limit,'status':status}
        if status == 'failed': failures.append(name + ' growth exceeded synthetic/supplied limit')
    return {'passed': bool(rows) and all(r['status']=='passed' for r in rows.values()), 'metrics':rows, 'failures':failures,
            'method':'Last minus first positive growth within this mixed-phase run; not a leak proof or comparable-phase calibration'}


def read_identity(pid):
    try:
        base=Path('/proc')/str(pid); fields=(base/'stat').read_text().rsplit(') ',1)[1].split()
        if fields[0]=='Z': return None
        return {'pid':pid,'parent':int(fields[1]),'start':fields[19], 'binary':os.path.realpath(base/'exe'),
                'args':(base/'cmdline').read_bytes().decode(errors='replace').split('\0')}
    except (OSError, ValueError, IndexError): return None


def same_identity(left, right):
    return bool(left and right and all(left.get(k)==right.get(k) for k in ['pid','start','binary','args']))


def read_metrics(pid):
    base=Path('/proc')/str(pid); fields=(base/'stat').read_text().rsplit(') ',1)[1].split()
    status={row.split(':',1)[0]:row.split(':',1)[1].strip() for row in (base/'status').read_text().splitlines() if ':' in row}
    return {'rss_bytes':int(status['VmRSS'].split()[0])*1024, 'threads':int(status['Threads']),
            'fds':len(list((base/'fd').iterdir())), 'cpu_ticks':int(fields[11])+int(fields[12])}


def sample_process(saved):
    before=read_identity(saved['pid'])
    if before is None: return {'status':'exited','identity':saved}
    if not same_identity(before,saved): return {'status':'identity_changed','identity':saved}
    try: metrics=read_metrics(saved['pid'])
    except (OSError, ValueError, KeyError, IndexError): return {'status':'unavailable','identity':saved}
    after=read_identity(saved['pid'])
    if not same_identity(after,saved): return {'status':'identity_changed' if after else 'exited','identity':saved}
    return {'status':'observed','identity':saved,**metrics}


def process_table():
    return {int(p.name):i for p in Path('/proc').iterdir() if p.name.isdigit() and (i:=read_identity(int(p.name))) is not None}


def owned_snapshot(root):
    table=process_table()
    if not same_identity(table.get(root['pid']), root): return {}
    owned={root['pid']:table[root['pid']]}
    while True:
        extra={pid:item for pid,item in table.items() if pid not in owned and item['parent'] in owned}
        if not extra:return owned
        owned.update(extra)


def role(item, parent_role, binary):
    name=Path(item['binary']).name.lower();args=' '.join(item['args'])
    if item['binary']==binary:return 'nvr'
    if parent_role in ['source','browser']:return parent_role
    if 'ffmpeg' in name and any(x in args for x in ['lavfi','testsrc','virtual?']):return 'source'
    if any(x in name for x in ['chrome','chromium','crashpad']):return 'browser'
    if parent_role in ['nvr','managed']:return 'managed'
    if 'go2rtc' in name or 'ffmpeg' in name:return 'source'
    return 'helper'


def sanitizer_findings(text):
    return re.findall(r'(?im)^.*(?:ERROR: AddressSanitizer|AddressSanitizer:DEADLYSIGNAL|LeakSanitizer:|WARNING: ThreadSanitizer|runtime error:).*$' , text)


def pin(saved):
    try: fd=os.pidfd_open(saved['pid'])
    except ProcessLookupError:return None
    if not same_identity(read_identity(saved['pid']),saved):os.close(fd);return None
    return fd


def stop_pinned(handles):
    reports=[]
    for saved,fd in reversed(list(handles.values())):
        if select.select([fd],[],[],0)[0]:continue
        try:signal.pidfd_send_signal(fd,signal.SIGTERM)
        except ProcessLookupError:continue
        reports.append({'identity':saved,'action':'SIGTERM'})
    deadline=time.monotonic()+3
    while time.monotonic()<deadline and any(not select.select([fd],[],[],0)[0] for _,fd in handles.values()):time.sleep(.05)
    for saved,fd in handles.values():
        if not select.select([fd],[],[],0)[0]:
            try:signal.pidfd_send_signal(fd,signal.SIGKILL);reports.append({'identity':saved,'action':'SIGKILL'})
            except ProcessLookupError:pass
    return reports


def run_child(command, directory, binary, interval, timeout):
    directory.mkdir(parents=True,exist_ok=True); samples=[]; failures=[]; handles={}; previous={}; unavailable=[]
    started=time.monotonic();timed_out=False;cleanup=[];last_observed={}
    with (directory/'runner.log').open('w') as log:
        process=subprocess.Popen(command,cwd=ROOT,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
        root=read_identity(process.pid)
        if root is None:
            process.wait();return {'returncode':process.returncode,'passed':False,'failures':['Runner exited before identity capture'],'samples':[],'timed_out':False}
        try:
            while process.poll() is None:
                now=time.monotonic();snapshot=owned_snapshot(root);roles={};observed=[]
                pending=dict(snapshot)
                while pending:
                    progress=False
                    for pid,item in list(pending.items()):
                        if item['parent'] in pending:continue
                        assigned=role(item,roles.get(item['parent']),binary);roles[pid]=assigned;pending.pop(pid);progress=True
                        key=(pid,item['start'])
                        if key not in handles:
                            fd=pin(item)
                            if fd is not None:handles[key]=(item,fd)
                        point=sample_process(item);point['role']=assigned
                        if point['status']=='observed':
                            old=previous.get(key)
                            point['cpu_one_core_percent']=(point['cpu_ticks']-old[1])/os.sysconf('SC_CLK_TCK')/(now-old[0])*100 if old and now>old[0] else None
                            previous[key]=(now,point['cpu_ticks'])
                        elif assigned!='helper':unavailable.append({'time':now-started,**point})
                        observed.append(point)
                    if not progress:failures.append('Cyclic or incomplete process ancestry');break
                present={(p['identity']['pid'],p['identity']['start']) for p in observed}
                for key,old in last_observed.items():
                    if key not in present and old['role']!='helper':
                        unavailable.append({'time':now-started,'identity':old['identity'],'role':old['role'],
                            'status':'exited_or_not_in_owned_tree','metrics':None})
                last_observed={(p['identity']['pid'],p['identity']['start']):p for p in observed}
                point={'elapsed_seconds':now-started,'processes':observed}
                for group,member in [('nvr',{'nvr','managed'}),('source',{'source'}),('browser',{'browser'})]:
                    selected=[p for p in observed if p['role'] in member]
                    values=[p for p in selected if p['status']=='observed']
                    point[group]=({k:sum(p[k] for p in values) if all(finite(p.get(k)) for p in values) else None
                        for k in ['rss_bytes','fds','threads','cpu_one_core_percent']} if values and len(values)==len(selected) else None)
                samples.append(point)
                if now-started>timeout:timed_out=True;failures.append('Scenario supervisor deadline exceeded');break
                time.sleep(interval)
            if timed_out:cleanup=stop_pinned(handles)
            process.wait(timeout=5)
            # A successful child must not leave previously observed owned descendants.
            survivors=[(item,fd) for item,fd in handles.values() if not select.select([fd],[],[],0)[0]]
            if survivors:failures.append('Runner left owned descendants');cleanup+=stop_pinned(handles)
        finally:
            if process.poll() is None:cleanup+=stop_pinned(handles);process.wait(timeout=5)
            for _,fd in handles.values():os.close(fd)
    if process.returncode:failures.append('Scenario child failed: '+str(process.returncode))
    if not any(s.get('nvr') for s in samples):failures.append('No NVR observations; no empty success')
    findings=sanitizer_findings((directory/'runner.log').read_text(errors='replace'))
    if findings:failures.append('Sanitizer findings')
    result={'command':command,'returncode':process.returncode,'elapsed_seconds':time.monotonic()-started,
        'timed_out':timed_out,'passed':not failures,'failures':failures,'samples':samples,'unavailable':unavailable,
        'observation_complete':not unavailable,'cleanup':cleanup,'sanitizer_findings':findings}
    (directory/'samples.json').write_text(json.dumps(samples,indent=2)+'\n')
    return result


def assess_report(scenario, report):
    failures=[];metrics={}
    if scenario=='lifecycle':
        if not isinstance(report,list) or len(report)!=13 or any(r.get('passed') is not True for r in report):failures.append('Incomplete or failed lifecycle assertions')
        else:
            stops=[r['detail']['elapsedMs'] for r in report if isinstance(r.get('detail'),dict) and 'elapsedMs' in r['detail']]
            if stops:metrics['nvr.shutdown_ms']=max(stops)
    else:
        outcomes=report.get('results',[]) if isinstance(report,dict) else []
        if len(outcomes)!=1:failures.append('Missing or unexpected media outcome count')
        for outcome in outcomes:
            failures+=outcome.get('failures',[])
            if not outcome.get('cleanup') or any(r.get('forced') or r.get('returncode')!=0 for r in outcome['cleanup']):failures.append('Media child cleanup failed')
            limits=outcome.get('limits',{});recovery=outcome.get('recovery_seconds')
            if not finite(recovery) or recovery>limits.get('recovery_deadline_seconds',-1):failures.append('Missing or late recovery')
            else:metrics['nvr.recovery_ms']=recovery*1000
            gaps=[]
            for phase in ['before','after']:
                rows=outcome.get(phase,[])
                if len(rows)<limits.get('consecutive_segments',3):failures.append('Incomplete media sequence')
                for a,b in zip(rows,rows[1:]):
                    try:gap=abs(b['row']['start_time']-a['row']['end_time'])
                    except (KeyError,TypeError):failures.append('Unavailable media gap');continue
                    gaps.append(gap)
                    if gap>limits.get('continuous_gap_tolerance_seconds',-1):failures.append('Unexpected media gap')
            if gaps:metrics['nvr.recording_gap_ms']=max(gaps)*1000
    return {'passed':not failures,'failures':failures,'metrics':metrics}


def command_for(scenario,binary):
    if scenario=='lifecycle':return ['node','tests/operational/lifecycle/run.mjs',str(binary)]
    return [sys.executable,'tests/operational/media/recovery.py','--binary',str(binary),'--scenario','video-only' if scenario=='media-video' else 'h264-aac']


def report_path(scenario,log):
    pattern=r'T29 artifacts: (.+)' if scenario=='lifecycle' else r'(?m)^(/[^\n]+/build/eng-artifacts/T27/media-[a-f0-9]+)$'
    found=re.findall(pattern,log)
    if len(found)!=1:raise ValueError('Expected exactly one owned scenario artifact path')
    path=Path(found[0]).resolve(strict=True)
    expected=ROOT/'build/eng-artifacts'/('T29' if scenario=='lifecycle' else 'T27')
    if path.parent!=expected:raise ValueError('Unexpected artifact directory')
    return path


def ensure_runtime_available(lock=LOCK):
    if lock.exists():raise RuntimeError('Runtime lock exists; refusing to adopt or remove it')


def provenance(binary):
    def output(command):
        try:return subprocess.check_output(command,cwd=ROOT,text=True,stderr=subprocess.STDOUT,timeout=15).strip()
        except (OSError,subprocess.SubprocessError) as error:return {'unavailable':str(error)}
    inputs=['quality/runtime-budgets.json','quality/soak-profiles.json','tests/fixtures/eng/media/scenarios.json',
        'tests/operational/media/recovery.py','tests/operational/lifecycle/run.mjs','scripts/quality/soak.py']
    source_files=output(['git','ls-files','src','include','config','tests/operational','tests/fixtures/eng','scripts/quality','CMakeLists.txt','tests/CMakeLists.txt'])
    source_hashes={}
    if isinstance(source_files,str):
        for name in source_files.splitlines():
            file=ROOT/name
            if file.is_file():source_hashes[name]=hashlib.sha256(file.read_bytes()).hexdigest()
    cache=binary.parent.parent/'CMakeCache.txt'
    cache_text=cache.read_text() if cache.exists() else None
    return {'source_hashes':source_hashes,'source_diff':output(['git','diff','--','src','include','config','tests/operational','tests/fixtures/eng','scripts/quality']),
        'build_cache':cache_text, 'build_cache_sha256':hashlib.sha256(cache.read_bytes()).hexdigest() if cache.exists() else None,
        'revision':output(['git','rev-parse','HEAD']),'working_tree':output(['git','status','--short']),
        'binary':str(binary),'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),
        'inputs':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in inputs},
        'host':platform.uname()._asdict(),'python':sys.version,'ffmpeg':output(['ffmpeg','-version']),
        'ffprobe':output(['ffprobe','-version']),'node':output(['node','--version']), 'libraries':output(['ldd',str(binary)]),
        'scope':'Informational host runs; mixed phases, uncontrolled OS page cache, synthetic sources local and accounted separately; no physical qualification'}


def main(argv=None):
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--profile',default='ci');parser.add_argument('--output',required=True,type=Path)
    parser.add_argument('--binary',type=Path,default=ROOT/'build/eng-native/T31.3-integration/integration-none/bin/lightnvr')
    args=parser.parse_args(argv)
    profiles=json.loads((ROOT/'quality/soak-profiles.json').read_text())
    profile=validate_profile(profiles['profiles'][args.profile]);binary=args.binary.resolve(strict=True)
    directory=args.output.resolve()/('soak-'+uuid.uuid4().hex[:12]);directory.mkdir(parents=True)
    baseline=provenance(binary)
    (directory/'provenance.json').write_text(json.dumps(baseline,indent=2)+'\n')
    outcomes=[];observations={'nvr.binary_bytes':binary.stat().st_size};failures=[]
    for repeat in range(profile['repetitions']):
        for scenario in profile['scenarios']:
            child_dir=directory/f'{repeat+1}-{scenario}'
            try:
                ensure_runtime_available()
                if hashlib.sha256(binary.read_bytes()).hexdigest()!=baseline['binary_sha256']:raise RuntimeError('Binary changed during soak')
                result=run_child(command_for(scenario,binary),child_dir,str(binary),profile['sample_interval_seconds'],profile['scenario_timeout_seconds'])
                result.update({'scenario':scenario,'repetition':repeat+1})
                (child_dir/'supervisor-result.json').write_text(json.dumps({k:v for k,v in result.items() if k!='samples'},indent=2)+'\n')
                artifact=report_path(scenario,(child_dir/'runner.log').read_text())
                shutil.copytree(artifact,child_dir/'artifacts')
                report=json.loads((artifact/('checks.json' if scenario=='lifecycle' else 'run.json')).read_text())
                assessment=assess_report(scenario,report);result['functional']=assessment
                findings=[]
                for log in artifact.rglob('*.log'):findings+=sanitizer_findings(log.read_text(errors='replace'))
                if findings:result['failures'].append('Scenario artifact sanitizer findings');result['sanitizer_findings']+=findings
                result['passed']=result['passed'] and assessment['passed'] and not findings
                result['resources']=resource_summary(result['samples'],profile['growth_limits'])
                if result['resources']['failures']:result['passed']=False
                for name,value in assessment['metrics'].items():observations[name]=max(observations.get(name,0),value)
                result['statistics']={group:{metric:distribution([s[group].get(metric) for s in result['samples'] if isinstance(s.get(group),dict)])
                    for metric in ['rss_bytes','cpu_one_core_percent','fds','threads']} for group in ['nvr','source','browser']}
                for source,target in [('cpu_one_core_percent','nvr.cpu_one_core_percent'),('fds','nvr.open_file_descriptors'),('threads','nvr.threads')]:
                    stat=result['statistics']['nvr'][source]
                    if stat:observations[target]=max(observations.get(target,0),stat['max'])
                result.pop('samples');(child_dir/'result.json').write_text(json.dumps(result,indent=2)+'\n');outcomes.append(result)
                ensure_runtime_available()
            except Exception as error:
                failures.append(f'{scenario} repetition {repeat+1}: {type(error).__name__}: {error}')
                break
        if failures:break
    budgets=json.loads((ROOT/'quality/runtime-budgets.json').read_text())['metrics']
    qualification=qualify(budgets,observations)
    functional=not failures and len(outcomes)==profile['repetitions']*len(profile['scenarios']) and all(r['passed'] for r in outcomes)
    complete=all(r['observation_complete'] for r in outcomes) and bool(outcomes)
    growth=all(r['resources']['passed'] for r in outcomes) and bool(outcomes)
    passed=functional and qualification['passed'] and complete and growth
    result={'profile':args.profile,'settings':profile,'functional_passed':functional,'qualification':qualification,
        'resource_observation_complete':complete,'growth_qualified':growth,'overall_passed':passed,
        'status':'passed' if passed else 'functional_failed' if not functional else 'unqualified',
        'failures':failures,'runs':outcomes,'race_detection':{'status':'not_run','assessment':'tests/operational/soak/README.md'},
        'limitations':['Short repeated host scenarios are not a sustained physical soak or leak proof.',
            'RSS sums may double-count shared pages. CPU uses one core=100%; first observations have no delta.',
            'Exited/unreadable identities are retained as unavailable, never zero. Idle/recording phase separation and browser performance metrics are not instrumented.',
            'Media gaps are existing DB segment adjacency controls, excluding intentional outage; no new packet continuity tolerance.',
            'Existing child runners serialize their own runtime lock. Abandoned locks are retained and halt composition.']}
    (directory/'summary.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({'artifacts':str(directory),'status':result['status'],'functional_passed':functional,'overall_passed':passed}))
    return 0 if passed else 1 if not functional else 2


if __name__=='__main__':
    try:raise SystemExit(main())
    except (ValueError,KeyError,OSError,RuntimeError) as error:print(str(error),file=sys.stderr);raise SystemExit(1)
