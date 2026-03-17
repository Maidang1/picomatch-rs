mod support;

use picomatch_rs::CompileOptions;

use support::{assert_is_match, default_compile_options};

fn bash_options(windows: bool) -> CompileOptions {
    CompileOptions {
        bash: true,
        windows,
        ..CompileOptions::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let opts = bash_options(true);
        assert_is_match("", "*(0|1|3|5|7|9)", opts.clone(), false);
    }

    #[test]
    fn test_2() {
        let opts = bash_options(true);
        assert_is_match("*(a|b[)", "*(a|b\\[)", opts.clone(), false);
    }

    #[test]
    fn test_3() {
        let opts = bash_options(true);
        assert_is_match("*(a|b[)", "\\*\\(a|b\\[\\)", opts.clone(), false);
    }

    #[test]
    fn test_4() {
        let opts = bash_options(true);
        assert_is_match("***", "\\*\\*\\*", opts.clone(), true);
    }

    #[test]
    fn test_5() {
        let opts = bash_options(true);
        assert_is_match(
            "-adobe-courier-bold-o-normal--12-120-75-75-/-70-iso8859-1",
            "-*-*-*-*-*-*-12-*-*-*-m-*-*-*",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_6() {
        let opts = bash_options(true);
        assert_is_match(
            "-adobe-courier-bold-o-normal--12-120-75-75-m-70-iso8859-1",
            "-*-*-*-*-*-*-12-*-*-*-m-*-*-*",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_7() {
        let opts = bash_options(true);
        assert_is_match(
            "-adobe-courier-bold-o-normal--12-120-75-75-X-70-iso8859-1",
            "-*-*-*-*-*-*-12-*-*-*-m-*-*-*",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_8() {
        let opts = bash_options(true);
        assert_is_match(
            "/dev/udp/129.22.8.102/45",
            "/dev\\/@(tcp|udp)\\/*\\/*",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_9() {
        let opts = bash_options(true);
        assert_is_match("/x/y/z", "/x/y/z", opts.clone(), true);
    }

    #[test]
    fn test_10() {
        let opts = bash_options(true);
        assert_is_match("0377", "+([0-7])", opts.clone(), true);
    }

    #[test]
    fn test_11() {
        let opts = bash_options(true);
        assert_is_match("07", "+([0-7])", opts.clone(), true);
    }

    #[test]
    fn test_12() {
        let opts = bash_options(true);
        assert_is_match("09", "+([0-7])", opts.clone(), false);
    }

    #[test]
    fn test_13() {
        let opts = bash_options(true);
        assert_is_match("1", "0|[1-9]*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_14() {
        let opts = bash_options(true);
        assert_is_match("12", "0|[1-9]*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_15() {
        let opts = bash_options(true);
        assert_is_match("123abc", "(a+|b)*", opts.clone(), false);
    }

    #[test]
    fn test_16() {
        let opts = bash_options(true);
        assert_is_match("123abc", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_17() {
        let opts = bash_options(true);
        assert_is_match("123abc", "*?(a)bc", opts.clone(), true);
    }

    #[test]
    fn test_18() {
        let opts = bash_options(true);
        assert_is_match("123abc", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_19() {
        let opts = bash_options(true);
        assert_is_match("123abc", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_20() {
        let opts = bash_options(true);
        assert_is_match("123abc", "ab**", opts.clone(), false);
    }

    #[test]
    fn test_21() {
        let opts = bash_options(true);
        assert_is_match("123abc", "ab**(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_22() {
        let opts = bash_options(true);
        assert_is_match("123abc", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_23() {
        let opts = bash_options(true);
        assert_is_match("123abc", "ab***ef", opts.clone(), false);
    }

    #[test]
    fn test_24() {
        let opts = bash_options(true);
        assert_is_match("123abc", "ab*+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_25() {
        let opts = bash_options(true);
        assert_is_match("123abc", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_26() {
        let opts = bash_options(true);
        assert_is_match("123abc", "ab?*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_27() {
        let opts = bash_options(true);
        assert_is_match("12abc", "0|[1-9]*([0-9])", opts.clone(), false);
    }

    #[test]
    fn test_28() {
        let opts = bash_options(true);
        assert_is_match("137577991", "*(0|1|3|5|7|9)", opts.clone(), true);
    }

    #[test]
    fn test_29() {
        let opts = bash_options(true);
        assert_is_match("2468", "*(0|1|3|5|7|9)", opts.clone(), false);
    }

    #[test]
    fn test_30() {
        let opts = bash_options(true);
        assert_is_match("?a?b", "\\??\\?b", opts.clone(), true);
    }

    #[test]
    fn test_31() {
        let opts = bash_options(true);
        assert_is_match("\\a\\b\\c", "abc", opts.clone(), false);
    }

    #[test]
    fn test_32() {
        let opts = bash_options(true);
        assert_is_match("a", "!(*.a|*.b|*.c)", opts.clone(), true);
    }

    #[test]
    fn test_33() {
        let opts = bash_options(true);
        assert_is_match("a", "!(a)", opts.clone(), false);
    }

    #[test]
    fn test_34() {
        let opts = bash_options(true);
        assert_is_match("a", "!(a)*", opts.clone(), false);
    }

    #[test]
    fn test_35() {
        let opts = bash_options(true);
        assert_is_match("a", "(a)", opts.clone(), true);
    }

    #[test]
    fn test_36() {
        let opts = bash_options(true);
        assert_is_match("a", "(b)", opts.clone(), false);
    }

    #[test]
    fn test_37() {
        let opts = bash_options(true);
        assert_is_match("a", "*(a)", opts.clone(), true);
    }

    #[test]
    fn test_38() {
        let opts = bash_options(true);
        assert_is_match("a", "+(a)", opts.clone(), true);
    }

    #[test]
    fn test_39() {
        let opts = bash_options(true);
        assert_is_match("a", "?", opts.clone(), true);
    }

    #[test]
    fn test_40() {
        let opts = bash_options(true);
        assert_is_match("a", "?(a|b)", opts.clone(), true);
    }

    #[test]
    fn test_41() {
        let opts = bash_options(true);
        assert_is_match("a", "??", opts.clone(), false);
    }

    #[test]
    fn test_42() {
        let opts = bash_options(true);
        assert_is_match("a", "a!(b)*", opts.clone(), true);
    }

    #[test]
    fn test_43() {
        let opts = bash_options(true);
        assert_is_match("a", "a?(a|b)", opts.clone(), true);
    }

    #[test]
    fn test_44() {
        let opts = bash_options(true);
        assert_is_match("a", "a?(x)", opts.clone(), true);
    }

    #[test]
    fn test_45() {
        let opts = bash_options(true);
        assert_is_match("a", "a??b", opts.clone(), false);
    }

    #[test]
    fn test_46() {
        let opts = bash_options(true);
        assert_is_match("a", "b?(a|b)", opts.clone(), false);
    }

    #[test]
    fn test_47() {
        let opts = bash_options(true);
        assert_is_match("a((((b", "a(*b", opts.clone(), true);
    }

    #[test]
    fn test_48() {
        let opts = bash_options(true);
        assert_is_match("a((((b", "a(b", opts.clone(), false);
    }

    #[test]
    fn test_49() {
        let opts = bash_options(true);
        assert_is_match("a((((b", "a\\(b", opts.clone(), false);
    }

    #[test]
    fn test_50() {
        let opts = bash_options(true);
        assert_is_match("a((b", "a(*b", opts.clone(), true);
    }

    #[test]
    fn test_51() {
        let opts = bash_options(true);
        assert_is_match("a((b", "a(b", opts.clone(), false);
    }

    #[test]
    fn test_52() {
        let opts = bash_options(true);
        assert_is_match("a((b", "a\\(b", opts.clone(), false);
    }

    #[test]
    fn test_53() {
        let opts = bash_options(true);
        assert_is_match("a(b", "a(*b", opts.clone(), true);
    }

    #[test]
    fn test_54() {
        let opts = bash_options(true);
        assert_is_match("a(b", "a(b", opts.clone(), true);
    }

    #[test]
    fn test_55() {
        let opts = bash_options(true);
        assert_is_match("a\\(b", "a\\(b", opts.clone(), true);
    }

    #[test]
    fn test_56() {
        let opts = bash_options(true);
        assert_is_match("a(b", "a\\(b", opts.clone(), true);
    }

    #[test]
    fn test_57() {
        let opts = bash_options(true);
        assert_is_match("a.", "!(*.a|*.b|*.c)", opts.clone(), true);
    }

    #[test]
    fn test_58() {
        let opts = bash_options(true);
        assert_is_match("a.", "*!(.a|.b|.c)", opts.clone(), true);
    }

    #[test]
    fn test_59() {
        let opts = bash_options(true);
        assert_is_match("a.", "*.!(a)", opts.clone(), true);
    }

    #[test]
    fn test_60() {
        let opts = bash_options(true);
        assert_is_match("a.", "*.!(a|b|c)", opts.clone(), true);
    }

    #[test]
    fn test_61() {
        let opts = bash_options(true);
        assert_is_match("a.", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), false);
    }

    #[test]
    fn test_62() {
        let opts = bash_options(true);
        assert_is_match("a.", "*.+(b|d)", opts.clone(), false);
    }

    #[test]
    fn test_63() {
        let opts = bash_options(true);
        assert_is_match("a.a", "!(*.[a-b]*)", opts.clone(), false);
    }

    #[test]
    fn test_64() {
        let opts = bash_options(true);
        assert_is_match("a.a", "!(*.a|*.b|*.c)", opts.clone(), false);
    }

    #[test]
    fn test_65() {
        let opts = bash_options(true);
        assert_is_match("a.a", "!(*[a-b].[a-b]*)", opts.clone(), false);
    }

    #[test]
    fn test_66() {
        let opts = bash_options(true);
        assert_is_match("a.a", "!*.(a|b)", opts.clone(), false);
    }

    #[test]
    fn test_67() {
        let opts = bash_options(true);
        assert_is_match("a.a", "!*.(a|b)*", opts.clone(), false);
    }

    #[test]
    fn test_68() {
        let opts = bash_options(true);
        assert_is_match("a.a", "(a|d).(a|b)*", opts.clone(), true);
    }

    #[test]
    fn test_69() {
        let opts = bash_options(true);
        assert_is_match("a.a", "(b|a).(a)", opts.clone(), true);
    }

    #[test]
    fn test_70() {
        let opts = bash_options(true);
        assert_is_match("a.a", "*!(.a|.b|.c)", opts.clone(), true);
    }

    #[test]
    fn test_71() {
        let opts = bash_options(true);
        assert_is_match("a.a", "*.!(a)", opts.clone(), false);
    }

    #[test]
    fn test_72() {
        let opts = bash_options(true);
        assert_is_match("a.a", "*.!(a|b|c)", opts.clone(), false);
    }

    #[test]
    fn test_73() {
        let opts = bash_options(true);
        assert_is_match("a.a", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), true);
    }

    #[test]
    fn test_74() {
        let opts = bash_options(true);
        assert_is_match("a.a", "*.+(b|d)", opts.clone(), false);
    }

    #[test]
    fn test_75() {
        let opts = bash_options(true);
        assert_is_match("a.a", "@(b|a).@(a)", opts.clone(), true);
    }

    #[test]
    fn test_76() {
        let opts = bash_options(true);
        assert_is_match("a.a.a", "!(*.[a-b]*)", opts.clone(), false);
    }

    #[test]
    fn test_77() {
        let opts = bash_options(true);
        assert_is_match("a.a.a", "!(*[a-b].[a-b]*)", opts.clone(), false);
    }

    #[test]
    fn test_78() {
        let opts = bash_options(true);
        assert_is_match("a.a.a", "!*.(a|b)", opts.clone(), false);
    }

    #[test]
    fn test_79() {
        let opts = bash_options(true);
        assert_is_match("a.a.a", "!*.(a|b)*", opts.clone(), false);
    }

    #[test]
    fn test_80() {
        let opts = bash_options(true);
        assert_is_match("a.a.a", "*.!(a)", opts.clone(), true);
    }

    #[test]
    fn test_81() {
        let opts = bash_options(true);
        assert_is_match("a.a.a", "*.+(b|d)", opts.clone(), false);
    }

    #[test]
    fn test_82() {
        let opts = bash_options(true);
        assert_is_match("a.aa.a", "(b|a).(a)", opts.clone(), false);
    }

    #[test]
    fn test_83() {
        let opts = bash_options(true);
        assert_is_match("a.aa.a", "@(b|a).@(a)", opts.clone(), false);
    }

    #[test]
    fn test_84() {
        let opts = bash_options(true);
        assert_is_match("a.abcd", "!(*.a|*.b|*.c)", opts.clone(), true);
    }

    #[test]
    fn test_85() {
        let opts = bash_options(true);
        assert_is_match("a.abcd", "!(*.a|*.b|*.c)*", opts.clone(), false);
    }

    #[test]
    fn test_86() {
        let opts = bash_options(true);
        assert_is_match("a.abcd", "*!(*.a|*.b|*.c)*", opts.clone(), true);
    }

    #[test]
    fn test_87() {
        let opts = bash_options(true);
        assert_is_match("a.abcd", "*!(.a|.b|.c)", opts.clone(), true);
    }

    #[test]
    fn test_88() {
        let opts = bash_options(true);
        assert_is_match("a.abcd", "*.!(a|b|c)", opts.clone(), true);
    }

    #[test]
    fn test_89() {
        let opts = bash_options(true);
        assert_is_match("a.abcd", "*.!(a|b|c)*", opts.clone(), false);
    }

    #[test]
    fn test_90() {
        let opts = bash_options(true);
        assert_is_match("a.abcd", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), true);
    }

    #[test]
    fn test_91() {
        let opts = bash_options(true);
        assert_is_match("a.b", "!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_92() {
        let opts = bash_options(true);
        assert_is_match("a.b", "!(*.[a-b]*)", opts.clone(), false);
    }

    #[test]
    fn test_93() {
        let opts = bash_options(true);
        assert_is_match("a.b", "!(*.a|*.b|*.c)", opts.clone(), false);
    }

    #[test]
    fn test_94() {
        let opts = bash_options(true);
        assert_is_match("a.b", "!(*[a-b].[a-b]*)", opts.clone(), false);
    }

    #[test]
    fn test_95() {
        let opts = bash_options(true);
        assert_is_match("a.b", "!*.(a|b)", opts.clone(), false);
    }

    #[test]
    fn test_96() {
        let opts = bash_options(true);
        assert_is_match("a.b", "!*.(a|b)*", opts.clone(), false);
    }

    #[test]
    fn test_97() {
        let opts = bash_options(true);
        assert_is_match("a.b", "(a|d).(a|b)*", opts.clone(), true);
    }

    #[test]
    fn test_98() {
        let opts = bash_options(true);
        assert_is_match("a.b", "*!(.a|.b|.c)", opts.clone(), true);
    }

    #[test]
    fn test_99() {
        let opts = bash_options(true);
        assert_is_match("a.b", "*.!(a)", opts.clone(), true);
    }

    #[test]
    fn test_100() {
        let opts = bash_options(true);
        assert_is_match("a.b", "*.!(a|b|c)", opts.clone(), false);
    }

    #[test]
    fn test_101() {
        let opts = bash_options(true);
        assert_is_match("a.b", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), true);
    }

    #[test]
    fn test_102() {
        let opts = bash_options(true);
        assert_is_match("a.b", "*.+(b|d)", opts.clone(), true);
    }

    #[test]
    fn test_103() {
        let opts = bash_options(true);
        assert_is_match("a.bb", "!(*.[a-b]*)", opts.clone(), false);
    }

    #[test]
    fn test_104() {
        let opts = bash_options(true);
        assert_is_match("a.bb", "!(*[a-b].[a-b]*)", opts.clone(), false);
    }

    #[test]
    fn test_105() {
        let opts = bash_options(true);
        assert_is_match("a.bb", "!*.(a|b)", opts.clone(), true);
    }

    #[test]
    fn test_106() {
        let opts = bash_options(true);
        assert_is_match("a.bb", "!*.(a|b)*", opts.clone(), false);
    }

    #[test]
    fn test_107() {
        let opts = bash_options(true);
        assert_is_match("a.bb", "!*.*(a|b)", opts.clone(), false);
    }

    #[test]
    fn test_108() {
        let opts = bash_options(true);
        assert_is_match("a.bb", "(a|d).(a|b)*", opts.clone(), true);
    }

    #[test]
    fn test_109() {
        let opts = bash_options(true);
        assert_is_match("a.bb", "(b|a).(a)", opts.clone(), false);
    }

    #[test]
    fn test_110() {
        let opts = bash_options(true);
        assert_is_match("a.bb", "*.+(b|d)", opts.clone(), true);
    }

    #[test]
    fn test_111() {
        let opts = bash_options(true);
        assert_is_match("a.bb", "@(b|a).@(a)", opts.clone(), false);
    }

    #[test]
    fn test_112() {
        let opts = bash_options(true);
        assert_is_match("a.c", "!(*.a|*.b|*.c)", opts.clone(), false);
    }

    #[test]
    fn test_113() {
        let opts = bash_options(true);
        assert_is_match("a.c", "*!(.a|.b|.c)", opts.clone(), true);
    }

    #[test]
    fn test_114() {
        let opts = bash_options(true);
        assert_is_match("a.c", "*.!(a|b|c)", opts.clone(), false);
    }

    #[test]
    fn test_115() {
        let opts = bash_options(true);
        assert_is_match("a.c", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), false);
    }

    #[test]
    fn test_116() {
        let opts = bash_options(true);
        assert_is_match("a.c.d", "!(*.a|*.b|*.c)", opts.clone(), true);
    }

    #[test]
    fn test_117() {
        let opts = bash_options(true);
        assert_is_match("a.c.d", "*!(.a|.b|.c)", opts.clone(), true);
    }

    #[test]
    fn test_118() {
        let opts = bash_options(true);
        assert_is_match("a.c.d", "*.!(a|b|c)", opts.clone(), true);
    }

    #[test]
    fn test_119() {
        let opts = bash_options(true);
        assert_is_match("a.c.d", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), false);
    }

    #[test]
    fn test_120() {
        let opts = bash_options(true);
        assert_is_match("a.ccc", "!(*.[a-b]*)", opts.clone(), true);
    }

    #[test]
    fn test_121() {
        let opts = bash_options(true);
        assert_is_match("a.ccc", "!(*[a-b].[a-b]*)", opts.clone(), true);
    }

    #[test]
    fn test_122() {
        let opts = bash_options(true);
        assert_is_match("a.ccc", "!*.(a|b)", opts.clone(), true);
    }

    #[test]
    fn test_123() {
        let opts = bash_options(true);
        assert_is_match("a.ccc", "!*.(a|b)*", opts.clone(), true);
    }

    #[test]
    fn test_124() {
        let opts = bash_options(true);
        assert_is_match("a.ccc", "*.+(b|d)", opts.clone(), false);
    }

    #[test]
    fn test_125() {
        let opts = bash_options(true);
        assert_is_match("a.js", "!(*.js)", opts.clone(), false);
    }

    #[test]
    fn test_126() {
        let opts = bash_options(true);
        assert_is_match("a.js", "*!(.js)", opts.clone(), true);
    }

    #[test]
    fn test_127() {
        let opts = bash_options(true);
        assert_is_match("a.js", "*.!(js)", opts.clone(), false);
    }

    #[test]
    fn test_128() {
        let opts = bash_options(true);
        assert_is_match("a.js", "a.!(js)", opts.clone(), false);
    }

    #[test]
    fn test_129() {
        let opts = bash_options(true);
        assert_is_match("a.js", "a.!(js)*", opts.clone(), false);
    }

    #[test]
    fn test_130() {
        let opts = bash_options(true);
        assert_is_match("a.js.js", "!(*.js)", opts.clone(), false);
    }

    #[test]
    fn test_131() {
        let opts = bash_options(true);
        assert_is_match("a.js.js", "*!(.js)", opts.clone(), true);
    }

    #[test]
    fn test_132() {
        let opts = bash_options(true);
        assert_is_match("a.js.js", "*.!(js)", opts.clone(), true);
    }

    #[test]
    fn test_133() {
        let opts = bash_options(true);
        assert_is_match("a.js.js", "*.*(js).js", opts.clone(), true);
    }

    #[test]
    fn test_134() {
        let opts = bash_options(true);
        assert_is_match("a.md", "!(*.js)", opts.clone(), true);
    }

    #[test]
    fn test_135() {
        let opts = bash_options(true);
        assert_is_match("a.md", "*!(.js)", opts.clone(), true);
    }

    #[test]
    fn test_136() {
        let opts = bash_options(true);
        assert_is_match("a.md", "*.!(js)", opts.clone(), true);
    }

    #[test]
    fn test_137() {
        let opts = bash_options(true);
        assert_is_match("a.md", "a.!(js)", opts.clone(), true);
    }

    #[test]
    fn test_138() {
        let opts = bash_options(true);
        assert_is_match("a.md", "a.!(js)*", opts.clone(), true);
    }

    #[test]
    fn test_139() {
        let opts = bash_options(true);
        assert_is_match("a.md.js", "*.*(js).js", opts.clone(), false);
    }

    #[test]
    fn test_140() {
        let opts = bash_options(true);
        assert_is_match("a.txt", "a.!(js)", opts.clone(), true);
    }

    #[test]
    fn test_141() {
        let opts = bash_options(true);
        assert_is_match("a.txt", "a.!(js)*", opts.clone(), true);
    }

    #[test]
    fn test_142() {
        let opts = bash_options(true);
        assert_is_match("a/!(z)", "a/!(z)", opts.clone(), true);
    }

    #[test]
    fn test_143() {
        let opts = bash_options(true);
        assert_is_match("a/b", "a/!(z)", opts.clone(), true);
    }

    #[test]
    fn test_144() {
        let opts = bash_options(true);
        assert_is_match("a/b/c.txt", "*/b/!(*).txt", opts.clone(), false);
    }

    #[test]
    fn test_145() {
        let opts = bash_options(true);
        assert_is_match("a/b/c.txt", "*/b/!(c).txt", opts.clone(), false);
    }

    #[test]
    fn test_146() {
        let opts = bash_options(true);
        assert_is_match("a/b/c.txt", "*/b/!(cc).txt", opts.clone(), true);
    }

    #[test]
    fn test_147() {
        let opts = bash_options(true);
        assert_is_match("a/b/cc.txt", "*/b/!(*).txt", opts.clone(), false);
    }

    #[test]
    fn test_148() {
        let opts = bash_options(true);
        assert_is_match("a/b/cc.txt", "*/b/!(c).txt", opts.clone(), false);
    }

    #[test]
    fn test_149() {
        let opts = bash_options(true);
        assert_is_match("a/b/cc.txt", "*/b/!(cc).txt", opts.clone(), false);
    }

    #[test]
    fn test_150() {
        let opts = bash_options(true);
        assert_is_match("a/dir/foo.txt", "*/dir/**/!(bar).txt", opts.clone(), true);
    }

    #[test]
    fn test_151() {
        let opts = bash_options(true);
        assert_is_match("a/z", "a/!(z)", opts.clone(), false);
    }

    #[test]
    fn test_152() {
        let opts = bash_options(true);
        assert_is_match("a\\(b", "a(*b", opts.clone(), false);
    }

    #[test]
    fn test_153() {
        let opts = bash_options(true);
        assert_is_match("a\\(b", "a(b", opts.clone(), false);
    }

    #[test]
    fn test_154() {
        let opts = bash_options(false);
        assert_is_match("a\\\\z", "a\\\\z", opts.clone(), true);
    }

    #[test]
    fn test_155() {
        let opts = bash_options(true);
        assert_is_match("a\\\\z", "a\\\\z", opts.clone(), true);
    }

    #[test]
    fn test_156() {
        let opts = bash_options(true);
        assert_is_match("a\\b", "a/b", opts.clone(), true);
    }

    #[test]
    fn test_157() {
        let opts = bash_options(true);
        assert_is_match("a\\\\z", "a\\\\z", opts.clone(), true);
    }

    #[test]
    fn test_158() {
        let opts = bash_options(true);
        assert_is_match("a\\z", "a\\z", opts.clone(), true);
    }

    #[test]
    fn test_159() {
        let opts = bash_options(true);
        assert_is_match("a\\z", "a\\z", opts.clone(), true);
    }

    #[test]
    fn test_160() {
        let opts = bash_options(true);
        assert_is_match("aa", "!(a!(b))", opts.clone(), false);
    }

    #[test]
    fn test_161() {
        let opts = bash_options(true);
        assert_is_match("aa", "!(a)", opts.clone(), true);
    }

    #[test]
    fn test_162() {
        let opts = bash_options(true);
        assert_is_match("aa", "!(a)*", opts.clone(), false);
    }

    #[test]
    fn test_163() {
        let opts = bash_options(true);
        assert_is_match("aa", "?", opts.clone(), false);
    }

    #[test]
    fn test_164() {
        let opts = bash_options(true);
        assert_is_match("aa", "@(a)b", opts.clone(), false);
    }

    #[test]
    fn test_165() {
        let opts = bash_options(true);
        assert_is_match("aa", "a!(b)*", opts.clone(), true);
    }

    #[test]
    fn test_166() {
        let opts = bash_options(true);
        assert_is_match("aa", "a??b", opts.clone(), false);
    }

    #[test]
    fn test_167() {
        let opts = bash_options(true);
        assert_is_match("aa.aa", "(b|a).(a)", opts.clone(), false);
    }

    #[test]
    fn test_168() {
        let opts = bash_options(true);
        assert_is_match("aa.aa", "@(b|a).@(a)", opts.clone(), false);
    }

    #[test]
    fn test_169() {
        let opts = bash_options(true);
        assert_is_match("aaa", "!(a)*", opts.clone(), false);
    }

    #[test]
    fn test_170() {
        let opts = bash_options(true);
        assert_is_match("aaa", "a!(b)*", opts.clone(), true);
    }

    #[test]
    fn test_171() {
        let opts = bash_options(true);
        assert_is_match("aaaaaaabababab", "*ab", opts.clone(), true);
    }

    #[test]
    fn test_172() {
        let opts = bash_options(true);
        assert_is_match("aaac", "*(@(a))a@(c)", opts.clone(), true);
    }

    #[test]
    fn test_173() {
        let opts = bash_options(true);
        assert_is_match("aaaz", "[a*(]*z", opts.clone(), true);
    }

    #[test]
    fn test_174() {
        let opts = bash_options(true);
        assert_is_match("aab", "!(a)*", opts.clone(), false);
    }

    #[test]
    fn test_175() {
        let opts = bash_options(true);
        assert_is_match("aab", "?", opts.clone(), false);
    }

    #[test]
    fn test_176() {
        let opts = bash_options(true);
        assert_is_match("aab", "??", opts.clone(), false);
    }

    #[test]
    fn test_177() {
        let opts = bash_options(true);
        assert_is_match("aab", "@(c)b", opts.clone(), false);
    }

    #[test]
    fn test_178() {
        let opts = bash_options(true);
        assert_is_match("aab", "a!(b)*", opts.clone(), true);
    }

    #[test]
    fn test_179() {
        let opts = bash_options(true);
        assert_is_match("aab", "a??b", opts.clone(), false);
    }

    #[test]
    fn test_180() {
        let opts = bash_options(true);
        assert_is_match("aac", "*(@(a))a@(c)", opts.clone(), true);
    }

    #[test]
    fn test_181() {
        let opts = bash_options(true);
        assert_is_match("aac", "*(@(a))b@(c)", opts.clone(), false);
    }

    #[test]
    fn test_182() {
        let opts = bash_options(true);
        assert_is_match("aax", "a!(a*|b)", opts.clone(), false);
    }

    #[test]
    fn test_183() {
        let opts = bash_options(true);
        assert_is_match("aax", "a!(x*|b)", opts.clone(), true);
    }

    #[test]
    fn test_184() {
        let opts = bash_options(true);
        assert_is_match("aax", "a?(a*|b)", opts.clone(), true);
    }

    #[test]
    fn test_185() {
        let opts = bash_options(true);
        assert_is_match("aaz", "[a*(]*z", opts.clone(), true);
    }

    #[test]
    fn test_186() {
        let opts = bash_options(true);
        assert_is_match("ab", "!(*.*)", opts.clone(), true);
    }

    #[test]
    fn test_187() {
        let opts = bash_options(true);
        assert_is_match("ab", "!(a!(b))", opts.clone(), true);
    }

    #[test]
    fn test_188() {
        let opts = bash_options(true);
        assert_is_match("ab", "!(a)*", opts.clone(), false);
    }

    #[test]
    fn test_189() {
        let opts = bash_options(true);
        assert_is_match("ab", "@(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_190() {
        let opts = bash_options(true);
        assert_is_match("ab", "(a+|b)+", opts.clone(), true);
    }

    #[test]
    fn test_191() {
        let opts = bash_options(true);
        assert_is_match("ab", "*?(a)bc", opts.clone(), false);
    }

    #[test]
    fn test_192() {
        let opts = bash_options(true);
        assert_is_match("ab", "a!(*(b|B))", opts.clone(), false);
    }

    #[test]
    fn test_193() {
        let opts = bash_options(true);
        assert_is_match("ab", "a!(@(b|B))", opts.clone(), false);
    }

    #[test]
    fn test_194() {
        let opts = bash_options(true);
        assert_is_match("aB", "a!(@(b|B))", opts.clone(), false);
    }

    #[test]
    fn test_195() {
        let opts = bash_options(true);
        assert_is_match("ab", "a!(b)*", opts.clone(), false);
    }

    #[test]
    fn test_196() {
        let opts = bash_options(true);
        assert_is_match("ab", "a(*b", opts.clone(), false);
    }

    #[test]
    fn test_197() {
        let opts = bash_options(true);
        assert_is_match("ab", "a(b", opts.clone(), false);
    }

    #[test]
    fn test_198() {
        let opts = bash_options(true);
        assert_is_match("ab", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_199() {
        let opts = bash_options(true);
        assert_is_match("ab", "a/b", opts.clone(), false);
    }

    #[test]
    fn test_200() {
        let opts = bash_options(true);
        assert_is_match("ab", "a\\(b", opts.clone(), false);
    }

    #[test]
    fn test_201() {
        let opts = bash_options(true);
        assert_is_match("ab", "ab*(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_202() {
        let opts = bash_options(true);
        assert_is_match("ab", "ab**", opts.clone(), true);
    }

    #[test]
    fn test_203() {
        let opts = bash_options(true);
        assert_is_match("ab", "ab**(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_204() {
        let opts = bash_options(true);
        assert_is_match("ab", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_205() {
        let opts = bash_options(true);
        assert_is_match("ab", "ab***ef", opts.clone(), false);
    }

    #[test]
    fn test_206() {
        let opts = bash_options(true);
        assert_is_match("ab", "ab*+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_207() {
        let opts = bash_options(true);
        assert_is_match("ab", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_208() {
        let opts = bash_options(true);
        assert_is_match("ab", "ab?*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_209() {
        let opts = bash_options(true);
        assert_is_match("ab/cXd/efXg/hi", "**/*X*/**/*i", opts.clone(), true);
    }

    #[test]
    fn test_210() {
        let opts = bash_options(true);
        assert_is_match("ab/cXd/efXg/hi", "*/*X*/*/*i", opts.clone(), true);
    }

    #[test]
    fn test_211() {
        let opts = bash_options(true);
        assert_is_match("ab/cXd/efXg/hi", "*X*i", opts.clone(), true);
    }

    #[test]
    fn test_212() {
        let opts = bash_options(true);
        assert_is_match("ab/cXd/efXg/hi", "*Xg*i", opts.clone(), true);
    }

    #[test]
    fn test_213() {
        let opts = bash_options(true);
        assert_is_match("ab]", "a!(@(b|B))", opts.clone(), true);
    }

    #[test]
    fn test_214() {
        let opts = bash_options(true);
        assert_is_match("abab", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_215() {
        let opts = bash_options(true);
        assert_is_match("abab", "(a+|b)+", opts.clone(), true);
    }

    #[test]
    fn test_216() {
        let opts = bash_options(true);
        assert_is_match("abab", "*?(a)bc", opts.clone(), false);
    }

    #[test]
    fn test_217() {
        let opts = bash_options(true);
        assert_is_match("abab", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_218() {
        let opts = bash_options(true);
        assert_is_match("abab", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_219() {
        let opts = bash_options(true);
        assert_is_match("abab", "ab**", opts.clone(), true);
    }

    #[test]
    fn test_220() {
        let opts = bash_options(true);
        assert_is_match("abab", "ab**(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_221() {
        let opts = bash_options(true);
        assert_is_match("abab", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_222() {
        let opts = bash_options(true);
        assert_is_match("abab", "ab***ef", opts.clone(), false);
    }

    #[test]
    fn test_223() {
        let opts = bash_options(true);
        assert_is_match("abab", "ab*+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_224() {
        let opts = bash_options(true);
        assert_is_match("abab", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_225() {
        let opts = bash_options(true);
        assert_is_match("abab", "ab?*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_226() {
        let opts = bash_options(true);
        assert_is_match("abb", "!(*.*)", opts.clone(), true);
    }

    #[test]
    fn test_227() {
        let opts = bash_options(true);
        assert_is_match("abb", "!(a)*", opts.clone(), false);
    }

    #[test]
    fn test_228() {
        let opts = bash_options(true);
        assert_is_match("abb", "a!(b)*", opts.clone(), false);
    }

    #[test]
    fn test_229() {
        let opts = bash_options(true);
        assert_is_match("abbcd", "@(ab|a*(b))*(c)d", opts.clone(), true);
    }

    #[test]
    fn test_230() {
        let opts = bash_options(true);
        assert_is_match("abc", "\\a\\b\\c", opts.clone(), false);
    }

    #[test]
    fn test_231() {
        let opts = bash_options(true);
        assert_is_match("aBc", "a!(@(b|B))", opts.clone(), true);
    }

    #[test]
    fn test_232() {
        let opts = bash_options(true);
        assert_is_match("abcd", "?@(a|b)*@(c)d", opts.clone(), true);
    }

    #[test]
    fn test_233() {
        let opts = bash_options(true);
        assert_is_match("abcd", "@(ab|a*@(b))*(c)d", opts.clone(), true);
    }

    #[test]
    fn test_234() {
        let opts = bash_options(true);
        assert_is_match(
            "abcd/abcdefg/abcdefghijk/abcdefghijklmnop.txt",
            "**/*a*b*g*n*t",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_235() {
        let opts = bash_options(true);
        assert_is_match(
            "abcd/abcdefg/abcdefghijk/abcdefghijklmnop.txtz",
            "**/*a*b*g*n*t",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_236() {
        let opts = bash_options(true);
        assert_is_match("abcdef", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_237() {
        let opts = bash_options(true);
        assert_is_match("abcdef", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_238() {
        let opts = bash_options(true);
        assert_is_match("abcdef", "*?(a)bc", opts.clone(), false);
    }

    #[test]
    fn test_239() {
        let opts = bash_options(true);
        assert_is_match("abcdef", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_240() {
        let opts = bash_options(true);
        assert_is_match("abcdef", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_241() {
        let opts = bash_options(true);
        assert_is_match("abcdef", "ab**", opts.clone(), true);
    }

    #[test]
    fn test_242() {
        let opts = bash_options(true);
        assert_is_match("abcdef", "ab**(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_243() {
        let opts = bash_options(true);
        assert_is_match("abcdef", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_244() {
        let opts = bash_options(true);
        assert_is_match("abcdef", "ab***ef", opts.clone(), true);
    }

    #[test]
    fn test_245() {
        let opts = bash_options(true);
        assert_is_match("abcdef", "ab*+(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_246() {
        let opts = bash_options(true);
        assert_is_match("abcdef", "ab*d+(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_247() {
        let opts = bash_options(true);
        assert_is_match("abcdef", "ab?*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_248() {
        let opts = bash_options(true);
        assert_is_match("abcfef", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_249() {
        let opts = bash_options(true);
        assert_is_match("abcfef", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_250() {
        let opts = bash_options(true);
        assert_is_match("abcfef", "*?(a)bc", opts.clone(), false);
    }

    #[test]
    fn test_251() {
        let opts = bash_options(true);
        assert_is_match("abcfef", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_252() {
        let opts = bash_options(true);
        assert_is_match("abcfef", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_253() {
        let opts = bash_options(true);
        assert_is_match("abcfef", "ab**", opts.clone(), true);
    }

    #[test]
    fn test_254() {
        let opts = bash_options(true);
        assert_is_match("abcfef", "ab**(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_255() {
        let opts = bash_options(true);
        assert_is_match("abcfef", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_256() {
        let opts = bash_options(true);
        assert_is_match("abcfef", "ab***ef", opts.clone(), true);
    }

    #[test]
    fn test_257() {
        let opts = bash_options(true);
        assert_is_match("abcfef", "ab*+(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_258() {
        let opts = bash_options(true);
        assert_is_match("abcfef", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_259() {
        let opts = bash_options(true);
        assert_is_match("abcfef", "ab?*(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_260() {
        let opts = bash_options(true);
        assert_is_match("abcfefg", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_261() {
        let opts = bash_options(true);
        assert_is_match("abcfefg", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_262() {
        let opts = bash_options(true);
        assert_is_match("abcfefg", "*?(a)bc", opts.clone(), false);
    }

    #[test]
    fn test_263() {
        let opts = bash_options(true);
        assert_is_match("abcfefg", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_264() {
        let opts = bash_options(true);
        assert_is_match("abcfefg", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_265() {
        let opts = bash_options(true);
        assert_is_match("abcfefg", "ab**", opts.clone(), true);
    }

    #[test]
    fn test_266() {
        let opts = bash_options(true);
        assert_is_match("abcfefg", "ab**(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_267() {
        let opts = bash_options(true);
        assert_is_match("abcfefg", "ab**(e|f)g", opts.clone(), true);
    }

    #[test]
    fn test_268() {
        let opts = bash_options(true);
        assert_is_match("abcfefg", "ab***ef", opts.clone(), false);
    }

    #[test]
    fn test_269() {
        let opts = bash_options(true);
        assert_is_match("abcfefg", "ab*+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_270() {
        let opts = bash_options(true);
        assert_is_match("abcfefg", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_271() {
        let opts = bash_options(true);
        assert_is_match("abcfefg", "ab?*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_272() {
        let opts = bash_options(true);
        assert_is_match("abcx", "!([[*])*", opts.clone(), true);
    }

    #[test]
    fn test_273() {
        let opts = bash_options(true);
        assert_is_match("abcx", "+(a|b\\[)*", opts.clone(), true);
    }

    #[test]
    fn test_274() {
        let opts = bash_options(true);
        assert_is_match("abcx", "[a*(]*z", opts.clone(), false);
    }

    #[test]
    fn test_275() {
        let opts = bash_options(true);
        assert_is_match("abcXdefXghi", "*X*i", opts.clone(), true);
    }

    #[test]
    fn test_276() {
        let opts = bash_options(true);
        assert_is_match("abcz", "!([[*])*", opts.clone(), true);
    }

    #[test]
    fn test_277() {
        let opts = bash_options(true);
        assert_is_match("abcz", "+(a|b\\[)*", opts.clone(), true);
    }

    #[test]
    fn test_278() {
        let opts = bash_options(true);
        assert_is_match("abcz", "[a*(]*z", opts.clone(), true);
    }

    #[test]
    fn test_279() {
        let opts = bash_options(true);
        assert_is_match("abd", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_280() {
        let opts = bash_options(true);
        assert_is_match("abd", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_281() {
        let opts = bash_options(true);
        assert_is_match("abd", "*?(a)bc", opts.clone(), false);
    }

    #[test]
    fn test_282() {
        let opts = bash_options(true);
        assert_is_match("abd", "a!(*(b|B))", opts.clone(), true);
    }

    #[test]
    fn test_283() {
        let opts = bash_options(true);
        assert_is_match("abd", "a!(@(b|B))", opts.clone(), true);
    }

    #[test]
    fn test_284() {
        let opts = bash_options(true);
        assert_is_match("abd", "a!(@(b|B))d", opts.clone(), false);
    }

    #[test]
    fn test_285() {
        let opts = bash_options(true);
        assert_is_match("abd", "a(b*(foo|bar))d", opts.clone(), true);
    }

    #[test]
    fn test_286() {
        let opts = bash_options(true);
        assert_is_match("abd", "a+(b|c)d", opts.clone(), true);
    }

    #[test]
    fn test_287() {
        let opts = bash_options(true);
        assert_is_match("abd", "a[b*(foo|bar)]d", opts.clone(), true);
    }

    #[test]
    fn test_288() {
        let opts = bash_options(true);
        assert_is_match("abd", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_289() {
        let opts = bash_options(true);
        assert_is_match("abd", "ab**", opts.clone(), true);
    }

    #[test]
    fn test_290() {
        let opts = bash_options(true);
        assert_is_match("abd", "ab**(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_291() {
        let opts = bash_options(true);
        assert_is_match("abd", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_292() {
        let opts = bash_options(true);
        assert_is_match("abd", "ab***ef", opts.clone(), false);
    }

    #[test]
    fn test_293() {
        let opts = bash_options(true);
        assert_is_match("abd", "ab*+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_294() {
        let opts = bash_options(true);
        assert_is_match("abd", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_295() {
        let opts = bash_options(true);
        assert_is_match("abd", "ab?*(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_296() {
        let opts = bash_options(true);
        assert_is_match("abef", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_297() {
        let opts = bash_options(true);
        assert_is_match("abef", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_298() {
        let opts = bash_options(true);
        assert_is_match("abef", "*(a+|b)", opts.clone(), false);
    }

    #[test]
    fn test_299() {
        let opts = bash_options(true);
        assert_is_match("abef", "*?(a)bc", opts.clone(), false);
    }

    #[test]
    fn test_300() {
        let opts = bash_options(true);
        assert_is_match("abef", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_301() {
        let opts = bash_options(true);
        assert_is_match("abef", "ab*(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_302() {
        let opts = bash_options(true);
        assert_is_match("abef", "ab**", opts.clone(), true);
    }

    #[test]
    fn test_303() {
        let opts = bash_options(true);
        assert_is_match("abef", "ab**(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_304() {
        let opts = bash_options(true);
        assert_is_match("abef", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_305() {
        let opts = bash_options(true);
        assert_is_match("abef", "ab***ef", opts.clone(), true);
    }

    #[test]
    fn test_306() {
        let opts = bash_options(true);
        assert_is_match("abef", "ab*+(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_307() {
        let opts = bash_options(true);
        assert_is_match("abef", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_308() {
        let opts = bash_options(true);
        assert_is_match("abef", "ab?*(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_309() {
        let opts = bash_options(true);
        assert_is_match("abz", "a!(*)", opts.clone(), false);
    }

    #[test]
    fn test_310() {
        let opts = bash_options(true);
        assert_is_match("abz", "a!(z)", opts.clone(), true);
    }

    #[test]
    fn test_311() {
        let opts = bash_options(true);
        assert_is_match("abz", "a*!(z)", opts.clone(), true);
    }

    #[test]
    fn test_312() {
        let opts = bash_options(true);
        assert_is_match("abz", "a*(z)", opts.clone(), false);
    }

    #[test]
    fn test_313() {
        let opts = bash_options(true);
        assert_is_match("abz", "a**(z)", opts.clone(), true);
    }

    #[test]
    fn test_314() {
        let opts = bash_options(true);
        assert_is_match("abz", "a*@(z)", opts.clone(), true);
    }

    #[test]
    fn test_315() {
        let opts = bash_options(true);
        assert_is_match("abz", "a+(z)", opts.clone(), false);
    }

    #[test]
    fn test_316() {
        let opts = bash_options(true);
        assert_is_match("abz", "a?(z)", opts.clone(), false);
    }

    #[test]
    fn test_317() {
        let opts = bash_options(true);
        assert_is_match("abz", "a@(z)", opts.clone(), false);
    }

    #[test]
    fn test_318() {
        let opts = bash_options(true);
        assert_is_match("ac", "!(a)*", opts.clone(), false);
    }

    #[test]
    fn test_319() {
        let opts = bash_options(true);
        assert_is_match("ac", "*(@(a))a@(c)", opts.clone(), true);
    }

    #[test]
    fn test_320() {
        let opts = bash_options(true);
        assert_is_match("ac", "a!(*(b|B))", opts.clone(), true);
    }

    #[test]
    fn test_321() {
        let opts = bash_options(true);
        assert_is_match("ac", "a!(@(b|B))", opts.clone(), true);
    }

    #[test]
    fn test_322() {
        let opts = bash_options(true);
        assert_is_match("ac", "a!(b)*", opts.clone(), true);
    }

    #[test]
    fn test_323() {
        let opts = bash_options(true);
        assert_is_match("accdef", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_324() {
        let opts = bash_options(true);
        assert_is_match("accdef", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_325() {
        let opts = bash_options(true);
        assert_is_match("accdef", "*?(a)bc", opts.clone(), false);
    }

    #[test]
    fn test_326() {
        let opts = bash_options(true);
        assert_is_match("accdef", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_327() {
        let opts = bash_options(true);
        assert_is_match("accdef", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_328() {
        let opts = bash_options(true);
        assert_is_match("accdef", "ab**", opts.clone(), false);
    }

    #[test]
    fn test_329() {
        let opts = bash_options(true);
        assert_is_match("accdef", "ab**(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_330() {
        let opts = bash_options(true);
        assert_is_match("accdef", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_331() {
        let opts = bash_options(true);
        assert_is_match("accdef", "ab***ef", opts.clone(), false);
    }

    #[test]
    fn test_332() {
        let opts = bash_options(true);
        assert_is_match("accdef", "ab*+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_333() {
        let opts = bash_options(true);
        assert_is_match("accdef", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_334() {
        let opts = bash_options(true);
        assert_is_match("accdef", "ab?*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_335() {
        let opts = bash_options(true);
        assert_is_match("acd", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_336() {
        let opts = bash_options(true);
        assert_is_match("acd", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_337() {
        let opts = bash_options(true);
        assert_is_match("acd", "*?(a)bc", opts.clone(), false);
    }

    #[test]
    fn test_338() {
        let opts = bash_options(true);
        assert_is_match("acd", "@(ab|a*(b))*(c)d", opts.clone(), true);
    }

    #[test]
    fn test_339() {
        let opts = bash_options(true);
        assert_is_match("acd", "a!(*(b|B))", opts.clone(), true);
    }

    #[test]
    fn test_340() {
        let opts = bash_options(true);
        assert_is_match("acd", "a!(@(b|B))", opts.clone(), true);
    }

    #[test]
    fn test_341() {
        let opts = bash_options(true);
        assert_is_match("acd", "a!(@(b|B))d", opts.clone(), true);
    }

    #[test]
    fn test_342() {
        let opts = bash_options(true);
        assert_is_match("acd", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_343() {
        let opts = bash_options(true);
        assert_is_match("acd", "a+(b|c)d", opts.clone(), true);
    }

    #[test]
    fn test_344() {
        let opts = bash_options(true);
        assert_is_match("acd", "a[b*(foo|bar)]d", opts.clone(), false);
    }

    #[test]
    fn test_345() {
        let opts = bash_options(true);
        assert_is_match("acd", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_346() {
        let opts = bash_options(true);
        assert_is_match("acd", "ab**", opts.clone(), false);
    }

    #[test]
    fn test_347() {
        let opts = bash_options(true);
        assert_is_match("acd", "ab**(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_348() {
        let opts = bash_options(true);
        assert_is_match("acd", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_349() {
        let opts = bash_options(true);
        assert_is_match("acd", "ab***ef", opts.clone(), false);
    }

    #[test]
    fn test_350() {
        let opts = bash_options(true);
        assert_is_match("acd", "ab*+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_351() {
        let opts = bash_options(true);
        assert_is_match("acd", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_352() {
        let opts = bash_options(true);
        assert_is_match("acd", "ab?*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_353() {
        let opts = bash_options(true);
        assert_is_match("ax", "?(a*|b)", opts.clone(), true);
    }

    #[test]
    fn test_354() {
        let opts = bash_options(true);
        assert_is_match("ax", "a?(b*)", opts.clone(), false);
    }

    #[test]
    fn test_355() {
        let opts = bash_options(true);
        assert_is_match("axz", "a+(z)", opts.clone(), false);
    }

    #[test]
    fn test_356() {
        let opts = bash_options(true);
        assert_is_match("az", "a!(*)", opts.clone(), false);
    }

    #[test]
    fn test_357() {
        let opts = bash_options(true);
        assert_is_match("az", "a!(z)", opts.clone(), false);
    }

    #[test]
    fn test_358() {
        let opts = bash_options(true);
        assert_is_match("az", "a*!(z)", opts.clone(), true);
    }

    #[test]
    fn test_359() {
        let opts = bash_options(true);
        assert_is_match("az", "a*(z)", opts.clone(), true);
    }

    #[test]
    fn test_360() {
        let opts = bash_options(true);
        assert_is_match("az", "a**(z)", opts.clone(), true);
    }

    #[test]
    fn test_361() {
        let opts = bash_options(true);
        assert_is_match("az", "a*@(z)", opts.clone(), true);
    }

    #[test]
    fn test_362() {
        let opts = bash_options(true);
        assert_is_match("az", "a+(z)", opts.clone(), true);
    }

    #[test]
    fn test_363() {
        let opts = bash_options(true);
        assert_is_match("az", "a?(z)", opts.clone(), true);
    }

    #[test]
    fn test_364() {
        let opts = bash_options(true);
        assert_is_match("az", "a@(z)", opts.clone(), true);
    }

    #[test]
    fn test_365() {
        let opts = bash_options(false);
        assert_is_match("az", "a\\\\z", opts.clone(), false);
    }

    #[test]
    fn test_366() {
        let opts = bash_options(true);
        assert_is_match("az", "a\\\\z", opts.clone(), false);
    }

    #[test]
    fn test_367() {
        let opts = bash_options(true);
        assert_is_match("b", "!(a)*", opts.clone(), true);
    }

    #[test]
    fn test_368() {
        let opts = bash_options(true);
        assert_is_match("b", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_369() {
        let opts = bash_options(true);
        assert_is_match("b", "a!(b)*", opts.clone(), false);
    }

    #[test]
    fn test_370() {
        let opts = bash_options(true);
        assert_is_match("b.a", "(b|a).(a)", opts.clone(), true);
    }

    #[test]
    fn test_371() {
        let opts = bash_options(true);
        assert_is_match("b.a", "@(b|a).@(a)", opts.clone(), true);
    }

    #[test]
    fn test_372() {
        let opts = bash_options(true);
        assert_is_match("b/a", "!(b/a)", opts.clone(), false);
    }

    #[test]
    fn test_373() {
        let opts = bash_options(true);
        assert_is_match("b/b", "!(b/a)", opts.clone(), true);
    }

    #[test]
    fn test_374() {
        let opts = bash_options(true);
        assert_is_match("b/c", "!(b/a)", opts.clone(), true);
    }

    #[test]
    fn test_375() {
        let opts = bash_options(true);
        assert_is_match("b/c", "b/!(c)", opts.clone(), false);
    }

    #[test]
    fn test_376() {
        let opts = bash_options(true);
        assert_is_match("b/c", "b/!(cc)", opts.clone(), true);
    }

    #[test]
    fn test_377() {
        let opts = bash_options(true);
        assert_is_match("b/c.txt", "b/!(c).txt", opts.clone(), false);
    }

    #[test]
    fn test_378() {
        let opts = bash_options(true);
        assert_is_match("b/c.txt", "b/!(cc).txt", opts.clone(), true);
    }

    #[test]
    fn test_379() {
        let opts = bash_options(true);
        assert_is_match("b/cc", "b/!(c)", opts.clone(), true);
    }

    #[test]
    fn test_380() {
        let opts = bash_options(true);
        assert_is_match("b/cc", "b/!(cc)", opts.clone(), false);
    }

    #[test]
    fn test_381() {
        let opts = bash_options(true);
        assert_is_match("b/cc.txt", "b/!(c).txt", opts.clone(), false);
    }

    #[test]
    fn test_382() {
        let opts = bash_options(true);
        assert_is_match("b/cc.txt", "b/!(cc).txt", opts.clone(), false);
    }

    #[test]
    fn test_383() {
        let opts = bash_options(true);
        assert_is_match("b/ccc", "b/!(c)", opts.clone(), true);
    }

    #[test]
    fn test_384() {
        let opts = bash_options(true);
        assert_is_match("ba", "!(a!(b))", opts.clone(), true);
    }

    #[test]
    fn test_385() {
        let opts = bash_options(true);
        assert_is_match("ba", "b?(a|b)", opts.clone(), true);
    }

    #[test]
    fn test_386() {
        let opts = bash_options(true);
        assert_is_match("baaac", "*(@(a))a@(c)", opts.clone(), false);
    }

    #[test]
    fn test_387() {
        let opts = bash_options(true);
        assert_is_match("bar", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_388() {
        let opts = bash_options(true);
        assert_is_match("bar", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_389() {
        let opts = bash_options(true);
        assert_is_match("bar", "!(foo)b*", opts.clone(), true);
    }

    #[test]
    fn test_390() {
        let opts = bash_options(true);
        assert_is_match("bar", "*(!(foo))", opts.clone(), true);
    }

    #[test]
    fn test_391() {
        let opts = bash_options(true);
        assert_is_match("baz", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_392() {
        let opts = bash_options(true);
        assert_is_match("baz", "!(foo)b*", opts.clone(), true);
    }

    #[test]
    fn test_393() {
        let opts = bash_options(true);
        assert_is_match("baz", "*(!(foo))", opts.clone(), true);
    }

    #[test]
    fn test_394() {
        let opts = bash_options(true);
        assert_is_match("bb", "!(a!(b))", opts.clone(), true);
    }

    #[test]
    fn test_395() {
        let opts = bash_options(true);
        assert_is_match("bb", "!(a)*", opts.clone(), true);
    }

    #[test]
    fn test_396() {
        let opts = bash_options(true);
        assert_is_match("bb", "a!(b)*", opts.clone(), false);
    }

    #[test]
    fn test_397() {
        let opts = bash_options(true);
        assert_is_match("bb", "a?(a|b)", opts.clone(), false);
    }

    #[test]
    fn test_398() {
        let opts = bash_options(true);
        assert_is_match("bbc", "!([[*])*", opts.clone(), true);
    }

    #[test]
    fn test_399() {
        let opts = bash_options(true);
        assert_is_match("bbc", "+(a|b\\[)*", opts.clone(), false);
    }

    #[test]
    fn test_400() {
        let opts = bash_options(true);
        assert_is_match("bbc", "[a*(]*z", opts.clone(), false);
    }

    #[test]
    fn test_401() {
        let opts = bash_options(true);
        assert_is_match("bz", "a+(z)", opts.clone(), false);
    }

    #[test]
    fn test_402() {
        let opts = bash_options(true);
        assert_is_match("c", "*(@(a))a@(c)", opts.clone(), false);
    }

    #[test]
    fn test_403() {
        let opts = bash_options(true);
        assert_is_match("c.a", "!(*.[a-b]*)", opts.clone(), false);
    }

    #[test]
    fn test_404() {
        let opts = bash_options(true);
        assert_is_match("c.a", "!(*[a-b].[a-b]*)", opts.clone(), true);
    }

    #[test]
    fn test_405() {
        let opts = bash_options(true);
        assert_is_match("c.a", "!*.(a|b)", opts.clone(), false);
    }

    #[test]
    fn test_406() {
        let opts = bash_options(true);
        assert_is_match("c.a", "!*.(a|b)*", opts.clone(), false);
    }

    #[test]
    fn test_407() {
        let opts = bash_options(true);
        assert_is_match("c.a", "(b|a).(a)", opts.clone(), false);
    }

    #[test]
    fn test_408() {
        let opts = bash_options(true);
        assert_is_match("c.a", "*.!(a)", opts.clone(), false);
    }

    #[test]
    fn test_409() {
        let opts = bash_options(true);
        assert_is_match("c.a", "*.+(b|d)", opts.clone(), false);
    }

    #[test]
    fn test_410() {
        let opts = bash_options(true);
        assert_is_match("c.a", "@(b|a).@(a)", opts.clone(), false);
    }

    #[test]
    fn test_411() {
        let opts = bash_options(true);
        assert_is_match("c.c", "!(*.a|*.b|*.c)", opts.clone(), false);
    }

    #[test]
    fn test_412() {
        let opts = bash_options(true);
        assert_is_match("c.c", "*!(.a|.b|.c)", opts.clone(), true);
    }

    #[test]
    fn test_413() {
        let opts = bash_options(true);
        assert_is_match("c.c", "*.!(a|b|c)", opts.clone(), false);
    }

    #[test]
    fn test_414() {
        let opts = bash_options(true);
        assert_is_match("c.c", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), false);
    }

    #[test]
    fn test_415() {
        let opts = bash_options(true);
        assert_is_match("c.ccc", "!(*.[a-b]*)", opts.clone(), true);
    }

    #[test]
    fn test_416() {
        let opts = bash_options(true);
        assert_is_match("c.ccc", "!(*[a-b].[a-b]*)", opts.clone(), true);
    }

    #[test]
    fn test_417() {
        let opts = bash_options(true);
        assert_is_match("c.js", "!(*.js)", opts.clone(), false);
    }

    #[test]
    fn test_418() {
        let opts = bash_options(true);
        assert_is_match("c.js", "*!(.js)", opts.clone(), true);
    }

    #[test]
    fn test_419() {
        let opts = bash_options(true);
        assert_is_match("c.js", "*.!(js)", opts.clone(), false);
    }

    #[test]
    fn test_420() {
        let opts = bash_options(true);
        assert_is_match("c/a/v", "c/!(z)/v", opts.clone(), true);
    }

    #[test]
    fn test_421() {
        let opts = bash_options(true);
        assert_is_match("c/a/v", "c/*(z)/v", opts.clone(), false);
    }

    #[test]
    fn test_422() {
        let opts = bash_options(true);
        assert_is_match("c/a/v", "c/+(z)/v", opts.clone(), false);
    }

    #[test]
    fn test_423() {
        let opts = bash_options(true);
        assert_is_match("c/a/v", "c/@(z)/v", opts.clone(), false);
    }

    #[test]
    fn test_424() {
        let opts = bash_options(true);
        assert_is_match("c/z/v", "*(z)", opts.clone(), false);
    }

    #[test]
    fn test_425() {
        let opts = bash_options(true);
        assert_is_match("c/z/v", "+(z)", opts.clone(), false);
    }

    #[test]
    fn test_426() {
        let opts = bash_options(true);
        assert_is_match("c/z/v", "?(z)", opts.clone(), false);
    }

    #[test]
    fn test_427() {
        let opts = bash_options(true);
        assert_is_match("c/z/v", "c/!(z)/v", opts.clone(), false);
    }

    #[test]
    fn test_428() {
        let opts = bash_options(true);
        assert_is_match("c/z/v", "c/*(z)/v", opts.clone(), true);
    }

    #[test]
    fn test_429() {
        let opts = bash_options(true);
        assert_is_match("c/z/v", "c/+(z)/v", opts.clone(), true);
    }

    #[test]
    fn test_430() {
        let opts = bash_options(true);
        assert_is_match("c/z/v", "c/@(z)/v", opts.clone(), true);
    }

    #[test]
    fn test_431() {
        let opts = bash_options(true);
        assert_is_match("c/z/v", "c/z/v", opts.clone(), true);
    }

    #[test]
    fn test_432() {
        let opts = bash_options(true);
        assert_is_match("cc.a", "(b|a).(a)", opts.clone(), false);
    }

    #[test]
    fn test_433() {
        let opts = bash_options(true);
        assert_is_match("cc.a", "@(b|a).@(a)", opts.clone(), false);
    }

    #[test]
    fn test_434() {
        let opts = bash_options(true);
        assert_is_match("ccc", "!(a)*", opts.clone(), true);
    }

    #[test]
    fn test_435() {
        let opts = bash_options(true);
        assert_is_match("ccc", "a!(b)*", opts.clone(), false);
    }

    #[test]
    fn test_436() {
        let opts = bash_options(true);
        assert_is_match("cow", "!(*.*)", opts.clone(), true);
    }

    #[test]
    fn test_437() {
        let opts = bash_options(true);
        assert_is_match("cow", "!(*.*).", opts.clone(), false);
    }

    #[test]
    fn test_438() {
        let opts = bash_options(true);
        assert_is_match("cow", ".!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_439() {
        let opts = bash_options(true);
        assert_is_match("cz", "a!(*)", opts.clone(), false);
    }

    #[test]
    fn test_440() {
        let opts = bash_options(true);
        assert_is_match("cz", "a!(z)", opts.clone(), false);
    }

    #[test]
    fn test_441() {
        let opts = bash_options(true);
        assert_is_match("cz", "a*!(z)", opts.clone(), false);
    }

    #[test]
    fn test_442() {
        let opts = bash_options(true);
        assert_is_match("cz", "a*(z)", opts.clone(), false);
    }

    #[test]
    fn test_443() {
        let opts = bash_options(true);
        assert_is_match("cz", "a**(z)", opts.clone(), false);
    }

    #[test]
    fn test_444() {
        let opts = bash_options(true);
        assert_is_match("cz", "a*@(z)", opts.clone(), false);
    }

    #[test]
    fn test_445() {
        let opts = bash_options(true);
        assert_is_match("cz", "a+(z)", opts.clone(), false);
    }

    #[test]
    fn test_446() {
        let opts = bash_options(true);
        assert_is_match("cz", "a?(z)", opts.clone(), false);
    }

    #[test]
    fn test_447() {
        let opts = bash_options(true);
        assert_is_match("cz", "a@(z)", opts.clone(), false);
    }

    #[test]
    fn test_448() {
        let opts = bash_options(true);
        assert_is_match("d.a.d", "!(*.[a-b]*)", opts.clone(), false);
    }

    #[test]
    fn test_449() {
        let opts = bash_options(true);
        assert_is_match("d.a.d", "!(*[a-b].[a-b]*)", opts.clone(), true);
    }

    #[test]
    fn test_450() {
        let opts = bash_options(true);
        assert_is_match("d.a.d", "!*.(a|b)*", opts.clone(), false);
    }

    #[test]
    fn test_451() {
        let opts = bash_options(true);
        assert_is_match("d.a.d", "!*.*(a|b)", opts.clone(), true);
    }

    #[test]
    fn test_452() {
        let opts = bash_options(true);
        assert_is_match("d.a.d", "!*.{a,b}*", opts.clone(), false);
    }

    #[test]
    fn test_453() {
        let opts = bash_options(true);
        assert_is_match("d.a.d", "*.!(a)", opts.clone(), true);
    }

    #[test]
    fn test_454() {
        let opts = bash_options(true);
        assert_is_match("d.a.d", "*.+(b|d)", opts.clone(), true);
    }

    #[test]
    fn test_455() {
        let opts = bash_options(true);
        assert_is_match("d.d", "!(*.a|*.b|*.c)", opts.clone(), true);
    }

    #[test]
    fn test_456() {
        let opts = bash_options(true);
        assert_is_match("d.d", "*!(.a|.b|.c)", opts.clone(), true);
    }

    #[test]
    fn test_457() {
        let opts = bash_options(true);
        assert_is_match("d.d", "*.!(a|b|c)", opts.clone(), true);
    }

    #[test]
    fn test_458() {
        let opts = bash_options(true);
        assert_is_match("d.d", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), false);
    }

    #[test]
    fn test_459() {
        let opts = bash_options(true);
        assert_is_match("d.js.d", "!(*.js)", opts.clone(), true);
    }

    #[test]
    fn test_460() {
        let opts = bash_options(true);
        assert_is_match("d.js.d", "*!(.js)", opts.clone(), true);
    }

    #[test]
    fn test_461() {
        let opts = bash_options(true);
        assert_is_match("d.js.d", "*.!(js)", opts.clone(), true);
    }

    #[test]
    fn test_462() {
        let opts = bash_options(true);
        assert_is_match("dd.aa.d", "(b|a).(a)", opts.clone(), false);
    }

    #[test]
    fn test_463() {
        let opts = bash_options(true);
        assert_is_match("dd.aa.d", "@(b|a).@(a)", opts.clone(), false);
    }

    #[test]
    fn test_464() {
        let opts = bash_options(true);
        assert_is_match("def", "()ef", opts.clone(), false);
    }

    #[test]
    fn test_465() {
        let opts = bash_options(true);
        assert_is_match("e.e", "!(*.a|*.b|*.c)", opts.clone(), true);
    }

    #[test]
    fn test_466() {
        let opts = bash_options(true);
        assert_is_match("e.e", "*!(.a|.b|.c)", opts.clone(), true);
    }

    #[test]
    fn test_467() {
        let opts = bash_options(true);
        assert_is_match("e.e", "*.!(a|b|c)", opts.clone(), true);
    }

    #[test]
    fn test_468() {
        let opts = bash_options(true);
        assert_is_match("e.e", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), false);
    }

    #[test]
    fn test_469() {
        let opts = bash_options(true);
        assert_is_match("ef", "()ef", opts.clone(), true);
    }

    #[test]
    fn test_470() {
        let opts = bash_options(true);
        assert_is_match("effgz", "@(b+(c)d|e*(f)g?|?(h)i@(j|k))", opts.clone(), true);
    }

    #[test]
    fn test_471() {
        let opts = bash_options(true);
        assert_is_match("efgz", "@(b+(c)d|e*(f)g?|?(h)i@(j|k))", opts.clone(), true);
    }

    #[test]
    fn test_472() {
        let opts = bash_options(true);
        assert_is_match("egz", "@(b+(c)d|e*(f)g?|?(h)i@(j|k))", opts.clone(), true);
    }

    #[test]
    fn test_473() {
        let opts = bash_options(true);
        assert_is_match("egz", "@(b+(c)d|e+(f)g?|?(h)i@(j|k))", opts.clone(), false);
    }

    #[test]
    fn test_474() {
        let opts = bash_options(true);
        assert_is_match(
            "egzefffgzbcdij",
            "*(b+(c)d|e*(f)g?|?(h)i@(j|k))",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_475() {
        let opts = bash_options(true);
        assert_is_match("f", "!(f!(o))", opts.clone(), false);
    }

    #[test]
    fn test_476() {
        let opts = bash_options(true);
        assert_is_match("f", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_477() {
        let opts = bash_options(true);
        assert_is_match("f", "!(f)", opts.clone(), false);
    }

    #[test]
    fn test_478() {
        let opts = bash_options(true);
        assert_is_match("f", "*(!(f))", opts.clone(), false);
    }

    #[test]
    fn test_479() {
        let opts = bash_options(true);
        assert_is_match("f", "+(!(f))", opts.clone(), false);
    }

    #[test]
    fn test_480() {
        let opts = bash_options(true);
        assert_is_match("f.a", "!(*.a|*.b|*.c)", opts.clone(), false);
    }

    #[test]
    fn test_481() {
        let opts = bash_options(true);
        assert_is_match("f.a", "*!(.a|.b|.c)", opts.clone(), true);
    }

    #[test]
    fn test_482() {
        let opts = bash_options(true);
        assert_is_match("f.a", "*.!(a|b|c)", opts.clone(), false);
    }

    #[test]
    fn test_483() {
        let opts = bash_options(true);
        assert_is_match("f.f", "!(*.a|*.b|*.c)", opts.clone(), true);
    }

    #[test]
    fn test_484() {
        let opts = bash_options(true);
        assert_is_match("f.f", "*!(.a|.b|.c)", opts.clone(), true);
    }

    #[test]
    fn test_485() {
        let opts = bash_options(true);
        assert_is_match("f.f", "*.!(a|b|c)", opts.clone(), true);
    }

    #[test]
    fn test_486() {
        let opts = bash_options(true);
        assert_is_match("f.f", "*.(a|b|@(ab|a*@(b))*(c)d)", opts.clone(), false);
    }

    #[test]
    fn test_487() {
        let opts = bash_options(true);
        assert_is_match("fa", "!(f!(o))", opts.clone(), false);
    }

    #[test]
    fn test_488() {
        let opts = bash_options(true);
        assert_is_match("fa", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_489() {
        let opts = bash_options(true);
        assert_is_match("fb", "!(f!(o))", opts.clone(), false);
    }

    #[test]
    fn test_490() {
        let opts = bash_options(true);
        assert_is_match("fb", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_491() {
        let opts = bash_options(true);
        assert_is_match("fff", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_492() {
        let opts = bash_options(true);
        assert_is_match("fff", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_493() {
        let opts = bash_options(true);
        assert_is_match("fff", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_494() {
        let opts = bash_options(true);
        assert_is_match(
            "fffooofoooooffoofffooofff",
            "*(*(f)*(o))",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_495() {
        let opts = bash_options(true);
        assert_is_match("ffo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_496() {
        let opts = bash_options(true);
        assert_is_match("file.C", "*.c?(c)", opts.clone(), false);
    }

    #[test]
    fn test_497() {
        let opts = bash_options(true);
        assert_is_match("file.c", "*.c?(c)", opts.clone(), true);
    }

    #[test]
    fn test_498() {
        let opts = bash_options(true);
        assert_is_match("file.cc", "*.c?(c)", opts.clone(), true);
    }

    #[test]
    fn test_499() {
        let opts = bash_options(true);
        assert_is_match("file.ccc", "*.c?(c)", opts.clone(), false);
    }

    #[test]
    fn test_500() {
        let opts = bash_options(true);
        assert_is_match("fo", "!(f!(o))", opts.clone(), true);
    }

    #[test]
    fn test_501() {
        let opts = bash_options(true);
        assert_is_match("fo", "!(f(o))", opts.clone(), false);
    }

    #[test]
    fn test_502() {
        let opts = bash_options(true);
        assert_is_match("fofo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_503() {
        let opts = bash_options(true);
        assert_is_match("fofoofoofofoo", "*(fo|foo)", opts.clone(), true);
    }

    #[test]
    fn test_504() {
        let opts = bash_options(true);
        assert_is_match("fofoofoofofoo", "*(fo|foo)", opts.clone(), true);
    }

    #[test]
    fn test_505() {
        let opts = bash_options(true);
        assert_is_match("foo", "!(!(foo))", opts.clone(), true);
    }

    #[test]
    fn test_506() {
        let opts = bash_options(true);
        assert_is_match("foo", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_507() {
        let opts = bash_options(true);
        assert_is_match("foo", "!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_508() {
        let opts = bash_options(true);
        assert_is_match("foo", "!(foo)*", opts.clone(), false);
    }

    #[test]
    fn test_509() {
        let opts = bash_options(true);
        assert_is_match("foo", "!(foo)*", opts.clone(), false);
    }

    #[test]
    fn test_510() {
        let opts = bash_options(true);
        assert_is_match("foo", "!(foo)+", opts.clone(), false);
    }

    #[test]
    fn test_511() {
        let opts = bash_options(true);
        assert_is_match("foo", "!(foo)b*", opts.clone(), false);
    }

    #[test]
    fn test_512() {
        let opts = bash_options(true);
        assert_is_match("foo", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_513() {
        let opts = bash_options(true);
        assert_is_match("foo", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_514() {
        let opts = bash_options(true);
        assert_is_match("foo", "*", opts.clone(), true);
    }

    #[test]
    fn test_515() {
        let opts = bash_options(true);
        assert_is_match("foo", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_516() {
        let opts = bash_options(true);
        assert_is_match("foo", "*(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_517() {
        let opts = bash_options(true);
        assert_is_match("foo", "*(@(a))a@(c)", opts.clone(), false);
    }

    #[test]
    fn test_518() {
        let opts = bash_options(true);
        assert_is_match("foo", "*(@(foo))", opts.clone(), true);
    }

    #[test]
    fn test_519() {
        let opts = bash_options(true);
        assert_is_match("foo", "*(a|b\\[)", opts.clone(), false);
    }

    #[test]
    fn test_520() {
        let opts = bash_options(true);
        assert_is_match("foo", "*(a|b\\[)|f*", opts.clone(), true);
    }

    #[test]
    fn test_521() {
        let opts = bash_options(true);
        assert_is_match("foo", "@(*(a|b\\[)|f*)", opts.clone(), true);
    }

    #[test]
    fn test_522() {
        let opts = bash_options(true);
        assert_is_match("foo", "*/*/*", opts.clone(), false);
    }

    #[test]
    fn test_523() {
        let opts = bash_options(true);
        assert_is_match("foo", "*f", opts.clone(), false);
    }

    #[test]
    fn test_524() {
        let opts = bash_options(true);
        assert_is_match("foo", "*foo*", opts.clone(), true);
    }

    #[test]
    fn test_525() {
        let opts = bash_options(true);
        assert_is_match("foo", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_526() {
        let opts = bash_options(true);
        assert_is_match("foo", "??", opts.clone(), false);
    }

    #[test]
    fn test_527() {
        let opts = bash_options(true);
        assert_is_match("foo", "???", opts.clone(), true);
    }

    #[test]
    fn test_528() {
        let opts = bash_options(true);
        assert_is_match("foo", "bar", opts.clone(), false);
    }

    #[test]
    fn test_529() {
        let opts = bash_options(true);
        assert_is_match("foo", "f*", opts.clone(), true);
    }

    #[test]
    fn test_530() {
        let opts = bash_options(true);
        assert_is_match("foo", "fo", opts.clone(), false);
    }

    #[test]
    fn test_531() {
        let opts = bash_options(true);
        assert_is_match("foo", "foo", opts.clone(), true);
    }

    #[test]
    fn test_532() {
        let opts = bash_options(true);
        assert_is_match("foo", "{*(a|b\\[),f*}", opts.clone(), true);
    }

    #[test]
    fn test_533() {
        let opts = bash_options(false);
        assert_is_match("foo*", "foo\\*", opts.clone(), true);
    }

    #[test]
    fn test_534() {
        let opts = bash_options(true);
        assert_is_match("foo*bar", "foo\\*bar", opts.clone(), true);
    }

    #[test]
    fn test_535() {
        let opts = bash_options(true);
        assert_is_match("foo.js", "!(foo).js", opts.clone(), false);
    }

    #[test]
    fn test_536() {
        let opts = bash_options(true);
        assert_is_match("foo.js.js", "*.!(js)", opts.clone(), true);
    }

    #[test]
    fn test_537() {
        let opts = bash_options(true);
        assert_is_match("foo.js.js", "*.!(js)*", opts.clone(), false);
    }

    #[test]
    fn test_538() {
        let opts = bash_options(true);
        assert_is_match("foo.js.js", "*.!(js)*.!(js)", opts.clone(), false);
    }

    #[test]
    fn test_539() {
        let opts = bash_options(true);
        assert_is_match("foo.js.js", "*.!(js)+", opts.clone(), false);
    }

    #[test]
    fn test_540() {
        let opts = bash_options(true);
        assert_is_match("foo.txt", "**/!(bar).txt", opts.clone(), true);
    }

    #[test]
    fn test_541() {
        let opts = bash_options(true);
        assert_is_match("foo/bar", "*/*/*", opts.clone(), false);
    }

    #[test]
    fn test_542() {
        let opts = bash_options(true);
        assert_is_match("foo/bar", "foo/!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_543() {
        let opts = bash_options(true);
        assert_is_match("foo/bar", "foo/*", opts.clone(), true);
    }

    #[test]
    fn test_544() {
        let opts = bash_options(true);
        assert_is_match("foo/bar", "foo/bar", opts.clone(), true);
    }

    #[test]
    fn test_545() {
        let opts = bash_options(true);
        assert_is_match("foo/bar", "foo?bar", opts.clone(), false);
    }

    #[test]
    fn test_546() {
        let opts = bash_options(true);
        assert_is_match("foo/bar", "foo[/]bar", opts.clone(), true);
    }

    #[test]
    fn test_547() {
        let opts = bash_options(true);
        assert_is_match(
            "foo/bar/baz.jsx",
            "foo/bar/**/*.+(js|jsx)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_548() {
        let opts = bash_options(true);
        assert_is_match("foo/bar/baz.jsx", "foo/bar/*.+(js|jsx)", opts.clone(), true);
    }

    #[test]
    fn test_549() {
        let opts = bash_options(true);
        assert_is_match("foo/bb/aa/rr", "**/**/**", opts.clone(), true);
    }

    #[test]
    fn test_550() {
        let opts = bash_options(true);
        assert_is_match("foo/bb/aa/rr", "*/*/*", opts.clone(), true);
    }

    #[test]
    fn test_551() {
        let opts = bash_options(true);
        assert_is_match("foo/bba/arr", "*/*/*", opts.clone(), true);
    }

    #[test]
    fn test_552() {
        let opts = bash_options(true);
        assert_is_match("foo/bba/arr", "foo*", opts.clone(), true);
    }

    #[test]
    fn test_553() {
        let opts = bash_options(true);
        assert_is_match("foo/bba/arr", "foo**", opts.clone(), true);
    }

    #[test]
    fn test_554() {
        let opts = bash_options(true);
        assert_is_match("foo/bba/arr", "foo/*", opts.clone(), true);
    }

    #[test]
    fn test_555() {
        let opts = bash_options(true);
        assert_is_match("foo/bba/arr", "foo/**", opts.clone(), true);
    }

    #[test]
    fn test_556() {
        let opts = bash_options(true);
        assert_is_match("foo/bba/arr", "foo/**arr", opts.clone(), true);
    }

    #[test]
    fn test_557() {
        let opts = bash_options(true);
        assert_is_match("foo/bba/arr", "foo/**z", opts.clone(), false);
    }

    #[test]
    fn test_558() {
        let opts = bash_options(true);
        assert_is_match("foo/bba/arr", "foo/*arr", opts.clone(), true);
    }

    #[test]
    fn test_559() {
        let opts = bash_options(true);
        assert_is_match("foo/bba/arr", "foo/*z", opts.clone(), false);
    }

    #[test]
    fn test_560() {
        let opts = bash_options(true);
        assert_is_match("foob", "!(foo)b*", opts.clone(), false);
    }

    #[test]
    fn test_561() {
        let opts = bash_options(true);
        assert_is_match("foob", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_562() {
        let opts = bash_options(true);
        assert_is_match("foobar", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_563() {
        let opts = bash_options(true);
        assert_is_match("foobar", "!(foo)*", opts.clone(), false);
    }

    #[test]
    fn test_564() {
        let opts = bash_options(true);
        assert_is_match("foobar", "!(foo)*", opts.clone(), false);
    }

    #[test]
    fn test_565() {
        let opts = bash_options(true);
        assert_is_match("foobar", "!(foo)b*", opts.clone(), false);
    }

    #[test]
    fn test_566() {
        let opts = bash_options(true);
        assert_is_match("foobar", "*(!(foo))", opts.clone(), true);
    }

    #[test]
    fn test_567() {
        let opts = bash_options(true);
        assert_is_match("foobar", "*ob*a*r*", opts.clone(), true);
    }

    #[test]
    fn test_568() {
        let opts = bash_options(true);
        assert_is_match("foobar", "foo*bar", opts.clone(), true);
    }

    #[test]
    fn test_569() {
        let opts = bash_options(true);
        assert_is_match("foobb", "!(foo)b*", opts.clone(), false);
    }

    #[test]
    fn test_570() {
        let opts = bash_options(true);
        assert_is_match("foobb", "(foo)bb", opts.clone(), true);
    }

    #[test]
    fn test_571() {
        let opts = bash_options(true);
        assert_is_match("(foo)bb", "\\(foo\\)bb", opts.clone(), true);
    }

    #[test]
    fn test_572() {
        let opts = bash_options(true);
        assert_is_match("foofoofo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), true);
    }

    #[test]
    fn test_573() {
        let opts = bash_options(true);
        assert_is_match("foofoofo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), true);
    }

    #[test]
    fn test_574() {
        let opts = bash_options(true);
        assert_is_match("fooofoofofooo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_575() {
        let opts = bash_options(true);
        assert_is_match("foooofo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_576() {
        let opts = bash_options(true);
        assert_is_match("foooofof", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_577() {
        let opts = bash_options(true);
        assert_is_match("foooofof", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_578() {
        let opts = bash_options(true);
        assert_is_match("foooofofx", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_579() {
        let opts = bash_options(true);
        assert_is_match("foooxfooxfoxfooox", "*(f*(o)x)", opts.clone(), true);
    }

    #[test]
    fn test_580() {
        let opts = bash_options(true);
        assert_is_match("foooxfooxfxfooox", "*(f*(o)x)", opts.clone(), true);
    }

    #[test]
    fn test_581() {
        let opts = bash_options(true);
        assert_is_match("foooxfooxofoxfooox", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_582() {
        let opts = bash_options(true);
        assert_is_match("foot", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_583() {
        let opts = bash_options(true);
        assert_is_match("foox", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_584() {
        let opts = bash_options(true);
        assert_is_match("fz", "*(z)", opts.clone(), false);
    }

    #[test]
    fn test_585() {
        let opts = bash_options(true);
        assert_is_match("fz", "+(z)", opts.clone(), false);
    }

    #[test]
    fn test_586() {
        let opts = bash_options(true);
        assert_is_match("fz", "?(z)", opts.clone(), false);
    }

    #[test]
    fn test_587() {
        let opts = bash_options(true);
        assert_is_match("moo.cow", "!(moo).!(cow)", opts.clone(), false);
    }

    #[test]
    fn test_588() {
        let opts = bash_options(true);
        assert_is_match("moo.cow", "!(*).!(*)", opts.clone(), false);
    }

    #[test]
    fn test_589() {
        let opts = bash_options(true);
        assert_is_match("moo.cow", "!(*.*).!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_590() {
        let opts = bash_options(true);
        assert_is_match("mad.moo.cow", "!(*.*).!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_591() {
        let opts = bash_options(true);
        assert_is_match("mad.moo.cow", ".!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_592() {
        let opts = bash_options(true);
        assert_is_match(
            "Makefile",
            "!(*.c|*.h|Makefile.in|config*|README)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_593() {
        let opts = bash_options(true);
        assert_is_match(
            "Makefile.in",
            "!(*.c|*.h|Makefile.in|config*|README)",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_594() {
        let opts = bash_options(true);
        assert_is_match("moo", "!(*.*)", opts.clone(), true);
    }

    #[test]
    fn test_595() {
        let opts = bash_options(true);
        assert_is_match("moo", "!(*.*).", opts.clone(), false);
    }

    #[test]
    fn test_596() {
        let opts = bash_options(true);
        assert_is_match("moo", ".!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_597() {
        let opts = bash_options(true);
        assert_is_match("moo.cow", "!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_598() {
        let opts = bash_options(true);
        assert_is_match("moo.cow", "!(*.*).", opts.clone(), false);
    }

    #[test]
    fn test_599() {
        let opts = bash_options(true);
        assert_is_match("moo.cow", ".!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_600() {
        let opts = bash_options(true);
        assert_is_match("mucca.pazza", "mu!(*(c))?.pa!(*(z))?", opts.clone(), false);
    }

    #[test]
    fn test_601() {
        let opts = bash_options(true);
        assert_is_match("ofoofo", "*(of+(o))", opts.clone(), true);
    }

    #[test]
    fn test_602() {
        let opts = bash_options(true);
        assert_is_match("ofoofo", "*(of+(o)|f)", opts.clone(), true);
    }

    #[test]
    fn test_603() {
        let opts = bash_options(true);
        assert_is_match("ofooofoofofooo", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_604() {
        let opts = bash_options(true);
        assert_is_match("ofoooxoofxo", "*(*(of*(o)x)o)", opts.clone(), true);
    }

    #[test]
    fn test_605() {
        let opts = bash_options(true);
        assert_is_match(
            "ofoooxoofxoofoooxoofxo",
            "*(*(of*(o)x)o)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_606() {
        let opts = bash_options(true);
        assert_is_match(
            "ofoooxoofxoofoooxoofxofo",
            "*(*(of*(o)x)o)",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_607() {
        let opts = bash_options(true);
        assert_is_match(
            "ofoooxoofxoofoooxoofxoo",
            "*(*(of*(o)x)o)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_608() {
        let opts = bash_options(true);
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "*(*(of*(o)x)o)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_609() {
        let opts = bash_options(true);
        assert_is_match("ofxoofxo", "*(*(of*(o)x)o)", opts.clone(), true);
    }

    #[test]
    fn test_610() {
        let opts = bash_options(true);
        assert_is_match("oofooofo", "*(of|oof+(o))", opts.clone(), true);
    }

    #[test]
    fn test_611() {
        let opts = bash_options(true);
        assert_is_match("ooo", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_612() {
        let opts = bash_options(true);
        assert_is_match("ooo", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_613() {
        let opts = bash_options(true);
        assert_is_match("ooo", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_614() {
        let opts = bash_options(true);
        assert_is_match("oxfoxfox", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_615() {
        let opts = bash_options(true);
        assert_is_match("oxfoxoxfox", "*(oxf+(ox))", opts.clone(), true);
    }

    #[test]
    fn test_616() {
        let opts = bash_options(true);
        assert_is_match("para", "para*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_617() {
        let opts = bash_options(true);
        assert_is_match("para", "para+([0-9])", opts.clone(), false);
    }

    #[test]
    fn test_618() {
        let opts = bash_options(true);
        assert_is_match("para.38", "para!(*.[00-09])", opts.clone(), true);
    }

    #[test]
    fn test_619() {
        let opts = bash_options(true);
        assert_is_match("para.graph", "para!(*.[0-9])", opts.clone(), true);
    }

    #[test]
    fn test_620() {
        let opts = bash_options(true);
        assert_is_match("para13829383746592", "para*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_621() {
        let opts = bash_options(true);
        assert_is_match("para381", "para?([345]|99)1", opts.clone(), false);
    }

    #[test]
    fn test_622() {
        let opts = bash_options(true);
        assert_is_match("para39", "para!(*.[0-9])", opts.clone(), true);
    }

    #[test]
    fn test_623() {
        let opts = bash_options(true);
        assert_is_match("para987346523", "para+([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_624() {
        let opts = bash_options(true);
        assert_is_match("para991", "para?([345]|99)1", opts.clone(), true);
    }

    #[test]
    fn test_625() {
        let opts = bash_options(true);
        assert_is_match("paragraph", "para!(*.[0-9])", opts.clone(), true);
    }

    #[test]
    fn test_626() {
        let opts = bash_options(true);
        assert_is_match("paragraph", "para*([0-9])", opts.clone(), false);
    }

    #[test]
    fn test_627() {
        let opts = bash_options(true);
        assert_is_match("paragraph", "para@(chute|graph)", opts.clone(), true);
    }

    #[test]
    fn test_628() {
        let opts = bash_options(true);
        assert_is_match("paramour", "para@(chute|graph)", opts.clone(), false);
    }

    #[test]
    fn test_629() {
        let opts = bash_options(true);
        assert_is_match(
            "parse.y",
            "!(*.c|*.h|Makefile.in|config*|README)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_630() {
        let opts = bash_options(true);
        assert_is_match(
            "shell.c",
            "!(*.c|*.h|Makefile.in|config*|README)",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_631() {
        let opts = bash_options(true);
        assert_is_match("VMS.FILE;", "*\\;[1-9]*([0-9])", opts.clone(), false);
    }

    #[test]
    fn test_632() {
        let opts = bash_options(true);
        assert_is_match("VMS.FILE;0", "*\\;[1-9]*([0-9])", opts.clone(), false);
    }

    #[test]
    fn test_633() {
        let opts = bash_options(true);
        assert_is_match("VMS.FILE;9", "*\\;[1-9]*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_634() {
        let opts = bash_options(true);
        assert_is_match("VMS.FILE;1", "*\\;[1-9]*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_635() {
        let opts = bash_options(true);
        assert_is_match("VMS.FILE;1", "*;[1-9]*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_636() {
        let opts = bash_options(true);
        assert_is_match("VMS.FILE;139", "*\\;[1-9]*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_637() {
        let opts = bash_options(true);
        assert_is_match("VMS.FILE;1N", "*\\;[1-9]*([0-9])", opts.clone(), false);
    }

    #[test]
    fn test_638() {
        let opts = bash_options(true);
        assert_is_match("xfoooofof", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_639() {
        let opts = bash_options(false);
        assert_is_match(
            "XXX/adobe/courier/bold/o/normal//12/120/75/75/m/70/iso8859/1",
            "XXX/*/*/*/*/*/*/12/*/*/*/m/*/*/*",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_640() {
        let opts = bash_options(true);
        assert_is_match(
            "XXX/adobe/courier/bold/o/normal//12/120/75/75/X/70/iso8859/1",
            "XXX/*/*/*/*/*/*/12/*/*/*/m/*/*/*",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_641() {
        let opts = bash_options(true);
        assert_is_match("z", "*(z)", opts.clone(), true);
    }

    #[test]
    fn test_642() {
        let opts = bash_options(true);
        assert_is_match("z", "+(z)", opts.clone(), true);
    }

    #[test]
    fn test_643() {
        let opts = bash_options(true);
        assert_is_match("z", "?(z)", opts.clone(), true);
    }

    #[test]
    fn test_644() {
        let opts = bash_options(true);
        assert_is_match("zf", "*(z)", opts.clone(), false);
    }

    #[test]
    fn test_645() {
        let opts = bash_options(true);
        assert_is_match("zf", "+(z)", opts.clone(), false);
    }

    #[test]
    fn test_646() {
        let opts = bash_options(true);
        assert_is_match("zf", "?(z)", opts.clone(), false);
    }

    #[test]
    fn test_647() {
        let opts = bash_options(true);
        assert_is_match("zoot", "@(!(z*)|*x)", opts.clone(), false);
    }

    #[test]
    fn test_648() {
        let opts = bash_options(true);
        assert_is_match("zoox", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_649() {
        let opts = bash_options(true);
        assert_is_match("zz", "(a+|b)*", opts.clone(), false);
    }
}
