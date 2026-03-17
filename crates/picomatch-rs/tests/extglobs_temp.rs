mod support;
use picomatch_rs::CompileOptions;
use support::{assert_is_match, default_compile_options};

fn windows_options() -> CompileOptions {
    CompileOptions {
        windows: true,
        ..CompileOptions::default()
    }
}

fn posix_options() -> CompileOptions {
    CompileOptions {
        posix: true,
        ..CompileOptions::default()
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let opts = windows_options();
        assert_is_match("bar", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_2() {
        let opts = windows_options();
        assert_is_match("f", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_3() {
        let opts = windows_options();
        assert_is_match("fa", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_4() {
        let opts = windows_options();
        assert_is_match("fb", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_5() {
        let opts = windows_options();
        assert_is_match("ff", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_6() {
        let opts = windows_options();
        assert_is_match("fff", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_7() {
        let opts = windows_options();
        assert_is_match("fo", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_8() {
        let opts = windows_options();
        assert_is_match("foo", "!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_9() {
        let opts = windows_options();
        assert_is_match("foo/bar", "!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_10() {
        let opts = windows_options();
        assert_is_match("foo/bar", "!(foo)/*", opts.clone(), false);
    }

    #[test]
    fn test_11() {
        let opts = windows_options();
        assert_is_match("foobar", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_12() {
        let opts = windows_options();
        assert_is_match("foot", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_13() {
        let opts = windows_options();
        assert_is_match("foox", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_14() {
        let opts = windows_options();
        assert_is_match("o", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_15() {
        let opts = windows_options();
        assert_is_match("of", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_16() {
        let opts = windows_options();
        assert_is_match("ooo", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_17() {
        let opts = windows_options();
        assert_is_match("ox", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_18() {
        let opts = windows_options();
        assert_is_match("x", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_19() {
        let opts = windows_options();
        assert_is_match("xx", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_20() {
        let opts = windows_options();
        assert_is_match("bar", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_21() {
        let opts = windows_options();
        assert_is_match("f", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_22() {
        let opts = windows_options();
        assert_is_match("fa", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_23() {
        let opts = windows_options();
        assert_is_match("fb", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_24() {
        let opts = windows_options();
        assert_is_match("ff", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_25() {
        let opts = windows_options();
        assert_is_match("fff", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_26() {
        let opts = windows_options();
        assert_is_match("fo", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_27() {
        let opts = windows_options();
        assert_is_match("foo", "!(!(foo))", opts.clone(), true);
    }

    #[test]
    fn test_28() {
        let opts = windows_options();
        assert_is_match("foo/bar", "!(!(bar)/baz)", opts.clone(), true);
    }

    #[test]
    fn test_29() {
        let opts = windows_options();
        assert_is_match("foo/bar", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_30() {
        let opts = windows_options();
        assert_is_match("foobar", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_31() {
        let opts = windows_options();
        assert_is_match("foot", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_32() {
        let opts = windows_options();
        assert_is_match("foox", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_33() {
        let opts = windows_options();
        assert_is_match("o", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_34() {
        let opts = windows_options();
        assert_is_match("of", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_35() {
        let opts = windows_options();
        assert_is_match("ooo", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_36() {
        let opts = windows_options();
        assert_is_match("ox", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_37() {
        let opts = windows_options();
        assert_is_match("x", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_38() {
        let opts = windows_options();
        assert_is_match("xx", "!(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_39() {
        let opts = windows_options();
        assert_is_match("bar", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_40() {
        let opts = windows_options();
        assert_is_match("f", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_41() {
        let opts = windows_options();
        assert_is_match("fa", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_42() {
        let opts = windows_options();
        assert_is_match("fb", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_43() {
        let opts = windows_options();
        assert_is_match("ff", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_44() {
        let opts = windows_options();
        assert_is_match("fff", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_45() {
        let opts = windows_options();
        assert_is_match("fo", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_46() {
        let opts = windows_options();
        assert_is_match("foo", "!(!(!(foo)))", opts.clone(), false);
    }

    #[test]
    fn test_47() {
        let opts = windows_options();
        assert_is_match("foo/bar", "!(!(!(foo)))", opts.clone(), false);
    }

    #[test]
    fn test_48() {
        let opts = windows_options();
        assert_is_match("foobar", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_49() {
        let opts = windows_options();
        assert_is_match("foot", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_50() {
        let opts = windows_options();
        assert_is_match("foox", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_51() {
        let opts = windows_options();
        assert_is_match("o", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_52() {
        let opts = windows_options();
        assert_is_match("of", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_53() {
        let opts = windows_options();
        assert_is_match("ooo", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_54() {
        let opts = windows_options();
        assert_is_match("ox", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_55() {
        let opts = windows_options();
        assert_is_match("x", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_56() {
        let opts = windows_options();
        assert_is_match("xx", "!(!(!(foo)))", opts.clone(), true);
    }

    #[test]
    fn test_57() {
        let opts = windows_options();
        assert_is_match("bar", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_58() {
        let opts = windows_options();
        assert_is_match("f", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_59() {
        let opts = windows_options();
        assert_is_match("fa", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_60() {
        let opts = windows_options();
        assert_is_match("fb", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_61() {
        let opts = windows_options();
        assert_is_match("ff", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_62() {
        let opts = windows_options();
        assert_is_match("fff", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_63() {
        let opts = windows_options();
        assert_is_match("fo", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_64() {
        let opts = windows_options();
        assert_is_match("foo", "!(!(!(!(foo))))", opts.clone(), true);
    }

    #[test]
    fn test_65() {
        let opts = windows_options();
        assert_is_match("foo/bar", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_66() {
        let opts = windows_options();
        assert_is_match("foot", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_67() {
        let opts = windows_options();
        assert_is_match("o", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_68() {
        let opts = windows_options();
        assert_is_match("of", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_69() {
        let opts = windows_options();
        assert_is_match("ooo", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_70() {
        let opts = windows_options();
        assert_is_match("ox", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_71() {
        let opts = windows_options();
        assert_is_match("x", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_72() {
        let opts = windows_options();
        assert_is_match("xx", "!(!(!(!(foo))))", opts.clone(), false);
    }

    #[test]
    fn test_73() {
        let opts = windows_options();
        assert_is_match("bar", "!(!(foo))*", opts.clone(), false);
    }

    #[test]
    fn test_74() {
        let opts = windows_options();
        assert_is_match("f", "!(!(foo))*", opts.clone(), false);
    }

    #[test]
    fn test_75() {
        let opts = windows_options();
        assert_is_match("fa", "!(!(foo))*", opts.clone(), false);
    }

    #[test]
    fn test_76() {
        let opts = windows_options();
        assert_is_match("fb", "!(!(foo))*", opts.clone(), false);
    }

    #[test]
    fn test_77() {
        let opts = windows_options();
        assert_is_match("ff", "!(!(foo))*", opts.clone(), false);
    }

    #[test]
    fn test_78() {
        let opts = windows_options();
        assert_is_match("fff", "!(!(foo))*", opts.clone(), false);
    }

    #[test]
    fn test_79() {
        let opts = windows_options();
        assert_is_match("fo", "!(!(foo))*", opts.clone(), false);
    }

    #[test]
    fn test_80() {
        let opts = windows_options();
        assert_is_match("foo", "!(!(foo))*", opts.clone(), true);
    }

    #[test]
    fn test_81() {
        let opts = windows_options();
        assert_is_match("foobar", "!(!(foo))*", opts.clone(), true);
    }

    #[test]
    fn test_82() {
        let opts = windows_options();
        assert_is_match("foot", "!(!(foo))*", opts.clone(), true);
    }

    #[test]
    fn test_83() {
        let opts = windows_options();
        assert_is_match("foox", "!(!(foo))*", opts.clone(), true);
    }

    #[test]
    fn test_84() {
        let opts = windows_options();
        assert_is_match("o", "!(!(foo))*", opts.clone(), false);
    }

    #[test]
    fn test_85() {
        let opts = windows_options();
        assert_is_match("of", "!(!(foo))*", opts.clone(), false);
    }

    #[test]
    fn test_86() {
        let opts = windows_options();
        assert_is_match("ooo", "!(!(foo))*", opts.clone(), false);
    }

    #[test]
    fn test_87() {
        let opts = windows_options();
        assert_is_match("ox", "!(!(foo))*", opts.clone(), false);
    }

    #[test]
    fn test_88() {
        let opts = windows_options();
        assert_is_match("x", "!(!(foo))*", opts.clone(), false);
    }

    #[test]
    fn test_89() {
        let opts = windows_options();
        assert_is_match("xx", "!(!(foo))*", opts.clone(), false);
    }

    #[test]
    fn test_90() {
        let opts = windows_options();
        assert_is_match("bar", "!(f!(o))", opts.clone(), true);
    }

    #[test]
    fn test_91() {
        let opts = windows_options();
        assert_is_match("f", "!(f!(o))", opts.clone(), false);
    }

    #[test]
    fn test_92() {
        let opts = windows_options();
        assert_is_match("fa", "!(f!(o))", opts.clone(), false);
    }

    #[test]
    fn test_93() {
        let opts = windows_options();
        assert_is_match("fb", "!(f!(o))", opts.clone(), false);
    }

    #[test]
    fn test_94() {
        let opts = windows_options();
        assert_is_match("ff", "!(f!(o))", opts.clone(), false);
    }

    #[test]
    fn test_95() {
        let opts = windows_options();
        assert_is_match("fff", "!(f!(o))", opts.clone(), false);
    }

    #[test]
    fn test_96() {
        let opts = windows_options();
        assert_is_match("fo", "!(f!(o))", opts.clone(), true);
    }

    #[test]
    fn test_97() {
        let opts = windows_options();
        assert_is_match("foo", "!(!(foo))", opts.clone(), true);
    }

    #[test]
    fn test_98() {
        let opts = windows_options();
        assert_is_match("foo", "!(f)!(o)!(o)", opts.clone(), false);
    }

    #[test]
    fn test_99() {
        let opts = windows_options();
        assert_is_match("foo", "!(fo)", opts.clone(), true);
    }

    #[test]
    fn test_100() {
        let opts = windows_options();
        assert_is_match("foo", "!(f!(o)*)", opts.clone(), true);
    }

    #[test]
    fn test_101() {
        let opts = windows_options();
        assert_is_match("foo", "!(f!(o))", opts.clone(), false);
    }

    #[test]
    fn test_102() {
        let opts = windows_options();
        assert_is_match("foo/bar", "!(f!(o))", opts.clone(), false);
    }

    #[test]
    fn test_103() {
        let opts = windows_options();
        assert_is_match("foobar", "!(f!(o))", opts.clone(), false);
    }

    #[test]
    fn test_104() {
        let opts = windows_options();
        assert_is_match("o", "!(f!(o))", opts.clone(), true);
    }

    #[test]
    fn test_105() {
        let opts = windows_options();
        assert_is_match("of", "!(f!(o))", opts.clone(), true);
    }

    #[test]
    fn test_106() {
        let opts = windows_options();
        assert_is_match("ooo", "!(f!(o))", opts.clone(), true);
    }

    #[test]
    fn test_107() {
        let opts = windows_options();
        assert_is_match("ox", "!(f!(o))", opts.clone(), true);
    }

    #[test]
    fn test_108() {
        let opts = windows_options();
        assert_is_match("x", "!(f!(o))", opts.clone(), true);
    }

    #[test]
    fn test_109() {
        let opts = windows_options();
        assert_is_match("xx", "!(f!(o))", opts.clone(), true);
    }

    #[test]
    fn test_110() {
        let opts = windows_options();
        assert_is_match("bar", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_111() {
        let opts = windows_options();
        assert_is_match("f", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_112() {
        let opts = windows_options();
        assert_is_match("fa", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_113() {
        let opts = windows_options();
        assert_is_match("fb", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_114() {
        let opts = windows_options();
        assert_is_match("ff", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_115() {
        let opts = windows_options();
        assert_is_match("fff", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_116() {
        let opts = windows_options();
        assert_is_match("fo", "!(f(o))", opts.clone(), false);
    }

    #[test]
    fn test_117() {
        let opts = windows_options();
        assert_is_match("foo", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_118() {
        let opts = windows_options();
        assert_is_match("foo/bar", "!(f(o))", opts.clone(), false);
    }

    #[test]
    fn test_119() {
        let opts = windows_options();
        assert_is_match("foobar", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_120() {
        let opts = windows_options();
        assert_is_match("foot", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_121() {
        let opts = windows_options();
        assert_is_match("foox", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_122() {
        let opts = windows_options();
        assert_is_match("o", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_123() {
        let opts = windows_options();
        assert_is_match("of", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_124() {
        let opts = windows_options();
        assert_is_match("ooo", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_125() {
        let opts = windows_options();
        assert_is_match("ox", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_126() {
        let opts = windows_options();
        assert_is_match("x", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_127() {
        let opts = windows_options();
        assert_is_match("xx", "!(f(o))", opts.clone(), true);
    }

    #[test]
    fn test_128() {
        let opts = windows_options();
        assert_is_match("bar", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_129() {
        let opts = windows_options();
        assert_is_match("f", "!(f)", opts.clone(), false);
    }

    #[test]
    fn test_130() {
        let opts = windows_options();
        assert_is_match("fa", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_131() {
        let opts = windows_options();
        assert_is_match("fb", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_132() {
        let opts = windows_options();
        assert_is_match("ff", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_133() {
        let opts = windows_options();
        assert_is_match("fff", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_134() {
        let opts = windows_options();
        assert_is_match("fo", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_135() {
        let opts = windows_options();
        assert_is_match("foo", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_136() {
        let opts = windows_options();
        assert_is_match("foo/bar", "!(f)", opts.clone(), false);
    }

    #[test]
    fn test_137() {
        let opts = windows_options();
        assert_is_match("foobar", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_138() {
        let opts = windows_options();
        assert_is_match("foot", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_139() {
        let opts = windows_options();
        assert_is_match("foox", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_140() {
        let opts = windows_options();
        assert_is_match("o", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_141() {
        let opts = windows_options();
        assert_is_match("of", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_142() {
        let opts = windows_options();
        assert_is_match("ooo", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_143() {
        let opts = windows_options();
        assert_is_match("ox", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_144() {
        let opts = windows_options();
        assert_is_match("x", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_145() {
        let opts = windows_options();
        assert_is_match("xx", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_146() {
        let opts = windows_options();
        assert_is_match("bar", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_147() {
        let opts = windows_options();
        assert_is_match("f", "!(f)", opts.clone(), false);
    }

    #[test]
    fn test_148() {
        let opts = windows_options();
        assert_is_match("fa", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_149() {
        let opts = windows_options();
        assert_is_match("fb", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_150() {
        let opts = windows_options();
        assert_is_match("ff", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_151() {
        let opts = windows_options();
        assert_is_match("fff", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_152() {
        let opts = windows_options();
        assert_is_match("fo", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_153() {
        let opts = windows_options();
        assert_is_match("foo", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_154() {
        let opts = windows_options();
        assert_is_match("foo/bar", "!(f)", opts.clone(), false);
    }

    #[test]
    fn test_155() {
        let opts = windows_options();
        assert_is_match("foobar", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_156() {
        let opts = windows_options();
        assert_is_match("foot", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_157() {
        let opts = windows_options();
        assert_is_match("foox", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_158() {
        let opts = windows_options();
        assert_is_match("o", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_159() {
        let opts = windows_options();
        assert_is_match("of", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_160() {
        let opts = windows_options();
        assert_is_match("ooo", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_161() {
        let opts = windows_options();
        assert_is_match("ox", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_162() {
        let opts = windows_options();
        assert_is_match("x", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_163() {
        let opts = windows_options();
        assert_is_match("xx", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_164() {
        let opts = windows_options();
        assert_is_match("bar", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_165() {
        let opts = windows_options();
        assert_is_match("f", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_166() {
        let opts = windows_options();
        assert_is_match("fa", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_167() {
        let opts = windows_options();
        assert_is_match("fb", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_168() {
        let opts = windows_options();
        assert_is_match("ff", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_169() {
        let opts = windows_options();
        assert_is_match("fff", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_170() {
        let opts = windows_options();
        assert_is_match("fo", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_171() {
        let opts = windows_options();
        assert_is_match("foo", "!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_172() {
        let opts = windows_options();
        assert_is_match("foo/bar", "!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_173() {
        let opts = windows_options();
        assert_is_match("foobar", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_174() {
        let opts = windows_options();
        assert_is_match("foot", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_175() {
        let opts = windows_options();
        assert_is_match("foox", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_176() {
        let opts = windows_options();
        assert_is_match("o", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_177() {
        let opts = windows_options();
        assert_is_match("of", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_178() {
        let opts = windows_options();
        assert_is_match("ooo", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_179() {
        let opts = windows_options();
        assert_is_match("ox", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_180() {
        let opts = windows_options();
        assert_is_match("x", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_181() {
        let opts = windows_options();
        assert_is_match("xx", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_182() {
        let opts = windows_options();
        assert_is_match("bar", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_183() {
        let opts = windows_options();
        assert_is_match("f", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_184() {
        let opts = windows_options();
        assert_is_match("fa", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_185() {
        let opts = windows_options();
        assert_is_match("fb", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_186() {
        let opts = windows_options();
        assert_is_match("ff", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_187() {
        let opts = windows_options();
        assert_is_match("fff", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_188() {
        let opts = windows_options();
        assert_is_match("fo", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_189() {
        let opts = windows_options();
        assert_is_match("foo", "!(foo)*", opts.clone(), false);
    }

    #[test]
    fn test_190() {
        let opts = windows_options();
        assert_is_match("foo/bar", "!(foo)*", opts.clone(), false);
    }

    #[test]
    fn test_191() {
        let opts = windows_options();
        assert_is_match("foobar", "!(foo)*", opts.clone(), false);
    }

    #[test]
    fn test_192() {
        let opts = windows_options();
        assert_is_match("foot", "!(foo)*", opts.clone(), false);
    }

    #[test]
    fn test_193() {
        let opts = windows_options();
        assert_is_match("foox", "!(foo)*", opts.clone(), false);
    }

    #[test]
    fn test_194() {
        let opts = windows_options();
        assert_is_match("o", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_195() {
        let opts = windows_options();
        assert_is_match("of", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_196() {
        let opts = windows_options();
        assert_is_match("ooo", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_197() {
        let opts = windows_options();
        assert_is_match("ox", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_198() {
        let opts = windows_options();
        assert_is_match("x", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_199() {
        let opts = windows_options();
        assert_is_match("xx", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_200() {
        let opts = windows_options();
        assert_is_match("bar", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_201() {
        let opts = windows_options();
        assert_is_match("f", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_202() {
        let opts = windows_options();
        assert_is_match("fa", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_203() {
        let opts = windows_options();
        assert_is_match("fb", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_204() {
        let opts = windows_options();
        assert_is_match("ff", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_205() {
        let opts = windows_options();
        assert_is_match("fff", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_206() {
        let opts = windows_options();
        assert_is_match("fo", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_207() {
        let opts = windows_options();
        assert_is_match("foo", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_208() {
        let opts = windows_options();
        assert_is_match("foo/bar", "!(x)", opts.clone(), false);
    }

    #[test]
    fn test_209() {
        let opts = windows_options();
        assert_is_match("foobar", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_210() {
        let opts = windows_options();
        assert_is_match("foot", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_211() {
        let opts = windows_options();
        assert_is_match("foox", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_212() {
        let opts = windows_options();
        assert_is_match("o", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_213() {
        let opts = windows_options();
        assert_is_match("of", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_214() {
        let opts = windows_options();
        assert_is_match("ooo", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_215() {
        let opts = windows_options();
        assert_is_match("ox", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_216() {
        let opts = windows_options();
        assert_is_match("x", "!(x)", opts.clone(), false);
    }

    #[test]
    fn test_217() {
        let opts = windows_options();
        assert_is_match("xx", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_218() {
        let opts = windows_options();
        assert_is_match("bar", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_219() {
        let opts = windows_options();
        assert_is_match("f", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_220() {
        let opts = windows_options();
        assert_is_match("fa", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_221() {
        let opts = windows_options();
        assert_is_match("fb", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_222() {
        let opts = windows_options();
        assert_is_match("ff", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_223() {
        let opts = windows_options();
        assert_is_match("fff", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_224() {
        let opts = windows_options();
        assert_is_match("fo", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_225() {
        let opts = windows_options();
        assert_is_match("foo", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_226() {
        let opts = windows_options();
        assert_is_match("foo/bar", "!(x)*", opts.clone(), false);
    }

    #[test]
    fn test_227() {
        let opts = windows_options();
        assert_is_match("foobar", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_228() {
        let opts = windows_options();
        assert_is_match("foot", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_229() {
        let opts = windows_options();
        assert_is_match("foox", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_230() {
        let opts = windows_options();
        assert_is_match("o", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_231() {
        let opts = windows_options();
        assert_is_match("of", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_232() {
        let opts = windows_options();
        assert_is_match("ooo", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_233() {
        let opts = windows_options();
        assert_is_match("ox", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_234() {
        let opts = windows_options();
        assert_is_match("x", "!(x)*", opts.clone(), false);
    }

    #[test]
    fn test_235() {
        let opts = windows_options();
        assert_is_match("xx", "!(x)*", opts.clone(), false);
    }

    #[test]
    fn test_236() {
        let opts = windows_options();
        assert_is_match("bar", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_237() {
        let opts = windows_options();
        assert_is_match("f", "*(!(f))", opts.clone(), false);
    }

    #[test]
    fn test_238() {
        let opts = windows_options();
        assert_is_match("fa", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_239() {
        let opts = windows_options();
        assert_is_match("fb", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_240() {
        let opts = windows_options();
        assert_is_match("ff", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_241() {
        let opts = windows_options();
        assert_is_match("fff", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_242() {
        let opts = windows_options();
        assert_is_match("fo", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_243() {
        let opts = windows_options();
        assert_is_match("foo", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_244() {
        let opts = windows_options();
        assert_is_match("foo/bar", "*(!(f))", opts.clone(), false);
    }

    #[test]
    fn test_245() {
        let opts = windows_options();
        assert_is_match("foobar", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_246() {
        let opts = windows_options();
        assert_is_match("foot", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_247() {
        let opts = windows_options();
        assert_is_match("foox", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_248() {
        let opts = windows_options();
        assert_is_match("o", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_249() {
        let opts = windows_options();
        assert_is_match("of", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_250() {
        let opts = windows_options();
        assert_is_match("ooo", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_251() {
        let opts = windows_options();
        assert_is_match("ox", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_252() {
        let opts = windows_options();
        assert_is_match("x", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_253() {
        let opts = windows_options();
        assert_is_match("xx", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_254() {
        let opts = windows_options();
        assert_is_match("bar", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_255() {
        let opts = windows_options();
        assert_is_match("f", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_256() {
        let opts = windows_options();
        assert_is_match("fa", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_257() {
        let opts = windows_options();
        assert_is_match("fb", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_258() {
        let opts = windows_options();
        assert_is_match("ff", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_259() {
        let opts = windows_options();
        assert_is_match("fff", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_260() {
        let opts = windows_options();
        assert_is_match("fo", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_261() {
        let opts = windows_options();
        assert_is_match("foo", "*((foo))", opts.clone(), true);
    }

    #[test]
    fn test_262() {
        let opts = windows_options();
        assert_is_match("foo/bar", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_263() {
        let opts = windows_options();
        assert_is_match("foobar", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_264() {
        let opts = windows_options();
        assert_is_match("foot", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_265() {
        let opts = windows_options();
        assert_is_match("foox", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_266() {
        let opts = windows_options();
        assert_is_match("o", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_267() {
        let opts = windows_options();
        assert_is_match("of", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_268() {
        let opts = windows_options();
        assert_is_match("ooo", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_269() {
        let opts = windows_options();
        assert_is_match("ox", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_270() {
        let opts = windows_options();
        assert_is_match("x", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_271() {
        let opts = windows_options();
        assert_is_match("xx", "*((foo))", opts.clone(), false);
    }

    #[test]
    fn test_272() {
        let opts = windows_options();
        assert_is_match("bar", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_273() {
        let opts = windows_options();
        assert_is_match("f", "+(!(f))", opts.clone(), false);
    }

    #[test]
    fn test_274() {
        let opts = windows_options();
        assert_is_match("fa", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_275() {
        let opts = windows_options();
        assert_is_match("fb", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_276() {
        let opts = windows_options();
        assert_is_match("ff", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_277() {
        let opts = windows_options();
        assert_is_match("fff", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_278() {
        let opts = windows_options();
        assert_is_match("fo", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_279() {
        let opts = windows_options();
        assert_is_match("foo", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_280() {
        let opts = windows_options();
        assert_is_match("foo/bar", "+(!(f))", opts.clone(), false);
    }

    #[test]
    fn test_281() {
        let opts = windows_options();
        assert_is_match("foobar", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_282() {
        let opts = windows_options();
        assert_is_match("foot", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_283() {
        let opts = windows_options();
        assert_is_match("foox", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_284() {
        let opts = windows_options();
        assert_is_match("o", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_285() {
        let opts = windows_options();
        assert_is_match("of", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_286() {
        let opts = windows_options();
        assert_is_match("ooo", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_287() {
        let opts = windows_options();
        assert_is_match("ox", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_288() {
        let opts = windows_options();
        assert_is_match("x", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_289() {
        let opts = windows_options();
        assert_is_match("xx", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_290() {
        let opts = windows_options();
        assert_is_match("bar", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_291() {
        let opts = windows_options();
        assert_is_match("f", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_292() {
        let opts = windows_options();
        assert_is_match("fa", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_293() {
        let opts = windows_options();
        assert_is_match("fb", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_294() {
        let opts = windows_options();
        assert_is_match("ff", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_295() {
        let opts = windows_options();
        assert_is_match("fff", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_296() {
        let opts = windows_options();
        assert_is_match("fo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_297() {
        let opts = windows_options();
        assert_is_match("foo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_298() {
        let opts = windows_options();
        assert_is_match("foo/bar", "@(!(z*/*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_299() {
        let opts = windows_options();
        assert_is_match("foo/bar", "@(!(z*)|*x)", opts.clone(), false);
    }

    #[test]
    fn test_300() {
        let opts = windows_options();
        assert_is_match("foobar", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_301() {
        let opts = windows_options();
        assert_is_match("foot", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_302() {
        let opts = windows_options();
        assert_is_match("foox", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_303() {
        let opts = windows_options();
        assert_is_match("o", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_304() {
        let opts = windows_options();
        assert_is_match("of", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_305() {
        let opts = windows_options();
        assert_is_match("ooo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_306() {
        let opts = windows_options();
        assert_is_match("ox", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_307() {
        let opts = windows_options();
        assert_is_match("x", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_308() {
        let opts = windows_options();
        assert_is_match("xx", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_309() {
        let opts = windows_options();
        assert_is_match("bar", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_310() {
        let opts = windows_options();
        assert_is_match("f", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_311() {
        let opts = windows_options();
        assert_is_match("fa", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_312() {
        let opts = windows_options();
        assert_is_match("fb", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_313() {
        let opts = windows_options();
        assert_is_match("ff", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_314() {
        let opts = windows_options();
        assert_is_match("fff", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_315() {
        let opts = windows_options();
        assert_is_match("fo", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_316() {
        let opts = windows_options();
        assert_is_match("foo", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_317() {
        let opts = windows_options();
        assert_is_match("foo/bar", "foo/!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_318() {
        let opts = windows_options();
        assert_is_match("foobar", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_319() {
        let opts = windows_options();
        assert_is_match("foot", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_320() {
        let opts = windows_options();
        assert_is_match("foox", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_321() {
        let opts = windows_options();
        assert_is_match("o", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_322() {
        let opts = windows_options();
        assert_is_match("of", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_323() {
        let opts = windows_options();
        assert_is_match("ooo", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_324() {
        let opts = windows_options();
        assert_is_match("ox", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_325() {
        let opts = windows_options();
        assert_is_match("x", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_326() {
        let opts = windows_options();
        assert_is_match("xx", "foo/!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_327() {
        let opts = windows_options();
        assert_is_match("ffffffo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_328() {
        let opts = windows_options();
        assert_is_match("fffooofoooooffoofffooofff", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_329() {
        let opts = windows_options();
        assert_is_match("ffo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_330() {
        let opts = windows_options();
        assert_is_match("fofo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_331() {
        let opts = windows_options();
        assert_is_match("fofoofoofofoo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_332() {
        let opts = windows_options();
        assert_is_match("foo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_333() {
        let opts = windows_options();
        assert_is_match("foob", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_334() {
        let opts = windows_options();
        assert_is_match("foobb", "(foo)bb", opts.clone(), true);
    }

    #[test]
    fn test_335() {
        let opts = windows_options();
        assert_is_match("foofoofo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_336() {
        let opts = windows_options();
        assert_is_match("fooofoofofooo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_337() {
        let opts = windows_options();
        assert_is_match("foooofo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_338() {
        let opts = windows_options();
        assert_is_match("foooofof", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_339() {
        let opts = windows_options();
        assert_is_match("foooofofx", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_340() {
        let opts = windows_options();
        assert_is_match("foooxfooxfoxfooox", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_341() {
        let opts = windows_options();
        assert_is_match("foooxfooxfxfooox", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_342() {
        let opts = windows_options();
        assert_is_match("foooxfooxofoxfooox", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_343() {
        let opts = windows_options();
        assert_is_match("foot", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_344() {
        let opts = windows_options();
        assert_is_match("foox", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_345() {
        let opts = windows_options();
        assert_is_match("ofoofo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_346() {
        let opts = windows_options();
        assert_is_match("ofooofoofofooo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_347() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_348() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_349() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxofo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_350() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxoo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_351() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "(foo)bb",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_352() {
        let opts = windows_options();
        assert_is_match("ofxoofxo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_353() {
        let opts = windows_options();
        assert_is_match("oofooofo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_354() {
        let opts = windows_options();
        assert_is_match("ooo", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_355() {
        let opts = windows_options();
        assert_is_match("oxfoxfox", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_356() {
        let opts = windows_options();
        assert_is_match("oxfoxoxfox", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_357() {
        let opts = windows_options();
        assert_is_match("xfoooofof", "(foo)bb", opts.clone(), false);
    }

    #[test]
    fn test_358() {
        let opts = windows_options();
        assert_is_match("ffffffo", "*(*(f)*(o))", opts.clone(), true);
    }

    #[test]
    fn test_359() {
        let opts = windows_options();
        assert_is_match(
            "fffooofoooooffoofffooofff",
            "*(*(f)*(o))",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_360() {
        let opts = windows_options();
        assert_is_match("ffo", "*(*(f)*(o))", opts.clone(), true);
    }

    #[test]
    fn test_361() {
        let opts = windows_options();
        assert_is_match("fofo", "*(*(f)*(o))", opts.clone(), true);
    }

    #[test]
    fn test_362() {
        let opts = windows_options();
        assert_is_match("fofoofoofofoo", "*(*(f)*(o))", opts.clone(), true);
    }

    #[test]
    fn test_363() {
        let opts = windows_options();
        assert_is_match("foo", "*(*(f)*(o))", opts.clone(), true);
    }

    #[test]
    fn test_364() {
        let opts = windows_options();
        assert_is_match("foob", "*(*(f)*(o))", opts.clone(), false);
    }

    #[test]
    fn test_365() {
        let opts = windows_options();
        assert_is_match("foobb", "*(*(f)*(o))", opts.clone(), false);
    }

    #[test]
    fn test_366() {
        let opts = windows_options();
        assert_is_match("foofoofo", "*(*(f)*(o))", opts.clone(), true);
    }

    #[test]
    fn test_367() {
        let opts = windows_options();
        assert_is_match("fooofoofofooo", "*(*(f)*(o))", opts.clone(), true);
    }

    #[test]
    fn test_368() {
        let opts = windows_options();
        assert_is_match("foooofo", "*(*(f)*(o))", opts.clone(), true);
    }

    #[test]
    fn test_369() {
        let opts = windows_options();
        assert_is_match("foooofof", "*(*(f)*(o))", opts.clone(), true);
    }

    #[test]
    fn test_370() {
        let opts = windows_options();
        assert_is_match("foooofofx", "*(*(f)*(o))", opts.clone(), false);
    }

    #[test]
    fn test_371() {
        let opts = windows_options();
        assert_is_match("foooxfooxfoxfooox", "*(*(f)*(o))", opts.clone(), false);
    }

    #[test]
    fn test_372() {
        let opts = windows_options();
        assert_is_match("foooxfooxfxfooox", "*(*(f)*(o))", opts.clone(), false);
    }

    #[test]
    fn test_373() {
        let opts = windows_options();
        assert_is_match("foooxfooxofoxfooox", "*(*(f)*(o))", opts.clone(), false);
    }

    #[test]
    fn test_374() {
        let opts = windows_options();
        assert_is_match("foot", "*(*(f)*(o))", opts.clone(), false);
    }

    #[test]
    fn test_375() {
        let opts = windows_options();
        assert_is_match("foox", "*(*(f)*(o))", opts.clone(), false);
    }

    #[test]
    fn test_376() {
        let opts = windows_options();
        assert_is_match("ofoofo", "*(*(f)*(o))", opts.clone(), true);
    }

    #[test]
    fn test_377() {
        let opts = windows_options();
        assert_is_match("ofooofoofofooo", "*(*(f)*(o))", opts.clone(), true);
    }

    #[test]
    fn test_378() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxo", "*(*(f)*(o))", opts.clone(), false);
    }

    #[test]
    fn test_379() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxo", "*(*(f)*(o))", opts.clone(), false);
    }

    #[test]
    fn test_380() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxofo",
            "*(*(f)*(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_381() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxoo",
            "*(*(f)*(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_382() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "*(*(f)*(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_383() {
        let opts = windows_options();
        assert_is_match("ofxoofxo", "*(*(f)*(o))", opts.clone(), false);
    }

    #[test]
    fn test_384() {
        let opts = windows_options();
        assert_is_match("oofooofo", "*(*(f)*(o))", opts.clone(), true);
    }

    #[test]
    fn test_385() {
        let opts = windows_options();
        assert_is_match("ooo", "*(*(f)*(o))", opts.clone(), true);
    }

    #[test]
    fn test_386() {
        let opts = windows_options();
        assert_is_match("oxfoxfox", "*(*(f)*(o))", opts.clone(), false);
    }

    #[test]
    fn test_387() {
        let opts = windows_options();
        assert_is_match("oxfoxoxfox", "*(*(f)*(o))", opts.clone(), false);
    }

    #[test]
    fn test_388() {
        let opts = windows_options();
        assert_is_match("xfoooofof", "*(*(f)*(o))", opts.clone(), false);
    }

    #[test]
    fn test_389() {
        let opts = windows_options();
        assert_is_match("ffffffo", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_390() {
        let opts = windows_options();
        assert_is_match(
            "fffooofoooooffoofffooofff",
            "*(*(of*(o)x)o)",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_391() {
        let opts = windows_options();
        assert_is_match("ffo", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_392() {
        let opts = windows_options();
        assert_is_match("fofo", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_393() {
        let opts = windows_options();
        assert_is_match("fofoofoofofoo", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_394() {
        let opts = windows_options();
        assert_is_match("foo", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_395() {
        let opts = windows_options();
        assert_is_match("foob", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_396() {
        let opts = windows_options();
        assert_is_match("foobb", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_397() {
        let opts = windows_options();
        assert_is_match("foofoofo", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_398() {
        let opts = windows_options();
        assert_is_match("fooofoofofooo", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_399() {
        let opts = windows_options();
        assert_is_match("foooofo", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_400() {
        let opts = windows_options();
        assert_is_match("foooofof", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_401() {
        let opts = windows_options();
        assert_is_match("foooofofx", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_402() {
        let opts = windows_options();
        assert_is_match("foooxfooxfoxfooox", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_403() {
        let opts = windows_options();
        assert_is_match("foooxfooxfxfooox", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_404() {
        let opts = windows_options();
        assert_is_match("foooxfooxofoxfooox", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_405() {
        let opts = windows_options();
        assert_is_match("foot", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_406() {
        let opts = windows_options();
        assert_is_match("foox", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_407() {
        let opts = windows_options();
        assert_is_match("ofoofo", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_408() {
        let opts = windows_options();
        assert_is_match("ofooofoofofooo", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_409() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxo", "*(*(of*(o)x)o)", opts.clone(), true);
    }

    #[test]
    fn test_410() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxo",
            "*(*(of*(o)x)o)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_411() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxofo",
            "*(*(of*(o)x)o)",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_412() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxoo",
            "*(*(of*(o)x)o)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_413() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "*(*(of*(o)x)o)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_414() {
        let opts = windows_options();
        assert_is_match("ofxoofxo", "*(*(of*(o)x)o)", opts.clone(), true);
    }

    #[test]
    fn test_415() {
        let opts = windows_options();
        assert_is_match("oofooofo", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_416() {
        let opts = windows_options();
        assert_is_match("ooo", "*(*(of*(o)x)o)", opts.clone(), true);
    }

    #[test]
    fn test_417() {
        let opts = windows_options();
        assert_is_match("oxfoxfox", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_418() {
        let opts = windows_options();
        assert_is_match("oxfoxoxfox", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_419() {
        let opts = windows_options();
        assert_is_match("xfoooofof", "*(*(of*(o)x)o)", opts.clone(), false);
    }

    #[test]
    fn test_420() {
        let opts = windows_options();
        assert_is_match("ffffffo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_421() {
        let opts = windows_options();
        assert_is_match("fffooofoooooffoofffooofff", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_422() {
        let opts = windows_options();
        assert_is_match("ffo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_423() {
        let opts = windows_options();
        assert_is_match("fofo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_424() {
        let opts = windows_options();
        assert_is_match("fofoofoofofoo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_425() {
        let opts = windows_options();
        assert_is_match("foo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_426() {
        let opts = windows_options();
        assert_is_match("foob", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_427() {
        let opts = windows_options();
        assert_is_match("foobb", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_428() {
        let opts = windows_options();
        assert_is_match("foofoofo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_429() {
        let opts = windows_options();
        assert_is_match("fooofoofofooo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_430() {
        let opts = windows_options();
        assert_is_match("foooofo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_431() {
        let opts = windows_options();
        assert_is_match("foooofof", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_432() {
        let opts = windows_options();
        assert_is_match("foooofofx", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_433() {
        let opts = windows_options();
        assert_is_match("foooxfooxfoxfooox", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_434() {
        let opts = windows_options();
        assert_is_match("foooxfooxfxfooox", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_435() {
        let opts = windows_options();
        assert_is_match("foooxfooxofoxfooox", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_436() {
        let opts = windows_options();
        assert_is_match("foot", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_437() {
        let opts = windows_options();
        assert_is_match("foox", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_438() {
        let opts = windows_options();
        assert_is_match("ofoofo", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_439() {
        let opts = windows_options();
        assert_is_match("ofooofoofofooo", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_440() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxo", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_441() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxo", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_442() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxofo", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_443() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxoo", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_444() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "*(f*(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_445() {
        let opts = windows_options();
        assert_is_match("ofxoofxo", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_446() {
        let opts = windows_options();
        assert_is_match("oofooofo", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_447() {
        let opts = windows_options();
        assert_is_match("ooo", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_448() {
        let opts = windows_options();
        assert_is_match("oxfoxfox", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_449() {
        let opts = windows_options();
        assert_is_match("oxfoxoxfox", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_450() {
        let opts = windows_options();
        assert_is_match("xfoooofof", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_451() {
        let opts = windows_options();
        assert_is_match("ffffffo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_452() {
        let opts = windows_options();
        assert_is_match(
            "fffooofoooooffoofffooofff",
            "*(f*(o)x)",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_453() {
        let opts = windows_options();
        assert_is_match("ffo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_454() {
        let opts = windows_options();
        assert_is_match("fofo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_455() {
        let opts = windows_options();
        assert_is_match("fofoofoofofoo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_456() {
        let opts = windows_options();
        assert_is_match("foo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_457() {
        let opts = windows_options();
        assert_is_match("foob", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_458() {
        let opts = windows_options();
        assert_is_match("foobb", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_459() {
        let opts = windows_options();
        assert_is_match("foofoofo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_460() {
        let opts = windows_options();
        assert_is_match("fooofoofofooo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_461() {
        let opts = windows_options();
        assert_is_match("foooofo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_462() {
        let opts = windows_options();
        assert_is_match("foooofof", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_463() {
        let opts = windows_options();
        assert_is_match("foooofofx", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_464() {
        let opts = windows_options();
        assert_is_match("foooxfooxfoxfooox", "*(f*(o)x)", opts.clone(), true);
    }

    #[test]
    fn test_465() {
        let opts = windows_options();
        assert_is_match("foooxfooxfxfooox", "*(f*(o)x)", opts.clone(), true);
    }

    #[test]
    fn test_466() {
        let opts = windows_options();
        assert_is_match("foooxfooxofoxfooox", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_467() {
        let opts = windows_options();
        assert_is_match("foot", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_468() {
        let opts = windows_options();
        assert_is_match("foox", "*(f*(o)x)", opts.clone(), true);
    }

    #[test]
    fn test_469() {
        let opts = windows_options();
        assert_is_match("ofoofo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_470() {
        let opts = windows_options();
        assert_is_match("ofooofoofofooo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_471() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_472() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_473() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxofo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_474() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxoo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_475() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "*(f*(o)x)",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_476() {
        let opts = windows_options();
        assert_is_match("ofxoofxo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_477() {
        let opts = windows_options();
        assert_is_match("oofooofo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_478() {
        let opts = windows_options();
        assert_is_match("ooo", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_479() {
        let opts = windows_options();
        assert_is_match("oxfoxfox", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_480() {
        let opts = windows_options();
        assert_is_match("oxfoxoxfox", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_481() {
        let opts = windows_options();
        assert_is_match("xfoooofof", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_482() {
        let opts = windows_options();
        assert_is_match("ffffffo", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_483() {
        let opts = windows_options();
        assert_is_match("fffooofoooooffoofffooofff", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_484() {
        let opts = windows_options();
        assert_is_match("ffo", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_485() {
        let opts = windows_options();
        assert_is_match("fofo", "*(f+(o))", opts.clone(), true);
    }

    #[test]
    fn test_486() {
        let opts = windows_options();
        assert_is_match("fofoofoofofoo", "*(f+(o))", opts.clone(), true);
    }

    #[test]
    fn test_487() {
        let opts = windows_options();
        assert_is_match("foo", "*(f+(o))", opts.clone(), true);
    }

    #[test]
    fn test_488() {
        let opts = windows_options();
        assert_is_match("foob", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_489() {
        let opts = windows_options();
        assert_is_match("foobb", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_490() {
        let opts = windows_options();
        assert_is_match("foofoofo", "*(f+(o))", opts.clone(), true);
    }

    #[test]
    fn test_491() {
        let opts = windows_options();
        assert_is_match("fooofoofofooo", "*(f+(o))", opts.clone(), true);
    }

    #[test]
    fn test_492() {
        let opts = windows_options();
        assert_is_match("foooofo", "*(f+(o))", opts.clone(), true);
    }

    #[test]
    fn test_493() {
        let opts = windows_options();
        assert_is_match("foooofof", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_494() {
        let opts = windows_options();
        assert_is_match("foooofofx", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_495() {
        let opts = windows_options();
        assert_is_match("foooxfooxfoxfooox", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_496() {
        let opts = windows_options();
        assert_is_match("foooxfooxfxfooox", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_497() {
        let opts = windows_options();
        assert_is_match("foooxfooxofoxfooox", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_498() {
        let opts = windows_options();
        assert_is_match("foot", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_499() {
        let opts = windows_options();
        assert_is_match("foox", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_500() {
        let opts = windows_options();
        assert_is_match("ofoofo", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_501() {
        let opts = windows_options();
        assert_is_match("ofooofoofofooo", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_502() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxo", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_503() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxo", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_504() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxofo", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_505() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxoo", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_506() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "*(f+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_507() {
        let opts = windows_options();
        assert_is_match("ofxoofxo", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_508() {
        let opts = windows_options();
        assert_is_match("oofooofo", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_509() {
        let opts = windows_options();
        assert_is_match("ooo", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_510() {
        let opts = windows_options();
        assert_is_match("oxfoxfox", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_511() {
        let opts = windows_options();
        assert_is_match("oxfoxoxfox", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_512() {
        let opts = windows_options();
        assert_is_match("xfoooofof", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_513() {
        let opts = windows_options();
        assert_is_match("ffffffo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_514() {
        let opts = windows_options();
        assert_is_match(
            "fffooofoooooffoofffooofff",
            "*(of+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_515() {
        let opts = windows_options();
        assert_is_match("ffo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_516() {
        let opts = windows_options();
        assert_is_match("fofo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_517() {
        let opts = windows_options();
        assert_is_match("fofoofoofofoo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_518() {
        let opts = windows_options();
        assert_is_match("foo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_519() {
        let opts = windows_options();
        assert_is_match("foob", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_520() {
        let opts = windows_options();
        assert_is_match("foobb", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_521() {
        let opts = windows_options();
        assert_is_match("foofoofo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_522() {
        let opts = windows_options();
        assert_is_match("fooofoofofooo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_523() {
        let opts = windows_options();
        assert_is_match("foooofo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_524() {
        let opts = windows_options();
        assert_is_match("foooofof", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_525() {
        let opts = windows_options();
        assert_is_match("foooofofx", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_526() {
        let opts = windows_options();
        assert_is_match("foooxfooxfoxfooox", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_527() {
        let opts = windows_options();
        assert_is_match("foooxfooxfxfooox", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_528() {
        let opts = windows_options();
        assert_is_match("foooxfooxofoxfooox", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_529() {
        let opts = windows_options();
        assert_is_match("foot", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_530() {
        let opts = windows_options();
        assert_is_match("foox", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_531() {
        let opts = windows_options();
        assert_is_match("ofoofo", "*(of+(o))", opts.clone(), true);
    }

    #[test]
    fn test_532() {
        let opts = windows_options();
        assert_is_match("ofooofoofofooo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_533() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_534() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_535() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxofo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_536() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxoo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_537() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "*(of+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_538() {
        let opts = windows_options();
        assert_is_match("ofxoofxo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_539() {
        let opts = windows_options();
        assert_is_match("oofooofo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_540() {
        let opts = windows_options();
        assert_is_match("ooo", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_541() {
        let opts = windows_options();
        assert_is_match("oxfoxfox", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_542() {
        let opts = windows_options();
        assert_is_match("oxfoxoxfox", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_543() {
        let opts = windows_options();
        assert_is_match("xfoooofof", "*(of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_544() {
        let opts = windows_options();
        assert_is_match("ffffffo", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_545() {
        let opts = windows_options();
        assert_is_match(
            "fffooofoooooffoofffooofff",
            "*(of+(o)|f)",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_546() {
        let opts = windows_options();
        assert_is_match("ffo", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_547() {
        let opts = windows_options();
        assert_is_match("fofo", "*(of+(o)|f)", opts.clone(), true);
    }

    #[test]
    fn test_548() {
        let opts = windows_options();
        assert_is_match("fofoofoofofoo", "*(of+(o)|f)", opts.clone(), true);
    }

    #[test]
    fn test_549() {
        let opts = windows_options();
        assert_is_match("foo", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_550() {
        let opts = windows_options();
        assert_is_match("foob", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_551() {
        let opts = windows_options();
        assert_is_match("foobb", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_552() {
        let opts = windows_options();
        assert_is_match("foofoofo", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_553() {
        let opts = windows_options();
        assert_is_match("fooofoofofooo", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_554() {
        let opts = windows_options();
        assert_is_match("foooofo", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_555() {
        let opts = windows_options();
        assert_is_match("foooofof", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_556() {
        let opts = windows_options();
        assert_is_match("foooofofx", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_557() {
        let opts = windows_options();
        assert_is_match("foooxfooxfoxfooox", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_558() {
        let opts = windows_options();
        assert_is_match("foooxfooxfxfooox", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_559() {
        let opts = windows_options();
        assert_is_match("foooxfooxofoxfooox", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_560() {
        let opts = windows_options();
        assert_is_match("foot", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_561() {
        let opts = windows_options();
        assert_is_match("foox", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_562() {
        let opts = windows_options();
        assert_is_match("ofoofo", "*(of+(o)|f)", opts.clone(), true);
    }

    #[test]
    fn test_563() {
        let opts = windows_options();
        assert_is_match("ofooofoofofooo", "*(of+(o)|f)", opts.clone(), true);
    }

    #[test]
    fn test_564() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxo", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_565() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxo", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_566() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxofo",
            "*(of+(o)|f)",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_567() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxoo",
            "*(of+(o)|f)",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_568() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "*(of+(o)|f)",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_569() {
        let opts = windows_options();
        assert_is_match("ofxoofxo", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_570() {
        let opts = windows_options();
        assert_is_match("oofooofo", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_571() {
        let opts = windows_options();
        assert_is_match("ooo", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_572() {
        let opts = windows_options();
        assert_is_match("oxfoxfox", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_573() {
        let opts = windows_options();
        assert_is_match("oxfoxoxfox", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_574() {
        let opts = windows_options();
        assert_is_match("xfoooofof", "*(of+(o)|f)", opts.clone(), false);
    }

    #[test]
    fn test_575() {
        let opts = windows_options();
        assert_is_match("ffffffo", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_576() {
        let opts = windows_options();
        assert_is_match(
            "fffooofoooooffoofffooofff",
            "*(of|oof+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_577() {
        let opts = windows_options();
        assert_is_match("ffo", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_578() {
        let opts = windows_options();
        assert_is_match("fofo", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_579() {
        let opts = windows_options();
        assert_is_match("fofoofoofofoo", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_580() {
        let opts = windows_options();
        assert_is_match("foo", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_581() {
        let opts = windows_options();
        assert_is_match("foob", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_582() {
        let opts = windows_options();
        assert_is_match("foobb", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_583() {
        let opts = windows_options();
        assert_is_match("foofoofo", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_584() {
        let opts = windows_options();
        assert_is_match("fooofoofofooo", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_585() {
        let opts = windows_options();
        assert_is_match("foooofo", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_586() {
        let opts = windows_options();
        assert_is_match("foooofof", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_587() {
        let opts = windows_options();
        assert_is_match("foooofofx", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_588() {
        let opts = windows_options();
        assert_is_match("foooxfooxfoxfooox", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_589() {
        let opts = windows_options();
        assert_is_match("foooxfooxfxfooox", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_590() {
        let opts = windows_options();
        assert_is_match("foooxfooxofoxfooox", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_591() {
        let opts = windows_options();
        assert_is_match("foot", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_592() {
        let opts = windows_options();
        assert_is_match("foox", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_593() {
        let opts = windows_options();
        assert_is_match("ofoofo", "*(of|oof+(o))", opts.clone(), true);
    }

    #[test]
    fn test_594() {
        let opts = windows_options();
        assert_is_match("ofooofoofofooo", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_595() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxo", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_596() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxo",
            "*(of|oof+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_597() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxofo",
            "*(of|oof+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_598() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxoo",
            "*(of|oof+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_599() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "*(of|oof+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_600() {
        let opts = windows_options();
        assert_is_match("ofxoofxo", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_601() {
        let opts = windows_options();
        assert_is_match("oofooofo", "*(of|oof+(o))", opts.clone(), true);
    }

    #[test]
    fn test_602() {
        let opts = windows_options();
        assert_is_match("ooo", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_603() {
        let opts = windows_options();
        assert_is_match("oxfoxfox", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_604() {
        let opts = windows_options();
        assert_is_match("oxfoxoxfox", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_605() {
        let opts = windows_options();
        assert_is_match("xfoooofof", "*(of|oof+(o))", opts.clone(), false);
    }

    #[test]
    fn test_606() {
        let opts = windows_options();
        assert_is_match("ffffffo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_607() {
        let opts = windows_options();
        assert_is_match(
            "fffooofoooooffoofffooofff",
            "*(oxf+(ox))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_608() {
        let opts = windows_options();
        assert_is_match("ffo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_609() {
        let opts = windows_options();
        assert_is_match("fofo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_610() {
        let opts = windows_options();
        assert_is_match("fofoofoofofoo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_611() {
        let opts = windows_options();
        assert_is_match("foo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_612() {
        let opts = windows_options();
        assert_is_match("foob", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_613() {
        let opts = windows_options();
        assert_is_match("foobb", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_614() {
        let opts = windows_options();
        assert_is_match("foofoofo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_615() {
        let opts = windows_options();
        assert_is_match("fooofoofofooo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_616() {
        let opts = windows_options();
        assert_is_match("foooofo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_617() {
        let opts = windows_options();
        assert_is_match("foooofof", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_618() {
        let opts = windows_options();
        assert_is_match("foooofofx", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_619() {
        let opts = windows_options();
        assert_is_match("foooxfooxfoxfooox", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_620() {
        let opts = windows_options();
        assert_is_match("foooxfooxfxfooox", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_621() {
        let opts = windows_options();
        assert_is_match("foooxfooxofoxfooox", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_622() {
        let opts = windows_options();
        assert_is_match("foot", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_623() {
        let opts = windows_options();
        assert_is_match("foox", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_624() {
        let opts = windows_options();
        assert_is_match("ofoofo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_625() {
        let opts = windows_options();
        assert_is_match("ofooofoofofooo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_626() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_627() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_628() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxofo",
            "*(oxf+(ox))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_629() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxoo",
            "*(oxf+(ox))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_630() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "*(oxf+(ox))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_631() {
        let opts = windows_options();
        assert_is_match("ofxoofxo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_632() {
        let opts = windows_options();
        assert_is_match("oofooofo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_633() {
        let opts = windows_options();
        assert_is_match("ooo", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_634() {
        let opts = windows_options();
        assert_is_match("oxfoxfox", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_635() {
        let opts = windows_options();
        assert_is_match("oxfoxoxfox", "*(oxf+(ox))", opts.clone(), true);
    }

    #[test]
    fn test_636() {
        let opts = windows_options();
        assert_is_match("xfoooofof", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_637() {
        let opts = windows_options();
        assert_is_match("ffffffo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_638() {
        let opts = windows_options();
        assert_is_match(
            "fffooofoooooffoofffooofff",
            "@(!(z*)|*x)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_639() {
        let opts = windows_options();
        assert_is_match("ffo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_640() {
        let opts = windows_options();
        assert_is_match("fofo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_641() {
        let opts = windows_options();
        assert_is_match("fofoofoofofoo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_642() {
        let opts = windows_options();
        assert_is_match("foo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_643() {
        let opts = windows_options();
        assert_is_match("foob", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_644() {
        let opts = windows_options();
        assert_is_match("foobb", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_645() {
        let opts = windows_options();
        assert_is_match("foofoofo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_646() {
        let opts = windows_options();
        assert_is_match("fooofoofofooo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_647() {
        let opts = windows_options();
        assert_is_match("foooofo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_648() {
        let opts = windows_options();
        assert_is_match("foooofof", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_649() {
        let opts = windows_options();
        assert_is_match("foooofofx", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_650() {
        let opts = windows_options();
        assert_is_match("foooxfooxfoxfooox", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_651() {
        let opts = windows_options();
        assert_is_match("foooxfooxfxfooox", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_652() {
        let opts = windows_options();
        assert_is_match("foooxfooxofoxfooox", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_653() {
        let opts = windows_options();
        assert_is_match("foot", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_654() {
        let opts = windows_options();
        assert_is_match("foox", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_655() {
        let opts = windows_options();
        assert_is_match("ofoofo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_656() {
        let opts = windows_options();
        assert_is_match("ofooofoofofooo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_657() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_658() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_659() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxofo",
            "@(!(z*)|*x)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_660() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxoo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_661() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "@(!(z*)|*x)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_662() {
        let opts = windows_options();
        assert_is_match("ofxoofxo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_663() {
        let opts = windows_options();
        assert_is_match("oofooofo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_664() {
        let opts = windows_options();
        assert_is_match("ooo", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_665() {
        let opts = windows_options();
        assert_is_match("oxfoxfox", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_666() {
        let opts = windows_options();
        assert_is_match("oxfoxoxfox", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_667() {
        let opts = windows_options();
        assert_is_match("xfoooofof", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_668() {
        let opts = windows_options();
        assert_is_match("ffffffo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_669() {
        let opts = windows_options();
        assert_is_match(
            "fffooofoooooffoofffooofff",
            "@(foo|f|fo)*(f|of+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_670() {
        let opts = windows_options();
        assert_is_match("ffo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_671() {
        let opts = windows_options();
        assert_is_match("fofo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), true);
    }

    #[test]
    fn test_672() {
        let opts = windows_options();
        assert_is_match(
            "fofoofoofofoo",
            "@(foo|f|fo)*(f|of+(o))",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_673() {
        let opts = windows_options();
        assert_is_match("foo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), true);
    }

    #[test]
    fn test_674() {
        let opts = windows_options();
        assert_is_match("foob", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_675() {
        let opts = windows_options();
        assert_is_match("foobb", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_676() {
        let opts = windows_options();
        assert_is_match("foofoofo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), true);
    }

    #[test]
    fn test_677() {
        let opts = windows_options();
        assert_is_match(
            "fooofoofofooo",
            "@(foo|f|fo)*(f|of+(o))",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_678() {
        let opts = windows_options();
        assert_is_match("foooofo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_679() {
        let opts = windows_options();
        assert_is_match("foooofof", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_680() {
        let opts = windows_options();
        assert_is_match("foooofofx", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_681() {
        let opts = windows_options();
        assert_is_match(
            "foooxfooxfoxfooox",
            "@(foo|f|fo)*(f|of+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_682() {
        let opts = windows_options();
        assert_is_match(
            "foooxfooxfxfooox",
            "@(foo|f|fo)*(f|of+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_683() {
        let opts = windows_options();
        assert_is_match(
            "foooxfooxofoxfooox",
            "@(foo|f|fo)*(f|of+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_684() {
        let opts = windows_options();
        assert_is_match("foot", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_685() {
        let opts = windows_options();
        assert_is_match("foox", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_686() {
        let opts = windows_options();
        assert_is_match("ofoofo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_687() {
        let opts = windows_options();
        assert_is_match(
            "ofooofoofofooo",
            "@(foo|f|fo)*(f|of+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_688() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_689() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxo",
            "@(foo|f|fo)*(f|of+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_690() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxofo",
            "@(foo|f|fo)*(f|of+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_691() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxoo",
            "@(foo|f|fo)*(f|of+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_692() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "@(foo|f|fo)*(f|of+(o))",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_693() {
        let opts = windows_options();
        assert_is_match("ofxoofxo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_694() {
        let opts = windows_options();
        assert_is_match("oofooofo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_695() {
        let opts = windows_options();
        assert_is_match("ooo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_696() {
        let opts = windows_options();
        assert_is_match("oxfoxfox", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_697() {
        let opts = windows_options();
        assert_is_match("oxfoxoxfox", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_698() {
        let opts = windows_options();
        assert_is_match("xfoooofof", "@(foo|f|fo)*(f|of+(o))", opts.clone(), false);
    }

    #[test]
    fn test_699() {
        let opts = windows_options();
        assert_is_match("aaac", "*(@(a))a@(c)", opts.clone(), true);
    }

    #[test]
    fn test_700() {
        let opts = windows_options();
        assert_is_match("aac", "*(@(a))a@(c)", opts.clone(), true);
    }

    #[test]
    fn test_701() {
        let opts = windows_options();
        assert_is_match("ac", "*(@(a))a@(c)", opts.clone(), true);
    }

    #[test]
    fn test_702() {
        let opts = windows_options();
        assert_is_match("abbcd", "*(@(a))a@(c)", opts.clone(), false);
    }

    #[test]
    fn test_703() {
        let opts = windows_options();
        assert_is_match("abcd", "*(@(a))a@(c)", opts.clone(), false);
    }

    #[test]
    fn test_704() {
        let opts = windows_options();
        assert_is_match("acd", "*(@(a))a@(c)", opts.clone(), false);
    }

    #[test]
    fn test_705() {
        let opts = windows_options();
        assert_is_match("baaac", "*(@(a))a@(c)", opts.clone(), false);
    }

    #[test]
    fn test_706() {
        let opts = windows_options();
        assert_is_match("c", "*(@(a))a@(c)", opts.clone(), false);
    }

    #[test]
    fn test_707() {
        let opts = windows_options();
        assert_is_match("foo", "*(@(a))a@(c)", opts.clone(), false);
    }

    #[test]
    fn test_708() {
        let opts = windows_options();
        assert_is_match("aaac", "@(ab|a*(b))*(c)d", opts.clone(), false);
    }

    #[test]
    fn test_709() {
        let opts = windows_options();
        assert_is_match("aac", "@(ab|a*(b))*(c)d", opts.clone(), false);
    }

    #[test]
    fn test_710() {
        let opts = windows_options();
        assert_is_match("ac", "@(ab|a*(b))*(c)d", opts.clone(), false);
    }

    #[test]
    fn test_711() {
        let opts = windows_options();
        assert_is_match("abbcd", "@(ab|a*(b))*(c)d", opts.clone(), true);
    }

    #[test]
    fn test_712() {
        let opts = windows_options();
        assert_is_match("abcd", "@(ab|a*(b))*(c)d", opts.clone(), true);
    }

    #[test]
    fn test_713() {
        let opts = windows_options();
        assert_is_match("acd", "@(ab|a*(b))*(c)d", opts.clone(), true);
    }

    #[test]
    fn test_714() {
        let opts = windows_options();
        assert_is_match("baaac", "@(ab|a*(b))*(c)d", opts.clone(), false);
    }

    #[test]
    fn test_715() {
        let opts = windows_options();
        assert_is_match("c", "@(ab|a*(b))*(c)d", opts.clone(), false);
    }

    #[test]
    fn test_716() {
        let opts = windows_options();
        assert_is_match("foo", "@(ab|a*(b))*(c)d", opts.clone(), false);
    }

    #[test]
    fn test_717() {
        let opts = windows_options();
        assert_is_match("aaac", "?@(a|b)*@(c)d", opts.clone(), false);
    }

    #[test]
    fn test_718() {
        let opts = windows_options();
        assert_is_match("aac", "?@(a|b)*@(c)d", opts.clone(), false);
    }

    #[test]
    fn test_719() {
        let opts = windows_options();
        assert_is_match("ac", "?@(a|b)*@(c)d", opts.clone(), false);
    }

    #[test]
    fn test_720() {
        let opts = windows_options();
        assert_is_match("abbcd", "?@(a|b)*@(c)d", opts.clone(), true);
    }

    #[test]
    fn test_721() {
        let opts = windows_options();
        assert_is_match("abcd", "?@(a|b)*@(c)d", opts.clone(), true);
    }

    #[test]
    fn test_722() {
        let opts = windows_options();
        assert_is_match("acd", "?@(a|b)*@(c)d", opts.clone(), false);
    }

    #[test]
    fn test_723() {
        let opts = windows_options();
        assert_is_match("baaac", "?@(a|b)*@(c)d", opts.clone(), false);
    }

    #[test]
    fn test_724() {
        let opts = windows_options();
        assert_is_match("c", "?@(a|b)*@(c)d", opts.clone(), false);
    }

    #[test]
    fn test_725() {
        let opts = windows_options();
        assert_is_match("foo", "?@(a|b)*@(c)d", opts.clone(), false);
    }

    #[test]
    fn test_726() {
        let opts = windows_options();
        assert_is_match("aaac", "@(ab|a*@(b))*(c)d", opts.clone(), false);
    }

    #[test]
    fn test_727() {
        let opts = windows_options();
        assert_is_match("aac", "@(ab|a*@(b))*(c)d", opts.clone(), false);
    }

    #[test]
    fn test_728() {
        let opts = windows_options();
        assert_is_match("ac", "@(ab|a*@(b))*(c)d", opts.clone(), false);
    }

    #[test]
    fn test_729() {
        let opts = windows_options();
        assert_is_match("abbcd", "@(ab|a*@(b))*(c)d", opts.clone(), true);
    }

    #[test]
    fn test_730() {
        let opts = windows_options();
        assert_is_match("abcd", "@(ab|a*@(b))*(c)d", opts.clone(), true);
    }

    #[test]
    fn test_731() {
        let opts = windows_options();
        assert_is_match("acd", "@(ab|a*@(b))*(c)d", opts.clone(), false);
    }

    #[test]
    fn test_732() {
        let opts = windows_options();
        assert_is_match("baaac", "@(ab|a*@(b))*(c)d", opts.clone(), false);
    }

    #[test]
    fn test_733() {
        let opts = windows_options();
        assert_is_match("c", "@(ab|a*@(b))*(c)d", opts.clone(), false);
    }

    #[test]
    fn test_734() {
        let opts = windows_options();
        assert_is_match("foo", "@(ab|a*@(b))*(c)d", opts.clone(), false);
    }

    #[test]
    fn test_735() {
        let opts = windows_options();
        assert_is_match("aac", "*(@(a))b@(c)", opts.clone(), false);
    }

    #[test]
    fn test_736() {
        let opts = windows_options();
        assert_is_match("fofoofoofofoo", "*(fo|foo)", opts.clone(), true);
    }

    #[test]
    fn test_737() {
        let opts = windows_options();
        assert_is_match("ffffffo", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_738() {
        let opts = windows_options();
        assert_is_match(
            "fffooofoooooffoofffooofff",
            "*(fo|foo)",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_739() {
        let opts = windows_options();
        assert_is_match("ffo", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_740() {
        let opts = windows_options();
        assert_is_match("fofo", "*(fo|foo)", opts.clone(), true);
    }

    #[test]
    fn test_741() {
        let opts = windows_options();
        assert_is_match("fofoofoofofoo", "*(fo|foo)", opts.clone(), true);
    }

    #[test]
    fn test_742() {
        let opts = windows_options();
        assert_is_match("foo", "*(fo|foo)", opts.clone(), true);
    }

    #[test]
    fn test_743() {
        let opts = windows_options();
        assert_is_match("foob", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_744() {
        let opts = windows_options();
        assert_is_match("foobb", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_745() {
        let opts = windows_options();
        assert_is_match("foofoofo", "*(fo|foo)", opts.clone(), true);
    }

    #[test]
    fn test_746() {
        let opts = windows_options();
        assert_is_match("fooofoofofooo", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_747() {
        let opts = windows_options();
        assert_is_match("foooofo", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_748() {
        let opts = windows_options();
        assert_is_match("foooofof", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_749() {
        let opts = windows_options();
        assert_is_match("foooofofx", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_750() {
        let opts = windows_options();
        assert_is_match("foooxfooxfoxfooox", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_751() {
        let opts = windows_options();
        assert_is_match("foooxfooxfxfooox", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_752() {
        let opts = windows_options();
        assert_is_match("foooxfooxofoxfooox", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_753() {
        let opts = windows_options();
        assert_is_match("foot", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_754() {
        let opts = windows_options();
        assert_is_match("foox", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_755() {
        let opts = windows_options();
        assert_is_match("ofoofo", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_756() {
        let opts = windows_options();
        assert_is_match("ofooofoofofooo", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_757() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxo", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_758() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxo", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_759() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxofo", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_760() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxoofoooxoofxoo", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_761() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "*(fo|foo)",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_762() {
        let opts = windows_options();
        assert_is_match("ofxoofxo", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_763() {
        let opts = windows_options();
        assert_is_match("oofooofo", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_764() {
        let opts = windows_options();
        assert_is_match("ooo", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_765() {
        let opts = windows_options();
        assert_is_match("oxfoxfox", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_766() {
        let opts = windows_options();
        assert_is_match("oxfoxoxfox", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_767() {
        let opts = windows_options();
        assert_is_match("xfoooofof", "*(fo|foo)", opts.clone(), false);
    }

    #[test]
    fn test_768() {
        let opts = windows_options();
        assert_is_match("foob", "!(foo)b*", opts.clone(), false);
    }

    #[test]
    fn test_769() {
        let opts = windows_options();
        assert_is_match("foobb", "!(foo)b*", opts.clone(), false);
    }

    #[test]
    fn test_770() {
        let opts = windows_options();
        assert_is_match("foo", "!(foo)b*", opts.clone(), false);
    }

    #[test]
    fn test_771() {
        let opts = windows_options();
        assert_is_match("bar", "!(foo)b*", opts.clone(), true);
    }

    #[test]
    fn test_772() {
        let opts = windows_options();
        assert_is_match("baz", "!(foo)b*", opts.clone(), true);
    }

    #[test]
    fn test_773() {
        let opts = windows_options();
        assert_is_match("foobar", "!(foo)b*", opts.clone(), false);
    }

    #[test]
    fn test_774() {
        let opts = windows_options();
        assert_is_match("foo", "*(!(foo))", opts.clone(), false);
    }

    #[test]
    fn test_775() {
        let opts = windows_options();
        assert_is_match("bar", "*(!(foo))", opts.clone(), true);
    }

    #[test]
    fn test_776() {
        let opts = windows_options();
        assert_is_match("baz", "*(!(foo))", opts.clone(), true);
    }

    #[test]
    fn test_777() {
        let opts = windows_options();
        assert_is_match("foobar", "*(!(foo))", opts.clone(), true);
    }

    #[test]
    fn test_778() {
        let opts = windows_options();
        assert_is_match("foo", "!(foo)*", opts.clone(), false);
    }

    #[test]
    fn test_779() {
        let opts = windows_options();
        assert_is_match("foobar", "!(foo)*", opts.clone(), false);
    }

    #[test]
    fn test_780() {
        let opts = windows_options();
        assert_is_match("bar", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_781() {
        let opts = windows_options();
        assert_is_match("baz", "!(foo)*", opts.clone(), true);
    }

    #[test]
    fn test_782() {
        let opts = windows_options();
        assert_is_match("moo.cow", "!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_783() {
        let opts = windows_options();
        assert_is_match("moo", "!(*.*)", opts.clone(), true);
    }

    #[test]
    fn test_784() {
        let opts = windows_options();
        assert_is_match("cow", "!(*.*)", opts.clone(), true);
    }

    #[test]
    fn test_785() {
        let opts = windows_options();
        assert_is_match("moo.cow", "!(a*).!(b*)", opts.clone(), true);
    }

    #[test]
    fn test_786() {
        let opts = windows_options();
        assert_is_match("moo.cow", "!(*).!(*)", opts.clone(), false);
    }

    #[test]
    fn test_787() {
        let opts = windows_options();
        assert_is_match("moo.cow.moo.cow", "!(*.*).!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_788() {
        let opts = windows_options();
        assert_is_match("mad.moo.cow", "!(*.*).!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_789() {
        let opts = windows_options();
        assert_is_match("moo.cow", "!(*.*).", opts.clone(), false);
    }

    #[test]
    fn test_790() {
        let opts = windows_options();
        assert_is_match("moo", "!(*.*).", opts.clone(), false);
    }

    #[test]
    fn test_791() {
        let opts = windows_options();
        assert_is_match("cow", "!(*.*).", opts.clone(), false);
    }

    #[test]
    fn test_792() {
        let opts = windows_options();
        assert_is_match("moo.cow", ".!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_793() {
        let opts = windows_options();
        assert_is_match("moo", ".!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_794() {
        let opts = windows_options();
        assert_is_match("cow", ".!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_795() {
        let opts = windows_options();
        assert_is_match("mucca.pazza", "mu!(*(c))?.pa!(*(z))?", opts.clone(), false);
    }

    #[test]
    fn test_796() {
        let opts = windows_options();
        assert_is_match("effgz", "@(b+(c)d|e*(f)g?|?(h)i@(j|k))", opts.clone(), true);
    }

    #[test]
    fn test_797() {
        let opts = windows_options();
        assert_is_match("efgz", "@(b+(c)d|e*(f)g?|?(h)i@(j|k))", opts.clone(), true);
    }

    #[test]
    fn test_798() {
        let opts = windows_options();
        assert_is_match("egz", "@(b+(c)d|e*(f)g?|?(h)i@(j|k))", opts.clone(), true);
    }

    #[test]
    fn test_799() {
        let opts = windows_options();
        assert_is_match("egz", "@(b+(c)d|e+(f)g?|?(h)i@(j|k))", opts.clone(), false);
    }

    #[test]
    fn test_800() {
        let opts = windows_options();
        assert_is_match(
            "egzefffgzbcdij",
            "*(b+(c)d|e*(f)g?|?(h)i@(j|k))",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_801() {
        let opts = windows_options();
        assert_is_match(
            "/dev/udp/129.22.8.102/45",
            "/dev/@(tcp|udp)/*/*",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_802() {
        let opts = windows_options();
        assert_is_match("0", "[1-6]([0-9])", opts.clone(), false);
    }

    #[test]
    fn test_803() {
        let opts = windows_options();
        assert_is_match("12", "[1-6]([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_804() {
        let opts = windows_options();
        assert_is_match("1", "[1-6]([0-9])", opts.clone(), false);
    }

    #[test]
    fn test_805() {
        let opts = windows_options();
        assert_is_match("12abc", "[1-6]([0-9])", opts.clone(), false);
    }

    #[test]
    fn test_806() {
        let opts = windows_options();
        assert_is_match("555", "[1-6]([0-9])", opts.clone(), false);
    }

    #[test]
    fn test_807() {
        let opts = windows_options();
        assert_is_match("0", "[1-6]*([0-9])", opts.clone(), false);
    }

    #[test]
    fn test_808() {
        let opts = windows_options();
        assert_is_match("12", "[1-6]*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_809() {
        let opts = windows_options();
        assert_is_match("1", "[1-6]*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_810() {
        let opts = windows_options();
        assert_is_match("12abc", "[1-6]*([0-9])", opts.clone(), false);
    }

    #[test]
    fn test_811() {
        let opts = windows_options();
        assert_is_match("555", "[1-6]*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_812() {
        let opts = windows_options();
        assert_is_match("0", "[1-5]*([6-9])", opts.clone(), false);
    }

    #[test]
    fn test_813() {
        let opts = windows_options();
        assert_is_match("12", "[1-5]*([6-9])", opts.clone(), false);
    }

    #[test]
    fn test_814() {
        let opts = windows_options();
        assert_is_match("1", "[1-5]*([6-9])", opts.clone(), true);
    }

    #[test]
    fn test_815() {
        let opts = windows_options();
        assert_is_match("12abc", "[1-5]*([6-9])", opts.clone(), false);
    }

    #[test]
    fn test_816() {
        let opts = windows_options();
        assert_is_match("555", "[1-5]*([6-9])", opts.clone(), false);
    }

    #[test]
    fn test_817() {
        let opts = windows_options();
        assert_is_match("0", "0|[1-6]*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_818() {
        let opts = windows_options();
        assert_is_match("12", "0|[1-6]*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_819() {
        let opts = windows_options();
        assert_is_match("1", "0|[1-6]*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_820() {
        let opts = windows_options();
        assert_is_match("12abc", "0|[1-6]*([0-9])", opts.clone(), false);
    }

    #[test]
    fn test_821() {
        let opts = windows_options();
        assert_is_match("555", "0|[1-6]*([0-9])", opts.clone(), true);
    }

    #[test]
    fn test_822() {
        let opts = windows_options();
        assert_is_match("07", "+([0-7])", opts.clone(), true);
    }

    #[test]
    fn test_823() {
        let opts = windows_options();
        assert_is_match("0377", "+([0-7])", opts.clone(), true);
    }

    #[test]
    fn test_824() {
        let opts = windows_options();
        assert_is_match("09", "+([0-7])", opts.clone(), false);
    }

    #[test]
    fn test_825() {
        let opts = windows_options();
        assert_is_match("a", "+(a|abc)", opts.clone(), true);
    }

    #[test]
    fn test_826() {
        let opts = windows_options();
        assert_is_match("abc", "+(a|abc)", opts.clone(), true);
    }

    #[test]
    fn test_827() {
        let opts = windows_options();
        assert_is_match("abcd", "+(a|abc)", opts.clone(), false);
    }

    #[test]
    fn test_828() {
        let opts = windows_options();
        assert_is_match("abcde", "+(a|abc)", opts.clone(), false);
    }

    #[test]
    fn test_829() {
        let opts = windows_options();
        assert_is_match("abcedf", "+(a|abc)", opts.clone(), false);
    }

    #[test]
    fn test_830() {
        let opts = windows_options();
        assert_is_match("f", "+(def|f)", opts.clone(), true);
    }

    #[test]
    fn test_831() {
        let opts = windows_options();
        assert_is_match("def", "+(f|def)", opts.clone(), true);
    }

    #[test]
    fn test_832() {
        let opts = windows_options();
        assert_is_match("cdef", "+(f|def)", opts.clone(), false);
    }

    #[test]
    fn test_833() {
        let opts = windows_options();
        assert_is_match("bcdef", "+(f|def)", opts.clone(), false);
    }

    #[test]
    fn test_834() {
        let opts = windows_options();
        assert_is_match("abcedf", "+(f|def)", opts.clone(), false);
    }

    #[test]
    fn test_835() {
        let opts = windows_options();
        assert_is_match("abcd", "*(a|b)cd", opts.clone(), true);
    }

    #[test]
    fn test_836() {
        let opts = windows_options();
        assert_is_match("a", "*(a|b)cd", opts.clone(), false);
    }

    #[test]
    fn test_837() {
        let opts = windows_options();
        assert_is_match("ab", "*(a|b)cd", opts.clone(), false);
    }

    #[test]
    fn test_838() {
        let opts = windows_options();
        assert_is_match("abc", "*(a|b)cd", opts.clone(), false);
    }

    #[test]
    fn test_839() {
        let opts = windows_options();
        assert_is_match("a", "\"*(a|b)cd\"", opts.clone(), false);
    }

    #[test]
    fn test_840() {
        let opts = windows_options();
        assert_is_match("ab", "\"*(a|b)cd\"", opts.clone(), false);
    }

    #[test]
    fn test_841() {
        let opts = windows_options();
        assert_is_match("abc", "\"*(a|b)cd\"", opts.clone(), false);
    }

    #[test]
    fn test_842() {
        let opts = windows_options();
        assert_is_match("abcde", "\"*(a|b)cd\"", opts.clone(), false);
    }

    #[test]
    fn test_843() {
        let opts = windows_options();
        assert_is_match("abcdef", "\"*(a|b)cd\"", opts.clone(), false);
    }

    #[test]
    fn test_844() {
        let opts = CompileOptions {
            bash: true,
            windows: true,
            ..CompileOptions::default()
        };
        assert_is_match(
            "/dev/udp/129.22.8.102/45",
            "/dev\\/@(tcp|udp)\\/*\\/*",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_845() {
        let opts = windows_options();
        assert_is_match("123abc", "(a+|b)*", opts.clone(), false);
    }

    #[test]
    fn test_846() {
        let opts = windows_options();
        assert_is_match("ab", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_847() {
        let opts = windows_options();
        assert_is_match("abab", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_848() {
        let opts = windows_options();
        assert_is_match("abcdef", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_849() {
        let opts = windows_options();
        assert_is_match("accdef", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_850() {
        let opts = windows_options();
        assert_is_match("abcfefg", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_851() {
        let opts = windows_options();
        assert_is_match("abef", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_852() {
        let opts = windows_options();
        assert_is_match("abcfef", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_853() {
        let opts = windows_options();
        assert_is_match("abd", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_854() {
        let opts = windows_options();
        assert_is_match("acd", "(a+|b)*", opts.clone(), true);
    }

    #[test]
    fn test_855() {
        let opts = windows_options();
        assert_is_match("123abc", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_856() {
        let opts = windows_options();
        assert_is_match("ab", "(a+|b)+", opts.clone(), true);
    }

    #[test]
    fn test_857() {
        let opts = windows_options();
        assert_is_match("abab", "(a+|b)+", opts.clone(), true);
    }

    #[test]
    fn test_858() {
        let opts = windows_options();
        assert_is_match("abcdef", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_859() {
        let opts = windows_options();
        assert_is_match("accdef", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_860() {
        let opts = windows_options();
        assert_is_match("abcfefg", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_861() {
        let opts = windows_options();
        assert_is_match("abef", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_862() {
        let opts = windows_options();
        assert_is_match("abcfef", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_863() {
        let opts = windows_options();
        assert_is_match("abd", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_864() {
        let opts = windows_options();
        assert_is_match("acd", "(a+|b)+", opts.clone(), false);
    }

    #[test]
    fn test_865() {
        let opts = windows_options();
        assert_is_match("123abc", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_866() {
        let opts = windows_options();
        assert_is_match("ab", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_867() {
        let opts = windows_options();
        assert_is_match("abab", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_868() {
        let opts = windows_options();
        assert_is_match("abcdef", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_869() {
        let opts = windows_options();
        assert_is_match("accdef", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_870() {
        let opts = windows_options();
        assert_is_match("abcfefg", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_871() {
        let opts = windows_options();
        assert_is_match("abef", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_872() {
        let opts = windows_options();
        assert_is_match("abcfef", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_873() {
        let opts = windows_options();
        assert_is_match("abd", "a(b*(foo|bar))d", opts.clone(), true);
    }

    #[test]
    fn test_874() {
        let opts = windows_options();
        assert_is_match("acd", "a(b*(foo|bar))d", opts.clone(), false);
    }

    #[test]
    fn test_875() {
        let opts = windows_options();
        assert_is_match("123abc", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_876() {
        let opts = windows_options();
        assert_is_match("ab", "ab*(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_877() {
        let opts = windows_options();
        assert_is_match("abab", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_878() {
        let opts = windows_options();
        assert_is_match("abcdef", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_879() {
        let opts = windows_options();
        assert_is_match("accdef", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_880() {
        let opts = windows_options();
        assert_is_match("abcfefg", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_881() {
        let opts = windows_options();
        assert_is_match("abef", "ab*(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_882() {
        let opts = windows_options();
        assert_is_match("abcfef", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_883() {
        let opts = windows_options();
        assert_is_match("abd", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_884() {
        let opts = windows_options();
        assert_is_match("acd", "ab*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_885() {
        let opts = windows_options();
        assert_is_match("123abc", "ab**(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_886() {
        let opts = windows_options();
        assert_is_match("ab", "ab**(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_887() {
        let opts = windows_options();
        assert_is_match("abab", "ab**(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_888() {
        let opts = windows_options();
        assert_is_match("abcdef", "ab**(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_889() {
        let opts = windows_options();
        assert_is_match("accdef", "ab**(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_890() {
        let opts = windows_options();
        assert_is_match("abcfefg", "ab**(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_891() {
        let opts = windows_options();
        assert_is_match("abef", "ab**(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_892() {
        let opts = windows_options();
        assert_is_match("abcfef", "ab**(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_893() {
        let opts = windows_options();
        assert_is_match("abd", "ab**(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_894() {
        let opts = windows_options();
        assert_is_match("acd", "ab**(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_895() {
        let opts = windows_options();
        assert_is_match("123abc", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_896() {
        let opts = windows_options();
        assert_is_match("ab", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_897() {
        let opts = windows_options();
        assert_is_match("abab", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_898() {
        let opts = windows_options();
        assert_is_match("abcdef", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_899() {
        let opts = windows_options();
        assert_is_match("accdef", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_900() {
        let opts = windows_options();
        assert_is_match("abcfefg", "ab**(e|f)g", opts.clone(), true);
    }

    #[test]
    fn test_901() {
        let opts = windows_options();
        assert_is_match("abef", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_902() {
        let opts = windows_options();
        assert_is_match("abcfef", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_903() {
        let opts = windows_options();
        assert_is_match("abd", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_904() {
        let opts = windows_options();
        assert_is_match("acd", "ab**(e|f)g", opts.clone(), false);
    }

    #[test]
    fn test_905() {
        let opts = windows_options();
        assert_is_match("123abc", "ab***ef", opts.clone(), false);
    }

    #[test]
    fn test_906() {
        let opts = windows_options();
        assert_is_match("ab", "ab***ef", opts.clone(), false);
    }

    #[test]
    fn test_907() {
        let opts = windows_options();
        assert_is_match("abab", "ab***ef", opts.clone(), false);
    }

    #[test]
    fn test_908() {
        let opts = windows_options();
        assert_is_match("abcdef", "ab***ef", opts.clone(), true);
    }

    #[test]
    fn test_909() {
        let opts = windows_options();
        assert_is_match("accdef", "ab***ef", opts.clone(), false);
    }

    #[test]
    fn test_910() {
        let opts = windows_options();
        assert_is_match("abcfefg", "ab***ef", opts.clone(), false);
    }

    #[test]
    fn test_911() {
        let opts = windows_options();
        assert_is_match("abef", "ab***ef", opts.clone(), true);
    }

    #[test]
    fn test_912() {
        let opts = windows_options();
        assert_is_match("abcfef", "ab***ef", opts.clone(), true);
    }

    #[test]
    fn test_913() {
        let opts = windows_options();
        assert_is_match("abd", "ab***ef", opts.clone(), false);
    }

    #[test]
    fn test_914() {
        let opts = windows_options();
        assert_is_match("acd", "ab***ef", opts.clone(), false);
    }

    #[test]
    fn test_915() {
        let opts = windows_options();
        assert_is_match("123abc", "ab*+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_916() {
        let opts = windows_options();
        assert_is_match("ab", "ab*+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_917() {
        let opts = windows_options();
        assert_is_match("abab", "ab*+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_918() {
        let opts = windows_options();
        assert_is_match("abcdef", "ab*+(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_919() {
        let opts = windows_options();
        assert_is_match("accdef", "ab*+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_920() {
        let opts = windows_options();
        assert_is_match("abcfefg", "ab*+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_921() {
        let opts = windows_options();
        assert_is_match("abef", "ab*+(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_922() {
        let opts = windows_options();
        assert_is_match("abcfef", "ab*+(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_923() {
        let opts = windows_options();
        assert_is_match("abd", "ab*+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_924() {
        let opts = windows_options();
        assert_is_match("acd", "ab*+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_925() {
        let opts = windows_options();
        assert_is_match("123abc", "ab*d*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_926() {
        let opts = windows_options();
        assert_is_match("ab", "ab*d*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_927() {
        let opts = windows_options();
        assert_is_match("abab", "ab*d*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_928() {
        let opts = windows_options();
        assert_is_match("abcdef", "ab*d*(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_929() {
        let opts = windows_options();
        assert_is_match("accdef", "ab*d*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_930() {
        let opts = windows_options();
        assert_is_match("abcfefg", "ab*d*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_931() {
        let opts = windows_options();
        assert_is_match("abef", "ab*d*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_932() {
        let opts = windows_options();
        assert_is_match("abcfef", "ab*d*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_933() {
        let opts = windows_options();
        assert_is_match("abd", "ab*d*(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_934() {
        let opts = windows_options();
        assert_is_match("acd", "ab*d*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_935() {
        let opts = windows_options();
        assert_is_match("123abc", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_936() {
        let opts = windows_options();
        assert_is_match("ab", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_937() {
        let opts = windows_options();
        assert_is_match("abab", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_938() {
        let opts = windows_options();
        assert_is_match("abcdef", "ab*d+(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_939() {
        let opts = windows_options();
        assert_is_match("accdef", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_940() {
        let opts = windows_options();
        assert_is_match("abcfefg", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_941() {
        let opts = windows_options();
        assert_is_match("abef", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_942() {
        let opts = windows_options();
        assert_is_match("abcfef", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_943() {
        let opts = windows_options();
        assert_is_match("abd", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_944() {
        let opts = windows_options();
        assert_is_match("acd", "ab*d+(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_945() {
        let opts = windows_options();
        assert_is_match("123abc", "ab?*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_946() {
        let opts = windows_options();
        assert_is_match("ab", "ab?*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_947() {
        let opts = windows_options();
        assert_is_match("abab", "ab?*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_948() {
        let opts = windows_options();
        assert_is_match("abcdef", "ab?*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_949() {
        let opts = windows_options();
        assert_is_match("accdef", "ab?*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_950() {
        let opts = windows_options();
        assert_is_match("abcfefg", "ab?*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_951() {
        let opts = windows_options();
        assert_is_match("abef", "ab?*(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_952() {
        let opts = windows_options();
        assert_is_match("abcfef", "ab?*(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_953() {
        let opts = windows_options();
        assert_is_match("abd", "ab?*(e|f)", opts.clone(), true);
    }

    #[test]
    fn test_954() {
        let opts = windows_options();
        assert_is_match("acd", "ab?*(e|f)", opts.clone(), false);
    }

    #[test]
    fn test_955() {
        let opts = windows_options();
        assert_is_match("123abc", "*?(a)bc", opts.clone(), true);
    }

    #[test]
    fn test_956() {
        let opts = windows_options();
        assert_is_match("a.b", "a[-.,:\\\\;\\\\ _]b", opts.clone(), true);
    }

    #[test]
    fn test_957() {
        let opts = windows_options();
        assert_is_match("a,b", "a[-.,:\\\\;\\\\ _]b", opts.clone(), true);
    }

    #[test]
    fn test_958() {
        let opts = windows_options();
        assert_is_match("a:b", "a[-.,:\\\\;\\\\ _]b", opts.clone(), true);
    }

    #[test]
    fn test_959() {
        let opts = windows_options();
        assert_is_match("a-b", "a[-.,:\\\\;\\\\ _]b", opts.clone(), true);
    }

    #[test]
    fn test_960() {
        let opts = windows_options();
        assert_is_match("a;b", "a[-.,:\\\\;\\\\ _]b", opts.clone(), true);
    }

    #[test]
    fn test_961() {
        let opts = windows_options();
        assert_is_match("a b", "a[-.,:\\\\;\\\\ _]b", opts.clone(), true);
    }

    #[test]
    fn test_962() {
        let opts = windows_options();
        assert_is_match("a_b", "a[-.,:\\\\;\\\\ _]b", opts.clone(), true);
    }

    #[test]
    fn test_963() {
        let opts = windows_options();
        assert_is_match("a.b", "a@([-.,:; _])b", opts.clone(), true);
    }

    #[test]
    fn test_964() {
        let opts = windows_options();
        assert_is_match("a,b", "a@([-.,:; _])b", opts.clone(), true);
    }

    #[test]
    fn test_965() {
        let opts = windows_options();
        assert_is_match("a:b", "a@([-.,:; _])b", opts.clone(), true);
    }

    #[test]
    fn test_966() {
        let opts = windows_options();
        assert_is_match("a-b", "a@([-.,:; _])b", opts.clone(), true);
    }

    #[test]
    fn test_967() {
        let opts = windows_options();
        assert_is_match("a;b", "a@([-.,:; _])b", opts.clone(), true);
    }

    #[test]
    fn test_968() {
        let opts = windows_options();
        assert_is_match("a b", "a@([-.,:; _])b", opts.clone(), true);
    }

    #[test]
    fn test_969() {
        let opts = windows_options();
        assert_is_match("a_b", "a@([-.,:; _])b", opts.clone(), true);
    }

    #[test]
    fn test_970() {
        let opts = windows_options();
        assert_is_match("a.b", "a@([.])b", opts.clone(), true);
    }

    #[test]
    fn test_971() {
        let opts = windows_options();
        assert_is_match("a,b", "a@([.])b", opts.clone(), false);
    }

    #[test]
    fn test_972() {
        let opts = windows_options();
        assert_is_match("a:b", "a@([.])b", opts.clone(), false);
    }

    #[test]
    fn test_973() {
        let opts = windows_options();
        assert_is_match("a-b", "a@([.])b", opts.clone(), false);
    }

    #[test]
    fn test_974() {
        let opts = windows_options();
        assert_is_match("a;b", "a@([.])b", opts.clone(), false);
    }

    #[test]
    fn test_975() {
        let opts = windows_options();
        assert_is_match("a b", "a@([.])b", opts.clone(), false);
    }

    #[test]
    fn test_976() {
        let opts = windows_options();
        assert_is_match("a_b", "a@([.])b", opts.clone(), false);
    }

    #[test]
    fn test_977() {
        let opts = windows_options();
        assert_is_match("a.b", "a@([^.])b", opts.clone(), false);
    }

    #[test]
    fn test_978() {
        let opts = windows_options();
        assert_is_match("a,b", "a@([^.])b", opts.clone(), true);
    }

    #[test]
    fn test_979() {
        let opts = windows_options();
        assert_is_match("a:b", "a@([^.])b", opts.clone(), true);
    }

    #[test]
    fn test_980() {
        let opts = windows_options();
        assert_is_match("a-b", "a@([^.])b", opts.clone(), true);
    }

    #[test]
    fn test_981() {
        let opts = windows_options();
        assert_is_match("a;b", "a@([^.])b", opts.clone(), true);
    }

    #[test]
    fn test_982() {
        let opts = windows_options();
        assert_is_match("a b", "a@([^.])b", opts.clone(), true);
    }

    #[test]
    fn test_983() {
        let opts = windows_options();
        assert_is_match("a_b", "a@([^.])b", opts.clone(), true);
    }

    #[test]
    fn test_984() {
        let opts = windows_options();
        assert_is_match("a.b", "a@([^x])b", opts.clone(), true);
    }

    #[test]
    fn test_985() {
        let opts = windows_options();
        assert_is_match("a,b", "a@([^x])b", opts.clone(), true);
    }

    #[test]
    fn test_986() {
        let opts = windows_options();
        assert_is_match("a:b", "a@([^x])b", opts.clone(), true);
    }

    #[test]
    fn test_987() {
        let opts = windows_options();
        assert_is_match("a-b", "a@([^x])b", opts.clone(), true);
    }

    #[test]
    fn test_988() {
        let opts = windows_options();
        assert_is_match("a;b", "a@([^x])b", opts.clone(), true);
    }

    #[test]
    fn test_989() {
        let opts = windows_options();
        assert_is_match("a b", "a@([^x])b", opts.clone(), true);
    }

    #[test]
    fn test_990() {
        let opts = windows_options();
        assert_is_match("a_b", "a@([^x])b", opts.clone(), true);
    }

    #[test]
    fn test_991() {
        let opts = windows_options();
        assert_is_match("baaac", "*(@(a))a@(c)", opts.clone(), false);
    }

    #[test]
    fn test_992() {
        let opts = windows_options();
        assert_is_match("c", "*(@(a))a@(c)", opts.clone(), false);
    }

    #[test]
    fn test_993() {
        let opts = windows_options();
        assert_is_match("egz", "@(b+(c)d|e+(f)g?|?(h)i@(j|k))", opts.clone(), false);
    }

    #[test]
    fn test_994() {
        let opts = windows_options();
        assert_is_match("foooofof", "*(f+(o))", opts.clone(), false);
    }

    #[test]
    fn test_995() {
        let opts = windows_options();
        assert_is_match("foooofofx", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_996() {
        let opts = windows_options();
        assert_is_match("foooxfooxofoxfooox", "*(f*(o)x)", opts.clone(), false);
    }

    #[test]
    fn test_997() {
        let opts = windows_options();
        assert_is_match("ofooofoofofooo", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_998() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxofo",
            "*(*(of*(o)x)o)",
            opts.clone(),
            false,
        );
    }

    #[test]
    fn test_999() {
        let opts = windows_options();
        assert_is_match("oxfoxfox", "*(oxf+(ox))", opts.clone(), false);
    }

    #[test]
    fn test_1000() {
        let opts = windows_options();
        assert_is_match("xfoooofof", "*(f*(o))", opts.clone(), false);
    }

    #[test]
    fn test_1001() {
        let opts = windows_options();
        assert_is_match("aaac", "*(@(a))a@(c)", opts.clone(), true);
    }

    #[test]
    fn test_1002() {
        let opts = windows_options();
        assert_is_match("aac", "*(@(a))a@(c)", opts.clone(), true);
    }

    #[test]
    fn test_1003() {
        let opts = windows_options();
        assert_is_match("abbcd", "@(ab|a*(b))*(c)d", opts.clone(), true);
    }

    #[test]
    fn test_1004() {
        let opts = windows_options();
        assert_is_match("abcd", "?@(a|b)*@(c)d", opts.clone(), true);
    }

    #[test]
    fn test_1005() {
        let opts = windows_options();
        assert_is_match("abcd", "@(ab|a*@(b))*(c)d", opts.clone(), true);
    }

    #[test]
    fn test_1006() {
        let opts = windows_options();
        assert_is_match("ac", "*(@(a))a@(c)", opts.clone(), true);
    }

    #[test]
    fn test_1007() {
        let opts = windows_options();
        assert_is_match("acd", "@(ab|a*(b))*(c)d", opts.clone(), true);
    }

    #[test]
    fn test_1008() {
        let opts = windows_options();
        assert_is_match("effgz", "@(b+(c)d|e*(f)g?|?(h)i@(j|k))", opts.clone(), true);
    }

    #[test]
    fn test_1009() {
        let opts = windows_options();
        assert_is_match("efgz", "@(b+(c)d|e*(f)g?|?(h)i@(j|k))", opts.clone(), true);
    }

    #[test]
    fn test_1010() {
        let opts = windows_options();
        assert_is_match("egz", "@(b+(c)d|e*(f)g?|?(h)i@(j|k))", opts.clone(), true);
    }

    #[test]
    fn test_1011() {
        let opts = windows_options();
        assert_is_match(
            "egzefffgzbcdij",
            "*(b+(c)d|e*(f)g?|?(h)i@(j|k))",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_1012() {
        let opts = windows_options();
        assert_is_match(
            "fffooofoooooffoofffooofff",
            "*(*(f)*(o))",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_1013() {
        let opts = windows_options();
        assert_is_match("ffo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_1014() {
        let opts = windows_options();
        assert_is_match("fofo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_1015() {
        let opts = windows_options();
        assert_is_match("foofoofo", "@(foo|f|fo)*(f|of+(o))", opts.clone(), true);
    }

    #[test]
    fn test_1016() {
        let opts = windows_options();
        assert_is_match("fooofoofofooo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_1017() {
        let opts = windows_options();
        assert_is_match("foooofo", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_1018() {
        let opts = windows_options();
        assert_is_match("foooofof", "*(f*(o))", opts.clone(), true);
    }

    #[test]
    fn test_1019() {
        let opts = windows_options();
        assert_is_match("foooxfooxfoxfooox", "*(f*(o)x)", opts.clone(), true);
    }

    #[test]
    fn test_1020() {
        let opts = windows_options();
        assert_is_match("foooxfooxfxfooox", "*(f*(o)x)", opts.clone(), true);
    }

    #[test]
    fn test_1021() {
        let opts = windows_options();
        assert_is_match("ofoofo", "*(of+(o))", opts.clone(), true);
    }

    #[test]
    fn test_1022() {
        let opts = windows_options();
        assert_is_match("ofoofo", "*(of+(o)|f)", opts.clone(), true);
    }

    #[test]
    fn test_1023() {
        let opts = windows_options();
        assert_is_match("ofoooxoofxo", "*(*(of*(o)x)o)", opts.clone(), true);
    }

    #[test]
    fn test_1024() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxo",
            "*(*(of*(o)x)o)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_1025() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxoo",
            "*(*(of*(o)x)o)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_1026() {
        let opts = windows_options();
        assert_is_match(
            "ofoooxoofxoofoooxoofxooofxofxo",
            "*(*(of*(o)x)o)",
            opts.clone(),
            true,
        );
    }

    #[test]
    fn test_1027() {
        let opts = windows_options();
        assert_is_match("ofxoofxo", "*(*(of*(o)x)o)", opts.clone(), true);
    }

    #[test]
    fn test_1028() {
        let opts = windows_options();
        assert_is_match("oofooofo", "*(of|oof+(o))", opts.clone(), true);
    }

    #[test]
    fn test_1029() {
        let opts = windows_options();
        assert_is_match("oxfoxoxfox", "*(oxf+(ox))", opts.clone(), true);
    }

    #[test]
    fn test_1030() {
        let opts = windows_options();
        assert_is_match("f", "!(f)", opts.clone(), false);
    }

    #[test]
    fn test_1031() {
        let opts = windows_options();
        assert_is_match("f", "*(!(f))", opts.clone(), false);
    }

    #[test]
    fn test_1032() {
        let opts = windows_options();
        assert_is_match("f", "+(!(f))", opts.clone(), false);
    }

    #[test]
    fn test_1033() {
        let opts = windows_options();
        assert_is_match("foo", "!(foo)", opts.clone(), false);
    }

    #[test]
    fn test_1034() {
        let opts = windows_options();
        assert_is_match("foob", "!(foo)b*", opts.clone(), false);
    }

    #[test]
    fn test_1035() {
        let opts = windows_options();
        assert_is_match("mad.moo.cow", "!(*.*).!(*.*)", opts.clone(), false);
    }

    #[test]
    fn test_1036() {
        let opts = windows_options();
        assert_is_match("mucca.pazza", "mu!(*(c))?.pa!(*(z))?", opts.clone(), false);
    }

    #[test]
    fn test_1037() {
        let opts = windows_options();
        assert_is_match("zoot", "@(!(z*)|*x)", opts.clone(), false);
    }

    #[test]
    fn test_1038() {
        let opts = windows_options();
        assert_is_match("fff", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_1039() {
        let opts = windows_options();
        assert_is_match("fff", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_1040() {
        let opts = windows_options();
        assert_is_match("fff", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_1041() {
        let opts = windows_options();
        assert_is_match("foo", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_1042() {
        let opts = windows_options();
        assert_is_match("foo", "!(x)", opts.clone(), true);
    }

    #[test]
    fn test_1043() {
        let opts = windows_options();
        assert_is_match("foo", "!(x)*", opts.clone(), true);
    }

    #[test]
    fn test_1044() {
        let opts = windows_options();
        assert_is_match("foo", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_1045() {
        let opts = windows_options();
        assert_is_match("foo", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_1046() {
        let opts = windows_options();
        assert_is_match("foobar", "!(foo)", opts.clone(), true);
    }

    #[test]
    fn test_1047() {
        let opts = windows_options();
        assert_is_match("foot", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_1048() {
        let opts = windows_options();
        assert_is_match("foox", "@(!(z*)|*x)", opts.clone(), true);
    }

    #[test]
    fn test_1049() {
        let opts = windows_options();
        assert_is_match("ooo", "!(f)", opts.clone(), true);
    }

    #[test]
    fn test_1050() {
        let opts = windows_options();
        assert_is_match("ooo", "*(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_1051() {
        let opts = windows_options();
        assert_is_match("ooo", "+(!(f))", opts.clone(), true);
    }

    #[test]
    fn test_1052() {
        let opts = windows_options();
        assert_is_match("zoox", "@(!(z*)|*x)", opts.clone(), true);
    }
}
