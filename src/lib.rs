use std::{
    borrow::Borrow,
    fmt,
    ops::{self, Bound, RangeBounds},
};

// --------------------------------------------------------------------------------------------------------------------
// Range types

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ContinuousRangeInclusive<Idx> {
    /// The lower bound of the range
    pub start: Idx,

    /// The upper bound of the range
    pub end: Idx,
}

impl<Idx: fmt::Debug> fmt::Debug for ContinuousRangeInclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "[")?;
        self.start.fmt(fmt)?;
        write!(fmt, "..")?;
        self.end.fmt(fmt)?;
        write!(fmt, "]")?;
        Ok(())
    }
}

impl<Idx> RangeBounds<Idx> for ContinuousRangeInclusive<Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Included(&self.end)
    }
}

impl<Idx> From<ops::RangeInclusive<Idx>> for ContinuousRangeInclusive<Idx> {
    fn from(r: ops::RangeInclusive<Idx>) -> Self {
        let (start, end) = r.into_inner();
        ContinuousRangeInclusive { start, end }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ContinuousRangeExclusive<Idx> {
    /// The lower bound of the range
    pub start: Idx,

    /// The upper bound of the range
    pub end: Idx,
}

impl<Idx: fmt::Debug> fmt::Debug for ContinuousRangeExclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "(")?;
        self.start.fmt(fmt)?;
        write!(fmt, "..")?;
        self.end.fmt(fmt)?;
        write!(fmt, ")")?;
        Ok(())
    }
}

impl<Idx> RangeBounds<Idx> for ContinuousRangeExclusive<Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Excluded(&self.start)
    }

    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Excluded(&self.end)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ContinuousRangeEndExclusive<Idx> {
    /// The lower bound of the range
    pub start: Idx,

    /// The upper bound of the range
    pub end: Idx,
}

impl<Idx: fmt::Debug> fmt::Debug for ContinuousRangeEndExclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "[")?;
        self.start.fmt(fmt)?;
        write!(fmt, "..")?;
        self.end.fmt(fmt)?;
        write!(fmt, ")")?;
        Ok(())
    }
}

impl<Idx> From<ops::Range<Idx>> for ContinuousRangeEndExclusive<Idx> {
    fn from(r: ops::Range<Idx>) -> Self {
        ContinuousRangeEndExclusive {
            start: r.start,
            end: r.end,
        }
    }
}

impl<Idx> RangeBounds<Idx> for ContinuousRangeEndExclusive<Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Excluded(&self.end)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ContinuousRangeStartExclusive<Idx> {
    /// The lower bound of the range
    pub start: Idx,

    /// The upper bound of the range
    pub end: Idx,
}

impl<Idx: fmt::Debug> fmt::Debug for ContinuousRangeStartExclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "(")?;
        self.start.fmt(fmt)?;
        write!(fmt, "..")?;
        self.end.fmt(fmt)?;
        write!(fmt, "]")?;
        Ok(())
    }
}

impl<Idx> RangeBounds<Idx> for ContinuousRangeStartExclusive<Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Excluded(&self.start)
    }

    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Included(&self.end)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ContinuousRangeFromInclusive<Idx> {
    /// The lower bound of the range
    pub start: Idx,
}

impl<Idx: fmt::Debug> fmt::Debug for ContinuousRangeFromInclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "[")?;
        self.start.fmt(fmt)?;
        write!(fmt, "..)")?;
        Ok(())
    }
}

impl<Idx> RangeBounds<Idx> for ContinuousRangeFromInclusive<Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Unbounded
    }
}

impl<Idx> From<ops::RangeFrom<Idx>> for ContinuousRangeFromInclusive<Idx> {
    fn from(r: ops::RangeFrom<Idx>) -> Self {
        ContinuousRangeFromInclusive { start: r.start }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ContinuousRangeFromExclusive<Idx> {
    /// The lower bound of the range
    pub start: Idx,
}

impl<Idx: fmt::Debug> fmt::Debug for ContinuousRangeFromExclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "(")?;
        self.start.fmt(fmt)?;
        write!(fmt, "..)")?;
        Ok(())
    }
}

impl<Idx> RangeBounds<Idx> for ContinuousRangeFromExclusive<Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Excluded(&self.start)
    }

    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Unbounded
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ContinuousRangeToInclusive<Idx> {
    /// The upper bound of the range
    pub end: Idx,
}

impl<Idx: fmt::Debug> fmt::Debug for ContinuousRangeToInclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "(..")?;
        self.end.fmt(fmt)?;
        write!(fmt, "]")?;
        Ok(())
    }
}

impl<Idx> RangeBounds<Idx> for ContinuousRangeToInclusive<Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Unbounded
    }

    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Included(&self.end)
    }
}

impl<Idx> From<ops::RangeTo<Idx>> for ContinuousRangeToInclusive<Idx> {
    fn from(r: ops::RangeTo<Idx>) -> Self {
        ContinuousRangeToInclusive { end: r.end }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ContinuousRangeToExclusive<Idx> {
    /// The upper bound of the range
    pub end: Idx,
}

impl<Idx: fmt::Debug> fmt::Debug for ContinuousRangeToExclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "(..")?;
        self.end.fmt(fmt)?;
        write!(fmt, ")")?;
        Ok(())
    }
}

impl<Idx> RangeBounds<Idx> for ContinuousRangeToExclusive<Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Unbounded
    }

    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Excluded(&self.end)
    }
}

// --------------------------------------------------------------------------------------------------------------------
// Any range type

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Range<Idx> {
    /// A range containing no value
    ///
    /// `[]`
    Empty,

    /// A range between `start` (inclusive) and `end` (inclusive)
    ///
    /// `[start..end]`
    Continuous(ContinuousRangeInclusive<Idx>),

    /// A range between `start` (exclusive) and `end` (exclusive)
    ///
    /// `(start..end)`
    ContinuousExclusive(ContinuousRangeExclusive<Idx>),

    /// A range between `start` (exclusive) and `end` (inclusive)
    ///
    /// `(start..end]`
    ContinuousStartExclusive(ContinuousRangeStartExclusive<Idx>),

    /// A range between `start` (inclusive) and `end` (exclusive)
    ///
    /// `[start..end)`
    ContinuousEndExclusive(ContinuousRangeEndExclusive<Idx>),

    /// A range starting from `start` (inclusive)
    ///
    /// `[start..)`
    From(ContinuousRangeFromInclusive<Idx>),

    /// A range starting from `start` (exclusive)
    ///
    /// `(start..)`
    FromExclusive(ContinuousRangeFromExclusive<Idx>),

    /// A range ending with `end` (inclusive)
    ///
    /// `(..end]`
    To(ContinuousRangeToInclusive<Idx>),

    /// A range ending with `end` (exclusive)
    ///
    /// `(..end)`
    ToExclusive(ContinuousRangeToExclusive<Idx>),

    /// A range containing all values
    Full,

    Composite(Vec<Range<Idx>>),
}

impl<Idx> Range<Idx> {
    /// A range containing no value
    ///
    /// `[]`
    pub fn empty() -> Range<Idx> {
        Range::Empty
    }

    /// A range between `start` (inclusive) and `end` (inclusive)
    ///
    /// `[start..end]`
    pub fn continuous(start: Idx, end: Idx) -> Range<Idx> {
        Range::Continuous(ContinuousRangeInclusive { start, end })
    }

    /// A range between `start` (exclusive) and `end` (exclusive)
    ///
    /// `(start..end)`
    pub fn continuous_exclusive(start: Idx, end: Idx) -> Range<Idx> {
        Range::ContinuousExclusive(ContinuousRangeExclusive { start, end })
    }

    /// A range between `start` (exclusive) and `end` (inclusive)
    ///
    /// `(start..end]`
    pub fn continuous_start_exclusive(start: Idx, end: Idx) -> Range<Idx> {
        Range::ContinuousStartExclusive(ContinuousRangeStartExclusive { start, end })
    }

    /// A range between `start` (inclusive) and `end` (exclusive)
    ///
    /// `[start..end)`
    pub fn continuous_end_exclusive(start: Idx, end: Idx) -> Range<Idx> {
        Range::ContinuousEndExclusive(ContinuousRangeEndExclusive { start, end })
    }

    /// A range starting from `start` (inclusive)
    ///
    /// `[start..)`
    pub fn from(start: Idx) -> Range<Idx> {
        Range::From(ContinuousRangeFromInclusive { start })
    }

    /// A range starting from `start` (exclusive)
    ///
    /// `(start..)`
    pub fn from_exclusive(start: Idx) -> Range<Idx> {
        Range::FromExclusive(ContinuousRangeFromExclusive { start })
    }

    /// A range ending with `end` (inclusive)
    ///
    /// `(..end]`
    pub fn to(end: Idx) -> Range<Idx> {
        Range::To(ContinuousRangeToInclusive { end })
    }

    /// A range ending with `end` (exclusive)
    ///
    /// `(..end)`
    pub fn to_exclusive(end: Idx) -> Range<Idx> {
        Range::ToExclusive(ContinuousRangeToExclusive { end })
    }

    /// A range containing all values
    pub fn full() -> Range<Idx> {
        Range::Full
    }

    pub fn composite(items: Vec<Range<Idx>>) -> Range<Idx> {
        Range::Composite(items)
    }

    pub fn range_bounds(&self) -> Option<(Bound<&Idx>, Bound<&Idx>)> {
        match self {
            Self::Empty => {
                // We can't implement RangeBounds due to this case, a possible trick would be to use a range with 2
                // exclusive bounds on the default value.
                // But as the result is a reference we would need a per-generic 'static to reference and so would
                // require something like the 'typemap' crate just for that.
                None
            },
            Self::Continuous(r) => Some((r.start_bound(), r.end_bound())),
            Self::ContinuousExclusive(r) => Some((r.start_bound(), r.end_bound())),
            Self::ContinuousStartExclusive(r) => Some((r.start_bound(), r.end_bound())),
            Self::ContinuousEndExclusive(r) => Some((r.start_bound(), r.end_bound())),
            Self::From(r) => Some((r.start_bound(), r.end_bound())),
            Self::FromExclusive(r) => Some((r.start_bound(), r.end_bound())),
            Self::To(r) => Some((r.start_bound(), r.end_bound())),
            Self::ToExclusive(r) => Some((r.start_bound(), r.end_bound())),
            Self::Full => Some((Bound::Unbounded, Bound::Unbounded)),
            Self::Composite(_) => None, // TODO: custom implementation of bounds
        }
    }

    pub fn contains(&self, value: impl Borrow<Idx>) -> bool
    where
        Idx: PartialOrd,
    {
        match self {
            Self::Empty => false,
            Self::Continuous(r) => r.contains(value.borrow()),
            Self::ContinuousExclusive(r) => r.contains(value.borrow()),
            Self::ContinuousStartExclusive(r) => r.contains(value.borrow()),
            Self::ContinuousEndExclusive(r) => r.contains(value.borrow()),
            Self::From(r) => r.contains(value.borrow()),
            Self::FromExclusive(r) => r.contains(value.borrow()),
            Self::To(r) => r.contains(value.borrow()),
            Self::ToExclusive(r) => r.contains(value.borrow()),
            Self::Full => true,
            Self::Composite(r) => {
                let value = value.borrow();
                r.iter().any(|x| x.contains(value))
            }
        }
    }

    pub fn union(self, other: Range<Idx>) -> Range<Idx> {
        // TODO: Quite a few cases can be optimized, specialized, ...
        // TODO: Also maybe the ranges should be kept sorted in composite
        match (self, other) {
            (Range::Empty, r) => r,
            (r, Range::Empty) => r,
            (Range::Full, _) => Range::Full,
            (_, Range::Full) => Range::Full,
            (Range::Composite(mut r1), Range::Composite(mut r2)) => {
                r1.append(&mut r2);
                Range::Composite(r1)
            }
            (Range::Composite(mut r1), r2) => {
                r1.push(r2);
                Range::Composite(r1)
            }
            (r1, Range::Composite(mut r2)) => {
                r2.push(r1);
                Range::Composite(r2)
            }
            (r1, r2) => Range::Composite(vec![r1, r2]),
        }
    }
}

impl<Idx> From<ops::RangeFull> for Range<Idx> {
    fn from(_: ops::RangeFull) -> Self {
        Self::Full
    }
}

impl<Idx> From<ops::Range<Idx>> for Range<Idx> {
    fn from(r: ops::Range<Idx>) -> Self {
        Self::ContinuousEndExclusive(r.into())
    }
}

impl<Idx> From<ops::RangeInclusive<Idx>> for Range<Idx> {
    fn from(r: ops::RangeInclusive<Idx>) -> Self {
        Self::Continuous(r.into())
    }
}

impl<Idx> From<ops::RangeFrom<Idx>> for Range<Idx> {
    fn from(r: ops::RangeFrom<Idx>) -> Self {
        Self::From(r.into())
    }
}

impl<Idx> From<ops::RangeTo<Idx>> for Range<Idx> {
    fn from(r: ops::RangeTo<Idx>) -> Self {
        Self::To(r.into())
    }
}

impl<Idx: fmt::Debug> fmt::Debug for Range<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Range::Empty => write!(fmt, "[]")?,
            Range::Full => write!(fmt, "(..)")?,
            Range::Continuous(r) => fmt::Debug::fmt(r, fmt)?,
            Range::ContinuousExclusive(r) => fmt::Debug::fmt(r, fmt)?,
            Range::ContinuousStartExclusive(r) => fmt::Debug::fmt(r, fmt)?,
            Range::ContinuousEndExclusive(r) => fmt::Debug::fmt(r, fmt)?,
            Range::From(r) => fmt::Debug::fmt(r, fmt)?,
            Range::FromExclusive(r) => fmt::Debug::fmt(r, fmt)?,
            Range::To(r) => fmt::Debug::fmt(r, fmt)?,
            Range::ToExclusive(r) => fmt::Debug::fmt(r, fmt)?,
            Range::Composite(r) => {
                write!(fmt, "{{")?;
                let mut first = true;
                for child in r.iter() {
                    if first {
                        first = false;
                    } else {
                        write!(fmt, "; ")?;
                    }

                    fmt::Debug::fmt(child, fmt)?;
                }
                write!(fmt, "}}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
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

#[cfg(test)]
mod test_contains {
    use crate::Range;

    #[test]
    pub fn empty() {
        let r: Range<i32> = Range::Empty;
        assert_eq!(r.contains(-500), false);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(42), false);
        assert_eq!(r.contains(i32::MAX), false);
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
        assert_eq!(r.contains(i32::MAX), true);
    }
}
