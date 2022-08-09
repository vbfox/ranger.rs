mod test_fmt_debug {
    use crate::ContinuousRange;

    #[test]
    pub fn empty() {
        let r = ContinuousRange::<i32>::Empty;
        assert_eq!(format!("{:?}", r), "[]");
    }

    #[test]
    pub fn single() {
        let r = ContinuousRange::Single(5);
        assert_eq!(format!("{:?}", r), "5");
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

#[allow(clippy::reversed_empty_ranges)]
mod test_from_stdlib {
    use assert_matches::assert_matches;

    use crate::ContinuousRange;

    #[test]
    pub fn default() {
        let r: ContinuousRange<i32> = Default::default();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn empty() {
        let r: ContinuousRange<i32> = ().into();
        assert_matches!(r, ContinuousRange::Empty);
    }

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

mod test_creation_functions {
    use assert_matches::assert_matches;

    use crate::ContinuousRange;

    #[test]
    pub fn empty() {
        let r = ContinuousRange::<i32>::empty();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn single() {
        let r = ContinuousRange::single(5);
        assert_matches!(r, ContinuousRange::Single(5));
    }

    #[test]
    pub fn inclusive() {
        let r = ContinuousRange::inclusive(1, 5);
        assert_matches!(r, ContinuousRange::Inclusive(1, 5));
    }

    #[test]
    pub fn inclusive_equal() {
        let r = ContinuousRange::inclusive(1, 1);
        assert_matches!(r, ContinuousRange::Single(1));
    }

    #[test]
    pub fn inclusive_inverted() {
        let r = ContinuousRange::inclusive(5, 1);
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn exclusive() {
        let r = ContinuousRange::exclusive(1, 5);
        assert_matches!(r, ContinuousRange::Exclusive(1, 5));
    }

    #[test]
    pub fn exclusive_equal() {
        let r = ContinuousRange::exclusive(1, 1);
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn exclusive_inverted() {
        let r = ContinuousRange::exclusive(5, 1);
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn start_exclusive() {
        let r = ContinuousRange::start_exclusive(1, 5);
        assert_matches!(r, ContinuousRange::StartExclusive(1, 5));
    }

    #[test]
    pub fn start_exclusive_equal() {
        let r = ContinuousRange::start_exclusive(1, 1);
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r = ContinuousRange::start_exclusive(5, 1);
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn end_exclusive() {
        let r = ContinuousRange::end_exclusive(1, 5);
        assert_matches!(r, ContinuousRange::EndExclusive(1, 5));
    }

    #[test]
    pub fn end_exclusive_equal() {
        let r = ContinuousRange::end_exclusive(1, 1);
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r = ContinuousRange::end_exclusive(5, 1);
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn from() {
        let r = ContinuousRange::from(1);
        assert_matches!(r, ContinuousRange::From(1));
    }

    #[test]
    pub fn from_exclusive() {
        let r = ContinuousRange::from_exclusive(5);
        assert_matches!(r, ContinuousRange::FromExclusive(5));
    }

    #[test]
    pub fn to() {
        let r = ContinuousRange::to(5);
        assert_matches!(r, ContinuousRange::To(5));
    }

    #[test]
    pub fn to_exclusive() {
        let r = ContinuousRange::to_exclusive(5);
        assert_matches!(r, ContinuousRange::ToExclusive(5));
    }

    #[test]
    pub fn full() {
        let r = ContinuousRange::<i32>::full();
        assert_matches!(r, ContinuousRange::Full);
    }
}

mod test_is_full {
    use crate::ContinuousRange;

    #[test]
    pub fn empty() {
        let r = ContinuousRange::<i32>::Empty;
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn single() {
        let r = ContinuousRange::Single(5);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn inclusive() {
        let r = ContinuousRange::Inclusive(1, 5);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn inclusive_equal() {
        let r = ContinuousRange::Inclusive(1, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn inclusive_inverted() {
        let r = ContinuousRange::Inclusive(5, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn exclusive() {
        let r = ContinuousRange::Exclusive(1, 5);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn exclusive_equal() {
        let r = ContinuousRange::Exclusive(1, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn exclusive_inverted() {
        let r = ContinuousRange::Exclusive(5, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn start_exclusive() {
        let r = ContinuousRange::StartExclusive(1, 5);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn start_exclusive_equal() {
        let r = ContinuousRange::StartExclusive(1, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r = ContinuousRange::StartExclusive(5, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn end_exclusive() {
        let r = ContinuousRange::EndExclusive(1, 5);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn end_exclusive_equal() {
        let r = ContinuousRange::EndExclusive(1, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r = ContinuousRange::EndExclusive(5, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn full() {
        let r = ContinuousRange::<i32>::Full;
        assert_eq!(r.is_full(), true);
    }

    #[test]
    pub fn from() {
        let r = ContinuousRange::From(1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn from_exclusive() {
        let r = ContinuousRange::FromExclusive(1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn to() {
        let r = ContinuousRange::To(5);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn to_exclusive() {
        let r = ContinuousRange::ToExclusive(5);
        assert_eq!(r.is_full(), false);
    }
}

mod test_is_empty {
    use crate::ContinuousRange;

    #[test]
    pub fn empty() {
        let r = ContinuousRange::<i32>::Empty;
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn single() {
        let r = ContinuousRange::Single(5);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn inclusive() {
        let r = ContinuousRange::Inclusive(1, 5);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn inclusive_equal() {
        let r = ContinuousRange::Inclusive(1, 1);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn inclusive_inverted() {
        let r = ContinuousRange::Inclusive(5, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn exclusive() {
        let r = ContinuousRange::Exclusive(1, 5);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn exclusive_equal() {
        let r = ContinuousRange::Exclusive(1, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn exclusive_inverted() {
        let r = ContinuousRange::Exclusive(5, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn start_exclusive() {
        let r = ContinuousRange::StartExclusive(1, 5);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn start_exclusive_equal() {
        let r = ContinuousRange::StartExclusive(1, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r = ContinuousRange::StartExclusive(5, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn end_exclusive() {
        let r = ContinuousRange::EndExclusive(1, 5);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn end_exclusive_equal() {
        let r = ContinuousRange::EndExclusive(1, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r = ContinuousRange::EndExclusive(5, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn full() {
        let r = ContinuousRange::<i32>::Full;
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn from() {
        let r = ContinuousRange::From(1);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn from_exclusive() {
        let r = ContinuousRange::FromExclusive(1);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn to() {
        let r = ContinuousRange::To(5);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn to_exclusive() {
        let r = ContinuousRange::ToExclusive(5);
        assert_eq!(r.is_empty(), false);
    }
}

mod test_simplify {
    use assert_matches::assert_matches;

    use crate::ContinuousRange;

    #[test]
    pub fn empty() {
        let r = ContinuousRange::<i32>::Empty.simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn single() {
        let r = ContinuousRange::Single(5).simplify();
        assert_matches!(r, ContinuousRange::Single(5));
    }

    #[test]
    pub fn inclusive() {
        let r = ContinuousRange::Inclusive(1, 5).simplify();
        assert_matches!(r, ContinuousRange::Inclusive(1, 5));
    }

    #[test]
    pub fn inclusive_equal() {
        let r = ContinuousRange::Inclusive(1, 1).simplify();
        assert_matches!(r, ContinuousRange::Single(1));
    }

    #[test]
    pub fn inclusive_inverted() {
        let r = ContinuousRange::Inclusive(5, 1).simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn exclusive() {
        let r = ContinuousRange::Exclusive(1, 5).simplify();
        assert_matches!(r, ContinuousRange::Exclusive(1, 5));
    }

    #[test]
    pub fn exclusive_equal() {
        let r = ContinuousRange::Exclusive(1, 1).simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn exclusive_inverted() {
        let r = ContinuousRange::Exclusive(5, 1).simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn start_exclusive() {
        let r = ContinuousRange::StartExclusive(1, 5).simplify();
        assert_matches!(r, ContinuousRange::StartExclusive(1, 5));
    }

    #[test]
    pub fn start_exclusive_equal() {
        let r = ContinuousRange::StartExclusive(1, 1).simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r = ContinuousRange::StartExclusive(5, 1).simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn end_exclusive() {
        let r = ContinuousRange::EndExclusive(1, 5).simplify();
        assert_matches!(r, ContinuousRange::EndExclusive(1, 5));
    }

    #[test]
    pub fn end_exclusive_equal() {
        let r = ContinuousRange::EndExclusive(1, 1).simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r = ContinuousRange::EndExclusive(5, 1).simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn from() {
        let r = ContinuousRange::From(1).simplify();
        assert_matches!(r, ContinuousRange::From(1));
    }

    #[test]
    pub fn from_exclusive() {
        let r = ContinuousRange::FromExclusive(5).simplify();
        assert_matches!(r, ContinuousRange::FromExclusive(5));
    }

    #[test]
    pub fn to() {
        let r = ContinuousRange::To(5).simplify();
        assert_matches!(r, ContinuousRange::To(5));
    }

    #[test]
    pub fn to_exclusive() {
        let r = ContinuousRange::ToExclusive(5).simplify();
        assert_matches!(r, ContinuousRange::ToExclusive(5));
    }

    #[test]
    pub fn full() {
        let r = ContinuousRange::<i32>::Full.simplify();
        assert_matches!(r, ContinuousRange::Full);
    }
}

mod test_contains {
    use crate::ContinuousRange;

    // i32::MAX didn't exist in our MSRV version
    pub const MAX_I32: i32 = 2_147_483_647i32;

    #[test]
    pub fn empty() {
        let r: ContinuousRange<i32> = ContinuousRange::empty();
        assert_eq!(r.contains(-500), false);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(42), false);
        assert_eq!(r.contains(MAX_I32), false);
    }

    #[test]
    pub fn single() {
        let r = ContinuousRange::Single(42);
        assert_eq!(r.contains(-500), false);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(42), true);
        assert_eq!(r.contains(MAX_I32), false);
    }

    #[test]
    pub fn inclusive() {
        let r = ContinuousRange::Inclusive(1, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), true);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), true);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn inclusive_equal() {
        let r = ContinuousRange::Inclusive(5, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), true);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn inclusive_inverted() {
        let r = ContinuousRange::Inclusive(5, 1);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn exclusive() {
        let r = ContinuousRange::Exclusive(1, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn exclusive_equal() {
        let r = ContinuousRange::Exclusive(5, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn exclusive_inverted() {
        let r = ContinuousRange::Exclusive(5, 1);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn start_exclusive() {
        let r = ContinuousRange::StartExclusive(1, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), true);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn start_exclusive_equal() {
        let r = ContinuousRange::StartExclusive(5, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r = ContinuousRange::StartExclusive(5, 1);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn end_exclusive() {
        let r = ContinuousRange::EndExclusive(1, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), true);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn end_exclusive_equal() {
        let r = ContinuousRange::EndExclusive(5, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r = ContinuousRange::EndExclusive(5, 1);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn full() {
        let r = ContinuousRange::<i32>::Full;
        assert_eq!(r.contains(-500), true);
        assert_eq!(r.contains(0), true);
        assert_eq!(r.contains(42), true);
        assert_eq!(r.contains(MAX_I32), true);
    }
}

mod test_compare {
    use crate::{ContinuousRange, RangesRelation};

    macro_rules! compare {
        ($a:expr, $b:expr, $relation:expr) => {
            assert_eq!($a.compare(&$b), $relation);
        };
    }

    #[test]
    pub fn empty() {
        compare!(
            ContinuousRange::<i32>::Empty,
            ContinuousRange::<i32>::Empty,
            Some(RangesRelation::Equal)
        );
        compare!(
            ContinuousRange::<i32>::Empty,
            ContinuousRange::Inclusive(5, 1),
            Some(RangesRelation::Equal)
        );
        compare!(
            ContinuousRange::<i32>::Empty,
            ContinuousRange::Inclusive(1, 5),
            None
        );
        compare!(
            ContinuousRange::Inclusive(1, 5),
            ContinuousRange::<i32>::Empty,
            None
        );
        compare!(
            ContinuousRange::Inclusive(1, 5),
            ContinuousRange::Inclusive(5, 1),
            None
        );
    }

    #[test]
    pub fn strictly_before() {
        macro_rules! strictly_before {
            ($a:expr, $b:expr) => {
                compare!($a, $b, Some(RangesRelation::StrictlyBefore));
            };
        }

        strictly_before!(
            ContinuousRange::Inclusive(1, 5),
            ContinuousRange::Inclusive(6, 10)
        );
        strictly_before!(
            ContinuousRange::Exclusive(1, 5),
            ContinuousRange::Exclusive(6, 10)
        );
        /*
        assert_eq!(
            ContinuousRange::Exclusive(1, 5).compare(&ContinuousRange::Exclusive(5, 10)),
            Some(RangesRelation::StrictlyBefore)
        );
         */
        strictly_before!(
            ContinuousRange::Exclusive(1, 5),
            ContinuousRange::Inclusive(5, 10)
        );
        strictly_before!(
            ContinuousRange::Inclusive(1, 5),
            ContinuousRange::Exclusive(5, 10)
        );
        strictly_before!(
            ContinuousRange::EndExclusive(1, 5),
            ContinuousRange::Inclusive(5, 10)
        );
        strictly_before!(
            ContinuousRange::Inclusive(1, 5),
            ContinuousRange::StartExclusive(5, 10)
        );
    }
}
