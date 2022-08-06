mod test_fmt_debug {
    use crate::ContinuousRange;

    #[test]
    pub fn empty() {
        let r = ContinuousRange::<i32>::Empty;
        assert_eq!(format!("{:?}", r), "[]");
    }

    #[test]
    pub fn inclusive() {
        let r = ContinuousRange::Inclusive(1, 5);
        assert_eq!(format!("{:?}", r), "[1..5]");
    }

    #[test]
    pub fn inclusive_equal() {
        let r = ContinuousRange::Inclusive(1, 1);
        assert_eq!(format!("{:?}", r), "[1..1]");
    }

    #[test]
    pub fn inclusive_inverted() {
        let r = ContinuousRange::Inclusive(5, 1);
        assert_eq!(format!("{:?}", r), "[5..1]");
    }

    #[test]
    pub fn exclusive() {
        let r = ContinuousRange::Exclusive(1, 5);
        assert_eq!(format!("{:?}", r), "(1..5)");
    }

    #[test]
    pub fn exclusive_equal() {
        let r = ContinuousRange::Exclusive(1, 1);
        assert_eq!(format!("{:?}", r), "(1..1)");
    }

    #[test]
    pub fn exclusive_inverted() {
        let r = ContinuousRange::Exclusive(5, 1);
        assert_eq!(format!("{:?}", r), "(5..1)");
    }

    #[test]
    pub fn start_exclusive() {
        let r = ContinuousRange::StartExclusive(1, 5);
        assert_eq!(format!("{:?}", r), "(1..5]");
    }

    #[test]
    pub fn start_exclusive_equal() {
        let r = ContinuousRange::StartExclusive(1, 1);
        assert_eq!(format!("{:?}", r), "(1..1]");
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r = ContinuousRange::StartExclusive(5, 1);
        assert_eq!(format!("{:?}", r), "(5..1]");
    }

    #[test]
    pub fn end_exclusive() {
        let r = ContinuousRange::EndExclusive(1, 5);
        assert_eq!(format!("{:?}", r), "[1..5)");
    }

    #[test]
    pub fn end_exclusive_equal() {
        let r = ContinuousRange::EndExclusive(1, 1);
        assert_eq!(format!("{:?}", r), "[1..1)");
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r = ContinuousRange::EndExclusive(5, 1);
        assert_eq!(format!("{:?}", r), "[5..1)");
    }

    #[test]
    pub fn full() {
        let r = ContinuousRange::<i32>::Full;
        assert_eq!(format!("{:?}", r), "(..)");
    }

    #[test]
    pub fn from() {
        let r = ContinuousRange::From(1);
        assert_eq!(format!("{:?}", r), "[1..)");
    }

    #[test]
    pub fn from_exclusive() {
        let r = ContinuousRange::FromExclusive(1);
        assert_eq!(format!("{:?}", r), "(1..)");
    }

    #[test]
    pub fn to() {
        let r = ContinuousRange::To(5);
        assert_eq!(format!("{:?}", r), "(..5]");
    }

    #[test]
    pub fn to_exclusive() {
        let r = ContinuousRange::ToExclusive(5);
        assert_eq!(format!("{:?}", r), "(..5)");
    }
}

mod test_from_stdlib {
    use assert_matches::assert_matches;

    use crate::ContinuousRange;

    #[test]
    pub fn inclusive() {
        let r: ContinuousRange<_> = (1..=5).into();
        assert_matches!(r, ContinuousRange::Inclusive(1, 5));
    }

    #[test]
    pub fn inclusive_equal() {
        let r: ContinuousRange<_> = (1..=1).into();
        assert_matches!(r, ContinuousRange::Single(1));
    }

    #[test]
    pub fn inclusive_inverted() {
        let r: ContinuousRange<_> = (5..=1).into();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn end_exclusive() {
        let r: ContinuousRange<_> = (1..5).into();
        assert_matches!(r, ContinuousRange::EndExclusive(1, 5));
    }

    #[test]
    pub fn end_exclusive_equal() {
        let r: ContinuousRange<_> = (1..1).into();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r: ContinuousRange<_> = (5..1).into();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn from() {
        let r: ContinuousRange<_> = (1..).into();
        assert_matches!(r, ContinuousRange::From(1));
    }

    #[test]
    pub fn to() {
        let r: ContinuousRange<_> = (..=5).into();
        assert_matches!(r, ContinuousRange::To(5));
    }

    #[test]
    pub fn to_exclusive() {
        let r: ContinuousRange<_> = (..5).into();
        assert_matches!(r, ContinuousRange::ToExclusive(5));
    }

    #[test]
    pub fn full() {
        let r: ContinuousRange<i32> = (..).into();
        assert_matches!(r, ContinuousRange::Full);
    }
}

/*
mod test_contains {
    use crate::Range;

    // i32::MAX didn't exist in our MSRV version
    pub const MAX_I32: i32 = 2_147_483_647i32;

    #[test]
    pub fn empty() {
        let r: Range<i32> = Range::empty();
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
    pub fn inverted() {
        let r: Range<_> = (5..=1).into();
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn exclusive() {
        let r: Range<_> = Range::continuous_exclusive(1, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn exclusive_inverted() {
        let r: Range<_> = Range::continuous_exclusive(5, 1);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn start_exclusive() {
        let r: Range<_> = Range::continuous_start_exclusive(1, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), true);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r: Range<_> = Range::continuous_start_exclusive(5, 1);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn end_exclusive() {
        let r: Range<_> = (1..5).into();
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), true);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r: Range<_> = (5..1).into();
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn full() {
        let r: Range<i32> = Range::full();
        assert_eq!(r.contains(-500), true);
        assert_eq!(r.contains(0), true);
        assert_eq!(r.contains(42), true);
        assert_eq!(r.contains(MAX_I32), true);
    }

    #[test]
    pub fn composite_empty() {
        let r: Range<u32> = Range::composite(vec![]);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn composite_simple() {
        let r: Range<_> = Range::composite(vec![(1..=5).into()]);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), true);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), true);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn composite_complex() {
        let r: Range<_> = Range::composite(vec![(1..3).into(), (5..).into()]);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), true);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), true);
        assert_eq!(r.contains(42), true);
        assert_eq!(r.contains(MAX_I32), true);
    }
}

mod test_is_empty {
    use crate::Range;

    #[test]
    pub fn empty() {
        let r: Range<i32> = Range::empty();
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn continuous() {
        let r: Range<_> = (1..=5).into();
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn inverted() {
        let r: Range<_> = (5..=1).into();
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn exclusive() {
        let r: Range<_> = Range::continuous_exclusive(1, 5);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn exclusive_inverted() {
        let r: Range<_> = Range::continuous_exclusive(5, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn start_exclusive() {
        let r: Range<_> = Range::continuous_start_exclusive(1, 5);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r: Range<_> = Range::continuous_start_exclusive(5, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn end_exclusive() {
        let r: Range<_> = (1..5).into();
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r: Range<_> = (5..1).into();
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn full() {
        let r: Range<i32> = Range::full();
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn composite_empty() {
        let r: Range<u32> = Range::composite(vec![]);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn composite_simple() {
        let r: Range<_> = Range::composite(vec![(1..=5).into()]);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn composite_simple_empty() {
        let r: Range<u32> = Range::composite(vec![Range::empty()]);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn composite_complex() {
        let r: Range<_> = Range::composite(vec![(1..3).into(), (5..).into()]);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn composite_complex_empty() {
        let r: Range<u32> = Range::composite(vec![Range::empty(), Range::empty(), Range::empty()]);
        assert_eq!(r.is_empty(), true);
    }
}

mod test_is_full {
    use crate::Range;

    #[test]
    pub fn empty() {
        let r: Range<i32> = Range::empty();
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn continuous() {
        let r: Range<_> = (1..=5).into();
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn inverted() {
        let r: Range<_> = (5..=1).into();
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn exclusive() {
        let r: Range<_> = Range::continuous_exclusive(1, 5);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn exclusive_inverted() {
        let r: Range<_> = Range::continuous_exclusive(5, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn start_exclusive() {
        let r: Range<_> = Range::continuous_start_exclusive(1, 5);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r: Range<_> = Range::continuous_start_exclusive(5, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn end_exclusive() {
        let r: Range<_> = (1..5).into();
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r: Range<_> = (5..1).into();
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn full() {
        let r: Range<i32> = Range::full();
        assert_eq!(r.is_full(), true);
    }

    #[test]
    pub fn composite_empty() {
        let r: Range<u32> = Range::composite(vec![]);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn composite_simple() {
        let r: Range<_> = Range::composite(vec![(1..=5).into()]);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn composite_simple_full() {
        let r: Range<u32> = Range::composite(vec![Range::full()]);
        assert_eq!(r.is_full(), true);
    }

    #[test]
    pub fn composite_complex() {
        let r: Range<_> = Range::composite(vec![(1..3).into(), (5..).into()]);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn composite_complex_full() {
        let r: Range<u32> = Range::composite(vec![(1..3).into(), (5..).into(), Range::full()]);
        assert_eq!(r.is_full(), true);
    }
}

mod test_composite_simplification {
    use crate::{ContinuousRange, Range};
    use assert_matches::assert_matches;

    #[test]
    pub fn empty_list() {
        let r: Range<i32> = Range::composite(vec![]);
        assert_matches!(r, Range::Continuous(ContinuousRange::Empty));
    }

    #[test]
    pub fn single_empty() {
        let r: Range<i32> = Range::composite(vec![Range::empty()]);
        assert_matches!(r, Range::Continuous(ContinuousRange::Empty));
    }

    #[test]
    pub fn multiple_empty() {
        let r: Range<i32> = Range::composite(vec![Range::empty(), Range::empty(), Range::empty()]);
        assert_matches!(r, Range::Continuous(ContinuousRange::Empty));
    }

    #[test]
    pub fn single_range() {
        let r: Range<i32> = Range::composite(vec![(1..=5).into()]);
        assert_matches!(r, Range::Continuous(ContinuousRange::Inclusive(1, 5)));
    }
}
*/
