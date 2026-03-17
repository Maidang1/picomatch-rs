mod support;

use support::{assert_is_match, default_compile_options};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let opts = default_compile_options();
        assert_is_match("cbz", "c!(.)z", opts, true);
    }

    #[test]
    fn test_2() {
        let opts = default_compile_options();
        assert_is_match("cbz", "c!(*)z", opts, false);
    }

    #[test]
    fn test_3() {
        let opts = default_compile_options();
        assert_is_match("cccz", "c!(b*)z", opts, true);
    }

    #[test]
    fn test_4() {
        let opts = default_compile_options();
        assert_is_match("cbz", "c!(+)z", opts, true);
    }

    #[test]
    fn test_5() {
        let opts = default_compile_options();
        assert_is_match("cbz", "c!(?)z", opts, true);
    }

    #[test]
    fn test_6() {
        let opts = default_compile_options();
        assert_is_match("cbz", "c!(@)z", opts, true);
    }

    #[test]
    fn test_7() {
        let opts = default_compile_options();
        assert_is_match("c/z", "c!(?:foo)?z", opts, false);
    }

    #[test]
    fn test_8() {
        let opts = default_compile_options();
        assert_is_match("c!fooz", "c!(?:foo)?z", opts, true);
    }

    #[test]
    fn test_9() {
        let opts = default_compile_options();
        assert_is_match("c!z", "c!(?:foo)?z", opts, true);
    }

    #[test]
    fn test_10() {
        let opts = default_compile_options();
        assert_is_match("abc", "!(abc)", opts, false);
    }

    #[test]
    fn test_11() {
        let opts = default_compile_options();
        assert_is_match("a", "!(a)", opts, false);
    }

    #[test]
    fn test_12() {
        let opts = default_compile_options();
        assert_is_match("aa", "!(a)", opts, true);
    }

    #[test]
    fn test_13() {
        let opts = default_compile_options();
        assert_is_match("b", "!(a)", opts, true);
    }

    #[test]
    fn test_14() {
        let opts = default_compile_options();
        assert_is_match("aac", "a!(b)c", opts, true);
    }

    #[test]
    fn test_15() {
        let opts = default_compile_options();
        assert_is_match("abc", "a!(b)c", opts, false);
    }

    #[test]
    fn test_16() {
        let opts = default_compile_options();
        assert_is_match("acc", "a!(b)c", opts, true);
    }

    #[test]
    fn test_17() {
        let opts = default_compile_options();
        assert_is_match("abz", "a!(z)", opts, true);
    }

    #[test]
    fn test_18() {
        let opts = default_compile_options();
        assert_is_match("az", "a!(z)", opts, false);
    }

    #[test]
    fn test_19() {
        let opts = default_compile_options();
        assert_is_match("a.", "a!(.)", opts, false);
    }

    #[test]
    fn test_20() {
        let opts = default_compile_options();
        assert_is_match(".a", "!(.)a", opts, false);
    }

    #[test]
    fn test_21() {
        let opts = default_compile_options();
        assert_is_match("a.c", "a!(.)c", opts, false);
    }

    #[test]
    fn test_22() {
        let opts = default_compile_options();
        assert_is_match("abc", "a!(.)c", opts, true);
    }

    #[test]
    fn test_23() {
        let opts = default_compile_options();
        assert_is_match("/file.d.ts", "/!(*.d).ts", opts, false);
    }

    #[test]
    fn test_24() {
        let opts = default_compile_options();
        assert_is_match("/file.ts", "/!(*.d).ts", opts, true);
    }

    #[test]
    fn test_25() {
        let opts = default_compile_options();
        assert_is_match("/file.something.ts", "/!(*.d).ts", opts, true);
    }

    #[test]
    fn test_26() {
        let opts = default_compile_options();
        assert_is_match("/file.d.something.ts", "/!(*.d).ts", opts, true);
    }

    #[test]
    fn test_27() {
        let opts = default_compile_options();
        assert_is_match("/file.dhello.ts", "/!(*.d).ts", opts, true);
    }

    #[test]
    fn test_28() {
        let opts = default_compile_options();
        assert_is_match("/file.d.ts", "**/!(*.d).ts", opts, false);
    }

    #[test]
    fn test_29() {
        let opts = default_compile_options();
        assert_is_match("/file.ts", "**/!(*.d).ts", opts, true);
    }

    #[test]
    fn test_30() {
        let opts = default_compile_options();
        assert_is_match("/file.something.ts", "**/!(*.d).ts", opts, true);
    }

    #[test]
    fn test_31() {
        let opts = default_compile_options();
        assert_is_match("/file.d.something.ts", "**/!(*.d).ts", opts, true);
    }

    #[test]
    fn test_32() {
        let opts = default_compile_options();
        assert_is_match("/file.dhello.ts", "**/!(*.d).ts", opts, true);
    }

    #[test]
    fn test_33() {
        let opts = default_compile_options();
        assert_is_match("/file.d.ts", "/!(*.d).{ts,tsx}", opts, false);
    }

    #[test]
    fn test_34() {
        let opts = default_compile_options();
        assert_is_match("/file.ts", "/!(*.d).{ts,tsx}", opts, true);
    }

    #[test]
    fn test_35() {
        let opts = default_compile_options();
        assert_is_match("/file.something.ts", "/!(*.d).{ts,tsx}", opts, true);
    }

    #[test]
    fn test_36() {
        let opts = default_compile_options();
        assert_is_match("/file.d.something.ts", "/!(*.d).{ts,tsx}", opts, true);
    }

    #[test]
    fn test_37() {
        let opts = default_compile_options();
        assert_is_match("/file.dhello.ts", "/!(*.d).{ts,tsx}", opts, true);
    }

    #[test]
    fn test_38() {
        let opts = default_compile_options();
        assert_is_match("/file.d.ts", "/!(*.d).@(ts)", opts, false);
    }

    #[test]
    fn test_39() {
        let opts = default_compile_options();
        assert_is_match("/file.ts", "/!(*.d).@(ts)", opts, true);
    }

    #[test]
    fn test_40() {
        let opts = default_compile_options();
        assert_is_match("/file.something.ts", "/!(*.d).@(ts)", opts, true);
    }

    #[test]
    fn test_41() {
        let opts = default_compile_options();
        assert_is_match("/file.d.something.ts", "/!(*.d).@(ts)", opts, true);
    }

    #[test]
    fn test_42() {
        let opts = default_compile_options();
        assert_is_match("/file.dhello.ts", "/!(*.d).@(ts)", opts, true);
    }

    #[test]
    fn test_43() {
        let opts = default_compile_options();
        assert_is_match("foo/abc", "foo/!(abc)", opts, false);
    }

    #[test]
    fn test_44() {
        let opts = default_compile_options();
        assert_is_match("foo/bar", "foo/!(abc)", opts, true);
    }

    #[test]
    fn test_45() {
        let opts = default_compile_options();
        assert_is_match("a/z", "a/!(z)", opts, false);
    }

    #[test]
    fn test_46() {
        let opts = default_compile_options();
        assert_is_match("a/b", "a/!(z)", opts, true);
    }

    #[test]
    fn test_47() {
        let opts = default_compile_options();
        assert_is_match("c/z/v", "c/!(z)/v", opts, false);
    }

    #[test]
    fn test_48() {
        let opts = default_compile_options();
        assert_is_match("c/a/v", "c/!(z)/v", opts, true);
    }

    #[test]
    fn test_49() {
        let opts = default_compile_options();
        assert_is_match("a/a", "!(b/a)", opts, true);
    }

    #[test]
    fn test_50() {
        let opts = default_compile_options();
        assert_is_match("b/a", "!(b/a)", opts, false);
    }

    #[test]
    fn test_51() {
        let opts = default_compile_options();
        assert_is_match("foo/bar", "!(!(foo))*", opts, false);
    }

    #[test]
    fn test_52() {
        let opts = default_compile_options();
        assert_is_match("a/a", "!(b/a)", opts, true);
    }

    #[test]
    fn test_53() {
        let opts = default_compile_options();
        assert_is_match("b/a", "!(b/a)", opts, false);
    }

    #[test]
    fn test_54() {
        let opts = default_compile_options();
        assert_is_match("a/a", "(!(b/a))", opts, true);
    }

    #[test]
    fn test_55() {
        let opts = default_compile_options();
        assert_is_match("a/a", "!((b/a))", opts, true);
    }

    #[test]
    fn test_56() {
        let opts = default_compile_options();
        assert_is_match("b/a", "!((b/a))", opts, false);
    }

    #[test]
    fn test_57() {
        let opts = default_compile_options();
        assert_is_match("a/a", "(!(?:b/a))", opts, false);
    }

    #[test]
    fn test_58() {
        let opts = default_compile_options();
        assert_is_match("b/a", "!((?:b/a))", opts, false);
    }

    #[test]
    fn test_59() {
        let opts = default_compile_options();
        assert_is_match("a/a", "!(b/(a))", opts, true);
    }

    #[test]
    fn test_60() {
        let opts = default_compile_options();
        assert_is_match("b/a", "!(b/(a))", opts, false);
    }

    #[test]
    fn test_61() {
        let opts = default_compile_options();
        assert_is_match("a/a", "!(b/a)", opts, true);
    }

    #[test]
    fn test_62() {
        let opts = default_compile_options();
        assert_is_match("b/a", "!(b/a)", opts, false);
    }

    #[test]
    fn test_63() {
        let opts = default_compile_options();
        assert_is_match("c/z", "c!(z)", opts, false);
    }

    #[test]
    fn test_64() {
        let opts = default_compile_options();
        assert_is_match("c/z", "c!(z)z", opts, false);
    }

    #[test]
    fn test_65() {
        let opts = default_compile_options();
        assert_is_match("c/z", "c!(.)z", opts, false);
    }

    #[test]
    fn test_66() {
        let opts = default_compile_options();
        assert_is_match("c/z", "c!(*)z", opts, false);
    }

    #[test]
    fn test_67() {
        let opts = default_compile_options();
        assert_is_match("c/z", "c!(+)z", opts, false);
    }

    #[test]
    fn test_68() {
        let opts = default_compile_options();
        assert_is_match("c/z", "c!(?)z", opts, false);
    }

    #[test]
    fn test_69() {
        let opts = default_compile_options();
        assert_is_match("c/z", "c!(@)z", opts, false);
    }

    #[test]
    fn test_70() {
        let opts = default_compile_options();
        assert_is_match("c/z", "a!(z)", opts, false);
    }

    #[test]
    fn test_71() {
        let opts = default_compile_options();
        assert_is_match("c/z", "c!(.)z", opts, false);
    }

    #[test]
    fn test_72() {
        let opts = default_compile_options();
        assert_is_match("c/z", "c!(/)z", opts, false);
    }

    #[test]
    fn test_73() {
        let opts = default_compile_options();
        assert_is_match("c/z", "c!(/z)z", opts, false);
    }

    #[test]
    fn test_74() {
        let opts = default_compile_options();
        assert_is_match("c/b", "c!(/z)z", opts, false);
    }

    #[test]
    fn test_75() {
        let opts = default_compile_options();
        assert_is_match("c/b/z", "c!(/z)z", opts, true);
    }

    #[test]
    fn test_76() {
        let opts = default_compile_options();
        assert_is_match("abc", "!!(abc)", opts, true);
    }

    #[test]
    fn test_77() {
        let opts = default_compile_options();
        assert_is_match("abc", "!!!(abc)", opts, false);
    }

    #[test]
    fn test_78() {
        let opts = default_compile_options();
        assert_is_match("abc", "!!!!(abc)", opts, true);
    }

    #[test]
    fn test_79() {
        let opts = default_compile_options();
        assert_is_match("abc", "!!!!!(abc)", opts, false);
    }

    #[test]
    fn test_80() {
        let opts = default_compile_options();
        assert_is_match("abc", "!!!!!!(abc)", opts, true);
    }

    #[test]
    fn test_81() {
        let opts = default_compile_options();
        assert_is_match("abc", "!!!!!!!(abc)", opts, false);
    }

    #[test]
    fn test_82() {
        let opts = default_compile_options();
        assert_is_match("abc", "!!!!!!!!(abc)", opts, true);
    }

    #[test]
    fn test_83() {
        let opts = default_compile_options();
        assert_is_match("abc", "!(!(abc))", opts, true);
    }

    #[test]
    fn test_84() {
        let opts = default_compile_options();
        assert_is_match("abc", "!(!(!(abc)))", opts, false);
    }

    #[test]
    fn test_85() {
        let opts = default_compile_options();
        assert_is_match("abc", "!(!(!(!(abc))))", opts, true);
    }

    #[test]
    fn test_86() {
        let opts = default_compile_options();
        assert_is_match("abc", "!(!(!(!(!(abc)))))", opts, false);
    }

    #[test]
    fn test_87() {
        let opts = default_compile_options();
        assert_is_match("abc", "!(!(!(!(!(!(abc))))))", opts, true);
    }

    #[test]
    fn test_88() {
        let opts = default_compile_options();
        assert_is_match("abc", "!(!(!(!(!(!(!(abc)))))))", opts, false);
    }

    #[test]
    fn test_89() {
        let opts = default_compile_options();
        assert_is_match("abc", "!(!(!(!(!(!(!(!(abc))))))))", opts, true);
    }

    #[test]
    fn test_90() {
        let opts = default_compile_options();
        assert_is_match("foo/abc", "foo/!(!(abc))", opts, true);
    }

    #[test]
    fn test_91() {
        let opts = default_compile_options();
        assert_is_match("foo/abc", "foo/!(!(!(abc)))", opts, false);
    }

    #[test]
    fn test_92() {
        let opts = default_compile_options();
        assert_is_match("foo/abc", "foo/!(!(!(!(abc))))", opts, true);
    }

    #[test]
    fn test_93() {
        let opts = default_compile_options();
        assert_is_match("foo/abc", "foo/!(!(!(!(!(abc)))))", opts, false);
    }

    #[test]
    fn test_94() {
        let opts = default_compile_options();
        assert_is_match("foo/abc", "foo/!(!(!(!(!(!(abc))))))", opts, true);
    }

    #[test]
    fn test_95() {
        let opts = default_compile_options();
        assert_is_match("foo/abc", "foo/!(!(!(!(!(!(!(abc)))))))", opts, false);
    }

    #[test]
    fn test_96() {
        let opts = default_compile_options();
        assert_is_match("foo/abc", "foo/!(!(!(!(!(!(!(!(abc))))))))", opts, true);
    }

    #[test]
    fn test_97() {
        let opts = default_compile_options();
        assert_is_match("moo.cow", "!(moo).!(cow)", opts, false);
    }

    #[test]
    fn test_98() {
        let opts = default_compile_options();
        assert_is_match("foo.cow", "!(moo).!(cow)", opts, false);
    }

    #[test]
    fn test_99() {
        let opts = default_compile_options();
        assert_is_match("moo.bar", "!(moo).!(cow)", opts, false);
    }

    #[test]
    fn test_100() {
        let opts = default_compile_options();
        assert_is_match("foo.bar", "!(moo).!(cow)", opts, true);
    }

    #[test]
    fn test_101() {
        let opts = default_compile_options();
        assert_is_match("a   ", "@(!(a) )*", opts, false);
    }

    #[test]
    fn test_102() {
        let opts = default_compile_options();
        assert_is_match("a   b", "@(!(a) )*", opts, false);
    }

    #[test]
    fn test_103() {
        let opts = default_compile_options();
        assert_is_match("a  b", "@(!(a) )*", opts, false);
    }

    #[test]
    fn test_104() {
        let opts = default_compile_options();
        assert_is_match("a  ", "@(!(a) )*", opts, false);
    }

    #[test]
    fn test_105() {
        let opts = default_compile_options();
        assert_is_match("a ", "@(!(a) )*", opts, false);
    }

    #[test]
    fn test_106() {
        let opts = default_compile_options();
        assert_is_match("a", "@(!(a) )*", opts, false);
    }

    #[test]
    fn test_107() {
        let opts = default_compile_options();
        assert_is_match("aa", "@(!(a) )*", opts, false);
    }

    #[test]
    fn test_108() {
        let opts = default_compile_options();
        assert_is_match("b", "@(!(a) )*", opts, false);
    }

    #[test]
    fn test_109() {
        let opts = default_compile_options();
        assert_is_match("bb", "@(!(a) )*", opts, false);
    }

    #[test]
    fn test_110() {
        let opts = default_compile_options();
        assert_is_match(" a ", "@(!(a) )*", opts, true);
    }

    #[test]
    fn test_111() {
        let opts = default_compile_options();
        assert_is_match("b  ", "@(!(a) )*", opts, true);
    }

    #[test]
    fn test_112() {
        let opts = default_compile_options();
        assert_is_match("b ", "@(!(a) )*", opts, true);
    }

    #[test]
    fn test_113() {
        let opts = default_compile_options();
        assert_is_match("c/z", "a*!(z)", opts, false);
    }

    #[test]
    fn test_114() {
        let opts = default_compile_options();
        assert_is_match("abz", "a*!(z)", opts, true);
    }

    #[test]
    fn test_115() {
        let opts = default_compile_options();
        assert_is_match("az", "a*!(z)", opts, true);
    }

    #[test]
    fn test_116() {
        let opts = default_compile_options();
        assert_is_match("a", "!(a*)", opts, false);
    }

    #[test]
    fn test_117() {
        let opts = default_compile_options();
        assert_is_match("aa", "!(a*)", opts, false);
    }

    #[test]
    fn test_118() {
        let opts = default_compile_options();
        assert_is_match("ab", "!(a*)", opts, false);
    }

    #[test]
    fn test_119() {
        let opts = default_compile_options();
        assert_is_match("b", "!(a*)", opts, true);
    }

    #[test]
    fn test_120() {
        let opts = default_compile_options();
        assert_is_match("a", "!(*a*)", opts, false);
    }

    #[test]
    fn test_121() {
        let opts = default_compile_options();
        assert_is_match("aa", "!(*a*)", opts, false);
    }

    #[test]
    fn test_122() {
        let opts = default_compile_options();
        assert_is_match("ab", "!(*a*)", opts, false);
    }

    #[test]
    fn test_123() {
        let opts = default_compile_options();
        assert_is_match("ac", "!(*a*)", opts, false);
    }

    #[test]
    fn test_124() {
        let opts = default_compile_options();
        assert_is_match("b", "!(*a*)", opts, true);
    }

    #[test]
    fn test_125() {
        let opts = default_compile_options();
        assert_is_match("a", "!(*a)", opts, false);
    }

    #[test]
    fn test_126() {
        let opts = default_compile_options();
        assert_is_match("aa", "!(*a)", opts, false);
    }

    #[test]
    fn test_127() {
        let opts = default_compile_options();
        assert_is_match("bba", "!(*a)", opts, false);
    }

    #[test]
    fn test_128() {
        let opts = default_compile_options();
        assert_is_match("ab", "!(*a)", opts, true);
    }

    #[test]
    fn test_129() {
        let opts = default_compile_options();
        assert_is_match("ac", "!(*a)", opts, true);
    }

    #[test]
    fn test_130() {
        let opts = default_compile_options();
        assert_is_match("b", "!(*a)", opts, true);
    }

    #[test]
    fn test_131() {
        let opts = default_compile_options();
        assert_is_match("a", "!(*a)*", opts, false);
    }

    #[test]
    fn test_132() {
        let opts = default_compile_options();
        assert_is_match("aa", "!(*a)*", opts, false);
    }

    #[test]
    fn test_133() {
        let opts = default_compile_options();
        assert_is_match("bba", "!(*a)*", opts, false);
    }

    #[test]
    fn test_134() {
        let opts = default_compile_options();
        assert_is_match("ab", "!(*a)*", opts, false);
    }

    #[test]
    fn test_135() {
        let opts = default_compile_options();
        assert_is_match("ac", "!(*a)*", opts, false);
    }

    #[test]
    fn test_136() {
        let opts = default_compile_options();
        assert_is_match("b", "!(*a)*", opts, true);
    }

    #[test]
    fn test_137() {
        let opts = default_compile_options();
        assert_is_match("a", "!(a)*", opts, false);
    }

    #[test]
    fn test_138() {
        let opts = default_compile_options();
        assert_is_match("abb", "!(a)*", opts, false);
    }

    #[test]
    fn test_139() {
        let opts = default_compile_options();
        assert_is_match("ba", "!(a)*", opts, true);
    }

    #[test]
    fn test_140() {
        let opts = default_compile_options();
        assert_is_match("aa", "a!(b)*", opts, true);
    }

    #[test]
    fn test_141() {
        let opts = default_compile_options();
        assert_is_match("ab", "a!(b)*", opts, false);
    }

    #[test]
    fn test_142() {
        let opts = default_compile_options();
        assert_is_match("aba", "a!(b)*", opts, false);
    }

    #[test]
    fn test_143() {
        let opts = default_compile_options();
        assert_is_match("ac", "a!(b)*", opts, true);
    }

    #[test]
    fn test_144() {
        let opts = default_compile_options();
        assert_is_match("moo.cow", "!(!(moo)).!(!(cow))", opts, true);
    }

    #[test]
    fn test_145() {
        let opts = default_compile_options();
        assert_is_match("ac", "!(a|b)c", opts, false);
    }

    #[test]
    fn test_146() {
        let opts = default_compile_options();
        assert_is_match("bc", "!(a|b)c", opts, false);
    }

    #[test]
    fn test_147() {
        let opts = default_compile_options();
        assert_is_match("cc", "!(a|b)c", opts, true);
    }

    #[test]
    fn test_148() {
        let opts = default_compile_options();
        assert_is_match("ac.d", "!(a|b)c.!(d|e)", opts, false);
    }

    #[test]
    fn test_149() {
        let opts = default_compile_options();
        assert_is_match("bc.d", "!(a|b)c.!(d|e)", opts, false);
    }

    #[test]
    fn test_150() {
        let opts = default_compile_options();
        assert_is_match("cc.d", "!(a|b)c.!(d|e)", opts, false);
    }

    #[test]
    fn test_151() {
        let opts = default_compile_options();
        assert_is_match("ac.e", "!(a|b)c.!(d|e)", opts, false);
    }

    #[test]
    fn test_152() {
        let opts = default_compile_options();
        assert_is_match("bc.e", "!(a|b)c.!(d|e)", opts, false);
    }

    #[test]
    fn test_153() {
        let opts = default_compile_options();
        assert_is_match("cc.e", "!(a|b)c.!(d|e)", opts, false);
    }

    #[test]
    fn test_154() {
        let opts = default_compile_options();
        assert_is_match("ac.f", "!(a|b)c.!(d|e)", opts, false);
    }

    #[test]
    fn test_155() {
        let opts = default_compile_options();
        assert_is_match("bc.f", "!(a|b)c.!(d|e)", opts, false);
    }

    #[test]
    fn test_156() {
        let opts = default_compile_options();
        assert_is_match("cc.f", "!(a|b)c.!(d|e)", opts, true);
    }

    #[test]
    fn test_157() {
        let opts = default_compile_options();
        assert_is_match("dc.g", "!(a|b)c.!(d|e)", opts, true);
    }

    #[test]
    fn test_158() {
        let opts = default_compile_options();
        assert_is_match("ac.d", "!(!(a|b)c.!(d|e))", opts, true);
    }

    #[test]
    fn test_159() {
        let opts = default_compile_options();
        assert_is_match("bc.d", "!(!(a|b)c.!(d|e))", opts, true);
    }

    #[test]
    fn test_160() {
        let opts = default_compile_options();
        assert_is_match("cc.d", "!(a|b)c.!(d|e)", opts, false);
    }

    #[test]
    fn test_161() {
        let opts = default_compile_options();
        assert_is_match("cc.d", "!(!(a|b)c.!(d|e))", opts, true);
    }

    #[test]
    fn test_162() {
        let opts = default_compile_options();
        assert_is_match("cc.d", "!(!(a|b)c.!(d|e))", opts, true);
    }

    #[test]
    fn test_163() {
        let opts = default_compile_options();
        assert_is_match("ac.e", "!(!(a|b)c.!(d|e))", opts, true);
    }

    #[test]
    fn test_164() {
        let opts = default_compile_options();
        assert_is_match("bc.e", "!(!(a|b)c.!(d|e))", opts, true);
    }

    #[test]
    fn test_165() {
        let opts = default_compile_options();
        assert_is_match("cc.e", "!(!(a|b)c.!(d|e))", opts, true);
    }

    #[test]
    fn test_166() {
        let opts = default_compile_options();
        assert_is_match("ac.f", "!(!(a|b)c.!(d|e))", opts, true);
    }

    #[test]
    fn test_167() {
        let opts = default_compile_options();
        assert_is_match("bc.f", "!(!(a|b)c.!(d|e))", opts, true);
    }

    #[test]
    fn test_168() {
        let opts = default_compile_options();
        assert_is_match("cc.f", "!(!(a|b)c.!(d|e))", opts, false);
    }

    #[test]
    fn test_169() {
        let opts = default_compile_options();
        assert_is_match("dc.g", "!(!(a|b)c.!(d|e))", opts, false);
    }

    #[test]
    fn test_170() {
        let opts = default_compile_options();
        assert_is_match(".md", "@(a|b).md", opts, false);
    }

    #[test]
    fn test_171() {
        let opts = default_compile_options();
        assert_is_match("a.js", "@(a|b).md", opts, false);
    }

    #[test]
    fn test_172() {
        let opts = default_compile_options();
        assert_is_match("c.md", "@(a|b).md", opts, false);
    }

    #[test]
    fn test_173() {
        let opts = default_compile_options();
        assert_is_match("a.md", "@(a|b).md", opts, true);
    }

    #[test]
    fn test_174() {
        let opts = default_compile_options();
        assert_is_match("b.md", "@(a|b).md", opts, true);
    }

    #[test]
    fn test_175() {
        let opts = default_compile_options();
        assert_is_match(".md", "+(a|b).md", opts, false);
    }

    #[test]
    fn test_176() {
        let opts = default_compile_options();
        assert_is_match("a.js", "+(a|b).md", opts, false);
    }

    #[test]
    fn test_177() {
        let opts = default_compile_options();
        assert_is_match("c.md", "+(a|b).md", opts, false);
    }

    #[test]
    fn test_178() {
        let opts = default_compile_options();
        assert_is_match("a.md", "+(a|b).md", opts, true);
    }

    #[test]
    fn test_179() {
        let opts = default_compile_options();
        assert_is_match("aa.md", "+(a|b).md", opts, true);
    }

    #[test]
    fn test_180() {
        let opts = default_compile_options();
        assert_is_match("ab.md", "+(a|b).md", opts, true);
    }

    #[test]
    fn test_181() {
        let opts = default_compile_options();
        assert_is_match("b.md", "+(a|b).md", opts, true);
    }

    #[test]
    fn test_182() {
        let opts = default_compile_options();
        assert_is_match("bb.md", "+(a|b).md", opts, true);
    }

    #[test]
    fn test_183() {
        let opts = default_compile_options();
        assert_is_match("a.js", "*(a|b).md", opts, false);
    }

    #[test]
    fn test_184() {
        let opts = default_compile_options();
        assert_is_match("c.md", "*(a|b).md", opts, false);
    }

    #[test]
    fn test_185() {
        let opts = default_compile_options();
        assert_is_match(".md", "*(a|b).md", opts, true);
    }

    #[test]
    fn test_186() {
        let opts = default_compile_options();
        assert_is_match("a.md", "*(a|b).md", opts, true);
    }

    #[test]
    fn test_187() {
        let opts = default_compile_options();
        assert_is_match("aa.md", "*(a|b).md", opts, true);
    }

    #[test]
    fn test_188() {
        let opts = default_compile_options();
        assert_is_match("ab.md", "*(a|b).md", opts, true);
    }

    #[test]
    fn test_189() {
        let opts = default_compile_options();
        assert_is_match("b.md", "*(a|b).md", opts, true);
    }

    #[test]
    fn test_190() {
        let opts = default_compile_options();
        assert_is_match("bb.md", "*(a|b).md", opts, true);
    }

    #[test]
    fn test_191() {
        let opts = default_compile_options();
        assert_is_match("a.js", "?(a|b).md", opts, false);
    }

    #[test]
    fn test_192() {
        let opts = default_compile_options();
        assert_is_match("bb.md", "?(a|b).md", opts, false);
    }

    #[test]
    fn test_193() {
        let opts = default_compile_options();
        assert_is_match("c.md", "?(a|b).md", opts, false);
    }

    #[test]
    fn test_194() {
        let opts = default_compile_options();
        assert_is_match(".md", "?(a|b).md", opts, true);
    }

    #[test]
    fn test_195() {
        let opts = default_compile_options();
        assert_is_match("a.md", "?(a|ab|b).md", opts, true);
    }

    #[test]
    fn test_196() {
        let opts = default_compile_options();
        assert_is_match("a.md", "?(a|b).md", opts, true);
    }

    #[test]
    fn test_197() {
        let opts = default_compile_options();
        assert_is_match("aa.md", "?(a|aa|b).md", opts, true);
    }

    #[test]
    fn test_198() {
        let opts = default_compile_options();
        assert_is_match("ab.md", "?(a|ab|b).md", opts, true);
    }

    #[test]
    fn test_199() {
        let opts = default_compile_options();
        assert_is_match("b.md", "?(a|ab|b).md", opts, true);
    }

    #[test]
    fn test_200() {
        let opts = default_compile_options();
        assert_is_match("ab", "+(a)?(b)", opts, true);
    }

    #[test]
    fn test_201() {
        let opts = default_compile_options();
        assert_is_match("aab", "+(a)?(b)", opts, true);
    }

    #[test]
    fn test_202() {
        let opts = default_compile_options();
        assert_is_match("aa", "+(a)?(b)", opts, true);
    }

    #[test]
    fn test_203() {
        let opts = default_compile_options();
        assert_is_match("a", "+(a)?(b)", opts, true);
    }

    #[test]
    fn test_204() {
        let opts = default_compile_options();
        assert_is_match("ax", "a?(b*)", opts, false);
    }

    #[test]
    fn test_205() {
        let opts = default_compile_options();
        assert_is_match("ax", "?(a*|b)", opts, true);
    }

    #[test]
    fn test_206() {
        let opts = default_compile_options();
        assert_is_match("ax", "a*(b*)", opts, false);
    }

    #[test]
    fn test_207() {
        let opts = default_compile_options();
        assert_is_match("ax", "*(a*|b)", opts, true);
    }

    #[test]
    fn test_208() {
        let opts = default_compile_options();
        assert_is_match("ax", "a@(b*)", opts, false);
    }

    #[test]
    fn test_209() {
        let opts = default_compile_options();
        assert_is_match("ax", "@(a*|b)", opts, true);
    }

    #[test]
    fn test_210() {
        let opts = default_compile_options();
        assert_is_match("ax", "a?(b*)", opts, false);
    }

    #[test]
    fn test_211() {
        let opts = default_compile_options();
        assert_is_match("ax", "?(a*|b)", opts, true);
    }

    #[test]
    fn test_212() {
        let opts = default_compile_options();
        assert_is_match("ax", "a!(b*)", opts, true);
    }

    #[test]
    fn test_213() {
        let opts = default_compile_options();
        assert_is_match("ax", "!(a*|b)", opts, false);
    }

    #[test]
    fn test_214() {
        let opts = default_compile_options();
        assert_is_match("a", "!(a/**)", opts, true);
    }

    #[test]
    fn test_215() {
        let opts = default_compile_options();
        assert_is_match("a/", "!(a/**)", opts, false);
    }

    #[test]
    fn test_216() {
        let opts = default_compile_options();
        assert_is_match("a/b", "!(a/**)", opts, false);
    }

    #[test]
    fn test_217() {
        let opts = default_compile_options();
        assert_is_match("a/b/c", "!(a/**)", opts, false);
    }

    #[test]
    fn test_218() {
        let opts = default_compile_options();
        assert_is_match("b", "!(a/**)", opts, true);
    }

    #[test]
    fn test_219() {
        let opts = default_compile_options();
        assert_is_match("b/c", "!(a/**)", opts, true);
    }

    #[test]
    fn test_220() {
        let opts = default_compile_options();
        assert_is_match("a/a", "a/!(b*)", opts, true);
    }

    #[test]
    fn test_221() {
        let opts = default_compile_options();
        assert_is_match("a/b", "a/!(b*)", opts, false);
    }

    #[test]
    fn test_222() {
        let opts = default_compile_options();
        assert_is_match("a/b/c", "a/!(b/*)", opts, false);
    }

    #[test]
    fn test_223() {
        let opts = default_compile_options();
        assert_is_match("a/b/c", "a/!(b*)", opts, false);
    }

    #[test]
    fn test_224() {
        let opts = default_compile_options();
        assert_is_match("a/c", "a/!(b*)", opts, true);
    }

    #[test]
    fn test_225() {
        let opts = default_compile_options();
        assert_is_match("a/a/", "a/!(b*)/**", opts, true);
    }

    #[test]
    fn test_226() {
        let opts = default_compile_options();
        assert_is_match("a/a", "a/!(b*)", opts, true);
    }

    #[test]
    fn test_227() {
        let opts = default_compile_options();
        assert_is_match("a/a", "a/!(b*)/**", opts, true);
    }

    #[test]
    fn test_228() {
        let opts = default_compile_options();
        assert_is_match("a/b", "a/!(b*)/**", opts, false);
    }

    #[test]
    fn test_229() {
        let opts = default_compile_options();
        assert_is_match("a/b/c", "a/!(b*)/**", opts, false);
    }

    #[test]
    fn test_230() {
        let opts = default_compile_options();
        assert_is_match("a/c", "a/!(b*)/**", opts, true);
    }

    #[test]
    fn test_231() {
        let opts = default_compile_options();
        assert_is_match("a/c", "a/!(b*)", opts, true);
    }

    #[test]
    fn test_232() {
        let opts = default_compile_options();
        assert_is_match("a/c/", "a/!(b*)/**", opts, true);
    }

    #[test]
    fn test_233() {
        let opts = default_compile_options();
        assert_is_match("a", "a*(z)", opts, true);
    }

    #[test]
    fn test_234() {
        let opts = default_compile_options();
        assert_is_match("az", "a*(z)", opts, true);
    }

    #[test]
    fn test_235() {
        let opts = default_compile_options();
        assert_is_match("azz", "a*(z)", opts, true);
    }

    #[test]
    fn test_236() {
        let opts = default_compile_options();
        assert_is_match("azzz", "a*(z)", opts, true);
    }

    #[test]
    fn test_237() {
        let opts = default_compile_options();
        assert_is_match("abz", "a*(z)", opts, false);
    }

    #[test]
    fn test_238() {
        let opts = default_compile_options();
        assert_is_match("cz", "a*(z)", opts, false);
    }

    #[test]
    fn test_239() {
        let opts = default_compile_options();
        assert_is_match("a/a", "*(b/a)", opts, false);
    }

    #[test]
    fn test_240() {
        let opts = default_compile_options();
        assert_is_match("a/b", "*(b/a)", opts, false);
    }

    #[test]
    fn test_241() {
        let opts = default_compile_options();
        assert_is_match("a/c", "*(b/a)", opts, false);
    }

    #[test]
    fn test_242() {
        let opts = default_compile_options();
        assert_is_match("b/a", "*(b/a)", opts, true);
    }

    #[test]
    fn test_243() {
        let opts = default_compile_options();
        assert_is_match("b/b", "*(b/a)", opts, false);
    }

    #[test]
    fn test_244() {
        let opts = default_compile_options();
        assert_is_match("b/c", "*(b/a)", opts, false);
    }

    #[test]
    fn test_245() {
        let opts = default_compile_options();
        assert_is_match("cz", "a**(z)", opts, false);
    }

    #[test]
    fn test_246() {
        let opts = default_compile_options();
        assert_is_match("abz", "a**(z)", opts, true);
    }

    #[test]
    fn test_247() {
        let opts = default_compile_options();
        assert_is_match("az", "a**(z)", opts, true);
    }

    #[test]
    fn test_248() {
        let opts = default_compile_options();
        assert_is_match("c/z/v", "*(z)", opts, false);
    }

    #[test]
    fn test_249() {
        let opts = default_compile_options();
        assert_is_match("z", "*(z)", opts, true);
    }

    #[test]
    fn test_250() {
        let opts = default_compile_options();
        assert_is_match("zf", "*(z)", opts, false);
    }

    #[test]
    fn test_251() {
        let opts = default_compile_options();
        assert_is_match("fz", "*(z)", opts, false);
    }

    #[test]
    fn test_252() {
        let opts = default_compile_options();
        assert_is_match("c/a/v", "c/*(z)/v", opts, false);
    }

    #[test]
    fn test_253() {
        let opts = default_compile_options();
        assert_is_match("c/z/v", "c/*(z)/v", opts, true);
    }

    #[test]
    fn test_254() {
        let opts = default_compile_options();
        assert_is_match("a.md.js", "*.*(js).js", opts, false);
    }

    #[test]
    fn test_255() {
        let opts = default_compile_options();
        assert_is_match("a.js.js", "*.*(js).js", opts, true);
    }

    #[test]
    fn test_256() {
        let opts = default_compile_options();
        assert_is_match("a", "a+(z)", opts, false);
    }

    #[test]
    fn test_257() {
        let opts = default_compile_options();
        assert_is_match("az", "a+(z)", opts, true);
    }

    #[test]
    fn test_258() {
        let opts = default_compile_options();
        assert_is_match("cz", "a+(z)", opts, false);
    }

    #[test]
    fn test_259() {
        let opts = default_compile_options();
        assert_is_match("abz", "a+(z)", opts, false);
    }

    #[test]
    fn test_260() {
        let opts = default_compile_options();
        assert_is_match("a+z", "a+(z)", opts, false);
    }

    #[test]
    fn test_261() {
        let opts = default_compile_options();
        assert_is_match("a+z", "a++(z)", opts, true);
    }

    #[test]
    fn test_262() {
        let opts = default_compile_options();
        assert_is_match("c+z", "a+(z)", opts, false);
    }

    #[test]
    fn test_263() {
        let opts = default_compile_options();
        assert_is_match("a+bz", "a+(z)", opts, false);
    }

    #[test]
    fn test_264() {
        let opts = default_compile_options();
        assert_is_match("az", "+(z)", opts, false);
    }

    #[test]
    fn test_265() {
        let opts = default_compile_options();
        assert_is_match("cz", "+(z)", opts, false);
    }

    #[test]
    fn test_266() {
        let opts = default_compile_options();
        assert_is_match("abz", "+(z)", opts, false);
    }

    #[test]
    fn test_267() {
        let opts = default_compile_options();
        assert_is_match("fz", "+(z)", opts, false);
    }

    #[test]
    fn test_268() {
        let opts = default_compile_options();
        assert_is_match("z", "+(z)", opts, true);
    }

    #[test]
    fn test_269() {
        let opts = default_compile_options();
        assert_is_match("zz", "+(z)", opts, true);
    }

    #[test]
    fn test_270() {
        let opts = default_compile_options();
        assert_is_match("c/z/v", "c/+(z)/v", opts, true);
    }

    #[test]
    fn test_271() {
        let opts = default_compile_options();
        assert_is_match("c/zz/v", "c/+(z)/v", opts, true);
    }

    #[test]
    fn test_272() {
        let opts = default_compile_options();
        assert_is_match("c/a/v", "c/+(z)/v", opts, false);
    }

    #[test]
    fn test_273() {
        let opts = default_compile_options();
        assert_is_match("a?z", "a??(z)", opts, true);
    }

    #[test]
    fn test_274() {
        let opts = default_compile_options();
        assert_is_match("a.z", "a??(z)", opts, true);
    }

    #[test]
    fn test_275() {
        let opts = default_compile_options();
        assert_is_match("a/z", "a??(z)", opts, false);
    }

    #[test]
    fn test_276() {
        let opts = default_compile_options();
        assert_is_match("a?", "a??(z)", opts, true);
    }

    #[test]
    fn test_277() {
        let opts = default_compile_options();
        assert_is_match("ab", "a??(z)", opts, true);
    }

    #[test]
    fn test_278() {
        let opts = default_compile_options();
        assert_is_match("a/", "a??(z)", opts, false);
    }

    #[test]
    fn test_279() {
        let opts = default_compile_options();
        assert_is_match("a?z", "a?(z)", opts, false);
    }

    #[test]
    fn test_280() {
        let opts = default_compile_options();
        assert_is_match("abz", "a?(z)", opts, false);
    }

    #[test]
    fn test_281() {
        let opts = default_compile_options();
        assert_is_match("z", "a?(z)", opts, false);
    }

    #[test]
    fn test_282() {
        let opts = default_compile_options();
        assert_is_match("a", "a?(z)", opts, true);
    }

    #[test]
    fn test_283() {
        let opts = default_compile_options();
        assert_is_match("az", "a?(z)", opts, true);
    }

    #[test]
    fn test_284() {
        let opts = default_compile_options();
        assert_is_match("abz", "?(z)", opts, false);
    }

    #[test]
    fn test_285() {
        let opts = default_compile_options();
        assert_is_match("az", "?(z)", opts, false);
    }

    #[test]
    fn test_286() {
        let opts = default_compile_options();
        assert_is_match("cz", "?(z)", opts, false);
    }

    #[test]
    fn test_287() {
        let opts = default_compile_options();
        assert_is_match("fz", "?(z)", opts, false);
    }

    #[test]
    fn test_288() {
        let opts = default_compile_options();
        assert_is_match("zz", "?(z)", opts, false);
    }

    #[test]
    fn test_289() {
        let opts = default_compile_options();
        assert_is_match("z", "?(z)", opts, true);
    }

    #[test]
    fn test_290() {
        let opts = default_compile_options();
        assert_is_match("c/a/v", "c/?(z)/v", opts, false);
    }

    #[test]
    fn test_291() {
        let opts = default_compile_options();
        assert_is_match("c/zz/v", "c/?(z)/v", opts, false);
    }

    #[test]
    fn test_292() {
        let opts = default_compile_options();
        assert_is_match("c/z/v", "c/?(z)/v", opts, true);
    }

    #[test]
    fn test_293() {
        let opts = default_compile_options();
        assert_is_match("c/z/v", "c/@(z)/v", opts, true);
    }

    #[test]
    fn test_294() {
        let opts = default_compile_options();
        assert_is_match("c/a/v", "c/@(z)/v", opts, false);
    }

    #[test]
    fn test_295() {
        let opts = default_compile_options();
        assert_is_match("moo.cow", "@(*.*)", opts, true);
    }

    #[test]
    fn test_296() {
        let opts = default_compile_options();
        assert_is_match("cz", "a*@(z)", opts, false);
    }

    #[test]
    fn test_297() {
        let opts = default_compile_options();
        assert_is_match("abz", "a*@(z)", opts, true);
    }

    #[test]
    fn test_298() {
        let opts = default_compile_options();
        assert_is_match("az", "a*@(z)", opts, true);
    }

    #[test]
    fn test_299() {
        let opts = default_compile_options();
        assert_is_match("cz", "a@(z)", opts, false);
    }

    #[test]
    fn test_300() {
        let opts = default_compile_options();
        assert_is_match("abz", "a@(z)", opts, false);
    }

    #[test]
    fn test_301() {
        let opts = default_compile_options();
        assert_is_match("az", "a@(z)", opts, true);
    }

    #[test]
    fn test_302() {
        let opts = default_compile_options();
        assert_is_match("aa.aa", "(b|a).(a)", opts, false);
    }

    #[test]
    fn test_303() {
        let opts = default_compile_options();
        assert_is_match("a.bb", "(b|a).(a)", opts, false);
    }

    #[test]
    fn test_304() {
        let opts = default_compile_options();
        assert_is_match("a.aa.a", "(b|a).(a)", opts, false);
    }

    #[test]
    fn test_305() {
        let opts = default_compile_options();
        assert_is_match("cc.a", "(b|a).(a)", opts, false);
    }

    #[test]
    fn test_306() {
        let opts = default_compile_options();
        assert_is_match("a.a", "(b|a).(a)", opts, true);
    }

    #[test]
    fn test_307() {
        let opts = default_compile_options();
        assert_is_match("c.a", "(b|a).(a)", opts, false);
    }

    #[test]
    fn test_308() {
        let opts = default_compile_options();
        assert_is_match("dd.aa.d", "(b|a).(a)", opts, false);
    }

    #[test]
    fn test_309() {
        let opts = default_compile_options();
        assert_is_match("b.a", "(b|a).(a)", opts, true);
    }

    #[test]
    fn test_310() {
        let opts = default_compile_options();
        assert_is_match("aa.aa", "@(b|a).@(a)", opts, false);
    }

    #[test]
    fn test_311() {
        let opts = default_compile_options();
        assert_is_match("a.bb", "@(b|a).@(a)", opts, false);
    }

    #[test]
    fn test_312() {
        let opts = default_compile_options();
        assert_is_match("a.aa.a", "@(b|a).@(a)", opts, false);
    }

    #[test]
    fn test_313() {
        let opts = default_compile_options();
        assert_is_match("cc.a", "@(b|a).@(a)", opts, false);
    }

    #[test]
    fn test_314() {
        let opts = default_compile_options();
        assert_is_match("a.a", "@(b|a).@(a)", opts, true);
    }

    #[test]
    fn test_315() {
        let opts = default_compile_options();
        assert_is_match("c.a", "@(b|a).@(a)", opts, false);
    }

    #[test]
    fn test_316() {
        let opts = default_compile_options();
        assert_is_match("dd.aa.d", "@(b|a).@(a)", opts, false);
    }

    #[test]
    fn test_317() {
        let opts = default_compile_options();
        assert_is_match("b.a", "@(b|a).@(a)", opts, true);
    }

    #[test]
    fn test_318() {
        let opts = default_compile_options();
        assert_is_match("", "*(0|1|3|5|7|9)", opts, false);
    }

    #[test]
    fn test_319() {
        let opts = default_compile_options();
        assert_is_match("137577991", "*(0|1|3|5|7|9)", opts, true);
    }

    #[test]
    fn test_320() {
        let opts = default_compile_options();
        assert_is_match("2468", "*(0|1|3|5|7|9)", opts, false);
    }

    #[test]
    fn test_321() {
        let opts = default_compile_options();
        assert_is_match("file.c", "*.c?(c)", opts, true);
    }

    #[test]
    fn test_322() {
        let opts = default_compile_options();
        assert_is_match("file.C", "*.c?(c)", opts, false);
    }

    #[test]
    fn test_323() {
        let opts = default_compile_options();
        assert_is_match("file.cc", "*.c?(c)", opts, true);
    }

    #[test]
    fn test_324() {
        let opts = default_compile_options();
        assert_is_match("file.ccc", "*.c?(c)", opts, false);
    }

    #[test]
    fn test_325() {
        let opts = default_compile_options();
        assert_is_match(
            "parse.y",
            "!(*.c|*.h|Makefile.in|config*|README)",
            opts,
            true,
        );
    }

    #[test]
    fn test_326() {
        let opts = default_compile_options();
        assert_is_match(
            "shell.c",
            "!(*.c|*.h|Makefile.in|config*|README)",
            opts,
            false,
        );
    }

    #[test]
    fn test_327() {
        let opts = default_compile_options();
        assert_is_match(
            "Makefile",
            "!(*.c|*.h|Makefile.in|config*|README)",
            opts,
            true,
        );
    }

    #[test]
    fn test_328() {
        let opts = default_compile_options();
        assert_is_match(
            "Makefile.in",
            "!(*.c|*.h|Makefile.in|config*|README)",
            opts,
            false,
        );
    }

    #[test]
    fn test_329() {
        let opts = default_compile_options();
        assert_is_match("VMS.FILE;", r"*\;[1-9]*([0-9])", opts, false);
    }

    #[test]
    fn test_330() {
        let opts = default_compile_options();
        assert_is_match("VMS.FILE;0", r"*\;[1-9]*([0-9])", opts, false);
    }

    #[test]
    fn test_331() {
        let opts = default_compile_options();
        assert_is_match("VMS.FILE;1", r"*\;[1-9]*([0-9])", opts, true);
    }

    #[test]
    fn test_332() {
        let opts = default_compile_options();
        assert_is_match("VMS.FILE;139", r"*\;[1-9]*([0-9])", opts, true);
    }

    #[test]
    fn test_333() {
        let opts = default_compile_options();
        assert_is_match("VMS.FILE;1N", r"*\;[1-9]*([0-9])", opts, false);
    }

    #[test]
    fn test_334() {
        let opts = default_compile_options();
        assert_is_match("abcx", "!([*)*", opts, true);
    }

    #[test]
    fn test_335() {
        let opts = default_compile_options();
        assert_is_match("abcz", "!([*)*", opts, true);
    }

    #[test]
    fn test_336() {
        let opts = default_compile_options();
        assert_is_match("bbc", "!([*)*", opts, true);
    }

    #[test]
    fn test_337() {
        let opts = default_compile_options();
        assert_is_match("abcx", "!([[*])*", opts, true);
    }

    #[test]
    fn test_338() {
        let opts = default_compile_options();
        assert_is_match("abcz", "!([[*])*", opts, true);
    }

    #[test]
    fn test_339() {
        let opts = default_compile_options();
        assert_is_match("bbc", "!([[*])*", opts, true);
    }

    #[test]
    fn test_340() {
        let opts = default_compile_options();
        assert_is_match("abcx", r"+(a|b\[)*", opts, true);
    }

    #[test]
    fn test_341() {
        let opts = default_compile_options();
        assert_is_match("abcz", r"+(a|b\[)*", opts, true);
    }

    #[test]
    fn test_342() {
        let opts = default_compile_options();
        assert_is_match("bbc", r"+(a|b\[)*", opts, false);
    }

    #[test]
    fn test_343() {
        let opts = default_compile_options();
        assert_is_match("abcx", "+(a|b[)*", opts, true);
    }

    #[test]
    fn test_344() {
        let opts = default_compile_options();
        assert_is_match("abcz", "+(a|b[)*", opts, true);
    }

    #[test]
    fn test_345() {
        let opts = default_compile_options();
        assert_is_match("bbc", "+(a|b[)*", opts, false);
    }

    #[test]
    fn test_346() {
        let opts = default_compile_options();
        assert_is_match("abcx", "[a*(]*z", opts, false);
    }

    #[test]
    fn test_347() {
        let opts = default_compile_options();
        assert_is_match("abcz", "[a*(]*z", opts, true);
    }

    #[test]
    fn test_348() {
        let opts = default_compile_options();
        assert_is_match("bbc", "[a*(]*z", opts, false);
    }

    #[test]
    fn test_349() {
        let opts = default_compile_options();
        assert_is_match("aaz", "[a*(]*z", opts, true);
    }

    #[test]
    fn test_350() {
        let opts = default_compile_options();
        assert_is_match("aaaz", "[a*(]*z", opts, true);
    }

    #[test]
    fn test_351() {
        let opts = default_compile_options();
        assert_is_match("abcx", "[a*(]*)z", opts, false);
    }

    #[test]
    fn test_352() {
        let opts = default_compile_options();
        assert_is_match("abcz", "[a*(]*)z", opts, false);
    }

    #[test]
    fn test_353() {
        let opts = default_compile_options();
        assert_is_match("bbc", "[a*(]*)z", opts, false);
    }

    #[test]
    fn test_354() {
        let opts = default_compile_options();
        assert_is_match("abc", "+()c", opts, false);
    }

    #[test]
    fn test_355() {
        let opts = default_compile_options();
        assert_is_match("abc", "+()x", opts, false);
    }

    #[test]
    fn test_356() {
        let opts = default_compile_options();
        assert_is_match("abc", "+(*)c", opts, true);
    }

    #[test]
    fn test_357() {
        let opts = default_compile_options();
        assert_is_match("abc", "+(*)x", opts, false);
    }

    #[test]
    fn test_358() {
        let opts = default_compile_options();
        assert_is_match("abc", "no-file+(a|b)stuff", opts, false);
    }

    #[test]
    fn test_359() {
        let opts = default_compile_options();
        assert_is_match("abc", "no-file+(a*(c)|b)stuff", opts, false);
    }

    #[test]
    fn test_360() {
        let opts = default_compile_options();
        assert_is_match("abd", "a+(b|c)d", opts, true);
    }

    #[test]
    fn test_361() {
        let opts = default_compile_options();
        assert_is_match("acd", "a+(b|c)d", opts, true);
    }

    #[test]
    fn test_362() {
        let opts = default_compile_options();
        assert_is_match("abc", "a+(b|c)d", opts, false);
    }

    #[test]
    fn test_363() {
        let opts = default_compile_options();
        assert_is_match("abd", "a!(b|B)", opts, true);
    }

    #[test]
    fn test_364() {
        let opts = default_compile_options();
        assert_is_match("acd", "a!(@(b|B))", opts, true);
    }

    #[test]
    fn test_365() {
        let opts = default_compile_options();
        assert_is_match("ac", "a!(@(b|B))", opts, true);
    }

    #[test]
    fn test_366() {
        let opts = default_compile_options();
        assert_is_match("ab", "a!(@(b|B))", opts, false);
    }

    #[test]
    fn test_367() {
        let opts = default_compile_options();
        assert_is_match("abc", "a!(@(b|B))d", opts, false);
    }

    #[test]
    fn test_368() {
        let opts = default_compile_options();
        assert_is_match("abd", "a!(@(b|B))d", opts, false);
    }

    #[test]
    fn test_369() {
        let opts = default_compile_options();
        assert_is_match("acd", "a!(@(b|B))d", opts, true);
    }

    #[test]
    fn test_370() {
        let opts = default_compile_options();
        assert_is_match("abd", "a[b*(foo|bar)]d", opts, true);
    }

    #[test]
    fn test_371() {
        let opts = default_compile_options();
        assert_is_match("abc", "a[b*(foo|bar)]d", opts, false);
    }

    #[test]
    fn test_372() {
        let opts = default_compile_options();
        assert_is_match("acd", "a[b*(foo|bar)]d", opts, false);
    }

    #[test]
    fn test_373() {
        let opts = default_compile_options();
        assert_is_match("para", "para+([0-9])", opts, false);
    }

    #[test]
    fn test_374() {
        let opts = default_compile_options();
        assert_is_match("para381", "para?([345]|99)1", opts, false);
    }

    #[test]
    fn test_375() {
        let opts = default_compile_options();
        assert_is_match("paragraph", "para*([0-9])", opts, false);
    }

    #[test]
    fn test_376() {
        let opts = default_compile_options();
        assert_is_match("paramour", "para@(chute|graph)", opts, false);
    }

    #[test]
    fn test_377() {
        let opts = default_compile_options();
        assert_is_match("para", "para*([0-9])", opts, true);
    }

    #[test]
    fn test_378() {
        let opts = default_compile_options();
        assert_is_match("para.38", "para!(*.[0-9])", opts, true);
    }

    #[test]
    fn test_379() {
        let opts = default_compile_options();
        assert_is_match("para.38", "para!(*.[00-09])", opts, true);
    }

    #[test]
    fn test_380() {
        let opts = default_compile_options();
        assert_is_match("para.graph", "para!(*.[0-9])", opts, true);
    }

    #[test]
    fn test_381() {
        let opts = default_compile_options();
        assert_is_match("para13829383746592", "para*([0-9])", opts, true);
    }

    #[test]
    fn test_382() {
        let opts = default_compile_options();
        assert_is_match("para39", "para!(*.[0-9])", opts, true);
    }

    #[test]
    fn test_383() {
        let opts = default_compile_options();
        assert_is_match("para987346523", "para+([0-9])", opts, true);
    }

    #[test]
    fn test_384() {
        let opts = default_compile_options();
        assert_is_match("para991", "para?([345]|99)1", opts, true);
    }

    #[test]
    fn test_385() {
        let opts = default_compile_options();
        assert_is_match("paragraph", "para!(*.[0-9])", opts, true);
    }

    #[test]
    fn test_386() {
        let opts = default_compile_options();
        assert_is_match("paragraph", "para@(chute|graph)", opts, true);
    }

    #[test]
    fn test_387() {
        let opts = default_compile_options();
        assert_is_match("foo", "*(a|b[)", opts, false);
    }

    #[test]
    fn test_388() {
        let opts = default_compile_options();
        assert_is_match("(", "*(a|b[)", opts, false);
    }

    #[test]
    fn test_389() {
        let opts = default_compile_options();
        assert_is_match(")", "*(a|b[)", opts, false);
    }

    #[test]
    fn test_390() {
        let opts = default_compile_options();
        assert_is_match("|", "*(a|b[)", opts, false);
    }

    #[test]
    fn test_391() {
        let opts = default_compile_options();
        assert_is_match("a", "*(a|b)", opts, true);
    }

    #[test]
    fn test_392() {
        let opts = default_compile_options();
        assert_is_match("b", "*(a|b)", opts, true);
    }

    #[test]
    fn test_393() {
        let opts = default_compile_options();
        assert_is_match("b[", r"*(a|b\[)", opts, true);
    }

    #[test]
    fn test_394() {
        let opts = default_compile_options();
        assert_is_match("ab[", r"+(a|b\[)", opts, true);
    }

    #[test]
    fn test_395() {
        let opts = default_compile_options();
        assert_is_match("ab[cde", r"+(a|b\[)", opts, false);
    }

    #[test]
    fn test_396() {
        let opts = default_compile_options();
        assert_is_match("ab[cde", r"+(a|b\[)*", opts, true);
    }

    #[test]
    fn test_397() {
        let opts = default_compile_options();
        assert_is_match("foo", "*(a|b|f)*", opts, true);
    }

    #[test]
    fn test_398() {
        let opts = default_compile_options();
        assert_is_match("foo", "*(a|b|o)*", opts, true);
    }

    #[test]
    fn test_399() {
        let opts = default_compile_options();
        assert_is_match("foo", "*(a|b|f|o)", opts, true);
    }

    #[test]
    fn test_400() {
        let opts = default_compile_options();
        assert_is_match("*(a|b[)", r"\*\(a\|b\[\)", opts, true);
    }

    #[test]
    fn test_401() {
        let opts = default_compile_options();
        assert_is_match("foo", "*(a|b)", opts, false);
    }

    #[test]
    fn test_402() {
        let opts = default_compile_options();
        assert_is_match("foo", r"*(a|b\[)", opts, false);
    }

    #[test]
    fn test_403() {
        let opts = default_compile_options();
        assert_is_match("foo", r"*(a|b\[)|f*", opts, true);
    }

    #[test]
    fn test_404() {
        let opts = default_compile_options();
        assert_is_match("moo.cow", "@(*).@(*)", opts, true);
    }

    #[test]
    fn test_405() {
        let opts = default_compile_options();
        assert_is_match("a.a", "*.@(a|b|@(ab|a*@(b))*@(c)d)", opts, true);
    }

    #[test]
    fn test_406() {
        let opts = default_compile_options();
        assert_is_match("a.b", "*.@(a|b|@(ab|a*@(b))*@(c)d)", opts, true);
    }

    #[test]
    fn test_407() {
        let opts = default_compile_options();
        assert_is_match("a.c", "*.@(a|b|@(ab|a*@(b))*@(c)d)", opts, false);
    }

    #[test]
    fn test_408() {
        let opts = default_compile_options();
        assert_is_match("a.c.d", "*.@(a|b|@(ab|a*@(b))*@(c)d)", opts, false);
    }

    #[test]
    fn test_409() {
        let opts = default_compile_options();
        assert_is_match("c.c", "*.@(a|b|@(ab|a*@(b))*@(c)d)", opts, false);
    }

    #[test]
    fn test_410() {
        let opts = default_compile_options();
        assert_is_match("a.", "*.@(a|b|@(ab|a*@(b))*@(c)d)", opts, false);
    }

    #[test]
    fn test_411() {
        let opts = default_compile_options();
        assert_is_match("d.d", "*.@(a|b|@(ab|a*@(b))*@(c)d)", opts, false);
    }

    #[test]
    fn test_412() {
        let opts = default_compile_options();
        assert_is_match("e.e", "*.@(a|b|@(ab|a*@(b))*@(c)d)", opts, false);
    }

    #[test]
    fn test_413() {
        let opts = default_compile_options();
        assert_is_match("f.f", "*.@(a|b|@(ab|a*@(b))*@(c)d)", opts, false);
    }

    #[test]
    fn test_414() {
        let opts = default_compile_options();
        assert_is_match("a.abcd", "*.@(a|b|@(ab|a*@(b))*@(c)d)", opts, true);
    }

    #[test]
    fn test_415() {
        let opts = default_compile_options();
        assert_is_match("a.a", "!(*.a|*.b|*.c)", opts, false);
    }

    #[test]
    fn test_416() {
        let opts = default_compile_options();
        assert_is_match("a.b", "!(*.a|*.b|*.c)", opts, false);
    }

    #[test]
    fn test_417() {
        let opts = default_compile_options();
        assert_is_match("a.c", "!(*.a|*.b|*.c)", opts, false);
    }

    #[test]
    fn test_418() {
        let opts = default_compile_options();
        assert_is_match("a.c.d", "!(*.a|*.b|*.c)", opts, true);
    }

    #[test]
    fn test_419() {
        let opts = default_compile_options();
        assert_is_match("c.c", "!(*.a|*.b|*.c)", opts, false);
    }

    #[test]
    fn test_420() {
        let opts = default_compile_options();
        assert_is_match("a.", "!(*.a|*.b|*.c)", opts, true);
    }

    #[test]
    fn test_421() {
        let opts = default_compile_options();
        assert_is_match("d.d", "!(*.a|*.b|*.c)", opts, true);
    }

    #[test]
    fn test_422() {
        let opts = default_compile_options();
        assert_is_match("e.e", "!(*.a|*.b|*.c)", opts, true);
    }

    #[test]
    fn test_423() {
        let opts = default_compile_options();
        assert_is_match("f.f", "!(*.a|*.b|*.c)", opts, true);
    }

    #[test]
    fn test_424() {
        let opts = default_compile_options();
        assert_is_match("a.abcd", "!(*.a|*.b|*.c)", opts, true);
    }

    #[test]
    fn test_425() {
        let opts = default_compile_options();
        assert_is_match("a.a", "!(*.[^a-c])", opts, true);
    }

    #[test]
    fn test_426() {
        let opts = default_compile_options();
        assert_is_match("a.b", "!(*.[^a-c])", opts, true);
    }

    #[test]
    fn test_427() {
        let opts = default_compile_options();
        assert_is_match("a.c", "!(*.[^a-c])", opts, true);
    }

    #[test]
    fn test_428() {
        let opts = default_compile_options();
        assert_is_match("a.c.d", "!(*.[^a-c])", opts, false);
    }

    #[test]
    fn test_429() {
        let opts = default_compile_options();
        assert_is_match("c.c", "!(*.[^a-c])", opts, true);
    }

    #[test]
    fn test_430() {
        let opts = default_compile_options();
        assert_is_match("a.", "!(*.[^a-c])", opts, true);
    }

    #[test]
    fn test_431() {
        let opts = default_compile_options();
        assert_is_match("d.d", "!(*.[^a-c])", opts, false);
    }

    #[test]
    fn test_432() {
        let opts = default_compile_options();
        assert_is_match("e.e", "!(*.[^a-c])", opts, false);
    }

    #[test]
    fn test_433() {
        let opts = default_compile_options();
        assert_is_match("f.f", "!(*.[^a-c])", opts, false);
    }

    #[test]
    fn test_434() {
        let opts = default_compile_options();
        assert_is_match("a.abcd", "!(*.[^a-c])", opts, true);
    }

    #[test]
    fn test_435() {
        let opts = default_compile_options();
        assert_is_match("a.a", "!(*.[a-c])", opts, false);
    }

    #[test]
    fn test_436() {
        let opts = default_compile_options();
        assert_is_match("a.b", "!(*.[a-c])", opts, false);
    }

    #[test]
    fn test_437() {
        let opts = default_compile_options();
        assert_is_match("a.c", "!(*.[a-c])", opts, false);
    }

    #[test]
    fn test_438() {
        let opts = default_compile_options();
        assert_is_match("a.c.d", "!(*.[a-c])", opts, true);
    }

    #[test]
    fn test_439() {
        let opts = default_compile_options();
        assert_is_match("c.c", "!(*.[a-c])", opts, false);
    }

    #[test]
    fn test_440() {
        let opts = default_compile_options();
        assert_is_match("a.", "!(*.[a-c])", opts, true);
    }

    #[test]
    fn test_441() {
        let opts = default_compile_options();
        assert_is_match("d.d", "!(*.[a-c])", opts, true);
    }

    #[test]
    fn test_442() {
        let opts = default_compile_options();
        assert_is_match("e.e", "!(*.[a-c])", opts, true);
    }

    #[test]
    fn test_443() {
        let opts = default_compile_options();
        assert_is_match("f.f", "!(*.[a-c])", opts, true);
    }

    #[test]
    fn test_444() {
        let opts = default_compile_options();
        assert_is_match("a.abcd", "!(*.[a-c])", opts, true);
    }

    #[test]
    fn test_445() {
        let opts = default_compile_options();
        assert_is_match("a.a", "!(*.[a-c]*)", opts, false);
    }

    #[test]
    fn test_446() {
        let opts = default_compile_options();
        assert_is_match("a.b", "!(*.[a-c]*)", opts, false);
    }

    #[test]
    fn test_447() {
        let opts = default_compile_options();
        assert_is_match("a.c", "!(*.[a-c]*)", opts, false);
    }

    #[test]
    fn test_448() {
        let opts = default_compile_options();
        assert_is_match("a.c.d", "!(*.[a-c]*)", opts, false);
    }

    #[test]
    fn test_449() {
        let opts = default_compile_options();
        assert_is_match("c.c", "!(*.[a-c]*)", opts, false);
    }

    #[test]
    fn test_450() {
        let opts = default_compile_options();
        assert_is_match("a.", "!(*.[a-c]*)", opts, true);
    }

    #[test]
    fn test_451() {
        let opts = default_compile_options();
        assert_is_match("d.d", "!(*.[a-c]*)", opts, true);
    }

    #[test]
    fn test_452() {
        let opts = default_compile_options();
        assert_is_match("e.e", "!(*.[a-c]*)", opts, true);
    }

    #[test]
    fn test_453() {
        let opts = default_compile_options();
        assert_is_match("f.f", "!(*.[a-c]*)", opts, true);
    }

    #[test]
    fn test_454() {
        let opts = default_compile_options();
        assert_is_match("a.abcd", "!(*.[a-c]*)", opts, false);
    }

    #[test]
    fn test_455() {
        let opts = default_compile_options();
        assert_is_match("a.a", "*.!(a|b|c)", opts, false);
    }

    #[test]
    fn test_456() {
        let opts = default_compile_options();
        assert_is_match("a.b", "*.!(a|b|c)", opts, false);
    }

    #[test]
    fn test_457() {
        let opts = default_compile_options();
        assert_is_match("a.c", "*.!(a|b|c)", opts, false);
    }

    #[test]
    fn test_458() {
        let opts = default_compile_options();
        assert_is_match("a.c.d", "*.!(a|b|c)", opts, true);
    }

    #[test]
    fn test_459() {
        let opts = default_compile_options();
        assert_is_match("c.c", "*.!(a|b|c)", opts, false);
    }

    #[test]
    fn test_460() {
        let opts = default_compile_options();
        assert_is_match("a.", "*.!(a|b|c)", opts, true);
    }

    #[test]
    fn test_461() {
        let opts = default_compile_options();
        assert_is_match("d.d", "*.!(a|b|c)", opts, true);
    }

    #[test]
    fn test_462() {
        let opts = default_compile_options();
        assert_is_match("e.e", "*.!(a|b|c)", opts, true);
    }

    #[test]
    fn test_463() {
        let opts = default_compile_options();
        assert_is_match("f.f", "*.!(a|b|c)", opts, true);
    }

    #[test]
    fn test_464() {
        let opts = default_compile_options();
        assert_is_match("a.abcd", "*.!(a|b|c)", opts, true);
    }

    #[test]
    fn test_465() {
        let opts = default_compile_options();
        assert_is_match("a.a", "*!(.a|.b|.c)", opts, true);
    }

    #[test]
    fn test_466() {
        let opts = default_compile_options();
        assert_is_match("a.b", "*!(.a|.b|.c)", opts, true);
    }

    #[test]
    fn test_467() {
        let opts = default_compile_options();
        assert_is_match("a.c", "*!(.a|.b|.c)", opts, true);
    }

    #[test]
    fn test_468() {
        let opts = default_compile_options();
        assert_is_match("a.c.d", "*!(.a|.b|.c)", opts, true);
    }

    #[test]
    fn test_469() {
        let opts = default_compile_options();
        assert_is_match("c.c", "*!(.a|.b|.c)", opts, true);
    }

    #[test]
    fn test_470() {
        let opts = default_compile_options();
        assert_is_match("a.", "*!(.a|.b|.c)", opts, true);
    }

    #[test]
    fn test_471() {
        let opts = default_compile_options();
        assert_is_match("d.d", "*!(.a|.b|.c)", opts, true);
    }

    #[test]
    fn test_472() {
        let opts = default_compile_options();
        assert_is_match("e.e", "*!(.a|.b|.c)", opts, true);
    }

    #[test]
    fn test_473() {
        let opts = default_compile_options();
        assert_is_match("f.f", "*!(.a|.b|.c)", opts, true);
    }

    #[test]
    fn test_474() {
        let opts = default_compile_options();
        assert_is_match("a.abcd", "*!(.a|.b|.c)", opts, true);
    }

    #[test]
    fn test_475() {
        let opts = default_compile_options();
        assert_is_match("a.a", "!(*.[a-c])*", opts, false);
    }

    #[test]
    fn test_476() {
        let opts = default_compile_options();
        assert_is_match("a.b", "!(*.[a-c])*", opts, false);
    }

    #[test]
    fn test_477() {
        let opts = default_compile_options();
        assert_is_match("a.c", "!(*.[a-c])*", opts, false);
    }

    #[test]
    fn test_478() {
        let opts = default_compile_options();
        assert_is_match("a.c.d", "!(*.[a-c])*", opts, false);
    }

    #[test]
    fn test_479() {
        let opts = default_compile_options();
        assert_is_match("c.c", "!(*.[a-c])*", opts, false);
    }

    #[test]
    fn test_480() {
        let opts = default_compile_options();
        assert_is_match("a.", "!(*.[a-c])*", opts, true);
    }

    #[test]
    fn test_481() {
        let opts = default_compile_options();
        assert_is_match("d.d", "!(*.[a-c])*", opts, true);
    }

    #[test]
    fn test_482() {
        let opts = default_compile_options();
        assert_is_match("e.e", "!(*.[a-c])*", opts, true);
    }

    #[test]
    fn test_483() {
        let opts = default_compile_options();
        assert_is_match("f.f", "!(*.[a-c])*", opts, true);
    }

    #[test]
    fn test_484() {
        let opts = default_compile_options();
        assert_is_match("a.abcd", "!(*.[a-c])*", opts, false);
    }

    #[test]
    fn test_485() {
        let opts = default_compile_options();
        assert_is_match("a.a", "*!(.a|.b|.c)*", opts, true);
    }

    #[test]
    fn test_486() {
        let opts = default_compile_options();
        assert_is_match("a.b", "*!(.a|.b|.c)*", opts, true);
    }

    #[test]
    fn test_487() {
        let opts = default_compile_options();
        assert_is_match("a.c", "*!(.a|.b|.c)*", opts, true);
    }

    #[test]
    fn test_488() {
        let opts = default_compile_options();
        assert_is_match("a.c.d", "*!(.a|.b|.c)*", opts, true);
    }

    #[test]
    fn test_489() {
        let opts = default_compile_options();
        assert_is_match("c.c", "*!(.a|.b|.c)*", opts, true);
    }

    #[test]
    fn test_490() {
        let opts = default_compile_options();
        assert_is_match("a.", "*!(.a|.b|.c)*", opts, true);
    }

    #[test]
    fn test_491() {
        let opts = default_compile_options();
        assert_is_match("d.d", "*!(.a|.b|.c)*", opts, true);
    }

    #[test]
    fn test_492() {
        let opts = default_compile_options();
        assert_is_match("e.e", "*!(.a|.b|.c)*", opts, true);
    }

    #[test]
    fn test_493() {
        let opts = default_compile_options();
        assert_is_match("f.f", "*!(.a|.b|.c)*", opts, true);
    }

    #[test]
    fn test_494() {
        let opts = default_compile_options();
        assert_is_match("a.abcd", "*!(.a|.b|.c)*", opts, true);
    }

    #[test]
    fn test_495() {
        let opts = default_compile_options();
        assert_is_match("a.a", "*.!(a|b|c)*", opts, false);
    }

    #[test]
    fn test_496() {
        let opts = default_compile_options();
        assert_is_match("a.b", "*.!(a|b|c)*", opts, false);
    }

    #[test]
    fn test_497() {
        let opts = default_compile_options();
        assert_is_match("a.c", "*.!(a|b|c)*", opts, false);
    }

    #[test]
    fn test_498() {
        let opts = default_compile_options();
        assert_is_match("a.c.d", "*.!(a|b|c)*", opts, true);
    }

    #[test]
    fn test_499() {
        let opts = default_compile_options();
        assert_is_match("c.c", "*.!(a|b|c)*", opts, false);
    }

    #[test]
    fn test_500() {
        let opts = default_compile_options();
        assert_is_match("a.", "*.!(a|b|c)*", opts, true);
    }

    #[test]
    fn test_501() {
        let opts = default_compile_options();
        assert_is_match("d.d", "*.!(a|b|c)*", opts, true);
    }

    #[test]
    fn test_502() {
        let opts = default_compile_options();
        assert_is_match("e.e", "*.!(a|b|c)*", opts, true);
    }

    #[test]
    fn test_503() {
        let opts = default_compile_options();
        assert_is_match("f.f", "*.!(a|b|c)*", opts, true);
    }

    #[test]
    fn test_504() {
        let opts = default_compile_options();
        assert_is_match("a.abcd", "*.!(a|b|c)*", opts, false);
    }

    #[test]
    fn test_505() {
        let opts = default_compile_options();
        assert_is_match("def", "@()ef", opts, false);
    }

    #[test]
    fn test_506() {
        let opts = default_compile_options();
        assert_is_match("ef", "@()ef", opts, true);
    }

    #[test]
    fn test_507() {
        let opts = default_compile_options();
        assert_is_match("def", "()ef", opts, false);
    }

    #[test]
    fn test_508() {
        let opts = default_compile_options();
        assert_is_match("ef", "()ef", opts, true);
    }

    // JS: assert(isMatch('a\\(b', 'a\\\\\\(b')) inside `if (process.platform !== 'win32')`
    // input='a\(b' (1 backslash), pattern='a\\\(b' (3 backslashes = matches literal \()
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_509() {
        let opts = default_compile_options();
        assert_is_match(r"a\(b", r"a\\\(b", opts, true);
    }

    #[test]
    fn test_510() {
        let opts = default_compile_options();
        assert_is_match("a(b", "a(b", opts, true);
    }

    #[test]
    fn test_511() {
        let opts = default_compile_options();
        assert_is_match("a(b", r"a\(b", opts, true);
    }

    #[test]
    fn test_512() {
        let opts = default_compile_options();
        assert_is_match("a((b", "a(b", opts, false);
    }

    #[test]
    fn test_513() {
        let opts = default_compile_options();
        assert_is_match("a((((b", "a(b", opts, false);
    }

    #[test]
    fn test_514() {
        let opts = default_compile_options();
        assert_is_match("ab", "a(b", opts, false);
    }

    #[test]
    fn test_515() {
        let opts = default_compile_options();
        assert_is_match("a(b", r"a\(b", opts, true);
    }

    #[test]
    fn test_516() {
        let opts = default_compile_options();
        assert_is_match("a((b", r"a\(b", opts, false);
    }

    #[test]
    fn test_517() {
        let opts = default_compile_options();
        assert_is_match("a((((b", r"a\(b", opts, false);
    }

    #[test]
    fn test_518() {
        let opts = default_compile_options();
        assert_is_match("ab", r"a\(b", opts, false);
    }

    #[test]
    fn test_519() {
        let opts = default_compile_options();
        assert_is_match("a(b", "a(*b", opts, true);
    }

    #[test]
    fn test_520() {
        let opts = default_compile_options();
        assert_is_match("a(ab", r"a\(*b", opts, true);
    }

    #[test]
    fn test_521() {
        let opts = default_compile_options();
        assert_is_match("a((b", "a(*b", opts, true);
    }

    #[test]
    fn test_522() {
        let opts = default_compile_options();
        assert_is_match("a((((b", "a(*b", opts, true);
    }

    #[test]
    fn test_523() {
        let opts = default_compile_options();
        assert_is_match("ab", "a(*b", opts, false);
    }

    #[test]
    fn test_524() {
        let opts = default_compile_options();
        assert_is_match("a(b", r"a\(b", opts, true);
    }

    #[test]
    fn test_525() {
        let opts = default_compile_options();
        assert_is_match("a((b", r"a\(\(b", opts, true);
    }

    #[test]
    fn test_526() {
        let opts = default_compile_options();
        assert_is_match("a((((b", r"a\(\(\(\(b", opts, true);
    }

    #[test]
    fn test_527() {
        let opts = default_compile_options();
        assert_is_match("a(b", r"a\\(b", opts, false);
    }

    #[test]
    fn test_528() {
        let opts = default_compile_options();
        assert_is_match("a((b", r"a\\(b", opts, false);
    }

    #[test]
    fn test_529() {
        let opts = default_compile_options();
        assert_is_match("a((((b", r"a\\(b", opts, false);
    }

    #[test]
    fn test_530() {
        let opts = default_compile_options();
        assert_is_match("ab", r"a\\(b", opts, false);
    }

    #[test]
    fn test_531() {
        let opts = default_compile_options();
        // JS: assert(!isMatch('a/b', 'a\\\\b')) - pattern is a\\b (2 backslashes)
        assert_is_match("a/b", r"a\\b", opts, false);
    }

    #[test]
    fn test_532() {
        let opts = default_compile_options();
        // JS: assert(!isMatch('ab', 'a\\\\b')) - pattern is a\\b (2 backslashes)
        assert_is_match("ab", r"a\\b", opts, false);
    }

    // ============================================================
    // Missing tests: not migrated in the previous pass
    // ============================================================

    // JS: it('should throw on imbalanced sets when `optionsBrackets` is true')
    // assert.throws(() => makeRe('a(b', opts), /Missing closing: "\)"/i)
    // assert.throws(() => makeRe('a)b', opts), /Missing opening: "\("/i)
    // NOTE: makeRe / error-on-compile is not directly testable via is_match;
    //       these two assertions have no Rust equivalent and are intentionally omitted.

    // JS: assert.strictEqual(makeRe('c!(?:foo)?z').source, '^(?:c!(?:foo)?z)$')
    // NOTE: regex source inspection is not available in the Rust API; omitted.

    // JS: it('should support regex characters') — expanded from assert.deepStrictEqual(match(...))
    // The JS comment says "these are not extglobs, and do not need to pass,
    // but they are included to test integration with other features".
    // All 159 resulting assertions pass in picomatch_rs.

    // JS: if (process.platform !== 'win32') { match(['a\\b', 'a/b', 'ab'], 'a/b') → ['a/b'] }
    // a\\b in JS source = actual string "a\b" (one backslash); excluded on Windows.
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_533() {
        let opts = default_compile_options();
        assert_is_match("a\\b", "a/b", opts, false);
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_534() {
        let opts = default_compile_options();
        assert_is_match("a/b", "a/b", opts, true);
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_535() {
        let opts = default_compile_options();
        assert_is_match("ab", "a/b", opts, false);
    }

    #[test]
    fn test_536() {
        let opts = default_compile_options();
        assert_is_match("a/b", "a/b", opts, true);
    }

    #[test]
    fn test_537() {
        let opts = default_compile_options();
        assert_is_match("ab", "a/b", opts, false);
    }

    #[test]
    fn test_538() {
        let opts = default_compile_options();
        assert_is_match("a c", "ab?bc", opts, false);
    }

    #[test]
    fn test_539() {
        let opts = default_compile_options();
        assert_is_match("a.c", "ab?bc", opts, false);
    }

    #[test]
    fn test_540() {
        let opts = default_compile_options();
        assert_is_match("a.xy.zc", "ab?bc", opts, false);
    }

    #[test]
    fn test_541() {
        let opts = default_compile_options();
        assert_is_match("a.zc", "ab?bc", opts, false);
    }

    #[test]
    fn test_542() {
        let opts = default_compile_options();
        assert_is_match("a123c", "ab?bc", opts, false);
    }

    #[test]
    fn test_543() {
        let opts = default_compile_options();
        assert_is_match("a1c", "ab?bc", opts, false);
    }

    #[test]
    fn test_544() {
        let opts = default_compile_options();
        assert_is_match("abbbbc", "ab?bc", opts, false);
    }

    #[test]
    fn test_545() {
        let opts = default_compile_options();
        assert_is_match("abbbc", "ab?bc", opts, true);
    }

    #[test]
    fn test_546() {
        let opts = default_compile_options();
        assert_is_match("abbc", "ab?bc", opts, false);
    }

    #[test]
    fn test_547() {
        let opts = default_compile_options();
        assert_is_match("abc", "ab?bc", opts, false);
    }

    #[test]
    fn test_548() {
        let opts = default_compile_options();
        assert_is_match("abq", "ab?bc", opts, false);
    }

    #[test]
    fn test_549() {
        let opts = default_compile_options();
        assert_is_match("axy zc", "ab?bc", opts, false);
    }

    #[test]
    fn test_550() {
        let opts = default_compile_options();
        assert_is_match("axy", "ab?bc", opts, false);
    }

    #[test]
    fn test_551() {
        let opts = default_compile_options();
        assert_is_match("axy.zc", "ab?bc", opts, false);
    }

    #[test]
    fn test_552() {
        let opts = default_compile_options();
        assert_is_match("axyzc", "ab?bc", opts, false);
    }

    #[test]
    fn test_553() {
        let opts = default_compile_options();
        assert_is_match("a c", "ab*c", opts, false);
    }

    #[test]
    fn test_554() {
        let opts = default_compile_options();
        assert_is_match("a.c", "ab*c", opts, false);
    }

    #[test]
    fn test_555() {
        let opts = default_compile_options();
        assert_is_match("a.xy.zc", "ab*c", opts, false);
    }

    #[test]
    fn test_556() {
        let opts = default_compile_options();
        assert_is_match("a.zc", "ab*c", opts, false);
    }

    #[test]
    fn test_557() {
        let opts = default_compile_options();
        assert_is_match("a123c", "ab*c", opts, false);
    }

    #[test]
    fn test_558() {
        let opts = default_compile_options();
        assert_is_match("a1c", "ab*c", opts, false);
    }

    #[test]
    fn test_559() {
        let opts = default_compile_options();
        assert_is_match("abbbbc", "ab*c", opts, true);
    }

    #[test]
    fn test_560() {
        let opts = default_compile_options();
        assert_is_match("abbbc", "ab*c", opts, true);
    }

    #[test]
    fn test_561() {
        let opts = default_compile_options();
        assert_is_match("abbc", "ab*c", opts, true);
    }

    #[test]
    fn test_562() {
        let opts = default_compile_options();
        assert_is_match("abc", "ab*c", opts, true);
    }

    #[test]
    fn test_563() {
        let opts = default_compile_options();
        assert_is_match("abq", "ab*c", opts, false);
    }

    #[test]
    fn test_564() {
        let opts = default_compile_options();
        assert_is_match("axy zc", "ab*c", opts, false);
    }

    #[test]
    fn test_565() {
        let opts = default_compile_options();
        assert_is_match("axy", "ab*c", opts, false);
    }

    #[test]
    fn test_566() {
        let opts = default_compile_options();
        assert_is_match("axy.zc", "ab*c", opts, false);
    }

    #[test]
    fn test_567() {
        let opts = default_compile_options();
        assert_is_match("axyzc", "ab*c", opts, false);
    }

    #[test]
    fn test_568() {
        let opts = default_compile_options();
        assert_is_match("a c", "a+(b)bc", opts, false);
    }

    #[test]
    fn test_569() {
        let opts = default_compile_options();
        assert_is_match("a.c", "a+(b)bc", opts, false);
    }

    #[test]
    fn test_570() {
        let opts = default_compile_options();
        assert_is_match("a.xy.zc", "a+(b)bc", opts, false);
    }

    #[test]
    fn test_571() {
        let opts = default_compile_options();
        assert_is_match("a.zc", "a+(b)bc", opts, false);
    }

    #[test]
    fn test_572() {
        let opts = default_compile_options();
        assert_is_match("a123c", "a+(b)bc", opts, false);
    }

    #[test]
    fn test_573() {
        let opts = default_compile_options();
        assert_is_match("a1c", "a+(b)bc", opts, false);
    }

    #[test]
    fn test_574() {
        let opts = default_compile_options();
        assert_is_match("abbbbc", "a+(b)bc", opts, true);
    }

    #[test]
    fn test_575() {
        let opts = default_compile_options();
        assert_is_match("abbbc", "a+(b)bc", opts, true);
    }

    #[test]
    fn test_576() {
        let opts = default_compile_options();
        assert_is_match("abbc", "a+(b)bc", opts, true);
    }

    #[test]
    fn test_577() {
        let opts = default_compile_options();
        assert_is_match("abc", "a+(b)bc", opts, false);
    }

    #[test]
    fn test_578() {
        let opts = default_compile_options();
        assert_is_match("abq", "a+(b)bc", opts, false);
    }

    #[test]
    fn test_579() {
        let opts = default_compile_options();
        assert_is_match("axy zc", "a+(b)bc", opts, false);
    }

    #[test]
    fn test_580() {
        let opts = default_compile_options();
        assert_is_match("axy", "a+(b)bc", opts, false);
    }

    #[test]
    fn test_581() {
        let opts = default_compile_options();
        assert_is_match("axy.zc", "a+(b)bc", opts, false);
    }

    #[test]
    fn test_582() {
        let opts = default_compile_options();
        assert_is_match("axyzc", "a+(b)bc", opts, false);
    }

    #[test]
    fn test_583() {
        let opts = default_compile_options();
        assert_is_match("a c", "^abc$", opts, false);
    }

    #[test]
    fn test_584() {
        let opts = default_compile_options();
        assert_is_match("a.c", "^abc$", opts, false);
    }

    #[test]
    fn test_585() {
        let opts = default_compile_options();
        assert_is_match("a.xy.zc", "^abc$", opts, false);
    }

    #[test]
    fn test_586() {
        let opts = default_compile_options();
        assert_is_match("a.zc", "^abc$", opts, false);
    }

    #[test]
    fn test_587() {
        let opts = default_compile_options();
        assert_is_match("a123c", "^abc$", opts, false);
    }

    #[test]
    fn test_588() {
        let opts = default_compile_options();
        assert_is_match("a1c", "^abc$", opts, false);
    }

    #[test]
    fn test_589() {
        let opts = default_compile_options();
        assert_is_match("abbbbc", "^abc$", opts, false);
    }

    #[test]
    fn test_590() {
        let opts = default_compile_options();
        assert_is_match("abbbc", "^abc$", opts, false);
    }

    #[test]
    fn test_591() {
        let opts = default_compile_options();
        assert_is_match("abbc", "^abc$", opts, false);
    }

    #[test]
    fn test_592() {
        let opts = default_compile_options();
        assert_is_match("abc", "^abc$", opts, false);
    }

    #[test]
    fn test_593() {
        let opts = default_compile_options();
        assert_is_match("abq", "^abc$", opts, false);
    }

    #[test]
    fn test_594() {
        let opts = default_compile_options();
        assert_is_match("axy zc", "^abc$", opts, false);
    }

    #[test]
    fn test_595() {
        let opts = default_compile_options();
        assert_is_match("axy", "^abc$", opts, false);
    }

    #[test]
    fn test_596() {
        let opts = default_compile_options();
        assert_is_match("axy.zc", "^abc$", opts, false);
    }

    #[test]
    fn test_597() {
        let opts = default_compile_options();
        assert_is_match("axyzc", "^abc$", opts, false);
    }

    #[test]
    fn test_598() {
        let opts = default_compile_options();
        assert_is_match("a c", "a.c", opts, false);
    }

    #[test]
    fn test_599() {
        let opts = default_compile_options();
        assert_is_match("a.c", "a.c", opts, true);
    }

    #[test]
    fn test_600() {
        let opts = default_compile_options();
        assert_is_match("a.xy.zc", "a.c", opts, false);
    }

    #[test]
    fn test_601() {
        let opts = default_compile_options();
        assert_is_match("a.zc", "a.c", opts, false);
    }

    #[test]
    fn test_602() {
        let opts = default_compile_options();
        assert_is_match("a123c", "a.c", opts, false);
    }

    #[test]
    fn test_603() {
        let opts = default_compile_options();
        assert_is_match("a1c", "a.c", opts, false);
    }

    #[test]
    fn test_604() {
        let opts = default_compile_options();
        assert_is_match("abbbbc", "a.c", opts, false);
    }

    #[test]
    fn test_605() {
        let opts = default_compile_options();
        assert_is_match("abbbc", "a.c", opts, false);
    }

    #[test]
    fn test_606() {
        let opts = default_compile_options();
        assert_is_match("abbc", "a.c", opts, false);
    }

    #[test]
    fn test_607() {
        let opts = default_compile_options();
        assert_is_match("abc", "a.c", opts, false);
    }

    #[test]
    fn test_608() {
        let opts = default_compile_options();
        assert_is_match("abq", "a.c", opts, false);
    }

    #[test]
    fn test_609() {
        let opts = default_compile_options();
        assert_is_match("axy zc", "a.c", opts, false);
    }

    #[test]
    fn test_610() {
        let opts = default_compile_options();
        assert_is_match("axy", "a.c", opts, false);
    }

    #[test]
    fn test_611() {
        let opts = default_compile_options();
        assert_is_match("axy.zc", "a.c", opts, false);
    }

    #[test]
    fn test_612() {
        let opts = default_compile_options();
        assert_is_match("axyzc", "a.c", opts, false);
    }

    #[test]
    fn test_613() {
        let opts = default_compile_options();
        assert_is_match("a c", "a.*c", opts, false);
    }

    #[test]
    fn test_614() {
        let opts = default_compile_options();
        assert_is_match("a.c", "a.*c", opts, true);
    }

    #[test]
    fn test_615() {
        let opts = default_compile_options();
        assert_is_match("a.xy.zc", "a.*c", opts, true);
    }

    #[test]
    fn test_616() {
        let opts = default_compile_options();
        assert_is_match("a.zc", "a.*c", opts, true);
    }

    #[test]
    fn test_617() {
        let opts = default_compile_options();
        assert_is_match("a123c", "a.*c", opts, false);
    }

    #[test]
    fn test_618() {
        let opts = default_compile_options();
        assert_is_match("a1c", "a.*c", opts, false);
    }

    #[test]
    fn test_619() {
        let opts = default_compile_options();
        assert_is_match("abbbbc", "a.*c", opts, false);
    }

    #[test]
    fn test_620() {
        let opts = default_compile_options();
        assert_is_match("abbbc", "a.*c", opts, false);
    }

    #[test]
    fn test_621() {
        let opts = default_compile_options();
        assert_is_match("abbc", "a.*c", opts, false);
    }

    #[test]
    fn test_622() {
        let opts = default_compile_options();
        assert_is_match("abc", "a.*c", opts, false);
    }

    #[test]
    fn test_623() {
        let opts = default_compile_options();
        assert_is_match("abq", "a.*c", opts, false);
    }

    #[test]
    fn test_624() {
        let opts = default_compile_options();
        assert_is_match("axy zc", "a.*c", opts, false);
    }

    #[test]
    fn test_625() {
        let opts = default_compile_options();
        assert_is_match("axy", "a.*c", opts, false);
    }

    #[test]
    fn test_626() {
        let opts = default_compile_options();
        assert_is_match("axy.zc", "a.*c", opts, false);
    }

    #[test]
    fn test_627() {
        let opts = default_compile_options();
        assert_is_match("axyzc", "a.*c", opts, false);
    }

    #[test]
    fn test_628() {
        let opts = default_compile_options();
        assert_is_match("a c", "a*c", opts, true);
    }

    #[test]
    fn test_629() {
        let opts = default_compile_options();
        assert_is_match("a.c", "a*c", opts, true);
    }

    #[test]
    fn test_630() {
        let opts = default_compile_options();
        assert_is_match("a.xy.zc", "a*c", opts, true);
    }

    #[test]
    fn test_631() {
        let opts = default_compile_options();
        assert_is_match("a.zc", "a*c", opts, true);
    }

    #[test]
    fn test_632() {
        let opts = default_compile_options();
        assert_is_match("a123c", "a*c", opts, true);
    }

    #[test]
    fn test_633() {
        let opts = default_compile_options();
        assert_is_match("a1c", "a*c", opts, true);
    }

    #[test]
    fn test_634() {
        let opts = default_compile_options();
        assert_is_match("abbbbc", "a*c", opts, true);
    }

    #[test]
    fn test_635() {
        let opts = default_compile_options();
        assert_is_match("abbbc", "a*c", opts, true);
    }

    #[test]
    fn test_636() {
        let opts = default_compile_options();
        assert_is_match("abbc", "a*c", opts, true);
    }

    #[test]
    fn test_637() {
        let opts = default_compile_options();
        assert_is_match("abc", "a*c", opts, true);
    }

    #[test]
    fn test_638() {
        let opts = default_compile_options();
        assert_is_match("abq", "a*c", opts, false);
    }

    #[test]
    fn test_639() {
        let opts = default_compile_options();
        assert_is_match("axy zc", "a*c", opts, true);
    }

    #[test]
    fn test_640() {
        let opts = default_compile_options();
        assert_is_match("axy", "a*c", opts, false);
    }

    #[test]
    fn test_641() {
        let opts = default_compile_options();
        assert_is_match("axy.zc", "a*c", opts, true);
    }

    #[test]
    fn test_642() {
        let opts = default_compile_options();
        assert_is_match("axyzc", "a*c", opts, true);
    }

    #[test]
    fn test_643() {
        let opts = default_compile_options();
        assert_is_match("a c", "a[\\w]+c", opts, false);
    }

    #[test]
    fn test_644() {
        let opts = default_compile_options();
        assert_is_match("a.c", "a[\\w]+c", opts, false);
    }

    #[test]
    fn test_645() {
        let opts = default_compile_options();
        assert_is_match("a.xy.zc", "a[\\w]+c", opts, false);
    }

    #[test]
    fn test_646() {
        let opts = default_compile_options();
        assert_is_match("a.zc", "a[\\w]+c", opts, false);
    }

    #[test]
    fn test_647() {
        let opts = default_compile_options();
        assert_is_match("a123c", "a[\\w]+c", opts, true);
    }

    #[test]
    fn test_648() {
        let opts = default_compile_options();
        assert_is_match("a1c", "a[\\w]+c", opts, true);
    }

    #[test]
    fn test_649() {
        let opts = default_compile_options();
        assert_is_match("abbbbc", "a[\\w]+c", opts, true);
    }

    #[test]
    fn test_650() {
        let opts = default_compile_options();
        assert_is_match("abbbc", "a[\\w]+c", opts, true);
    }

    #[test]
    fn test_651() {
        let opts = default_compile_options();
        assert_is_match("abbc", "a[\\w]+c", opts, true);
    }

    #[test]
    fn test_652() {
        let opts = default_compile_options();
        assert_is_match("abc", "a[\\w]+c", opts, true);
    }

    #[test]
    fn test_653() {
        let opts = default_compile_options();
        assert_is_match("abq", "a[\\w]+c", opts, false);
    }

    #[test]
    fn test_654() {
        let opts = default_compile_options();
        assert_is_match("axy zc", "a[\\w]+c", opts, false);
    }

    #[test]
    fn test_655() {
        let opts = default_compile_options();
        assert_is_match("axy", "a[\\w]+c", opts, false);
    }

    #[test]
    fn test_656() {
        let opts = default_compile_options();
        assert_is_match("axy.zc", "a[\\w]+c", opts, false);
    }

    #[test]
    fn test_657() {
        let opts = default_compile_options();
        assert_is_match("axyzc", "a[\\w]+c", opts, true);
    }

    #[test]
    fn test_658() {
        let opts = default_compile_options();
        assert_is_match("a c", "a[\\W]+c", opts, true);
    }

    #[test]
    fn test_659() {
        let opts = default_compile_options();
        assert_is_match("a.c", "a[\\W]+c", opts, true);
    }

    #[test]
    fn test_660() {
        let opts = default_compile_options();
        assert_is_match("a.xy.zc", "a[\\W]+c", opts, false);
    }

    #[test]
    fn test_661() {
        let opts = default_compile_options();
        assert_is_match("a.zc", "a[\\W]+c", opts, false);
    }

    #[test]
    fn test_662() {
        let opts = default_compile_options();
        assert_is_match("a123c", "a[\\W]+c", opts, false);
    }

    #[test]
    fn test_663() {
        let opts = default_compile_options();
        assert_is_match("a1c", "a[\\W]+c", opts, false);
    }

    #[test]
    fn test_664() {
        let opts = default_compile_options();
        assert_is_match("abbbbc", "a[\\W]+c", opts, false);
    }

    #[test]
    fn test_665() {
        let opts = default_compile_options();
        assert_is_match("abbbc", "a[\\W]+c", opts, false);
    }

    #[test]
    fn test_666() {
        let opts = default_compile_options();
        assert_is_match("abbc", "a[\\W]+c", opts, false);
    }

    #[test]
    fn test_667() {
        let opts = default_compile_options();
        assert_is_match("abc", "a[\\W]+c", opts, false);
    }

    #[test]
    fn test_668() {
        let opts = default_compile_options();
        assert_is_match("abq", "a[\\W]+c", opts, false);
    }

    #[test]
    fn test_669() {
        let opts = default_compile_options();
        assert_is_match("axy zc", "a[\\W]+c", opts, false);
    }

    #[test]
    fn test_670() {
        let opts = default_compile_options();
        assert_is_match("axy", "a[\\W]+c", opts, false);
    }

    #[test]
    fn test_671() {
        let opts = default_compile_options();
        assert_is_match("axy.zc", "a[\\W]+c", opts, false);
    }

    #[test]
    fn test_672() {
        let opts = default_compile_options();
        assert_is_match("axyzc", "a[\\W]+c", opts, false);
    }

    #[test]
    fn test_673() {
        let opts = default_compile_options();
        assert_is_match("a c", "a[\\d]+c", opts, false);
    }

    #[test]
    fn test_674() {
        let opts = default_compile_options();
        assert_is_match("a.c", "a[\\d]+c", opts, false);
    }

    #[test]
    fn test_675() {
        let opts = default_compile_options();
        assert_is_match("a.xy.zc", "a[\\d]+c", opts, false);
    }

    #[test]
    fn test_676() {
        let opts = default_compile_options();
        assert_is_match("a.zc", "a[\\d]+c", opts, false);
    }

    #[test]
    fn test_677() {
        let opts = default_compile_options();
        assert_is_match("a123c", "a[\\d]+c", opts, true);
    }

    #[test]
    fn test_678() {
        let opts = default_compile_options();
        assert_is_match("a1c", "a[\\d]+c", opts, true);
    }

    #[test]
    fn test_679() {
        let opts = default_compile_options();
        assert_is_match("abbbbc", "a[\\d]+c", opts, false);
    }

    #[test]
    fn test_680() {
        let opts = default_compile_options();
        assert_is_match("abbbc", "a[\\d]+c", opts, false);
    }

    #[test]
    fn test_681() {
        let opts = default_compile_options();
        assert_is_match("abbc", "a[\\d]+c", opts, false);
    }

    #[test]
    fn test_682() {
        let opts = default_compile_options();
        assert_is_match("abc", "a[\\d]+c", opts, false);
    }

    #[test]
    fn test_683() {
        let opts = default_compile_options();
        assert_is_match("abq", "a[\\d]+c", opts, false);
    }

    #[test]
    fn test_684() {
        let opts = default_compile_options();
        assert_is_match("axy zc", "a[\\d]+c", opts, false);
    }

    #[test]
    fn test_685() {
        let opts = default_compile_options();
        assert_is_match("axy", "a[\\d]+c", opts, false);
    }

    #[test]
    fn test_686() {
        let opts = default_compile_options();
        assert_is_match("axy.zc", "a[\\d]+c", opts, false);
    }

    #[test]
    fn test_687() {
        let opts = default_compile_options();
        assert_is_match("axyzc", "a[\\d]+c", opts, false);
    }

    #[test]
    fn test_688() {
        let opts = default_compile_options();
        assert_is_match("foo@#$%123ASD #$$%^&", "[\\d]+", opts, false);
    }

    #[test]
    fn test_689() {
        let opts = default_compile_options();
        assert_is_match("foo!@#$asdfl;", "[\\d]+", opts, false);
    }

    #[test]
    fn test_690() {
        let opts = default_compile_options();
        assert_is_match("123", "[\\d]+", opts, true);
    }

    #[test]
    fn test_691() {
        let opts = default_compile_options();
        assert_is_match("a123c", "a[\\D]+c", opts, false);
    }

    #[test]
    fn test_692() {
        let opts = default_compile_options();
        assert_is_match("abbbc", "a[\\D]+c", opts, true);
    }

    #[test]
    fn test_693() {
        let opts = default_compile_options();
        assert_is_match("foo", "(f|o)+\\b", opts, true);
    }

    #[test]
    fn test_694() {
        let opts = default_compile_options();
        assert_is_match(" foo ", "(f|o)+\\b", opts, false);
    }
}
