use std::{
    fmt,
    ops::{self, Bound, RangeBounds},
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ContinuousRangeInclusive<Idx> {
    /// The lower bound of the range
    pub start: Idx,

    /// The upper bound of the range
    pub end: Idx,
}

impl<Idx> ContinuousRangeInclusive<Idx> {
    #[must_use]
    pub fn is_empty(&self) -> bool
    where
        Idx: PartialOrd,
    {
        self.start > self.end
    }
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

impl<Idx> ContinuousRangeExclusive<Idx> {
    #[must_use]
    pub fn is_empty(&self) -> bool
    where
        Idx: PartialOrd,
    {
        self.start >= self.end
    }
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

impl<Idx> ContinuousRangeEndExclusive<Idx> {
    #[must_use]
    pub fn is_empty(&self) -> bool
    where
        Idx: PartialOrd,
    {
        self.start >= self.end
    }
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

impl<Idx> ContinuousRangeStartExclusive<Idx> {
    #[must_use]
    pub fn is_empty(&self) -> bool
    where
        Idx: PartialOrd,
    {
        self.start >= self.end
    }
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

impl<Idx> ContinuousRangeFromInclusive<Idx> {
    #[must_use]
    pub fn is_empty(&self) -> bool
    where
        Idx: PartialOrd,
    {
        false
    }
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

impl<Idx> ContinuousRangeFromExclusive<Idx> {
    #[must_use]
    pub fn is_empty(&self) -> bool
    where
        Idx: PartialOrd,
    {
        false
    }
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

impl<Idx> ContinuousRangeToInclusive<Idx> {
    #[must_use]
    pub fn is_empty(&self) -> bool
    where
        Idx: PartialOrd,
    {
        false
    }
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

impl<Idx> From<ops::RangeToInclusive<Idx>> for ContinuousRangeToInclusive<Idx> {
    fn from(r: ops::RangeToInclusive<Idx>) -> Self {
        ContinuousRangeToInclusive { end: r.end }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ContinuousRangeToExclusive<Idx> {
    /// The upper bound of the range
    pub end: Idx,
}

impl<Idx> ContinuousRangeToExclusive<Idx> {
    #[must_use]
    pub fn is_empty(&self) -> bool
    where
        Idx: PartialOrd,
    {
        false
    }
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

impl<Idx> From<ops::RangeTo<Idx>> for ContinuousRangeToExclusive<Idx> {
    fn from(r: ops::RangeTo<Idx>) -> Self {
        ContinuousRangeToExclusive { end: r.end }
    }
}
