use std::{
    borrow::Borrow,
    fmt,
    ops::{self, Bound},
};

use crate::RangesRelation;

#[derive(Clone, Hash, PartialEq)]
pub enum ContinuousRange<Idx> {
    /// A range containing no value
    ///
    /// `[]`
    Empty,

    /// A range between `start` (inclusive) and `end` (inclusive)
    ///
    /// `[start..end]`
    Inclusive(Idx, Idx),

    /// A range between `start` (exclusive) and `end` (exclusive)
    ///
    /// `(start..end)`
    Exclusive(Idx, Idx),

    /// A range between `start` (exclusive) and `end` (inclusive)
    ///
    /// `(start..end]`
    StartExclusive(Idx, Idx),

    /// A range between `start` (inclusive) and `end` (exclusive)
    ///
    /// `[start..end)`
    EndExclusive(Idx, Idx),

    /// A range starting from `start` (inclusive)
    ///
    /// `[start..)`
    From(Idx),

    /// A range starting from `start` (exclusive)
    ///
    /// `(start..)`
    FromExclusive(Idx),

    /// A range ending with `end` (inclusive)
    ///
    /// `(..end]`
    To(Idx),

    /// A range ending with `end` (exclusive)
    ///
    /// `(..end)`
    ToExclusive(Idx),

    /// A range containing all values
    Full,
}

impl<Idx: PartialOrd + Clone> ContinuousRange<Idx> {
    /// A range containing no value
    ///
    /// `[]`
    #[must_use]
    pub fn empty() -> ContinuousRange<Idx> {
        ContinuousRange::Empty
    }

    /// A range between `start` (inclusive) and `end` (inclusive)
    ///
    /// `[start..end]`
    #[must_use]
    pub fn inclusive(start: Idx, end: Idx) -> ContinuousRange<Idx>
    where
        Idx: PartialOrd,
    {
        if start > end {
            ContinuousRange::Empty
        } else {
            ContinuousRange::Inclusive(start, end)
        }
    }

    /// A range between `start` (exclusive) and `end` (exclusive)
    ///
    /// `(start..end)`
    #[must_use]
    pub fn exclusive(start: Idx, end: Idx) -> ContinuousRange<Idx>
    where
        Idx: PartialOrd,
    {
        if start >= end {
            ContinuousRange::Empty
        } else {
            ContinuousRange::Exclusive(start, end)
        }
    }

    /// A range between `start` (exclusive) and `end` (inclusive)
    ///
    /// `(start..end]`
    #[must_use]
    pub fn start_exclusive(start: Idx, end: Idx) -> ContinuousRange<Idx>
    where
        Idx: PartialOrd,
    {
        if start >= end {
            ContinuousRange::Empty
        } else {
            ContinuousRange::StartExclusive(start, end)
        }
    }

    /// A range between `start` (inclusive) and `end` (exclusive)
    ///
    /// `[start..end)`
    #[must_use]
    pub fn end_exclusive(start: Idx, end: Idx) -> ContinuousRange<Idx>
    where
        Idx: PartialOrd,
    {
        if start >= end {
            ContinuousRange::Empty
        } else {
            ContinuousRange::EndExclusive(start, end)
        }
    }

    /// A range starting from `start` (inclusive)
    ///
    /// `[start..)`
    #[must_use]
    pub fn from(start: Idx) -> ContinuousRange<Idx> {
        ContinuousRange::From(start)
    }

    /// A range starting from `start` (exclusive)
    ///
    /// `(start..)`
    #[must_use]
    pub fn from_exclusive(start: Idx) -> ContinuousRange<Idx> {
        ContinuousRange::FromExclusive(start)
    }

    /// A range ending with `end` (inclusive)
    ///
    /// `(..end]`
    #[must_use]
    pub fn to(end: Idx) -> ContinuousRange<Idx> {
        ContinuousRange::To(end)
    }

    /// A range ending with `end` (exclusive)
    ///
    /// `(..end)`
    #[must_use]
    pub fn to_exclusive(end: Idx) -> ContinuousRange<Idx> {
        ContinuousRange::ToExclusive(end)
    }

    /// A range containing all values
    #[must_use]
    pub fn full() -> ContinuousRange<Idx> {
        ContinuousRange::Full
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
            Self::Inclusive(start, end) => Some((Bound::Included(start), Bound::Included(end))),
            Self::Exclusive(start, end) => Some((Bound::Excluded(start), Bound::Excluded(end))),
            Self::StartExclusive(start, end) => {
                Some((Bound::Excluded(start), Bound::Included(end)))
            }
            Self::EndExclusive(start, end) => Some((Bound::Included(start), Bound::Excluded(end))),
            Self::From(start) => Some((Bound::Included(start), Bound::Unbounded)),
            Self::FromExclusive(start) => Some((Bound::Excluded(start), Bound::Unbounded)),
            Self::To(end) => Some((Bound::Unbounded, Bound::Included(end))),
            Self::ToExclusive(end) => Some((Bound::Unbounded, Bound::Excluded(end))),
            Self::Full => Some((Bound::Unbounded, Bound::Unbounded)),
        }
    }

    #[must_use]
    pub fn contains(&self, value: impl Borrow<Idx>) -> bool
    where
        Idx: PartialOrd,
    {
        match self {
            Self::Empty => false,
            Self::Inclusive(start, end) => {
                let value = value.borrow();
                value >= start && value <= end
            }
            Self::Exclusive(start, end) => {
                let value = value.borrow();
                value > start && value < end
            }
            Self::StartExclusive(start, end) => {
                let value = value.borrow();
                value > start && value <= end
            }
            Self::EndExclusive(start, end) => {
                let value = value.borrow();
                value >= start && value < end
            }
            Self::From(start) => value.borrow() >= start,
            Self::FromExclusive(start) => value.borrow() > start,
            Self::To(end) => value.borrow() <= end,
            Self::ToExclusive(end) => value.borrow() < end,
            Self::Full => true,
        }
    }

    #[must_use]
    pub fn union(self, other: ContinuousRange<Idx>) -> Option<ContinuousRange<Idx>>
    where
        Idx: PartialOrd,
    {
        match (self, other) {
            (ContinuousRange::Empty, r) | (r, ContinuousRange::Empty) => Some(r),
            (ContinuousRange::Full, _) | (_, ContinuousRange::Full) => Some(ContinuousRange::Full),
            (_r1, _r2) => todo!(),
        }
    }

    #[must_use]
    pub fn intersection(self, _other: ContinuousRange<Idx>) -> ContinuousRange<Idx> {
        todo!()
    }

    #[must_use]
    pub fn difference(self, _other: ContinuousRange<Idx>) -> Option<ContinuousRange<Idx>> {
        todo!()
    }

    #[must_use]
    pub fn overlaps(self, _other: ContinuousRange<Idx>) -> bool {
        todo!()
    }

    #[must_use]
    /// Compare the bounds of two ranges
    pub fn compare(self, _other: ContinuousRange<Idx>) -> RangesRelation {
        // Inspired from "Maintaining Knowledge about Temporal Intervals"
        todo!()
    }

    pub fn simplify_mut(&mut self)
    where
        Idx: PartialOrd,
    {
        match self {
            ContinuousRange::Empty => {}
            ContinuousRange::Inclusive(start, end) => {
                if start > end {
                    *self = ContinuousRange::Empty
                }
            }
            ContinuousRange::Exclusive(start, end) => {
                if start >= end {
                    *self = ContinuousRange::Empty
                }
            }
            ContinuousRange::StartExclusive(start, end) => {
                if start >= end {
                    *self = ContinuousRange::Empty
                }
            }
            ContinuousRange::EndExclusive(start, end) => {
                if start >= end {
                    *self = ContinuousRange::Empty
                }
            }
            ContinuousRange::From(_) => {}
            ContinuousRange::FromExclusive(_) => {}
            ContinuousRange::To(_) => {}
            ContinuousRange::ToExclusive(_) => {}
            ContinuousRange::Full => {}
        }
    }

    #[must_use]
    pub fn simplify(&self) -> Self
    where
        Idx: PartialOrd,
        Idx: Clone,
    {
        let mut clone = (*self).clone();
        clone.simplify_mut();
        clone
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,

            // bounded ranges with inverted bounds are considered empty
            Self::Inclusive(start, end) => start > end,
            Self::Exclusive(start, end) => start >= end,
            Self::StartExclusive(start, end) => start >= end,
            Self::EndExclusive(start, end) => start >= end,

            // unbounded ranges can't be empty
            Self::From(_) => false,
            Self::FromExclusive(_) => false,
            Self::To(_) => false,
            Self::ToExclusive(_) => false,
            Self::Full => false,
        }
    }

    #[must_use]
    pub fn is_full(&self) -> bool {
        match self {
            Self::Full => true,
            _ => false,
        }
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeFull> for ContinuousRange<Idx> {
    fn from(_: ops::RangeFull) -> Self {
        Self::full()
    }
}

impl<Idx: PartialOrd + Clone> From<ops::Range<Idx>> for ContinuousRange<Idx> {
    fn from(r: ops::Range<Idx>) -> Self {
        Self::end_exclusive(r.start, r.end)
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeInclusive<Idx>> for ContinuousRange<Idx> {
    fn from(r: ops::RangeInclusive<Idx>) -> Self {
        let (start, end) = r.into_inner();
        Self::inclusive(start, end)
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeFrom<Idx>> for ContinuousRange<Idx> {
    fn from(r: ops::RangeFrom<Idx>) -> Self {
        Self::from(r.start)
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeToInclusive<Idx>> for ContinuousRange<Idx> {
    fn from(r: ops::RangeToInclusive<Idx>) -> Self {
        Self::to(r.end)
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeTo<Idx>> for ContinuousRange<Idx> {
    fn from(r: ops::RangeTo<Idx>) -> Self {
        Self::to_exclusive(r.end)
    }
}

impl<Idx: fmt::Debug> fmt::Debug for ContinuousRange<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContinuousRange::Empty => write!(fmt, "[]")?,
            ContinuousRange::Full => write!(fmt, "(..)")?,
            ContinuousRange::Inclusive(start, end) => {
                write!(fmt, "[")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, "]")?;
            }
            ContinuousRange::Exclusive(start, end) => {
                write!(fmt, "(")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, ")")?;
            }
            ContinuousRange::StartExclusive(start, end) => {
                write!(fmt, "(")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, "]")?;
            }
            ContinuousRange::EndExclusive(start, end) => {
                write!(fmt, "[")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, ")")?;
            }
            ContinuousRange::From(start) => {
                write!(fmt, "[")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                write!(fmt, ")")?;
            }
            ContinuousRange::FromExclusive(start) => {
                write!(fmt, "(")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                write!(fmt, ")")?;
            }
            ContinuousRange::To(end) => {
                write!(fmt, "(")?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, "]")?;
            }
            ContinuousRange::ToExclusive(end) => {
                write!(fmt, "(")?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, ")")?;
            }
        }
        Ok(())
    }
}
