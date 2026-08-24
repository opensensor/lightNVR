import fs from 'node:fs';
import path from 'node:path';

const layoutCss = fs.readFileSync(
  path.join(process.cwd(), 'css/layout.css'),
  'utf8',
);
const mainCss = fs.readFileSync(
  path.join(process.cwd(), 'css/main.css'),
  'utf8',
);

test('site header layout rules do not override semantic modal headers', () => {
  expect(layoutCss).toMatch(/\.app-header\s*\{/);
  expect(layoutCss).not.toMatch(/(^|\n)\s*header\s*\{/);
  expect(mainCss).toMatch(/\.app-header, \.app-header \*/);
  expect(mainCss).not.toMatch(/(^|\n)\s*header, header \*/);
});
