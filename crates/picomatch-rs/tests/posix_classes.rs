mod support;

use picomatch_rs::{is_match, make_re, parse, CompileOptions};
use support::assert_is_match;

fn convert(input: &str, options: &CompileOptions) -> String {
    parse(input, options).map(|s| s.output).unwrap_or_default()
}

fn posix_opts() -> CompileOptions {
    CompileOptions {
        strict_slashes: true,
        posix: true,
        regex: true,
        ..CompileOptions::default()
    }
}

#[test]
fn posix_bracket_type_conversion() {
    let opts = posix_opts();
    assert_eq!(convert("foo[[:lower:]]bar", &opts), "foo[a-z]bar");
    assert_eq!(
        convert("foo[[:lower:][:upper:]]bar", &opts),
        "foo[a-zA-Z]bar"
    );
    assert_eq!(convert("[[:alpha:]123]", &opts), "(?=.)[a-zA-Z123]");
    assert_eq!(convert("[[:lower:]]", &opts), "(?=.)[a-z]");
    assert_eq!(convert("[![:lower:]]", &opts), "(?=.)[^a-z]");
    assert_eq!(
        convert("[[:digit:][:upper:][:space:]]", &opts),
        r"(?=.)[0-9A-Z \t\r\n\v\f]"
    );
    assert_eq!(convert("[[:xdigit:]]", &opts), "(?=.)[A-Fa-f0-9]");

    // Use r## for very complex punct set
    let expected_all = "(?=.)[a-zA-Z0-9a-zA-Z \t\\x00-\\x1F\\x7F0-9\\x21-\\x7Ea-z\\x20-\\x7E \\-!\"#$%&'()\\*+,./:;<=>?@\\[\\\\\\]^_`{|}~ \\t\\r\\n\\v\\fA-ZA-Fa-f0-9]";
    assert_eq!(
        convert("[[:alnum:][:alpha:][:blank:][:cntrl:][:digit:][:graph:][:lower:][:print:][:punct:][:space:][:upper:][:xdigit:]]", &opts),
        expected_all
    );

    assert_eq!(
        convert(
            "[^[:alnum:][:alpha:][:blank:][:cntrl:][:digit:][:lower:][:space:][:upper:][:xdigit:]]",
            &opts
        ),
        "(?=.)[^a-zA-Z0-9a-zA-Z \t\\x00-\\x1F\\x7F0-9a-z \\t\\r\\n\\v\\fA-ZA-Fa-f0-9]"
    );
    assert_eq!(convert("[a-c[:digit:]x-z]", &opts), "(?=.)[a-c0-9x-z]");
    assert_eq!(
        convert("[_[:alpha:]][_[:alnum:]][_[:alnum:]]*", &opts),
        "(?=.)[_a-zA-Z][_a-zA-Z0-9][_a-zA-Z0-9]*"
    );
}

#[test]
fn is_match_posix_classes() {
    let opts = posix_opts();

    assert_is_match("e", "[[:xdigit:]]", opts.clone(), true);
    assert_is_match("a", "[[:alpha:]123]", opts.clone(), true);
    assert_is_match("1", "[[:alpha:]123]", opts.clone(), true);
    assert_is_match("5", "[[:alpha:]123]", opts.clone(), false);
    assert_is_match("A", "[[:alpha:]123]", opts.clone(), true);

    assert_is_match("A", "[[:alpha:]]", opts.clone(), true);
    assert_is_match("9", "[[:alpha:]]", opts.clone(), false);
    assert_is_match("b", "[[:alpha:]]", opts.clone(), true);

    assert_is_match("A", "[![:alpha:]]", opts.clone(), false);
    assert_is_match("9", "[![:alpha:]]", opts.clone(), true);
    assert_is_match("b", "[![:alpha:]]", opts.clone(), false);

    assert_is_match("A", "[^[:alpha:]]", opts.clone(), false);
    assert_is_match("9", "[^[:alpha:]]", opts.clone(), true);
    assert_is_match("b", "[^[:alpha:]]", opts.clone(), false);

    assert_is_match("A", "[[:digit:]]", opts.clone(), false);
    assert_is_match("9", "[[:digit:]]", opts.clone(), true);
    assert_is_match("b", "[[:digit:]]", opts.clone(), false);

    assert_is_match("A", "[^[:digit:]]", opts.clone(), true);
    assert_is_match("9", "[^[:digit:]]", opts.clone(), false);
    assert_is_match("b", "[^[:digit:]]", opts.clone(), true);

    assert_is_match("A", "[![:digit:]]", opts.clone(), true);
    assert_is_match("9", "[![:digit:]]", opts.clone(), false);
    assert_is_match("b", "[![:digit:]]", opts.clone(), true);

    assert_is_match("a", "[[:lower:]]", opts.clone(), true);
    assert_is_match("A", "[[:lower:]]", opts.clone(), false);
    assert_is_match("9", "[[:lower:]]", opts.clone(), false);

    assert_is_match("a", "[:alpha:]", opts.clone(), true);
    assert_is_match("l", "[:alpha:]", opts.clone(), true);
    assert_is_match("p", "[:alpha:]", opts.clone(), true);
    assert_is_match("h", "[:alpha:]", opts.clone(), true);
    assert_is_match(":", "[:alpha:]", opts.clone(), true);
    assert_is_match("b", "[:alpha:]", opts.clone(), false);
}

#[test]
fn multiple_posix_brackets() {
    let opts = posix_opts();

    assert_is_match("9", "[[:lower:][:digit:]]", opts.clone(), true);
    assert_is_match("a", "[[:lower:][:digit:]]", opts.clone(), true);
    assert_is_match("A", "[[:lower:][:digit:]]", opts.clone(), false);
    assert_is_match("aa", "[[:lower:][:digit:]]", opts.clone(), false);
    assert_is_match("99", "[[:lower:][:digit:]]", opts.clone(), false);
    assert_is_match("a9", "[[:lower:][:digit:]]", opts.clone(), false);
    assert_is_match("9a", "[[:lower:][:digit:]]", opts.clone(), false);
    assert_is_match("aA", "[[:lower:][:digit:]]", opts.clone(), false);
    assert_is_match("9A", "[[:lower:][:digit:]]", opts.clone(), false);

    assert_is_match("aa", "[[:lower:][:digit:]]+", opts.clone(), true);
    assert_is_match("99", "[[:lower:][:digit:]]+", opts.clone(), true);
    assert_is_match("a9", "[[:lower:][:digit:]]+", opts.clone(), true);
    assert_is_match("9a", "[[:lower:][:digit:]]+", opts.clone(), true);
    assert_is_match("aA", "[[:lower:][:digit:]]+", opts.clone(), false);
    assert_is_match("9A", "[[:lower:][:digit:]]+", opts.clone(), false);

    assert_is_match("a", "[[:lower:][:digit:]]*", opts.clone(), true);
    assert_is_match("A", "[[:lower:][:digit:]]*", opts.clone(), false);
    assert_is_match("AA", "[[:lower:][:digit:]]*", opts.clone(), false);
    assert_is_match("aa", "[[:lower:][:digit:]]*", opts.clone(), true);
    assert_is_match("aaa", "[[:lower:][:digit:]]*", opts.clone(), true);
    assert_is_match("999", "[[:lower:][:digit:]]*", opts.clone(), true);
}

#[test]
fn word_characters() {
    let opts = posix_opts();

    assert_is_match("a c", "a[[:word:]]+c", opts.clone(), false);
    assert_is_match("a.c", "a[[:word:]]+c", opts.clone(), false);
    assert_is_match("a.xy.zc", "a[[:word:]]+c", opts.clone(), false);
    assert_is_match("a.zc", "a[[:word:]]+c", opts.clone(), false);
    assert_is_match("abq", "a[[:word:]]+c", opts.clone(), false);
    assert_is_match("axy zc", "a[[:word:]]+c", opts.clone(), false);
    assert_is_match("axy", "a[[:word:]]+c", opts.clone(), false);
    assert_is_match("axy.zc", "a[[:word:]]+c", opts.clone(), false);
    assert_is_match("a123c", "a[[:word:]]+c", opts.clone(), true);
    assert_is_match("a1c", "a[[:word:]]+c", opts.clone(), true);
    assert_is_match("abbbbc", "a[[:word:]]+c", opts.clone(), true);
    assert_is_match("abbbc", "a[[:word:]]+c", opts.clone(), true);
    assert_is_match("abbc", "a[[:word:]]+c", opts.clone(), true);
    assert_is_match("abc", "a[[:word:]]+c", opts.clone(), true);

    assert_is_match("a c", "a[[:word:]]+", opts.clone(), false);
    assert_is_match("a.c", "a[[:word:]]+", opts.clone(), false);
    assert_is_match("a.xy.zc", "a[[:word:]]+", opts.clone(), false);
    assert_is_match("a.zc", "a[[:word:]]+", opts.clone(), false);
    assert_is_match("axy zc", "a[[:word:]]+", opts.clone(), false);
    assert_is_match("axy.zc", "a[[:word:]]+", opts.clone(), false);
    assert_is_match("a123c", "a[[:word:]]+", opts.clone(), true);
    assert_is_match("a1c", "a[[:word:]]+", opts.clone(), true);
    assert_is_match("abbbbc", "a[[:word:]]+", opts.clone(), true);
    assert_is_match("abbbc", "a[[:word:]]+", opts.clone(), true);
    assert_is_match("abbc", "a[[:word:]]+", opts.clone(), true);
    assert_is_match("abc", "a[[:word:]]+", opts.clone(), true);
    assert_is_match("abq", "a[[:word:]]+", opts.clone(), true);
    assert_is_match("axy", "a[[:word:]]+", opts.clone(), true);
    assert_is_match("axyzc", "a[[:word:]]+", opts.clone(), true);
}

#[test]
fn should_not_create_an_invalid_posix_character_class() {
    let opts = posix_opts();
    let res1 = convert("[:al:]", &opts);
    assert_eq!(res1, r"(?:\[:al:\]|[:al:])", "convert('[:al:]') failed");

    let res2 = convert("[abc[:punct:][0-9]", &opts);
    let expected = r##"(?=.)[abc\-!"#$%&'()\*+,./:;<=>?@\[\\\]^_`{|}~\[0-9]"##;
    assert_eq!(res2, expected, "convert('[abc[:punct:][0-9]') failed");
}

#[test]
fn matches_positive() {
    let opts = posix_opts();
    assert_is_match("a", "[[:lower:]]", opts.clone(), true);
    assert_is_match("A", "[[:upper:]]", opts.clone(), true);
    assert_is_match("A", "[[:digit:][:upper:][:space:]]", opts.clone(), true);
    assert_is_match("1", "[[:digit:][:upper:][:space:]]", opts.clone(), true);
    assert_is_match(" ", "[[:digit:][:upper:][:space:]]", opts.clone(), true);
    assert_is_match("5", "[[:xdigit:]]", opts.clone(), true);
    assert_is_match("f", "[[:xdigit:]]", opts.clone(), true);
    assert_is_match("D", "[[:xdigit:]]", opts.clone(), true);
    assert_is_match("_", "[[:alnum:][:alpha:][:blank:][:cntrl:][:digit:][:graph:][:lower:][:print:][:punct:][:space:][:upper:][:xdigit:]]", opts.clone(), true);
    assert_is_match(
        ".",
        "[^[:alnum:][:alpha:][:blank:][:cntrl:][:digit:][:lower:][:space:][:upper:][:xdigit:]]",
        opts.clone(),
        true,
    );
    assert_is_match("5", "[a-c[:digit:]x-z]", opts.clone(), true);
    assert_is_match("b", "[a-c[:digit:]x-z]", opts.clone(), true);
    assert_is_match("y", "[a-c[:digit:]x-z]", opts.clone(), true);
}

#[test]
fn matches_negative() {
    let opts = posix_opts();
    assert_is_match("A", "[[:lower:]]", opts.clone(), false);
    assert_is_match("A", "[![:lower:]]", opts.clone(), true);
    assert_is_match("a", "[[:upper:]]", opts.clone(), false);
    assert_is_match("a", "[[:digit:][:upper:][:space:]]", opts.clone(), false);
    assert_is_match(".", "[[:digit:][:upper:][:space:]]", opts.clone(), false);
    assert_is_match(
        ".",
        "[[:alnum:][:alpha:][:blank:][:cntrl:][:digit:][:lower:][:space:][:upper:][:xdigit:]]",
        opts.clone(),
        false,
    );
    assert_is_match("q", "[a-c[:digit:]x-z]", opts.clone(), false);
}

#[test]
fn literals_and_escaping() {
    let opts = posix_opts();
    assert_is_match("a [b]", "a [b]", opts.clone(), true);
    assert_is_match("a b", "a [b]", opts.clone(), true);

    assert_is_match("a [b] c", "a [b] c", opts.clone(), true);
    assert_is_match("a b c", "a [b] c", opts.clone(), true);

    assert_is_match("a [b]", "a \\[b\\]", opts.clone(), true);
    assert_is_match("a b", "a \\[b\\]", opts.clone(), false);

    assert_is_match("a [b]", "a ([b])", opts.clone(), true);
    assert_is_match("a b", "a ([b])", opts.clone(), true);

    assert_is_match("a b", "a (\\[b\\]|[b])", opts.clone(), true);
    assert_is_match("a [b]", "a (\\[b\\]|[b])", opts.clone(), true);
}

#[test]
fn make_re_tests() {
    let opts = posix_opts();
    let res = make_re("[[:alpha:]123]", &opts, false).unwrap();
    assert_eq!(res.source, "^(?:(?=.)[a-zA-Z123])$");

    let res = make_re("[![:lower:]]", &opts, false).unwrap();
    assert_eq!(res.source, "^(?:(?=.)[^a-z])$");
}

#[test]
fn posix_2_bre_tests() {
    let opts = posix_opts();

    assert_is_match("e", "[[:xdigit:]]", opts.clone(), true);
    assert_is_match("1", "[[:xdigit:]]", opts.clone(), true);
    assert_is_match("a", "[[:alpha:]123]", opts.clone(), true);
    assert_is_match("1", "[[:alpha:]123]", opts.clone(), true);

    assert_is_match("9", "[![:alpha:]]", opts.clone(), true);
    assert_is_match("9", "[^[:alpha:]]", opts.clone(), true);

    assert_is_match("A", "[[:word:]]", opts.clone(), true);
    assert_is_match("B", "[[:word:]]", opts.clone(), true);
    assert_is_match("a", "[[:word:]]", opts.clone(), true);
    assert_is_match("b", "[[:word:]]", opts.clone(), true);
    assert_is_match("1", "[[:word:]]", opts.clone(), true);
    assert_is_match("2", "[[:word:]]", opts.clone(), true);

    assert_is_match("1", "[[:digit:]]", opts.clone(), true);
    assert_is_match("2", "[[:digit:]]", opts.clone(), true);
    assert_is_match("a", "[[:digit:]]", opts.clone(), false);
    assert_is_match("A", "[[:digit:]]", opts.clone(), false);

    assert_is_match("A", "[[:upper:]]", opts.clone(), true);
    assert_is_match("B", "[[:upper:]]", opts.clone(), true);
    assert_is_match("a", "[[:upper:]]", opts.clone(), false);
    assert_is_match("b", "[[:upper:]]", opts.clone(), false);
    assert_is_match("1", "[[:upper:]]", opts.clone(), false);

    assert_is_match("a", "[[:lower:]]", opts.clone(), true);
    assert_is_match("b", "[[:lower:]]", opts.clone(), true);
    assert_is_match("A", "[[:lower:]]", opts.clone(), false);
    assert_is_match("B", "[[:lower:]]", opts.clone(), false);

    assert_is_match("aA", "[[:lower:]][[:upper:]]", opts.clone(), true);
    assert_is_match("AA", "[[:lower:]][[:upper:]]", opts.clone(), false);
    assert_is_match("Aa", "[[:lower:]][[:upper:]]", opts.clone(), false);

    assert_is_match("ababab", "[[:xdigit:]]*", opts.clone(), true);
    assert_is_match("020202", "[[:xdigit:]]*", opts.clone(), true);
    assert_is_match("900", "[[:xdigit:]]*", opts.clone(), true);

    assert_is_match("!", "[[:punct:]]", opts.clone(), true);
    assert_is_match("?", "[[:punct:]]", opts.clone(), true);
    assert_is_match("#", "[[:punct:]]", opts.clone(), true);
    assert_is_match("&", "[[:punct:]]", opts.clone(), true);
    assert_is_match("@", "[[:punct:]]", opts.clone(), true);
    assert_is_match("+", "[[:punct:]]", opts.clone(), true);
    assert_is_match("*", "[[:punct:]]", opts.clone(), true);
    assert_is_match(":", "[[:punct:]]", opts.clone(), true);
    assert_eq!(is_match("=", "[[:punct:]]", &opts).unwrap_or(false), true);
    assert_is_match("|", "[[:punct:]]", opts.clone(), true);
    assert_is_match("|++", "[[:punct:]]*", opts.clone(), true);

    assert_is_match("?*+", "[[:punct:]]", opts.clone(), false);
    assert_is_match("?*+", "[[:punct:]]*", opts.clone(), true);
    assert_is_match("foo", "foo[[:punct:]]*", opts.clone(), true);
    assert_is_match("foo?*+", "foo[[:punct:]]*", opts.clone(), true);

    assert_eq!(is_match("a", "[:al:]", &opts).unwrap_or(false), true);
    assert_eq!(is_match("a", "[[:al:]", &opts).unwrap_or(false), false);
    assert_eq!(
        is_match("!", "[abc[:punct:][0-9]", &opts).unwrap_or(false),
        true
    );

    assert_is_match("PATH", "[_[:alpha:]]*", opts.clone(), true);
    assert_is_match("PATH", "[_[:alpha:]][_[:alnum:]]*", opts.clone(), true);

    assert_is_match(
        "a1B",
        "[[:alpha:]][[:digit:]][[:upper:]]",
        opts.clone(),
        true,
    );
    assert_is_match(
        "a1b",
        "[[:alpha:]][[:digit:]][[:upper:]]",
        opts.clone(),
        false,
    );
    assert_is_match(".", "[[:digit:][:punct:][:space:]]", opts.clone(), true);
    assert_is_match("a", "[[:digit:][:punct:][:space:]]", opts.clone(), false);
    assert_is_match("!", "[[:digit:][:punct:][:space:]]", opts.clone(), true);
    assert_is_match(
        "!",
        "[[:digit:]][[:punct:]][[:space:]]",
        opts.clone(),
        false,
    );
    assert_is_match(
        "1! ",
        "[[:digit:]][[:punct:]][[:space:]]",
        opts.clone(),
        true,
    );
    assert_is_match(
        "1!  ",
        "[[:digit:]][[:punct:]][[:space:]]",
        opts.clone(),
        false,
    );
}

#[test]
fn bash_unit_tests_ported() {
    let opts = posix_opts();

    assert_is_match("9", "[[:digit:]]", opts.clone(), true);
    assert_is_match("X", "[[:digit:]]", opts.clone(), false);
    assert_is_match("aB", "[[:lower:]][[:upper:]]", opts.clone(), true);
    assert_is_match("a", "[[:alpha:][:digit:]]", opts.clone(), true);
    assert_is_match("3", "[[:alpha:][:digit:]]", opts.clone(), true);
    assert_is_match("aa", "[[:alpha:][:digit:]]", opts.clone(), false);
    assert_is_match("a3", "[[:alpha:][:digit:]]", opts.clone(), false);

    assert_eq!(
        is_match("a", "[[:alpha:]\\]", &opts).unwrap_or(false),
        false
    );
    assert_eq!(
        is_match("b", "[[:alpha:]\\]", &opts).unwrap_or(false),
        false
    );

    assert_is_match("\t", "[[:blank:]]", opts.clone(), true);
    assert_is_match("\t", "[[:space:]]", opts.clone(), true);
    assert_is_match(" ", "[[:space:]]", opts.clone(), true);
    assert_is_match("9", "[1[:alpha:]123]", opts.clone(), false);
    assert_is_match(" ", "[[:punct:]]", opts.clone(), false);
    assert_is_match("A", "[[:graph:]]", opts.clone(), true);
    assert_is_match("\x08", "[[:graph:]]", opts.clone(), false);
    assert_is_match("\n", "[[:graph:]]", opts.clone(), false);
    assert_is_match(" ", "[[:graph:]]", opts.clone(), false);
}
