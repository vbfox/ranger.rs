use std::{
    borrow::Borrow,
    cmp::Ordering,
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

    /// A range containing a single value
    ///
    /// `value`
    Single(Idx),

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

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum BoundSide {
    Start,
    End,
}

fn partial_cmp_bounds<Idx: PartialOrd>(
    this: &Bound<&Idx>,
    this_side: BoundSide,
    other: &Bound<&Idx>,
    other_side: BoundSide,
) -> Option<Ordering> {
    match this {
        Bound::Included(this_value) => match other {
            Bound::Included(other_value) => this_value.partial_cmp(other_value),
            Bound::Excluded(other_value) => match this_value.partial_cmp(other_value) {
                Some(Ordering::Equal) => match (this_side, other_side) {
                    (BoundSide::Start, BoundSide::Start) => Some(Ordering::Less),
                    (BoundSide::End, BoundSide::End) => Some(Ordering::Greater),
                    (BoundSide::Start, BoundSide::End) => Some(Ordering::Greater),
                    (BoundSide::End, BoundSide::Start) => Some(Ordering::Greater),
                },
                other => other,
            },
            Bound::Unbounded => Some(Ordering::Greater),
        },
        Bound::Excluded(this_value) => match other {
            Bound::Included(other_value) => match this_value.partial_cmp(other_value) {
                Some(Ordering::Equal) => match (this_side, other_side) {
                    (BoundSide::Start, BoundSide::Start) => Some(Ordering::Greater),
                    (BoundSide::End, BoundSide::End) => Some(Ordering::Less),
                    (BoundSide::Start, BoundSide::End) => Some(Ordering::Less),
                    (BoundSide::End, BoundSide::Start) => Some(Ordering::Less),
                },
                other => other,
            },
            Bound::Excluded(other_value) => this_value.partial_cmp(other_value),
            Bound::Unbounded => Some(Ordering::Greater),
        },
        Bound::Unbounded => match other {
            Bound::Included(_) => Some(Ordering::Greater),
            Bound::Excluded(_) => Some(Ordering::Greater),
            Bound::Unbounded => Some(Ordering::Equal),
        },
    }
}

impl<Idx: PartialOrd + Clone> ContinuousRange<Idx> {
    /// A range containing no value
    ///
    /// `[]`
    #[must_use]
    pub fn empty() -> ContinuousRange<Idx> {
        ContinuousRange::Empty
    }

    /// A range containing a single value
    ///
    /// `value`
    #[must_use]
    pub fn single(value: Idx) -> ContinuousRange<Idx> {
        ContinuousRange::Single(value)
    }

    /// A range between `start` (inclusive) and `end` (inclusive)
    ///
    /// `[start..end]`
    #[must_use]
    pub fn inclusive(start: Idx, end: Idx) -> ContinuousRange<Idx>
    where
        Idx: PartialOrd,
    {
        if start == end {
            ContinuousRange::Single(start)
        } else if start > end {
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
            Self::Single(value) => Some((Bound::Included(value), Bound::Included(value))),
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
        Idx: PartialOrd + PartialEq,
    {
        match self {
            Self::Empty => false,
            Self::Single(single_value) => single_value == value.borrow(),
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
    pub fn intersects(self, other: ContinuousRange<Idx>) -> bool {
        // Two empty ranges are 'equal' but we don't want to return true for them
        if self.is_empty() & other.is_empty() {
            false
        } else {
            self.compare(other).intersects()
        }
    }

    #[must_use]
    /// Compare the bounds of two ranges
    pub fn compare(self, other: ContinuousRange<Idx>) -> RangesRelation {
        // Inspired from "Maintaining Knowledge about Temporal Intervals"

        // Empty ranges don't have bounds so we need to special case them before anything else
        if self.is_empty() {
            if other.is_empty() {
                return RangesRelation::Equal;
            } else {
                return RangesRelation::None;
            }
        } else if other.is_empty() {
            return RangesRelation::None;
        }

        let (self_start, self_end) = self.range_bounds().unwrap();
        let (other_start, other_end) = self.range_bounds().unwrap();

        let cmp_end_start =
            partial_cmp_bounds(&self_end, BoundSide::End, &other_start, BoundSide::Start);
        if cmp_end_start.is_none() {
            return RangesRelation::None;
        }
        let cmp_end_start = cmp_end_start.expect("end-start ordering should be Some");

        if cmp_end_start == Ordering::Less {
            return RangesRelation::StrictlyBefore;
        }

        let cmd_start_end =
            partial_cmp_bounds(&self_start, BoundSide::Start, &other_end, BoundSide::End);
        if cmd_start_end.is_none() {
            return RangesRelation::None;
        }
        let cmp_start_end = cmd_start_end.expect("start-end ordering should be Some");

        if cmp_start_end == Ordering::Less {
            return RangesRelation::StrictlyAfter;
        }

        let self_cmp = partial_cmp_bounds(&self_start, BoundSide::Start, &self_end, BoundSide::End);
        if self_cmp.is_none() {
            return RangesRelation::None;
        }
        let self_cmp = self_cmp.expect("self ordering should be Some");

        let other_cmp =
            partial_cmp_bounds(&other_start, BoundSide::Start, &other_end, BoundSide::End);
        if other_cmp.is_none() {
            return RangesRelation::None;
        }
        let other_cmp = other_cmp.expect("other ordering should be Some");

        if cmp_end_start == Ordering::Equal
            && self_cmp != Ordering::Equal
            && other_cmp != Ordering::Equal
        {
            return RangesRelation::Meets;
        }
        if cmp_start_end == Ordering::Equal
            && self_cmp != Ordering::Equal
            && other_cmp != Ordering::Equal
        {
            return RangesRelation::IsMet;
        }

        let cmp_start_start = partial_cmp_bounds(
            &self_start,
            BoundSide::Start,
            &other_start,
            BoundSide::Start,
        );
        if cmp_start_start.is_none() {
            return RangesRelation::None;
        }
        let cmp_start_start = cmp_start_start.expect("start-start ordering should be Some");

        let cmp_end_end = partial_cmp_bounds(&self_end, BoundSide::End, &other_end, BoundSide::End);
        if cmp_end_end.is_none() {
            return RangesRelation::None;
        }
        let cmp_end_end = cmp_end_end.expect("end-end ordering should be Some");

        if cmp_start_start == Ordering::Less
            && cmp_end_start == Ordering::Greater
            && cmp_end_end == Ordering::Less
        {
            return RangesRelation::Overlaps;
        }
        if cmp_start_start == Ordering::Greater
            && cmp_start_end == Ordering::Less
            && cmp_end_end == Ordering::Greater
        {
            return RangesRelation::IsOverlapped;
        }
        if cmp_start_start == Ordering::Equal && cmp_end_end == Ordering::Less {
            return RangesRelation::Starts;
        }
        if cmp_start_start == Ordering::Equal && cmp_end_end == Ordering::Greater {
            return RangesRelation::IsStarted;
        }
        if cmp_start_start == Ordering::Greater && cmp_end_end == Ordering::Equal {
            return RangesRelation::Finishes;
        }
        if cmp_start_start == Ordering::Less && cmp_end_end == Ordering::Equal {
            return RangesRelation::IsFinished;
        }
        if cmp_start_start == Ordering::Less && cmp_end_end == Ordering::Greater {
            return RangesRelation::StrictlyContains;
        }
        if cmp_start_start == Ordering::Greater && cmp_end_end == Ordering::Less {
            return RangesRelation::IsStrictlyContained;
        }
        if cmp_start_start == Ordering::Equal && cmp_end_end == Ordering::Equal {
            return RangesRelation::Equal;
        }

        panic!("Impossible case");
    }

    pub fn simplify_mut(&mut self)
    where
        Idx: PartialOrd,
    {
        match self {
            ContinuousRange::Empty => {}
            ContinuousRange::Single(_) => {}
            ContinuousRange::Inclusive(start, end) => {
                if start == end {
                    *self = ContinuousRange::Single(start.clone())
                } else if start > end {
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
            Self::Single(_) => false,

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

impl<Idx: PartialOrd + Clone> From<()> for ContinuousRange<Idx> {
    fn from(_: ()) -> Self {
        Self::empty()
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
            ContinuousRange::Single(value) => value.fmt(fmt)?,
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

impl<Idx> Default for ContinuousRange<Idx> {
    fn default() -> Self {
        Self::Empty
    }
}
