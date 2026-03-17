mod support;

use picomatch_rs::CompileOptions;

use support::{assert_is_match, default_compile_options};

#[test]
fn should_handle_regular_globbing() {
    let opts = default_compile_options();
    assert_is_match("*", "a*", opts.clone(), false);
    assert_is_match("**", "a*", opts.clone(), false);
    assert_is_match("\\*", "a*", opts.clone(), false);
    assert_is_match("a/*", "a*", opts.clone(), false);
    assert_is_match("b", "a*", opts.clone(), false);
    assert_is_match("bc", "a*", opts.clone(), false);
    assert_is_match("bcd", "a*", opts.clone(), false);
    assert_is_match("bdir/", "a*", opts.clone(), false);
    assert_is_match("Beware", "a*", opts.clone(), false);
    assert_is_match("a", "a*", opts.clone(), true);
    assert_is_match("ab", "a*", opts.clone(), true);
    assert_is_match("abc", "a*", opts.clone(), true);

    assert_is_match("*", "\\a*", opts.clone(), false);
    assert_is_match("**", "\\a*", opts.clone(), false);
    assert_is_match("\\*", "\\a*", opts.clone(), false);

    assert_is_match("a", "\\a*", opts.clone(), true);
    assert_is_match("a/*", "\\a*", opts.clone(), false);
    assert_is_match("abc", "\\a*", opts.clone(), true);
    assert_is_match("abd", "\\a*", opts.clone(), true);
    assert_is_match("abe", "\\a*", opts.clone(), true);
    assert_is_match("b", "\\a*", opts.clone(), false);
    assert_is_match("bb", "\\a*", opts.clone(), false);
    assert_is_match("bcd", "\\a*", opts.clone(), false);
    assert_is_match("bdir/", "\\a*", opts.clone(), false);
    assert_is_match("Beware", "\\a*", opts.clone(), false);
    assert_is_match("c", "\\a*", opts.clone(), false);
    assert_is_match("ca", "\\a*", opts.clone(), false);
    assert_is_match("cb", "\\a*", opts.clone(), false);
    assert_is_match("d", "\\a*", opts.clone(), false);
    assert_is_match("dd", "\\a*", opts.clone(), false);
    assert_is_match("de", "\\a*", opts.clone(), false);
}

#[test]
fn should_match_directories() {
    let opts = default_compile_options();
    assert_is_match("*", "b*/", opts.clone(), false);
    assert_is_match("**", "b*/", opts.clone(), false);
    assert_is_match("\\*", "b*/", opts.clone(), false);
    assert_is_match("a", "b*/", opts.clone(), false);
    assert_is_match("a/*", "b*/", opts.clone(), false);
    assert_is_match("abc", "b*/", opts.clone(), false);
    assert_is_match("abd", "b*/", opts.clone(), false);
    assert_is_match("abe", "b*/", opts.clone(), false);
    assert_is_match("b", "b*/", opts.clone(), false);
    assert_is_match("bb", "b*/", opts.clone(), false);
    assert_is_match("bcd", "b*/", opts.clone(), false);
    assert_is_match("bdir/", "b*/", opts.clone(), true);
    assert_is_match("Beware", "b*/", opts.clone(), false);
    assert_is_match("c", "b*/", opts.clone(), false);
    assert_is_match("ca", "b*/", opts.clone(), false);
    assert_is_match("cb", "b*/", opts.clone(), false);
    assert_is_match("d", "b*/", opts.clone(), false);
    assert_is_match("dd", "b*/", opts.clone(), false);
    assert_is_match("de", "b*/", opts.clone(), false);
}

#[test]
fn should_use_escaped_characters_as_literals() {
    let opts = default_compile_options();
    assert_is_match("*", "\\^", opts.clone(), false);
    assert_is_match("**", "\\^", opts.clone(), false);
    assert_is_match("\\*", "\\^", opts.clone(), false);
    assert_is_match("a", "\\^", opts.clone(), false);
    assert_is_match("a/*", "\\^", opts.clone(), false);
    assert_is_match("abc", "\\^", opts.clone(), false);
    assert_is_match("abd", "\\^", opts.clone(), false);
    assert_is_match("abe", "\\^", opts.clone(), false);
    assert_is_match("b", "\\^", opts.clone(), false);
    assert_is_match("bb", "\\^", opts.clone(), false);
    assert_is_match("bcd", "\\^", opts.clone(), false);
    assert_is_match("bdir/", "\\^", opts.clone(), false);
    assert_is_match("Beware", "\\^", opts.clone(), false);
    assert_is_match("c", "\\^", opts.clone(), false);
    assert_is_match("ca", "\\^", opts.clone(), false);
    assert_is_match("cb", "\\^", opts.clone(), false);
    assert_is_match("d", "\\^", opts.clone(), false);
    assert_is_match("dd", "\\^", opts.clone(), false);
    assert_is_match("de", "\\^", opts.clone(), false);

    assert_is_match("*", "\\*", opts.clone(), true);
    assert_is_match("\\*", "\\*", opts.clone(), true);
    assert_is_match("**", "\\*", opts.clone(), false);
    assert_is_match("a", "\\*", opts.clone(), false);
    assert_is_match("a/*", "\\*", opts.clone(), false);
    assert_is_match("abc", "\\*", opts.clone(), false);
    assert_is_match("abd", "\\*", opts.clone(), false);
    assert_is_match("abe", "\\*", opts.clone(), false);
    assert_is_match("b", "\\*", opts.clone(), false);
    assert_is_match("bb", "\\*", opts.clone(), false);
    assert_is_match("bcd", "\\*", opts.clone(), false);
    assert_is_match("bdir/", "\\*", opts.clone(), false);
    assert_is_match("Beware", "\\*", opts.clone(), false);
    assert_is_match("c", "\\*", opts.clone(), false);
    assert_is_match("ca", "\\*", opts.clone(), false);
    assert_is_match("cb", "\\*", opts.clone(), false);
    assert_is_match("d", "\\*", opts.clone(), false);
    assert_is_match("dd", "\\*", opts.clone(), false);
    assert_is_match("de", "\\*", opts.clone(), false);

    assert_is_match("*", "a\\*", opts.clone(), false);
    assert_is_match("**", "a\\*", opts.clone(), false);
    assert_is_match("\\*", "a\\*", opts.clone(), false);
    assert_is_match("a", "a\\*", opts.clone(), false);
    assert_is_match("a/*", "a\\*", opts.clone(), false);
    assert_is_match("abc", "a\\*", opts.clone(), false);
    assert_is_match("abd", "a\\*", opts.clone(), false);
    assert_is_match("abe", "a\\*", opts.clone(), false);
    assert_is_match("b", "a\\*", opts.clone(), false);
    assert_is_match("bb", "a\\*", opts.clone(), false);
    assert_is_match("bcd", "a\\*", opts.clone(), false);
    assert_is_match("bdir/", "a\\*", opts.clone(), false);
    assert_is_match("Beware", "a\\*", opts.clone(), false);
    assert_is_match("c", "a\\*", opts.clone(), false);
    assert_is_match("ca", "a\\*", opts.clone(), false);
    assert_is_match("cb", "a\\*", opts.clone(), false);
    assert_is_match("d", "a\\*", opts.clone(), false);
    assert_is_match("dd", "a\\*", opts.clone(), false);
    assert_is_match("de", "a\\*", opts.clone(), false);

    assert_is_match("aqa", "*q*", opts.clone(), true);
    assert_is_match("aaqaa", "*q*", opts.clone(), true);
    assert_is_match("*", "*q*", opts.clone(), false);
    assert_is_match("**", "*q*", opts.clone(), false);
    assert_is_match("\\*", "*q*", opts.clone(), false);
    assert_is_match("a", "*q*", opts.clone(), false);
    assert_is_match("a/*", "*q*", opts.clone(), false);
    assert_is_match("abc", "*q*", opts.clone(), false);
    assert_is_match("abd", "*q*", opts.clone(), false);
    assert_is_match("abe", "*q*", opts.clone(), false);
    assert_is_match("b", "*q*", opts.clone(), false);
    assert_is_match("bb", "*q*", opts.clone(), false);
    assert_is_match("bcd", "*q*", opts.clone(), false);
    assert_is_match("bdir/", "*q*", opts.clone(), false);
    assert_is_match("Beware", "*q*", opts.clone(), false);
    assert_is_match("c", "*q*", opts.clone(), false);
    assert_is_match("ca", "*q*", opts.clone(), false);
    assert_is_match("cb", "*q*", opts.clone(), false);
    assert_is_match("d", "*q*", opts.clone(), false);
    assert_is_match("dd", "*q*", opts.clone(), false);
    assert_is_match("de", "*q*", opts.clone(), false);

    assert_is_match("*", "\\**", opts.clone(), true);
    assert_is_match("**", "\\**", opts.clone(), true);
    assert_is_match("\\*", "\\**", opts.clone(), false);
    assert_is_match("a", "\\**", opts.clone(), false);
    assert_is_match("a/*", "\\**", opts.clone(), false);
    assert_is_match("abc", "\\**", opts.clone(), false);
    assert_is_match("abd", "\\**", opts.clone(), false);
    assert_is_match("abe", "\\**", opts.clone(), false);
    assert_is_match("b", "\\**", opts.clone(), false);
    assert_is_match("bb", "\\**", opts.clone(), false);
    assert_is_match("bcd", "\\**", opts.clone(), false);
    assert_is_match("bdir/", "\\**", opts.clone(), false);
    assert_is_match("Beware", "\\**", opts.clone(), false);
    assert_is_match("c", "\\**", opts.clone(), false);
    assert_is_match("ca", "\\**", opts.clone(), false);
    assert_is_match("cb", "\\**", opts.clone(), false);
    assert_is_match("d", "\\**", opts.clone(), false);
    assert_is_match("dd", "\\**", opts.clone(), false);
    assert_is_match("de", "\\**", opts.clone(), false);
}

#[test]
fn should_work_for_quoted_characters() {
    let opts = default_compile_options();
    assert_is_match("*", "\"***\"", opts.clone(), false);
    assert_is_match("**", "\"***\"", opts.clone(), false);
    assert_is_match("\\*", "\"***\"", opts.clone(), false);
    assert_is_match("a", "\"***\"", opts.clone(), false);
    assert_is_match("a/*", "\"***\"", opts.clone(), false);
    assert_is_match("abc", "\"***\"", opts.clone(), false);
    assert_is_match("abd", "\"***\"", opts.clone(), false);
    assert_is_match("abe", "\"***\"", opts.clone(), false);
    assert_is_match("b", "\"***\"", opts.clone(), false);
    assert_is_match("bb", "\"***\"", opts.clone(), false);
    assert_is_match("bcd", "\"***\"", opts.clone(), false);
    assert_is_match("bdir/", "\"***\"", opts.clone(), false);
    assert_is_match("Beware", "\"***\"", opts.clone(), false);
    assert_is_match("c", "\"***\"", opts.clone(), false);
    assert_is_match("ca", "\"***\"", opts.clone(), false);
    assert_is_match("cb", "\"***\"", opts.clone(), false);
    assert_is_match("d", "\"***\"", opts.clone(), false);
    assert_is_match("dd", "\"***\"", opts.clone(), false);
    assert_is_match("de", "\"***\"", opts.clone(), false);
    assert_is_match("***", "\"***\"", opts.clone(), true);

    assert_is_match("*", "'***'", opts.clone(), false);
    assert_is_match("**", "'***'", opts.clone(), false);
    assert_is_match("\\*", "'***'", opts.clone(), false);
    assert_is_match("a", "'***'", opts.clone(), false);
    assert_is_match("a/*", "'***'", opts.clone(), false);
    assert_is_match("abc", "'***'", opts.clone(), false);
    assert_is_match("abd", "'***'", opts.clone(), false);
    assert_is_match("abe", "'***'", opts.clone(), false);
    assert_is_match("b", "'***'", opts.clone(), false);
    assert_is_match("bb", "'***'", opts.clone(), false);
    assert_is_match("bcd", "'***'", opts.clone(), false);
    assert_is_match("bdir/", "'***'", opts.clone(), false);
    assert_is_match("Beware", "'***'", opts.clone(), false);
    assert_is_match("c", "'***'", opts.clone(), false);
    assert_is_match("ca", "'***'", opts.clone(), false);
    assert_is_match("cb", "'***'", opts.clone(), false);
    assert_is_match("d", "'***'", opts.clone(), false);
    assert_is_match("dd", "'***'", opts.clone(), false);
    assert_is_match("de", "'***'", opts.clone(), false);
    assert_is_match("'***'", "'***'", opts.clone(), true);

    assert_is_match("*", "\"*\"*", opts.clone(), true);
    assert_is_match("**", "\"*\"*", opts.clone(), true);
    assert_is_match("\\*", "\"*\"*", opts.clone(), false);
    assert_is_match("a", "\"*\"*", opts.clone(), false);
    assert_is_match("a/*", "\"*\"*", opts.clone(), false);
    assert_is_match("abc", "\"*\"*", opts.clone(), false);
    assert_is_match("abd", "\"*\"*", opts.clone(), false);
    assert_is_match("abe", "\"*\"*", opts.clone(), false);
    assert_is_match("b", "\"*\"*", opts.clone(), false);
    assert_is_match("bb", "\"*\"*", opts.clone(), false);
    assert_is_match("bcd", "\"*\"*", opts.clone(), false);
    assert_is_match("bdir/", "\"*\"*", opts.clone(), false);
    assert_is_match("Beware", "\"*\"*", opts.clone(), false);
    assert_is_match("c", "\"*\"*", opts.clone(), false);
    assert_is_match("ca", "\"*\"*", opts.clone(), false);
    assert_is_match("cb", "\"*\"*", opts.clone(), false);
    assert_is_match("d", "\"*\"*", opts.clone(), false);
    assert_is_match("dd", "\"*\"*", opts.clone(), false);
    assert_is_match("de", "\"*\"*", opts.clone(), false);
}

#[test]
fn should_match_escaped_quotes() {
    let opts = default_compile_options();
    assert_is_match("*", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("**", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("\\*", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("a", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("a/*", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("abc", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("abd", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("abe", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("b", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("bb", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("bcd", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("bdir/", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("Beware", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("c", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("ca", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("cb", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("d", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("dd", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("de", "\\\"**\\\"", opts.clone(), false);
    assert_is_match("\"**\"", "\\\"**\\\"", opts.clone(), true);

    assert_is_match("foo/\"**\"/bar", "foo/\\\"**\\\"/bar", opts.clone(), true);

    assert_is_match("foo/\"*\"/bar", "foo/\\\"*\\\"/bar", opts.clone(), true);
    assert_is_match("foo/\"a\"/bar", "foo/\\\"*\\\"/bar", opts.clone(), true);
    assert_is_match("foo/\"b\"/bar", "foo/\\\"*\\\"/bar", opts.clone(), true);
    assert_is_match("foo/\"c\"/bar", "foo/\\\"*\\\"/bar", opts.clone(), true);
    assert_is_match("foo/'*'/bar", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("foo/'a'/bar", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("foo/'b'/bar", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("foo/'c'/bar", "foo/\\\"*\\\"/bar", opts.clone(), false);

    assert_is_match("foo/*/bar", "foo/\"*\"/bar", opts.clone(), true);
    assert_is_match("foo/\"*\"/bar", "foo/\"*\"/bar", opts.clone(), true);
    assert_is_match("foo/\"a\"/bar", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("foo/\"b\"/bar", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("foo/\"c\"/bar", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("foo/'*'/bar", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("foo/'a'/bar", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("foo/'b'/bar", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("foo/'c'/bar", "foo/\"*\"/bar", opts.clone(), false);

    assert_is_match("*", "\\'**\\'", opts.clone(), false);
    assert_is_match("**", "\\'**\\'", opts.clone(), false);
    assert_is_match("\\*", "\\'**\\'", opts.clone(), false);
    assert_is_match("a", "\\'**\\'", opts.clone(), false);
    assert_is_match("a/*", "\\'**\\'", opts.clone(), false);
    assert_is_match("abc", "\\'**\\'", opts.clone(), false);
    assert_is_match("abd", "\\'**\\'", opts.clone(), false);
    assert_is_match("abe", "\\'**\\'", opts.clone(), false);
    assert_is_match("b", "\\'**\\'", opts.clone(), false);
    assert_is_match("bb", "\\'**\\'", opts.clone(), false);
    assert_is_match("bcd", "\\'**\\'", opts.clone(), false);
    assert_is_match("bdir/", "\\'**\\'", opts.clone(), false);
    assert_is_match("Beware", "\\'**\\'", opts.clone(), false);
    assert_is_match("c", "\\'**\\'", opts.clone(), false);
    assert_is_match("ca", "\\'**\\'", opts.clone(), false);
    assert_is_match("cb", "\\'**\\'", opts.clone(), false);
    assert_is_match("d", "\\'**\\'", opts.clone(), false);
    assert_is_match("dd", "\\'**\\'", opts.clone(), false);
    assert_is_match("de", "\\'**\\'", opts.clone(), false);
    assert_is_match("'**'", "\\'**\\'", opts.clone(), true);
    // Additional assertions migrated from test/bash.js.
    assert_is_match("*", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("**", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("\\*", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("a", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("a/*", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("abc", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("abd", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("abe", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("b", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("bb", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("bcd", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("bdir/", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("Beware", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("c", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("ca", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("cb", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("d", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("dd", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("de", "foo/\\\"**\\\"/bar", opts.clone(), false);
    assert_is_match("*", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("**", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("\\*", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("a", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("a/*", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("abc", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("abd", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("abe", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("b", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("bb", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("bcd", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("bdir/", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("Beware", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("c", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("ca", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("cb", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("d", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("dd", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("de", "foo/\\\"*\\\"/bar", opts.clone(), false);
    assert_is_match("*", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("**", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("\\*", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("a", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("a/*", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("abc", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("abd", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("abe", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("b", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("bb", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("bcd", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("bdir/", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("Beware", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("c", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("ca", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("cb", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("d", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("dd", "foo/\"*\"/bar", opts.clone(), false);
    assert_is_match("de", "foo/\"*\"/bar", opts.clone(), false);

}

#[test]
fn pattern_from_larry_walls_configure_that_caused_bash_to_blow_up() {
    let opts = default_compile_options();
    assert_is_match("*", "[a-c]b*", opts.clone(), false);
    assert_is_match("**", "[a-c]b*", opts.clone(), false);
    assert_is_match("\\*", "[a-c]b*", opts.clone(), false);
    assert_is_match("a", "[a-c]b*", opts.clone(), false);
    assert_is_match("a/*", "[a-c]b*", opts.clone(), false);
    assert_is_match("abc", "[a-c]b*", opts.clone(), true);
    assert_is_match("abd", "[a-c]b*", opts.clone(), true);
    assert_is_match("abe", "[a-c]b*", opts.clone(), true);
    assert_is_match("b", "[a-c]b*", opts.clone(), false);
    assert_is_match("bb", "[a-c]b*", opts.clone(), true);
    assert_is_match("bcd", "[a-c]b*", opts.clone(), false);
    assert_is_match("bdir/", "[a-c]b*", opts.clone(), false);
    assert_is_match("Beware", "[a-c]b*", opts.clone(), false);
    assert_is_match("c", "[a-c]b*", opts.clone(), false);
    assert_is_match("ca", "[a-c]b*", opts.clone(), false);
    assert_is_match("cb", "[a-c]b*", opts.clone(), true);
    assert_is_match("d", "[a-c]b*", opts.clone(), false);
    assert_is_match("dd", "[a-c]b*", opts.clone(), false);
    assert_is_match("de", "[a-c]b*", opts.clone(), false);
}

#[test]
fn should_support_character_classes() {
    let opts = default_compile_options();
    assert_is_match("*", "a*[^c]", opts.clone(), false);
    assert_is_match("**", "a*[^c]", opts.clone(), false);
    assert_is_match("\\*", "a*[^c]", opts.clone(), false);
    assert_is_match("a", "a*[^c]", opts.clone(), false);
    assert_is_match("a/*", "a*[^c]", opts.clone(), false);
    assert_is_match("abc", "a*[^c]", opts.clone(), false);
    assert_is_match("abd", "a*[^c]", opts.clone(), true);
    assert_is_match("abe", "a*[^c]", opts.clone(), true);
    assert_is_match("b", "a*[^c]", opts.clone(), false);
    assert_is_match("bb", "a*[^c]", opts.clone(), false);
    assert_is_match("bcd", "a*[^c]", opts.clone(), false);
    assert_is_match("bdir/", "a*[^c]", opts.clone(), false);
    assert_is_match("Beware", "a*[^c]", opts.clone(), false);
    assert_is_match("c", "a*[^c]", opts.clone(), false);
    assert_is_match("ca", "a*[^c]", opts.clone(), false);
    assert_is_match("cb", "a*[^c]", opts.clone(), false);
    assert_is_match("d", "a*[^c]", opts.clone(), false);
    assert_is_match("dd", "a*[^c]", opts.clone(), false);
    assert_is_match("de", "a*[^c]", opts.clone(), false);
    assert_is_match("baz", "a*[^c]", opts.clone(), false);
    assert_is_match("bzz", "a*[^c]", opts.clone(), false);
    assert_is_match("BZZ", "a*[^c]", opts.clone(), false);
    assert_is_match("beware", "a*[^c]", opts.clone(), false);
    assert_is_match("BewAre", "a*[^c]", opts.clone(), false);

    assert_is_match("a-b", "a[X-]b", opts.clone(), true);
    assert_is_match("aXb", "a[X-]b", opts.clone(), true);

    let bash_opts = CompileOptions {
        bash: true,
        ..CompileOptions::default()
    };

    let regex_opts = CompileOptions {
        regex: true,
        ..CompileOptions::default()
    };

    assert_is_match("a*", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("a*", "[a-y]*[^c]", bash_opts.clone(), true);
    assert_is_match("a123b", "[a-y]*[^c]", bash_opts.clone(), true);
    assert_is_match("a123c", "[a-y]*[^c]", bash_opts.clone(), false);
    assert_is_match("ab", "[a-y]*[^c]", bash_opts.clone(), true);
    assert_is_match("bd", "[a-y]*[^c]", bash_opts.clone(), true);

    assert_is_match("a*b/ooo", "a\\*b/*", opts.clone(), true);
    assert_is_match("a*b/ooo", "a\\*?/*", opts.clone(), true);

    assert_is_match("*", "a[b]c", opts.clone(), false);
    assert_is_match("**", "a[b]c", opts.clone(), false);
    assert_is_match("\\*", "a[b]c", opts.clone(), false);
    assert_is_match("a", "a[b]c", opts.clone(), false);
    assert_is_match("a/*", "a[b]c", opts.clone(), false);
    assert_is_match("abc", "a[b]c", opts.clone(), true);
    assert_is_match("abd", "a[b]c", opts.clone(), false);
    assert_is_match("abe", "a[b]c", opts.clone(), false);

    assert_is_match("*", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("abc", "a[\"b\"]c", opts.clone(), true);

    assert_is_match("*", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("abc", "a[\\\\b]c", opts.clone(), true);

    assert_is_match("abc", "a[\\b]c", opts.clone(), false);

    assert_is_match("*", "a[b-d]c", opts.clone(), false);
    assert_is_match("abc", "a[b-d]c", opts.clone(), true);

    assert_is_match("*", "a?c", opts.clone(), false);
    assert_is_match("abc", "a?c", opts.clone(), true);

    assert_is_match("man/man1/bash.1", "*/man*/bash.*", opts.clone(), true);

    assert_is_match("*", "[^a-c]*", opts.clone(), true);
    assert_is_match("**", "[^a-c]*", opts.clone(), true);
    assert_is_match("a", "[^a-c]*", opts.clone(), false);
    assert_is_match("abc", "[^a-c]*", opts.clone(), false);
    assert_is_match("abd", "[^a-c]*", opts.clone(), false);
    assert_is_match("abe", "[^a-c]*", opts.clone(), false);
    assert_is_match("b", "[^a-c]*", opts.clone(), false);
    assert_is_match("bb", "[^a-c]*", opts.clone(), false);
    assert_is_match("bcd", "[^a-c]*", opts.clone(), false);
    assert_is_match("bdir/", "[^a-c]*", opts.clone(), false);
    assert_is_match("Beware", "[^a-c]*", opts.clone(), true);
    assert_is_match("Beware", "[^a-c]*", bash_opts.clone(), true);
    assert_is_match("c", "[^a-c]*", opts.clone(), false);
    assert_is_match("ca", "[^a-c]*", opts.clone(), false);
    assert_is_match("cb", "[^a-c]*", opts.clone(), false);
    assert_is_match("d", "[^a-c]*", opts.clone(), true);
    assert_is_match("dd", "[^a-c]*", opts.clone(), true);
    assert_is_match("de", "[^a-c]*", opts.clone(), true);
    assert_is_match("baz", "[^a-c]*", opts.clone(), false);
    assert_is_match("bzz", "[^a-c]*", opts.clone(), false);
    assert_is_match("BZZ", "[^a-c]*", opts.clone(), true);
    assert_is_match("beware", "[^a-c]*", opts.clone(), false);
    assert_is_match("BewAre", "[^a-c]*", opts.clone(), true);
    // Additional assertions migrated from test/bash.js.
    assert_is_match("*", "[a-y]*[^c]", opts.clone(), false);
    assert_is_match("**", "[a-y]*[^c]", opts.clone(), false);
    assert_is_match("\\*", "[a-y]*[^c]", opts.clone(), false);
    assert_is_match("a", "[a-y]*[^c]", opts.clone(), false);
    assert_is_match("a/*", "[a-y]*[^c]", opts.clone(), false);
    assert_is_match("abc", "[a-y]*[^c]", opts.clone(), false);
    assert_is_match("abd", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("abe", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("b", "[a-y]*[^c]", opts.clone(), false);
    assert_is_match("bb", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("bcd", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("bdir/", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("Beware", "[a-y]*[^c]", opts.clone(), false);
    assert_is_match("c", "[a-y]*[^c]", opts.clone(), false);
    assert_is_match("ca", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("cb", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("d", "[a-y]*[^c]", opts.clone(), false);
    assert_is_match("dd", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("dd", "[a-y]*[^c]", regex_opts.clone(), true);
    assert_is_match("dd", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("de", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("baz", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("bzz", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("bzz", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("bzz", "[a-y]*[^c]", regex_opts.clone(), false);
    assert_is_match("BZZ", "[a-y]*[^c]", opts.clone(), false);
    assert_is_match("beware", "[a-y]*[^c]", opts.clone(), true);
    assert_is_match("BewAre", "[a-y]*[^c]", opts.clone(), false);
    assert_is_match("b", "a[b]c", opts.clone(), false);
    assert_is_match("bb", "a[b]c", opts.clone(), false);
    assert_is_match("bcd", "a[b]c", opts.clone(), false);
    assert_is_match("bdir/", "a[b]c", opts.clone(), false);
    assert_is_match("Beware", "a[b]c", opts.clone(), false);
    assert_is_match("c", "a[b]c", opts.clone(), false);
    assert_is_match("ca", "a[b]c", opts.clone(), false);
    assert_is_match("cb", "a[b]c", opts.clone(), false);
    assert_is_match("d", "a[b]c", opts.clone(), false);
    assert_is_match("dd", "a[b]c", opts.clone(), false);
    assert_is_match("de", "a[b]c", opts.clone(), false);
    assert_is_match("baz", "a[b]c", opts.clone(), false);
    assert_is_match("bzz", "a[b]c", opts.clone(), false);
    assert_is_match("BZZ", "a[b]c", opts.clone(), false);
    assert_is_match("beware", "a[b]c", opts.clone(), false);
    assert_is_match("BewAre", "a[b]c", opts.clone(), false);
    assert_is_match("**", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("\\*", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("a", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("a/*", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("abd", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("abe", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("b", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("bb", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("bcd", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("bdir/", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("Beware", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("c", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("ca", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("cb", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("d", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("dd", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("de", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("baz", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("bzz", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("BZZ", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("beware", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("BewAre", "a[\"b\"]c", opts.clone(), false);
    assert_is_match("**", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("\\*", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("a", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("a/*", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("abd", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("abe", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("b", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("bb", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("bcd", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("bdir/", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("Beware", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("c", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("ca", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("cb", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("d", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("dd", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("de", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("baz", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("bzz", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("BZZ", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("beware", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("BewAre", "a[\\\\b]c", opts.clone(), false);
    assert_is_match("*", "a[\\b]c", opts.clone(), false);
    assert_is_match("**", "a[\\b]c", opts.clone(), false);
    assert_is_match("\\*", "a[\\b]c", opts.clone(), false);
    assert_is_match("a", "a[\\b]c", opts.clone(), false);
    assert_is_match("a/*", "a[\\b]c", opts.clone(), false);
    assert_is_match("abd", "a[\\b]c", opts.clone(), false);
    assert_is_match("abe", "a[\\b]c", opts.clone(), false);
    assert_is_match("b", "a[\\b]c", opts.clone(), false);
    assert_is_match("bb", "a[\\b]c", opts.clone(), false);
    assert_is_match("bcd", "a[\\b]c", opts.clone(), false);
    assert_is_match("bdir/", "a[\\b]c", opts.clone(), false);
    assert_is_match("Beware", "a[\\b]c", opts.clone(), false);
    assert_is_match("c", "a[\\b]c", opts.clone(), false);
    assert_is_match("ca", "a[\\b]c", opts.clone(), false);
    assert_is_match("cb", "a[\\b]c", opts.clone(), false);
    assert_is_match("d", "a[\\b]c", opts.clone(), false);
    assert_is_match("dd", "a[\\b]c", opts.clone(), false);
    assert_is_match("de", "a[\\b]c", opts.clone(), false);
    assert_is_match("baz", "a[\\b]c", opts.clone(), false);
    assert_is_match("bzz", "a[\\b]c", opts.clone(), false);
    assert_is_match("BZZ", "a[\\b]c", opts.clone(), false);
    assert_is_match("beware", "a[\\b]c", opts.clone(), false);
    assert_is_match("BewAre", "a[\\b]c", opts.clone(), false);
    assert_is_match("**", "a[b-d]c", opts.clone(), false);
    assert_is_match("\\*", "a[b-d]c", opts.clone(), false);
    assert_is_match("a", "a[b-d]c", opts.clone(), false);
    assert_is_match("a/*", "a[b-d]c", opts.clone(), false);
    assert_is_match("abd", "a[b-d]c", opts.clone(), false);
    assert_is_match("abe", "a[b-d]c", opts.clone(), false);
    assert_is_match("b", "a[b-d]c", opts.clone(), false);
    assert_is_match("bb", "a[b-d]c", opts.clone(), false);
    assert_is_match("bcd", "a[b-d]c", opts.clone(), false);
    assert_is_match("bdir/", "a[b-d]c", opts.clone(), false);
    assert_is_match("Beware", "a[b-d]c", opts.clone(), false);
    assert_is_match("c", "a[b-d]c", opts.clone(), false);
    assert_is_match("ca", "a[b-d]c", opts.clone(), false);
    assert_is_match("cb", "a[b-d]c", opts.clone(), false);
    assert_is_match("d", "a[b-d]c", opts.clone(), false);
    assert_is_match("dd", "a[b-d]c", opts.clone(), false);
    assert_is_match("de", "a[b-d]c", opts.clone(), false);
    assert_is_match("baz", "a[b-d]c", opts.clone(), false);
    assert_is_match("bzz", "a[b-d]c", opts.clone(), false);
    assert_is_match("BZZ", "a[b-d]c", opts.clone(), false);
    assert_is_match("beware", "a[b-d]c", opts.clone(), false);
    assert_is_match("BewAre", "a[b-d]c", opts.clone(), false);
    assert_is_match("**", "a?c", opts.clone(), false);
    assert_is_match("\\*", "a?c", opts.clone(), false);
    assert_is_match("a", "a?c", opts.clone(), false);
    assert_is_match("a/*", "a?c", opts.clone(), false);
    assert_is_match("abd", "a?c", opts.clone(), false);
    assert_is_match("abe", "a?c", opts.clone(), false);
    assert_is_match("b", "a?c", opts.clone(), false);
    assert_is_match("bb", "a?c", opts.clone(), false);
    assert_is_match("bcd", "a?c", opts.clone(), false);
    assert_is_match("bdir/", "a?c", opts.clone(), false);
    assert_is_match("Beware", "a?c", opts.clone(), false);
    assert_is_match("c", "a?c", opts.clone(), false);
    assert_is_match("ca", "a?c", opts.clone(), false);
    assert_is_match("cb", "a?c", opts.clone(), false);
    assert_is_match("d", "a?c", opts.clone(), false);
    assert_is_match("dd", "a?c", opts.clone(), false);
    assert_is_match("de", "a?c", opts.clone(), false);
    assert_is_match("baz", "a?c", opts.clone(), false);
    assert_is_match("bzz", "a?c", opts.clone(), false);
    assert_is_match("BZZ", "a?c", opts.clone(), false);
    assert_is_match("beware", "a?c", opts.clone(), false);
    assert_is_match("BewAre", "a?c", opts.clone(), false);
    assert_is_match("a/*", "[^a-c]*", opts.clone(), false);

}

#[test]
fn should_support_basic_wildmatch_brackets_features() {
    let opts = default_compile_options();
    assert_is_match("aab", "a[]-]b", opts.clone(), false);
    assert_is_match("ten", "[ten]", opts.clone(), false);
    assert_is_match("]", "]", opts.clone(), true);
    assert_is_match("a-b", "a[]-]b", opts.clone(), true);
    assert_is_match("a]b", "a[]-]b", opts.clone(), true);
    assert_is_match("a]b", "a[]]b", opts.clone(), true);
    assert_is_match("aab", "a[\\]a\\-]b", opts.clone(), true);
    assert_is_match("ten", "t[a-g]n", opts.clone(), true);
    assert_is_match("ton", "t[^a-g]n", opts.clone(), true);
}

#[test]
fn should_support_extended_slash_matching_features() {
    let opts = default_compile_options();
    assert_is_match("foo/bar", "f[^eiu][^eiu][^eiu][^eiu][^eiu]r", opts.clone(), false);
    assert_is_match("foo/bar", "foo[/]bar", opts.clone(), true);
    assert_is_match("foo-bar", "f[^eiu][^eiu][^eiu][^eiu][^eiu]r", opts.clone(), true);
}

#[test]
fn should_match_escaped_characters() {
    let opts = default_compile_options();
    if !cfg!(windows) {
        assert_is_match("\\*", "\\*", opts.clone(), true);
        assert_is_match("XXX/\\", "[A-Z]+/\\\\", opts.clone(), true);
    }

    assert_is_match("[ab]", "\\[ab]", opts.clone(), true);
    assert_is_match("[ab]", "[\\[:]ab]", opts.clone(), true);
}

#[test]
fn should_consolidate_extra_stars() {
    let opts = default_compile_options();
    assert_is_match("bbc", "a**c", opts.clone(), false);
    assert_is_match("abc", "a**c", opts.clone(), true);
    assert_is_match("bbd", "a**c", opts.clone(), false);

    assert_is_match("bbc", "a***c", opts.clone(), false);
    assert_is_match("abc", "a***c", opts.clone(), true);
    assert_is_match("bbd", "a***c", opts.clone(), false);

    assert_is_match("bbc", "a*****?c", opts.clone(), false);
    assert_is_match("abc", "a*****?c", opts.clone(), true);
    assert_is_match("bbc", "a*****?c", opts.clone(), false);

    assert_is_match("bbc", "?*****??", opts.clone(), true);
    assert_is_match("abc", "?*****??", opts.clone(), true);

    assert_is_match("bbc", "*****??", opts.clone(), true);
    assert_is_match("abc", "*****??", opts.clone(), true);

    assert_is_match("bbc", "?*****?c", opts.clone(), true);
    assert_is_match("abc", "?*****?c", opts.clone(), true);

    assert_is_match("bbc", "?***?****c", opts.clone(), true);
    assert_is_match("abc", "?***?****c", opts.clone(), true);
    assert_is_match("bbd", "?***?****c", opts.clone(), false);

    assert_is_match("bbc", "?***?****?", opts.clone(), true);
    assert_is_match("abc", "?***?****?", opts.clone(), true);

    assert_is_match("bbc", "?***?****", opts.clone(), true);
    assert_is_match("abc", "?***?****", opts.clone(), true);

    assert_is_match("bbc", "*******c", opts.clone(), true);
    assert_is_match("abc", "*******c", opts.clone(), true);

    assert_is_match("bbc", "*******?", opts.clone(), true);
    assert_is_match("abc", "*******?", opts.clone(), true);

    assert_is_match("abcdecdhjk", "a*cd**?**??k", opts.clone(), true);
    assert_is_match("abcdecdhjk", "a**?**cd**?**??k", opts.clone(), true);
    assert_is_match("abcdecdhjk", "a**?**cd**?**??k***", opts.clone(), true);
    assert_is_match("abcdecdhjk", "a**?**cd**?**??***k", opts.clone(), true);
    assert_is_match("abcdecdhjk", "a**?**cd**?**??***k**", opts.clone(), true);
    assert_is_match("abcdecdhjk", "a****c**?**??*****", opts.clone(), true);
}

#[test]
fn none_of_these_should_output_anything() {
    let opts = default_compile_options();
    assert_is_match("abc", "??**********?****?", opts.clone(), false);
    assert_is_match("abc", "??**********?****c", opts.clone(), false);
    assert_is_match("abc", "?************c****?****", opts.clone(), false);
    assert_is_match("abc", "*c*?**", opts.clone(), false);
    assert_is_match("abc", "a*****c*?**", opts.clone(), false);
    assert_is_match("abc", "a********???*******", opts.clone(), false);
    assert_is_match("a", "[]", opts.clone(), false);
    assert_is_match("[", "[abc", opts.clone(), false);
}
