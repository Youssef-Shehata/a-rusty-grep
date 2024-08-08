use crate::grep;

#[cfg(test)]
pub mod true_tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(grep("ass", "ass"));
    }

    #[test]
    fn case2() {
        assert!(grep("2", "\\d"));
    }
    #[test]
    fn case3() {
        assert!(grep("012", "\\d\\d\\d"));
    }
    #[test]
    fn case4() {
        assert!(grep("oopspp", "[so]"));
    }
    #[test]
    fn case5() {
        assert!(grep("019248apapopopiw23", "[^nmbv]"));
    }
    #[test]
    fn case6() {
        assert!(grep("qwe", "[sw]"));
    }
    #[test]
    fn case7() {
        assert!(grep("d2d apple", "\\w\\d\\w apple"));
    }
    #[test]
    fn case8() {
        assert!(grep("22w a", "\\d\\dw [sa]"));
    }
    #[test]
    fn case9() {
        assert_eq!(grep("opac", "[^c]"), true);
    }
    #[test]
    fn case10() {
        assert_eq!(grep("opac", "^opa"), true);
    }
    #[test]
    fn case11() {
        assert_eq!(grep("opac", "^o"), true);
    }
    #[test]
    fn case12() {
        assert_eq!(grep("a", "^a"), true);
    }
}

#[cfg(test)]
pub mod false_tests {
    use super::*;

    #[test]
    fn case1() {
        assert_ne!(grep("w", "\\d"), true);
    }

    #[test]
    fn case2() {}
    #[test]
    fn case3() {
        assert_ne!(grep("w29d", "[sa]"), true);
    }
    #[test]
    fn case4() {
        assert_ne!(grep("dsx", "d[pw]x"), true);
    }
    #[test]
    fn case5() {
        assert_ne!(grep("12 ds 21", "12 ds [^2]1"), true);
    }
    #[test]
    fn case6() {
        assert_ne!(grep("22w ", "\\d\\dw [^sa]"), true);
    }
    #[test]
    fn case7() {
        assert_ne!(grep("da", "^das"), true);
    }
    #[test]
    fn case8() {
        assert_ne!(grep("ad", "^d"), true);
    }
    #[test]
    fn case9() {
        assert_ne!(grep("1p", "^1 "), true);
    }
    #[test]
    fn case10() {
        assert_ne!(grep("daas", "^aas"), true);
    }
    #[test]
    fn case11() {
        assert_ne!(grep("slog", "^log"), true);
    }
}
