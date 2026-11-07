import {spawn} from 'node:child_process';
import {createHash} from 'node:crypto';

/** Hash raw diff bytes without buffering potentially large regenerated sources. */
export async function trackedDiffSha256(cwd = process.cwd()) {
  const hash = createHash('sha256');
  const child = spawn('git', ['diff', '--no-ext-diff', '--no-textconv', 'HEAD'], {
    cwd, stdio: ['ignore', 'pipe', 'inherit'],
  });
  child.stdout.on('data', chunk => hash.update(chunk));
  await new Promise((resolve, reject) => {
    child.once('error', reject);
    child.once('close', (code, signal) => {
      if (code === 0) resolve();
      else reject(new Error(`git diff failed (${signal || code})`));
    });
  });
  return hash.digest('hex');
}
