use std::{
    borrow::Borrow,
    fmt,
    ops::{self, Bound, RangeBounds},
};

pub use crate::{
    ContinuousRangeInclusive,
    ContinuousRangeExclusive,
    ContinuousRangeStartExclusive,
    ContinuousRangeEndExclusive,
    ContinuousRangeFromInclusive,
    ContinuousRangeFromExclusive,
    ContinuousRangeToInclusive,
    ContinuousRangeToExclusive
};

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
    #[must_use]
    pub fn empty() -> Range<Idx> {
        Range::Empty
    }

    /// A range between `start` (inclusive) and `end` (inclusive)
    ///
    /// `[start..end]`
    #[must_use]
    pub fn continuous(start: Idx, end: Idx) -> Range<Idx> {
        Range::Continuous(ContinuousRangeInclusive { start, end })
    }

    /// A range between `start` (exclusive) and `end` (exclusive)
    ///
    /// `(start..end)`
    #[must_use]
    pub fn continuous_exclusive(start: Idx, end: Idx) -> Range<Idx> {
        Range::ContinuousExclusive(ContinuousRangeExclusive { start, end })
    }

    /// A range between `start` (exclusive) and `end` (inclusive)
    ///
    /// `(start..end]`
    #[must_use]
    pub fn continuous_start_exclusive(start: Idx, end: Idx) -> Range<Idx> {
        Range::ContinuousStartExclusive(ContinuousRangeStartExclusive { start, end })
    }

    /// A range between `start` (inclusive) and `end` (exclusive)
    ///
    /// `[start..end)`
    #[must_use]
    pub fn continuous_end_exclusive(start: Idx, end: Idx) -> Range<Idx> {
        Range::ContinuousEndExclusive(ContinuousRangeEndExclusive { start, end })
    }

    /// A range starting from `start` (inclusive)
    ///
    /// `[start..)`
    #[must_use]
    pub fn from(start: Idx) -> Range<Idx> {
        Range::From(ContinuousRangeFromInclusive { start })
    }

    /// A range starting from `start` (exclusive)
    ///
    /// `(start..)`
    #[must_use]
    pub fn from_exclusive(start: Idx) -> Range<Idx> {
        Range::FromExclusive(ContinuousRangeFromExclusive { start })
    }

    /// A range ending with `end` (inclusive)
    ///
    /// `(..end]`
    #[must_use]
    pub fn to(end: Idx) -> Range<Idx> {
        Range::To(ContinuousRangeToInclusive { end })
    }

    /// A range ending with `end` (exclusive)
    ///
    /// `(..end)`
    #[must_use]
    pub fn to_exclusive(end: Idx) -> Range<Idx> {
        Range::ToExclusive(ContinuousRangeToExclusive { end })
    }

    /// A range containing all values
    #[must_use]
    pub fn full() -> Range<Idx> {
        Range::Full
    }

    #[must_use]
    pub fn composite(items: Vec<Range<Idx>>) -> Range<Idx> {
        Range::Composite(items)
    }

    #[must_use]
    pub fn range_bounds(&self) -> Option<(Bound<&Idx>, Bound<&Idx>)> {
        match self {
            Self::Empty => {
                // We can't implement RangeBounds due to this case, a possible trick would be to use a range with 2
                // exclusive bounds on the default value.
                // But as the result is a reference we would need a per-generic 'static to reference and so would
                // require something like the 'typemap' crate just for that.
                None
            }
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

    #[must_use]
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

    #[must_use]
    pub fn union(self, other: Range<Idx>) -> Range<Idx> {
        // TODO: Quite a few cases can be optimized, specialized, ...
        // TODO: Also maybe the ranges should be kept sorted in composite
        match (self, other) {
            (Range::Empty, r) | (r, Range::Empty) => r,
            (Range::Full, _) | (_, Range::Full) => Range::Full,
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
