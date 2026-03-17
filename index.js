'use strict';

const native = require('./native');

const createMatcher = compiled => {
  const matcher = (input, returnObject) => compiled.test(input, returnObject);
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

const picomatch = (patterns, options, _returnState) => {
  const compiled = native.compileMatcher(patterns, options);
  return createMatcher(compiled);
};

module.exports = Object.assign(picomatch, native);
