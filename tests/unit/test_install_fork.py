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

# Регрессия с настоящим Git: после clone --no-checkout ветка есть только в origin.
import shutil
real_git=shutil.which('git')
with tempfile.TemporaryDirectory(prefix='nvr-real-git-test-') as root:
 p=pathlib.Path(root); fixture=p/'fixture'; fixture.mkdir()
 def git(*args):
  return subprocess.check_output([real_git, '-C', str(fixture), *args], text=True).strip()
 git('init', '-b', 'codex/mobile-live-favorites')
 (fixture/'README').write_text('fixture')
 git('add', '.')
 git('-c','user.name=Test','-c','user.email=test@example.invalid','commit','-m','fixture')
 sha=git('rev-parse','HEAD')
 b=p/'bin'; b.mkdir()
 (b/'docker').write_text('''#!/bin/bash
if [[ $1 == inspect ]]; then echo healthy; fi
if [[ $1 == compose && " $* " == *" ps "* ]]; then echo fixture-container; fi
''')
 (b/'docker').chmod(0o755)
 env=dict(os.environ, PATH=str(b)+':'+os.environ['PATH'], GIT_CONFIG_COUNT='1',
          GIT_CONFIG_KEY_0='url.'+str(fixture)+'.insteadOf',
          GIT_CONFIG_VALUE_0='https://github.com/zirocool93/NVR.git')
 for resume in (False, True):
  dest=p/('resume' if resume else 'fresh')
  if resume:
   dest.mkdir()
   subprocess.check_call([real_git,'clone','--no-checkout',str(fixture),str(dest/'source')],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
   subprocess.check_call([real_git,'-C',str(dest/'source'),'remote','set-url','origin','https://github.com/zirocool93/NVR.git'])
  args=['bash',script,'--dir',str(dest)] + (['--resume-source-only'] if resume else [])
  result=subprocess.run(args,env=env,capture_output=True,text=True)
  assert result.returncode==0, result.stdout+result.stderr
  assert sha in (dest/'.env').read_text()
  assert subprocess.check_output([real_git,'-C',str(dest/'source'),'rev-parse','HEAD'],text=True).strip()==sha
  if resume:
   result=subprocess.run(args,env=env,capture_output=True,text=True)
   assert result.returncode!=0 # config/data уже созданы: повторное продолжение запрещено
 print('PASS: настоящий Git, первый detached checkout удалённой ветки, продолжение source-only и запрет после создания данных')
