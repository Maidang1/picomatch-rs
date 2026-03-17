'use strict';

const native = require('./native');

const stripFunctions = options => {
  if (!options || typeof options !== 'object') return options;
  const cleaned = {};
  for (const key of Object.keys(options)) {
    if (typeof options[key] !== 'function') {
      cleaned[key] = options[key];
    }
  }
  return cleaned;
};

const nativeIsMatch = native.isMatch;
const nativeMakeRe = native.makeRe;
const nativeCompileMatcher = native.compileMatcher;

const applyFormat = (input, format) => {
  if (typeof format === 'function') return format(input);
  return input;
};

const buildIgnoreMatcher = (ignore, baseOptions) => {
  if (!ignore) return null;
  const patterns = [].concat(ignore);
  if (patterns.length === 0) return null;
  return nativeCompileMatcher(patterns, stripFunctions(baseOptions));
};

const createMatcher = (compiled, options) => {
  const format = options && typeof options.format === 'function' ? options.format : null;
  const onMatch = options && typeof options.onMatch === 'function' ? options.onMatch : null;
  const ignoreMatcher = buildIgnoreMatcher(options && options.ignore, options);

  const matcher = (input, returnObject) => {
    const formatted = format ? applyFormat(input, format) : input;
    const result = compiled.test(formatted, returnObject || !!onMatch);

    if (ignoreMatcher) {
      const ignoredFormatted = format ? applyFormat(input, format) : input;
      if (ignoreMatcher.test(ignoredFormatted)) {
        if (returnObject || onMatch) {
          if (result && typeof result === 'object') {
            return Object.assign({}, result, { isMatch: false });
          }
          return false;
        }
        return false;
      }
    }

    if (onMatch && result && typeof result === 'object' && result.isMatch) {
      const matches = options.matches || new Set();
      onMatch(result, matches);
    }

    if (returnObject) return result;
    if (result && typeof result === 'object') return result.isMatch === true;
    return result === true;
  };

  matcher.test = matcher;

  Object.defineProperties(matcher, {
    state: {
      enumerable: true,
      get: () => compiled.state
    },
    regex: {
      enumerable: true,
      get: () => compiled.regex
    }
  });

  return matcher;
};

const isMatch = (input, patterns, options) => {
  const format = options && typeof options.format === 'function' ? options.format : null;
  const formatted = format ? applyFormat(input, format) : input;

  const ignore = options && options.ignore;
  if (ignore) {
    const ignorePatterns = [].concat(ignore);
    if (ignorePatterns.length > 0) {
      const ignoreMatcher = nativeCompileMatcher(ignorePatterns, stripFunctions(options));
      if (ignoreMatcher.test(formatted)) return false;
    }
  }

  return nativeIsMatch(formatted, patterns, stripFunctions(options));
};

const makeRe = (input, options, returnOutput, returnState) => {
  const descriptor = nativeMakeRe(input, stripFunctions(options), returnOutput, returnState);
  if (!descriptor || returnOutput) return descriptor;
  return new RegExp(descriptor.source, descriptor.flags);
};

const picomatch = (patterns, options, _returnState) => {
  const compiled = nativeCompileMatcher(patterns, stripFunctions(options));
  return createMatcher(compiled, options);
};

module.exports = Object.assign(picomatch, native, { isMatch, makeRe });
