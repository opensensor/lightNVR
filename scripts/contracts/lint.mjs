import {readdir} from 'node:fs/promises';
import {execFileSync} from 'node:child_process';
import path from 'node:path';
import {root, validateSource} from './generate.mjs';
const args = process.argv.slice(2);
let files, projected = false;
if (args.length === 2 && args[0] === '--projections') {
  projected = true;
  const dir = path.resolve(args[1]);
  files = (await readdir(dir, {withFileTypes:true})).filter(e => e.isFile() && e.name.endsWith('.json')).map(e => path.join(dir,e.name)).sort();
  if (!files.length) throw new Error(`No projection JSON documents found in ${dir}`);
} else if (args.length === 2 && args[0] === '--source') files = [path.resolve(args[1])];
else if (args.length) throw new Error('Usage: contracts:lint [--source FILE | --projections DIR]');
else files = [path.join(root,'docs/api/openapi.json')];
for (const file of files) validateSource(file, projected);
execFileSync(path.join(root,'node_modules/.bin/redocly'), ['lint','--config',path.join(root,'redocly.yaml'), ...files], {stdio:'inherit'});
