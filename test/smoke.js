'use strict';

const assert = require('assert');
const native = require('..');

describe('package smoke', () => {
  it('loads the native surface from the package root', () => {
    assert.equal(typeof native.scan, 'function');
    assert.equal(typeof native.parse, 'function');
    assert.equal(typeof native.compileRe, 'function');
    assert.equal(typeof native.makeRe, 'function');
    assert.equal(typeof native.toRegex, 'function');
    assert.equal(typeof native.test, 'function');
    assert.equal(typeof native.matchBase, 'function');
    assert.equal(typeof native.isMatch, 'function');
    assert.equal(typeof native.compileMatcher, 'function');
    assert.equal(typeof native.NativeMatcher, 'function');
  });

  it('supports representative matching flows', () => {
    const scanState = native.scan('src/**/*.rs');
    assert.equal(scanState.isGlob, true);

    const descriptor = native.makeRe('**/*.rs', null, false, true);
    assert.equal(typeof descriptor.source, 'string');
    assert.equal(descriptor.state.input, '**/*.rs');

    assert.equal(native.isMatch('src/lib.rs', '**/*.rs'), true);
    assert.equal(native.matchBase('src/lib.rs', '*.rs'), true);

    const matcher = native.compileMatcher('**/*.rs');
    assert.equal(matcher.test('src/lib.rs'), true);
    assert.equal(matcher.test('src/lib.ts'), false);
  });
});
