mod support;

use support::{assert_is_match, default_compile_options};

#[test]
fn basic_wildmat_features() {
    assert_is_match("foo", "*f", default_compile_options(), false);
    assert_is_match("foo", "??", default_compile_options(), false);
    assert_is_match("foo", "bar", default_compile_options(), false);
    assert_is_match("foobar", "foo\\*bar", default_compile_options(), false);
    assert_is_match("?a?b", "\\??\\?b", default_compile_options(), true);
    assert_is_match("aaaaaaabababab", "*ab", default_compile_options(), true);
    assert_is_match("foo", "*", default_compile_options(), true);
    assert_is_match("foo", "*foo*", default_compile_options(), true);
    assert_is_match("foo", "???", default_compile_options(), true);
    assert_is_match("foo", "f*", default_compile_options(), true);
    assert_is_match("foo", "foo", default_compile_options(), true);
    assert_is_match("foobar", "*ob*a*r*", default_compile_options(), true);
}

#[test]
fn should_support_recursion() {
    assert_is_match(
        "-adobe-courier-bold-o-normal--12-120-75-75-/-70-iso8859-1",
        "-*-*-*-*-*-*-12-*-*-*-m-*-*-*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "-adobe-courier-bold-o-normal--12-120-75-75-X-70-iso8859-1",
        "-*-*-*-*-*-*-12-*-*-*-m-*-*-*",
        default_compile_options(),
        false,
    );
    assert_is_match("ab/cXd/efXg/hi", "*X*i", default_compile_options(), false);
    assert_is_match("ab/cXd/efXg/hi", "*Xg*i", default_compile_options(), false);
    assert_is_match(
        "abcd/abcdefg/abcdefghijk/abcdefghijklmnop.txtz",
        "**/*a*b*g*n*t",
        default_compile_options(),
        false,
    );
    assert_is_match("foo", "*/*/*", default_compile_options(), false);
    assert_is_match("foo", "fo", default_compile_options(), false);
    assert_is_match("foo/bar", "*/*/*", default_compile_options(), false);
    assert_is_match("foo/bar", "foo?bar", default_compile_options(), false);
    assert_is_match("foo/bb/aa/rr", "*/*/*", default_compile_options(), false);
    assert_is_match("foo/bba/arr", "foo*", default_compile_options(), false);
    assert_is_match("foo/bba/arr", "foo**", default_compile_options(), false);
    assert_is_match("foo/bba/arr", "foo/*", default_compile_options(), false);
    assert_is_match("foo/bba/arr", "foo/**arr", default_compile_options(), false);
    assert_is_match("foo/bba/arr", "foo/**z", default_compile_options(), false);
    assert_is_match("foo/bba/arr", "foo/*arr", default_compile_options(), false);
    assert_is_match("foo/bba/arr", "foo/*z", default_compile_options(), false);
    assert_is_match(
        "XXX/adobe/courier/bold/o/normal//12/120/75/75/X/70/iso8859/1",
        "XXX/*/*/*/*/*/*/12/*/*/*/m/*/*/*",
        default_compile_options(),
        false,
    );
    assert_is_match(
        "-adobe-courier-bold-o-normal--12-120-75-75-m-70-iso8859-1",
        "-*-*-*-*-*-*-12-*-*-*-m-*-*-*",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "ab/cXd/efXg/hi",
        "**/*X*/**/*i",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "ab/cXd/efXg/hi",
        "*/*X*/*/*i",
        default_compile_options(),
        true,
    );
    assert_is_match(
        "abcd/abcdefg/abcdefghijk/abcdefghijklmnop.txt",
        "**/*a*b*g*n*t",
        default_compile_options(),
        true,
    );
    assert_is_match("abcXdefXghi", "*X*i", default_compile_options(), true);
    assert_is_match("foo", "foo", default_compile_options(), true);
    assert_is_match("foo/bar", "foo/*", default_compile_options(), true);
    assert_is_match("foo/bar", "foo/bar", default_compile_options(), true);
    assert_is_match("foo/bar", "foo[/]bar", default_compile_options(), true);
    assert_is_match("foo/bb/aa/rr", "**/**/**", default_compile_options(), true);
    assert_is_match("foo/bba/arr", "*/*/*", default_compile_options(), true);
    assert_is_match("foo/bba/arr", "foo/**", default_compile_options(), true);
}
