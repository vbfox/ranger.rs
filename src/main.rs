use std::{fmt, ops::{self, Bound, RangeBounds}};

// --------------------------------------------------------------------------------------------------------------------
// Range types

#[derive(Clone, PartialEq, Eq, Hash)]
struct ContinuousRangeInclusive<Idx> {
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
struct ContinuousRangeExclusive<Idx> {
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
struct ContinuousRangeEndExclusive<Idx> {
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
        ContinuousRangeEndExclusive { start: r.start, end: r.end }
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
struct ContinuousRangeStartExclusive<Idx> {
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
struct ContinuousRangeFromInclusive<Idx> {
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
struct ContinuousRangeFromExclusive<Idx> {
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
struct ContinuousRangeToInclusive<Idx> {
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
struct ContinuousRangeToExclusive<Idx> {
    /// The upper bound of the range
    pub end: Idx,
}

impl<Idx: fmt::Debug> fmt::Debug for ContinuousRangeToExclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "(-∞..")?;
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
enum Range<Idx> {
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
    ToExclusive(ContinuousRangeToInclusive<Idx>),

    /// A range containing all values
    Full,
}

impl<Idx> Range<Idx> {
    pub fn range_bounds(&self) -> Option<(Bound<&Idx>, Bound<&Idx>)> {
        match self {
            Range::Empty => None,
            Range::Continuous(r) => Some((r.start_bound(), r.end_bound())),
            Range::ContinuousExclusive(r) => Some((r.start_bound(), r.end_bound())),
            Range::ContinuousStartExclusive(r) => Some((r.start_bound(), r.end_bound())),
            Range::ContinuousEndExclusive(r) => Some((r.start_bound(), r.end_bound())),
            Range::From(r) => Some((r.start_bound(), r.end_bound())),
            Range::FromExclusive(r) => Some((r.start_bound(), r.end_bound())),
            Range::To(r) => Some((r.start_bound(), r.end_bound())),
            Range::ToExclusive(r) => Some((r.start_bound(), r.end_bound())),
            Range::Full => Some((Bound::Unbounded, Bound::Unbounded)),
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
        }
        Ok(())
    }
}

fn main() {
    let r = Range::<i32>::Empty;
    println!("Hello, world! {:?}", r);
}

#[cfg(test)]
mod tests {
    use crate::Range;

    #[test]
    pub fn a() {
        let r: Range<_> = (1..5).into();
        assert_eq!(format!("{:?}", r), "[1..5)");
    }

    #[test]
    pub fn b() {
        let r: Range<_> = (1..=5).into();
        assert_eq!(format!("{:?}", r), "[1..5]");
    }

    #[test]
    pub fn c() {
        let r: Range<u32> = (..).into();
        assert_eq!(format!("{:?}", r), "(..)");
    }

    #[test]
    pub fn d() {
        let r: Range<u32> = (1..).into();
        assert_eq!(format!("{:?}", r), "[1..)");
    }

    #[test]
    pub fn e() {
        let r: Range<u32> = (..5).into();
        assert_eq!(format!("{:?}", r), "(..5]");
    }
}