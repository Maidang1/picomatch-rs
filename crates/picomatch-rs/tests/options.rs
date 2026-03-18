mod support;

use picomatch_rs::{is_match, CompileOptions};
use support::{assert_is_match, default_compile_options};

fn match_list(fixtures: &[&str], pattern: &str, options: &CompileOptions) -> Vec<String> {
    let mut matches = Vec::new();

    for fixture in fixtures {
        // In picomatch-rs, normalization (like windows backslash swap) happens inside is_match
        match is_match(fixture, pattern, options) {
            Ok(true) => {
                // Mirror JS support/match.js: normalize windows paths and dedupe matches.
                let value = if options.windows {
                    fixture.replace('\\', "/")
                } else {
                    fixture.to_string()
                };

                if !matches.contains(&value) {
                    matches.push(value);
                }
            }
            Ok(false) => {}
            Err(err) => panic!("is_match({fixture:?}, {pattern:?}) failed: {err:?}"),
        }
    }

    matches
}

#[test]
fn options_match_base() {
    let mut opts = default_compile_options();
    opts.windows = true;

    // should match the basename of file paths when `options.matchBase` is true
    assert_eq!(
        match_list(&["a/b/c/d.md"], "*.md", &opts),
        Vec::<String>::new()
    );
    assert_eq!(
        match_list(&["a/b/c/foo.md"], "*.md", &opts),
        Vec::<String>::new()
    );
    assert_eq!(
        match_list(
            &["ab", "acb", "acb/", "acb/d/e", "x/y/acb", "x/y/acb/d"],
            "a?b",
            &opts
        ),
        vec!["acb"]
    );

    let mut opts_mb = opts.clone();
    opts_mb.match_base = true;
    assert_eq!(
        match_list(&["a/b/c/d.md"], "*.md", &opts_mb),
        vec!["a/b/c/d.md"]
    );
    assert_eq!(
        match_list(&["a/b/c/foo.md"], "*.md", &opts_mb),
        vec!["a/b/c/foo.md"]
    );

    let matches = match_list(
        &["x/y/acb", "acb/", "acb/d/e", "x/y/acb/d"],
        "a?b",
        &opts_mb,
    );
    assert!(matches.contains(&"x/y/acb".to_string()));
    assert!(matches.contains(&"acb/".to_string()));
    assert_eq!(matches.len(), 2);

    // should work with negation patterns
    assert_is_match("./x/y.js", "*.js", opts_mb.clone(), true);
    assert_is_match("./x/y.js", "!*.js", opts_mb.clone(), false);
    assert_is_match("./x/y.js", "**/*.js", opts_mb.clone(), true);
    assert_is_match("./x/y.js", "!**/*.js", opts_mb.clone(), false);
}

#[test]
fn options_flags() {
    let mut opts = default_compile_options();
    opts.windows = true;

    // should be case-sensitive by default
    assert_eq!(
        match_list(&["a/b/d/e.md"], "a/b/D/*.md", &opts),
        Vec::<String>::new()
    );
    assert_eq!(
        match_list(&["a/b/c/e.md"], "A/b/*/E.md", &opts),
        Vec::<String>::new()
    );
    assert_eq!(
        match_list(&["a/b/c/e.md"], "A/b/C/*.MD", &opts),
        Vec::<String>::new()
    );

    // should not be case-sensitive when `i` is set on `options.flags`
    let mut opts_i = opts.clone();
    opts_i.flags = "i".to_string();
    assert_eq!(
        match_list(&["a/b/d/e.md"], "a/b/D/*.md", &opts_i),
        vec!["a/b/d/e.md"]
    );
    assert_eq!(
        match_list(&["a/b/c/e.md"], "A/b/*/E.md", &opts_i),
        vec!["a/b/c/e.md"]
    );
    assert_eq!(
        match_list(&["a/b/c/e.md"], "A/b/C/*.MD", &opts_i),
        vec!["a/b/c/e.md"]
    );
}

#[test]
fn options_nocase() {
    let mut opts = default_compile_options();
    opts.windows = true;
    opts.nocase = true;

    // should not be case-sensitive when `options.nocase` is true
    assert_eq!(
        match_list(&["a/b/c/e.md"], "A/b/*/E.md", &opts),
        vec!["a/b/c/e.md"]
    );
    assert_eq!(
        match_list(&["a/b/c/e.md"], "A/b/C/*.MD", &opts),
        vec!["a/b/c/e.md"]
    );
    assert_eq!(
        match_list(&["a/b/c/e.md"], "A/b/C/*.md", &opts),
        vec!["a/b/c/e.md"]
    );
    assert_eq!(
        match_list(&["a/b/d/e.md"], "a/b/D/*.md", &opts),
        vec!["a/b/d/e.md"]
    );

    // should not double-set `i` when both `nocase` and the `i` flag are set
    let mut opts_dual = opts.clone();
    opts_dual.flags = "i".to_string();
    assert_eq!(
        match_list(&["a/b/d/e.md"], "a/b/D/*.md", &opts_dual),
        vec!["a/b/d/e.md"]
    );
    assert_eq!(
        match_list(&["a/b/c/e.md"], "A/b/*/E.md", &opts_dual),
        vec!["a/b/c/e.md"]
    );
    assert_eq!(
        match_list(&["a/b/c/e.md"], "A/b/C/*.MD", &opts_dual),
        vec!["a/b/c/e.md"]
    );
}

#[test]
fn options_noextglob() {
    let mut opts = default_compile_options();
    opts.windows = true;
    opts.noextglob = true;

    // should match literal parens when noextglob is true (issue #116)
    assert_is_match("a/(dir)", "a/(dir)", opts.clone(), true);

    // should not match extglobs when noextglob is true
    assert_is_match("ax", "?(a*|b)", opts.clone(), false);
    assert_eq!(
        match_list(&["a.j.js", "a.md.js"], "*.*(j).js", &opts),
        vec!["a.j.js"]
    );
    assert_eq!(
        match_list(&["a/z", "a/b", "a/!(z)"], "a/!(z)", &opts),
        vec!["a/!(z)"]
    );
    assert_eq!(
        match_list(&["a/z", "a/b"], "a/!(z)", &opts),
        Vec::<String>::new()
    );
    assert_eq!(
        match_list(&["c/a/v"], "c/!(z)/v", &opts),
        Vec::<String>::new()
    );
    assert_eq!(
        match_list(&["c/z/v", "c/a/v"], "c/!(z)/v", &opts),
        Vec::<String>::new()
    );
    assert_eq!(
        match_list(&["c/z/v", "c/a/v"], "c/@(z)/v", &opts),
        Vec::<String>::new()
    );
    assert_eq!(
        match_list(&["c/z/v", "c/a/v"], "c/+(z)/v", &opts),
        Vec::<String>::new()
    );

    // In JS: match(['c/z/v', 'c/a/v'], 'c/*(z)/v', { noextglob: true }) => ['c/z/v']
    // Because *(z) becomes literal * followed by (z)
    assert_eq!(
        match_list(&["c/z/v", "c/a/v"], "c/*(z)/v", &opts),
        vec!["c/z/v"]
    );

    assert_eq!(
        match_list(&["c/z/v", "z", "zf", "fz"], "?(z)", &opts),
        vec!["fz"]
    );
    assert_eq!(
        match_list(&["c/z/v", "z", "zf", "fz"], "+(z)", &opts),
        Vec::<String>::new()
    );
    assert_eq!(
        match_list(&["c/z/v", "z", "zf", "fz"], "*(z)", &opts),
        vec!["z", "fz"]
    );

    assert_eq!(
        match_list(&["cz", "abz", "az"], "a@(z)", &opts),
        Vec::<String>::new()
    );
    assert_eq!(
        match_list(&["cz", "abz", "az"], "a*@(z)", &opts),
        Vec::<String>::new()
    );
    assert_eq!(
        match_list(&["cz", "abz", "az"], "a!(z)", &opts),
        Vec::<String>::new()
    );

    assert_eq!(
        match_list(&["cz", "abz", "az", "azz"], "a?(z)", &opts),
        vec!["abz", "azz"]
    );
    assert_eq!(
        match_list(&["cz", "abz", "az", "azz", "a+z"], "a+(z)", &opts),
        vec!["a+z"]
    );
    assert_eq!(
        match_list(&["cz", "abz", "az"], "a*(z)", &opts),
        vec!["abz", "az"]
    );
    assert_eq!(
        match_list(&["cz", "abz", "az"], "a**(z)", &opts),
        vec!["abz", "az"]
    );
    assert_eq!(
        match_list(&["cz", "abz", "az"], "a*!(z)", &opts),
        Vec::<String>::new()
    );
}

#[test]
fn options_unescape() {
    let mut opts = default_compile_options();
    opts.windows = true;

    let fixtures = &["abc", "/a/b/c", "\\a\\b\\c"];

    // should remove backslashes in glob patterns
    assert_eq!(match_list(fixtures, "\\a\\b\\c", &opts), vec!["/a/b/c"]);

    let mut opts_un = opts.clone();
    opts_un.unescape = true;
    let results = match_list(fixtures, "\\a\\b\\c", &opts_un);
    assert!(results.contains(&"abc".to_string()));
    assert!(results.contains(&"/a/b/c".to_string()));
    assert_eq!(results.len(), 2);

    let mut opts_no_un = opts.clone();
    opts_no_un.unescape = false;
    assert_eq!(
        match_list(fixtures, "\\a\\b\\c", &opts_no_un),
        vec!["/a/b/c"]
    );
}

#[test]
fn options_nonegate() {
    let mut opts = default_compile_options();
    opts.windows = true;

    // should support the `nonegate` option
    assert_eq!(
        match_list(
            &["a/a/a", "a/b/a", "b/b/a", "c/c/a", "c/c/b"],
            "!**/a",
            &opts
        ),
        vec!["c/c/b"]
    );

    let mut opts_ng = opts.clone();
    opts_ng.nonegate = true;

    assert_eq!(
        match_list(&["a.md", "!a.md", "a.txt"], "!*.md", &opts_ng),
        vec!["!a.md"]
    );
    assert_eq!(
        match_list(
            &["!a/a/a", "!a/a", "a/b/a", "b/b/a", "!c/c/a", "!c/a"],
            "!**/a",
            &opts_ng
        ),
        vec!["!a/a", "!c/a"]
    );
    assert_eq!(
        match_list(
            &["!*.md", ".dotfile.txt", "a/b/.dotfile"],
            "!*.md",
            &opts_ng
        ),
        vec!["!*.md"]
    );
}

#[test]
fn options_windows() {
    let mut opts_win = default_compile_options();
    opts_win.windows = true;

    // should windows file paths by default
    assert_eq!(
        match_list(&["a\\b\\c.md"], "**/*.md", &opts_win),
        vec!["a/b/c.md"]
    );

    let mut opts_no_win = default_compile_options();
    opts_no_win.windows = false;
    assert_eq!(
        match_list(&["a\\b\\c.md"], "**/*.md", &opts_no_win),
        vec!["a\\b\\c.md"]
    );

    // should windows absolute paths
    assert_eq!(
        match_list(&["E:\\a\\b\\c.md"], "E:/**/*.md", &opts_win),
        vec!["E:/a/b/c.md"]
    );
    assert_eq!(
        match_list(&["E:\\a\\b\\c.md"], "E:/**/*.md", &opts_no_win),
        Vec::<String>::new()
    );
}

#[test]
fn options_windows_strip_leading_dot_slash() {
    let mut fixtures = vec![
        "./a",
        "./a/a/a",
        "./a/a/a/a",
        "./a/a/a/a/a",
        "./a/b",
        "./a/x",
        "./z/z",
        "a",
        "a/a",
        "a/a/b",
        "a/c",
        "b",
        "x/y",
    ];
    fixtures.sort();

    let mut opts = default_compile_options();
    opts.windows = true;

    // In JS: const format = str => str.replace(/^\.\//, '');
    // Note: JS test uses match(fixtures, '*', { format, windows: true })
    // It means it applies format to each item in fixtures before matching.
    // However, it also means it uses a Set to deduplicate since both "a" and "./a" become "a".
    // picomatch-rs match_list helper should handle duplicates if needed, or we just expect them to be there.
    // support/match.js uses a Set.

    fn match_list_formatted(
        fixtures: &[&str],
        pattern: &str,
        options: &CompileOptions,
    ) -> Vec<String> {
        let mut matches = std::collections::HashSet::new();
        for fixture in fixtures {
            let formatted = fixture.strip_prefix("./").unwrap_or(fixture);
            if let Ok(true) = is_match(formatted, pattern, options) {
                if options.windows {
                    matches.insert(formatted.replace('\\', "/"));
                } else {
                    matches.insert(formatted.to_string());
                }
            }
        }
        let mut res: Vec<String> = matches.into_iter().collect();
        res.sort();
        res
    }

    assert_eq!(match_list_formatted(&fixtures, "*", &opts), vec!["a", "b"]);
    assert_eq!(
        match_list_formatted(&fixtures, "**/a/**", &opts),
        vec![
            "a",
            "a/a",
            "a/a/a",
            "a/a/a/a",
            "a/a/a/a/a",
            "a/a/b",
            "a/b",
            "a/c",
            "a/x"
        ]
    );

    let mut res_1_2 = match_list_formatted(&fixtures, "*/*", &opts);
    res_1_2.sort();
    assert_eq!(res_1_2, vec!["a/a", "a/b", "a/c", "a/x", "x/y", "z/z"]);

    assert_eq!(
        match_list_formatted(&fixtures, "*/*/*", &opts),
        vec!["a/a/a", "a/a/b"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "*/*/*/*", &opts),
        vec!["a/a/a/a"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "*/*/*/*/*", &opts),
        vec!["a/a/a/a/a"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "./*", &opts),
        vec!["a", "b"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "./**/a/**", &opts),
        vec![
            "a",
            "a/a",
            "a/a/a",
            "a/a/a/a",
            "a/a/a/a/a",
            "a/a/b",
            "a/b",
            "a/c",
            "a/x"
        ]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "./a/*/a", &opts),
        vec!["a/a/a"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "a/*", &opts),
        vec!["a/a", "a/b", "a/c", "a/x"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "a/*/*", &opts),
        vec!["a/a/a", "a/a/b"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "a/*/*/*", &opts),
        vec!["a/a/a/a"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "a/*/*/*/*", &opts),
        vec!["a/a/a/a/a"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "a/*/a", &opts),
        vec!["a/a/a"]
    );

    let mut opts_no_win = opts.clone();
    opts_no_win.windows = false;

    assert_eq!(
        match_list_formatted(&fixtures, "*", &opts_no_win),
        vec!["a", "b"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "**/a/**", &opts_no_win),
        vec![
            "a",
            "a/a",
            "a/a/a",
            "a/a/a/a",
            "a/a/a/a/a",
            "a/a/b",
            "a/b",
            "a/c",
            "a/x"
        ]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "*/*", &opts_no_win),
        vec!["a/a", "a/b", "a/c", "a/x", "x/y", "z/z"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "*/*/*", &opts_no_win),
        vec!["a/a/a", "a/a/b"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "*/*/*/*", &opts_no_win),
        vec!["a/a/a/a"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "*/*/*/*/*", &opts_no_win),
        vec!["a/a/a/a/a"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "./*", &opts_no_win),
        vec!["a", "b"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "./**/a/**", &opts_no_win),
        vec![
            "a",
            "a/a",
            "a/a/a",
            "a/a/a/a",
            "a/a/a/a/a",
            "a/a/b",
            "a/b",
            "a/c",
            "a/x"
        ]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "./a/*/a", &opts_no_win),
        vec!["a/a/a"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "a/*", &opts_no_win),
        vec!["a/a", "a/b", "a/c", "a/x"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "a/*/*", &opts_no_win),
        vec!["a/a/a", "a/a/b"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "a/*/*/*", &opts_no_win),
        vec!["a/a/a/a"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "a/*/*/*/*", &opts_no_win),
        vec!["a/a/a/a/a"]
    );
    assert_eq!(
        match_list_formatted(&fixtures, "a/*/a", &opts_no_win),
        vec!["a/a/a"]
    );
}

#[test]
fn windows_paths() {
    let mut opts_win = default_compile_options();
    opts_win.windows = true;

    // should convert file paths to posix slashes
    assert_eq!(
        match_list(&["a\\b\\c.md"], "**/*.md", &opts_win),
        vec!["a/b/c.md"]
    );

    let mut opts_no_win = default_compile_options();
    opts_no_win.windows = false;
    assert_eq!(
        match_list(&["a\\b\\c.md"], "**/*.md", &opts_no_win),
        vec!["a\\b\\c.md"]
    );

    // should convert absolute paths to posix slashes
    assert_eq!(
        match_list(&["E:\\a\\b\\c.md"], "E:/**/*.md", &opts_win),
        vec!["E:/a/b/c.md"]
    );
    assert_eq!(
        match_list(&["E:\\a\\b\\c.md"], "E:/**/*.md", &opts_no_win),
        Vec::<String>::new()
    );
}
