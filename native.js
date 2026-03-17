'use strict';

const fs = require('fs');
const path = require('path');

const ROOT = __dirname;
const SEARCH_DIRS = [ROOT, path.join(ROOT, 'napi')];
const BINARY_ENV_VARS = [
  'PICOMATCH_RS_NATIVE_BINARY',
  'PICOMATCH_NATIVE_BINARY'
];

const scoreCandidate = entry => {
  if (/^picomatch-rs\./.test(entry)) {
    return 3;
  }

  if (/^picomatch_rs\./.test(entry)) {
    return 2;
  }

  if (entry.endsWith('.node')) {
    return 1;
  }

  return 0;
};

const explicitBinary = () => {
  for (const key of BINARY_ENV_VARS) {
    if (process.env[key]) {
      return process.env[key];
    }
  }
};

const collectCandidates = () => {
  const override = explicitBinary();
  if (override) {
    return [path.resolve(ROOT, override)];
  }

  const candidates = [];
  const seen = new Set();

  for (const dir of SEARCH_DIRS) {
    if (!fs.existsSync(dir)) {
      continue;
    }

    const entries = fs.readdirSync(dir)
      .filter(entry => entry.endsWith('.node'))
      .sort((left, right) => scoreCandidate(right) - scoreCandidate(left));

    for (const entry of entries) {
      const candidate = path.join(dir, entry);
      if (!seen.has(candidate)) {
        seen.add(candidate);
        candidates.push(candidate);
      }
    }
  }

  return candidates;
};

const loadNative = () => {
  const candidates = collectCandidates();
  const errors = [];

  for (const candidate of candidates) {
    if (!fs.existsSync(candidate)) {
      continue;
    }

    try {
      return require(candidate);
    } catch (error) {
      errors.push(`${candidate}: ${error.message}`);
    }
  }

  const details = errors.length === 0
    ? 'No built .node artifact was found.'
    : errors.join('\n');

  throw new Error(
    `Unable to load the picomatch-rs native addon.\n${details}\nRun "npm run build" first.`
  );
};

module.exports = loadNative();
