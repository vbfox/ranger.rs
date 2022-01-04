mod test_fmt_debug {
    use crate::Range;

    #[test]
    pub fn empty() {
        let r = Range::<i32>::empty();
        assert_eq!(format!("{:?}", r), "[]");
    }

    #[test]
    pub fn continuous() {
        let r: Range<_> = (1..=5).into();
        assert_eq!(format!("{:?}", r), "[1..5]");
    }

    #[test]
    pub fn continuous_exclusive() {
        let r: Range<_> = Range::continuous_exclusive(1, 5);
        assert_eq!(format!("{:?}", r), "(1..5)");
    }

    #[test]
    pub fn continuous_start_exclusive() {
        let r: Range<_> = Range::continuous_start_exclusive(1, 5);
        assert_eq!(format!("{:?}", r), "(1..5]");
    }

    #[test]
    pub fn continuous_end_exclusive() {
        let r: Range<_> = (1..5).into();
        assert_eq!(format!("{:?}", r), "[1..5)");
    }

    #[test]
    pub fn full() {
        let r: Range<u32> = (..).into();
        assert_eq!(format!("{:?}", r), "(..)");
    }

    #[test]
    pub fn from() {
        let r: Range<_> = (1..).into();
        assert_eq!(format!("{:?}", r), "[1..)");
    }

    #[test]
    pub fn from_exclusive() {
        let r: Range<_> = Range::from_exclusive(1);
        assert_eq!(format!("{:?}", r), "(1..)");
    }

    #[test]
    pub fn to() {
        let r: Range<_> = (..5).into();
        assert_eq!(format!("{:?}", r), "(..5]");
    }

    #[test]
    pub fn to_exclusive() {
        let r: Range<_> = Range::to_exclusive(5);
        assert_eq!(format!("{:?}", r), "(..5)");
    }
}

mod test_contains {
    use crate::Range;

    // i32::MAX didn't exist in our MSRV version
    pub const MAX_I32: i32 = 2_147_483_647i32;

    #[test]
    pub fn empty() {
        let r: Range<i32> = Range::Empty;
        assert_eq!(r.contains(-500), false);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(42), false);
        assert_eq!(r.contains(MAX_I32), false);
    }

    #[test]
    pub fn continuous() {
        let r: Range<_> = (1..=5).into();
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), true);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), true);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn continuous_exclusive() {
        let r: Range<_> = Range::continuous_exclusive(1, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn continuous_start_exclusive() {
        let r: Range<_> = Range::continuous_start_exclusive(1, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), true);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn continuous_end_exclusive() {
        let r: Range<_> = (1..5).into();
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), true);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn full() {
        let r: Range<i32> = Range::Full;
        assert_eq!(r.contains(-500), true);
        assert_eq!(r.contains(0), true);
        assert_eq!(r.contains(42), true);
        assert_eq!(r.contains(MAX_I32), true);
    }
}
