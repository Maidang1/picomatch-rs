'use strict';

const fs = require('fs');
const path = require('path');

const root = path.resolve(__dirname, '..');
const napiDir = path.join(root, 'napi');
const targetRoot = process.env.CARGO_TARGET_DIR
  ? path.resolve(process.env.CARGO_TARGET_DIR)
  : path.join(root, 'target');
const platform = process.platform;
const arch = process.arch;
const extension = platform === 'darwin'
  ? 'dylib'
  : platform === 'win32'
    ? 'dll'
    : 'so';
const outputName = `picomatch-rs.${platform}-${arch}.node`;
const outputPath = path.join(root, outputName);
const candidateNames = [
  `libpicomatch_rs_napi.${extension}`,
  `libpicomatch_napi.${extension}`,
  `picomatch_rs_napi.${extension}`,
  `picomatch_napi.${extension}`
];
const candidateDirs = [
  path.join(targetRoot, 'debug'),
  path.join(targetRoot, 'release')
];

for (const generated of ['index.js']) {
  const generatedPath = path.join(napiDir, generated);
  if (fs.existsSync(generatedPath)) {
    fs.rmSync(generatedPath, { force: true });
  }
}

for (const entry of fs.existsSync(napiDir) ? fs.readdirSync(napiDir) : []) {
  if (entry.endsWith('.node')) {
    fs.rmSync(path.join(napiDir, entry), { force: true });
  }
}

let sourcePath;
for (const dir of candidateDirs) {
  for (const name of candidateNames) {
    const candidate = path.join(dir, name);
    if (fs.existsSync(candidate)) {
      sourcePath = candidate;
      break;
    }
  }

  if (sourcePath) {
    break;
  }
}

if (!sourcePath) {
  throw new Error(
    `Unable to locate a native library in ${candidateDirs.join(', ')}`
  );
}

fs.copyFileSync(sourcePath, outputPath);
