use std::{
    borrow::Borrow,
    fmt,
    ops::{self, Add, Bound, Sub},
};

#[derive(Clone, Hash)]
pub enum Range<Idx> {
    /// A range containing no value
    ///
    /// `[]`
    Empty,

    /// A range between `start` (inclusive) and `end` (inclusive)
    ///
    /// `[start..end]`
    Continuous(Idx, Idx),

    /// A range between `start` (exclusive) and `end` (exclusive)
    ///
    /// `(start..end)`
    ContinuousExclusive(Idx, Idx),

    /// A range between `start` (exclusive) and `end` (inclusive)
    ///
    /// `(start..end]`
    ContinuousStartExclusive(Idx, Idx),

    /// A range between `start` (inclusive) and `end` (exclusive)
    ///
    /// `[start..end)`
    ContinuousEndExclusive(Idx, Idx),

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

    Composite(Vec<Range<Idx>>),
}

pub enum RangesRelation {
    /// The first range is strictly before the second one with no overlap
    ///
    /// ```text
    /// [ A ]
    ///       [ B ]
    /// ```
    StrictlyBefore,

    /// The first range is strictly after the second one with no overlap
    ///
    /// ```text
    ///       [ A ]
    /// [ B ]
    /// ```
    StrictlyAfter,

    ///*
    /// ```text
    /// [ A ]
    ///     [ B ]
    /// ```
    ////
    Meets,

    ///*
    /// ```text
    ///     [ A ]
    /// [ B ]
    /// ```
    ////
    IsMet,

    ///*
    /// ```text
    /// [ A ]
    ///   [ B ]
    /// ```
    ////
    Overlaps,

    ///*
    /// ```text
    ///   [ A ]
    /// [ B ]
    /// ```
    ////
    IsOverlapped,

    ///*
    /// ```text
    /// [ A ]
    /// [   B   ]
    /// ```
    ////
    Starts,

    ///*
    /// ```text
    /// [   A   ]
    /// [ B ]
    /// ```
    ////
    IsStarted,

    ///*
    /// ```text
    /// [   A   ]
    ///   [ B ]
    /// ```
    ////
    StrictlyContains,

    ///*
    /// ```text
    ///   [ A ]
    /// [   B   ]
    /// ```
    ////
    IsStrictlyContained,

    ///*
    /// ```text
    ///     [ A ]
    /// [   B   ]
    /// ```
    ////
    Finishes,

    ///*
    /// ```text
    /// [   A   ]
    ///     [ B ]
    /// ```
    ////
    IsFinished,

    ///*
    /// ```text
    /// [ A ]
    /// [ B ]
    /// ```
    ////
    Equal,
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
    pub fn continuous(start: Idx, end: Idx) -> Range<Idx>
    where
        Idx: PartialOrd,
    {
        if start > end {
            Range::Empty
        } else {
            Range::Continuous(start, end)
        }
    }

    /// A range between `start` (exclusive) and `end` (exclusive)
    ///
    /// `(start..end)`
    #[must_use]
    pub fn continuous_exclusive(start: Idx, end: Idx) -> Range<Idx>
    where
        Idx: PartialOrd,
    {
        if start >= end {
            Range::Empty
        } else {
            Range::ContinuousExclusive(start, end)
        }
    }

    /// A range between `start` (exclusive) and `end` (inclusive)
    ///
    /// `(start..end]`
    #[must_use]
    pub fn continuous_start_exclusive(start: Idx, end: Idx) -> Range<Idx>
    where
        Idx: PartialOrd,
    {
        if start >= end {
            Range::Empty
        } else {
            Range::ContinuousStartExclusive(start, end)
        }
    }

    /// A range between `start` (inclusive) and `end` (exclusive)
    ///
    /// `[start..end)`
    #[must_use]
    pub fn continuous_end_exclusive(start: Idx, end: Idx) -> Range<Idx>
    where
        Idx: PartialOrd,
    {
        if start >= end {
            Range::Empty
        } else {
            Range::ContinuousEndExclusive(start, end)
        }
    }

    /// A range starting from `start` (inclusive)
    ///
    /// `[start..)`
    #[must_use]
    pub fn from(start: Idx) -> Range<Idx> {
        Range::From(start)
    }

    /// A range starting from `start` (exclusive)
    ///
    /// `(start..)`
    #[must_use]
    pub fn from_exclusive(start: Idx) -> Range<Idx> {
        Range::FromExclusive(start)
    }

    /// A range ending with `end` (inclusive)
    ///
    /// `(..end]`
    #[must_use]
    pub fn to(end: Idx) -> Range<Idx> {
        Range::To(end)
    }

    /// A range ending with `end` (exclusive)
    ///
    /// `(..end)`
    #[must_use]
    pub fn to_exclusive(end: Idx) -> Range<Idx> {
        Range::ToExclusive(end)
    }

    /// A range containing all values
    #[must_use]
    pub fn full() -> Range<Idx> {
        Range::Full
    }

    #[must_use]
    pub fn composite(items: Vec<Range<Idx>>) -> Range<Idx>
    where
        Idx: PartialOrd,
    {
        match items.len() {
            0 => Range::Empty,
            1 => items.into_iter().next().unwrap(),
            _ => {
                let mut new_items = vec![];

                for item in items.into_iter() {
                    match item {
                        Range::Empty => continue,
                        Range::Full => {
                            return Range::Full;
                        }
                        Range::Composite(sub_items) => {
                            new_items.extend(sub_items);
                        }
                        _ => new_items.push(item),
                    }
                }

                match new_items.len() {
                    0 => Range::Empty,
                    1 => new_items.into_iter().next().unwrap(),
                    _ => Range::Composite(new_items),
                }
            }
        }
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
            Self::Continuous(start, end) => Some((Bound::Included(start), Bound::Included(end))),
            Self::ContinuousExclusive(start, end) => {
                Some((Bound::Excluded(start), Bound::Excluded(end)))
            }
            Self::ContinuousStartExclusive(start, end) => {
                Some((Bound::Excluded(start), Bound::Included(end)))
            }
            Self::ContinuousEndExclusive(start, end) => {
                Some((Bound::Included(start), Bound::Excluded(end)))
            }
            Self::From(start) => Some((Bound::Included(start), Bound::Unbounded)),
            Self::FromExclusive(start) => Some((Bound::Excluded(start), Bound::Unbounded)),
            Self::To(end) => Some((Bound::Unbounded, Bound::Included(end))),
            Self::ToExclusive(end) => Some((Bound::Unbounded, Bound::Excluded(end))),
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
            Self::Continuous(start, end) => {
                let value = value.borrow();
                value >= start && value <= end
            }
            Self::ContinuousExclusive(start, end) => {
                let value = value.borrow();
                value > start && value < end
            }
            Self::ContinuousStartExclusive(start, end) => {
                let value = value.borrow();
                value > start && value <= end
            }
            Self::ContinuousEndExclusive(start, end) => {
                let value = value.borrow();
                value >= start && value < end
            }
            Self::From(start) => value.borrow() >= start,
            Self::FromExclusive(start) => value.borrow() > start,
            Self::To(end) => value.borrow() <= end,
            Self::ToExclusive(end) => value.borrow() < end,
            Self::Full => true,
            Self::Composite(r) => {
                let value = value.borrow();
                r.iter().any(|x| x.contains(value))
            }
        }
    }

    #[must_use]
    pub fn union(self, other: Range<Idx>) -> Range<Idx>
    where
        Idx: PartialOrd,
    {
        // TODO: Quite a few cases can be optimized, specialized, ...
        // TODO: Also maybe the ranges should be kept sorted in composite
        match (self, other) {
            (Range::Empty, r) | (r, Range::Empty) => r,
            (Range::Full, _) | (_, Range::Full) => Range::Full,
            (r1, r2) => Range::composite(vec![r1, r2]),
        }
    }

    #[must_use]
    pub fn intersection(self, _other: Range<Idx>) -> Range<Idx> {
        todo!()
    }

    #[must_use]
    pub fn difference(self, _other: Range<Idx>) -> Range<Idx> {
        todo!()
    }

    #[must_use]
    pub fn overlaps(self, _other: Range<Idx>) -> bool {
        todo!()
    }

    #[must_use]
    pub fn compare(self, _other: Range<Idx>) -> RangesRelation {
        todo!()
    }

    #[must_use]
    pub fn simplify(self) -> Range<Idx>
    where
        Idx: PartialOrd,
    {
        match self {
            Range::Empty => self,
            Range::Continuous(ref start, ref end) => {
                if start > end {
                    Range::Empty
                } else {
                    self
                }
            }
            Range::ContinuousExclusive(_, _) => todo!(),
            Range::ContinuousStartExclusive(_, _) => todo!(),
            Range::ContinuousEndExclusive(_, _) => todo!(),
            Range::From(_) => self,
            Range::FromExclusive(_) => self,
            Range::To(_) => self,
            Range::ToExclusive(_) => self,
            Range::Full => self,
            Range::Composite(_) => todo!(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
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

impl<Idx: PartialOrd> Add<Range<Idx>> for Range<Idx> {
    type Output = Range<Idx>;

    fn add(self, other: Range<Idx>) -> Range<Idx> {
        self.union(other)
    }
}

impl<Idx: PartialOrd> Sub<Range<Idx>> for Range<Idx> {
    type Output = Range<Idx>;

    fn sub(self, other: Range<Idx>) -> Range<Idx> {
        self.difference(other)
    }
}

impl<Idx: PartialEq> PartialEq for Range<Idx> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Continuous(l0, l1), Self::Continuous(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::ContinuousExclusive(l0, l1), Self::ContinuousExclusive(r0, r1)) => {
                l0 == r0 && l1 == r1
            }
            (Self::ContinuousStartExclusive(l0, l1), Self::ContinuousStartExclusive(r0, r1)) => {
                l0 == r0 && l1 == r1
            }
            (Self::ContinuousEndExclusive(l0, l1), Self::ContinuousEndExclusive(r0, r1)) => {
                l0 == r0 && l1 == r1
            }
            (Self::From(l0), Self::From(r0)) => l0 == r0,
            (Self::FromExclusive(l0), Self::FromExclusive(r0)) => l0 == r0,
            (Self::To(l0), Self::To(r0)) => l0 == r0,
            (Self::ToExclusive(l0), Self::ToExclusive(r0)) => l0 == r0,
            (Self::Composite(l0), Self::Composite(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<Idx> From<ops::RangeFull> for Range<Idx> {
    fn from(_: ops::RangeFull) -> Self {
        Self::full()
    }
}

impl<Idx: PartialOrd> From<ops::Range<Idx>> for Range<Idx> {
    fn from(r: ops::Range<Idx>) -> Self {
        Self::continuous_end_exclusive(r.start, r.end)
    }
}

impl<Idx: PartialOrd> From<ops::RangeInclusive<Idx>> for Range<Idx> {
    fn from(r: ops::RangeInclusive<Idx>) -> Self {
        let (start, end) = r.into_inner();
        Self::continuous(start, end)
    }
}

impl<Idx> From<ops::RangeFrom<Idx>> for Range<Idx> {
    fn from(r: ops::RangeFrom<Idx>) -> Self {
        Self::from(r.start)
    }
}

impl<Idx> From<ops::RangeToInclusive<Idx>> for Range<Idx> {
    fn from(r: ops::RangeToInclusive<Idx>) -> Self {
        Self::to(r.end)
    }
}

impl<Idx> From<ops::RangeTo<Idx>> for Range<Idx> {
    fn from(r: ops::RangeTo<Idx>) -> Self {
        Self::to_exclusive(r.end)
    }
}

impl<Idx: fmt::Debug> fmt::Debug for Range<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Range::Empty => write!(fmt, "[]")?,
            Range::Full => write!(fmt, "(..)")?,
            Range::Continuous(start, end) => {
                write!(fmt, "[")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, "]")?;
            }
            Range::ContinuousExclusive(start, end) => {
                write!(fmt, "(")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, ")")?;
            }
            Range::ContinuousStartExclusive(start, end) => {
                write!(fmt, "(")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, "]")?;
            }
            Range::ContinuousEndExclusive(start, end) => {
                write!(fmt, "[")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, ")")?;
            }
            Range::From(start) => {
                write!(fmt, "[")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                write!(fmt, ")")?;
            }
            Range::FromExclusive(start) => {
                write!(fmt, "(")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                write!(fmt, ")")?;
            }
            Range::To(end) => {
                write!(fmt, "(")?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, "]")?;
            }
            Range::ToExclusive(end) => {
                write!(fmt, "(")?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, ")")?;
            }
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
