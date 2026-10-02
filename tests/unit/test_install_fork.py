import os, pathlib, subprocess, tempfile
script=str(pathlib.Path(__file__).resolve().parents[2] / 'scripts/install-fork.sh')
with tempfile.TemporaryDirectory(prefix='nvr-installer-test-') as root:
 p=pathlib.Path(root); b=p/'bin'; b.mkdir()
 (b/'git').write_text('''#!/bin/bash
if [[ $1 == clone ]]; then mkdir -p "${@: -1}"; exit; fi
if [[ $3 == rev-parse ]]; then echo 0123456789abcdef0123456789abcdef01234567; fi
''')
 (b/'docker').write_text('''#!/bin/bash
printf '%s\\n' "$*" >> "$TEST_LOG"
if [[ $1 == inspect ]]; then echo healthy; fi
if [[ $1 == compose && " $* " == *" ps "* ]]; then echo test-container; fi
''')
 for f in b.iterdir(): f.chmod(0o755)
 env=dict(os.environ,PATH=str(b)+':'+os.environ['PATH'],TEST_LOG=str(p/'calls'))
 dest=p/'install'
 r=subprocess.run(['bash',script,'--dir',str(dest)],env=env,capture_output=True,text=True)
 assert r.returncode==0, r.stderr+r.stdout
 assert '0123456789abcdef' in (dest/'.env').read_text()
 assert '127.0.0.1' in (dest/'.env').read_text()
 assert 'context: ./source' in (dest/'compose.yaml').read_text()
 assert 'build --pull' in (p/'calls').read_text()
 assert 'up -d' in (p/'calls').read_text()
 for args in (['--dir',str(dest)],['--port','0'],['--ref','--bad'],['--dir','/']):
  r=subprocess.run(['bash',script,*args],env=env,capture_output=True,text=True)
  assert r.returncode!=0, args
 print('PASS: чистая установка с mock Git/Docker, pinned commit, compose, отказ существующему каталогу и неверным параметрам')
