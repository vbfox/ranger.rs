#![allow(clippy::bool_assert_comparison)]

mod range_structs;

pub use range_structs::{
    ContinuousRangeEndExclusive, ContinuousRangeExclusive, ContinuousRangeFromExclusive,
    ContinuousRangeFromInclusive, ContinuousRangeInclusive, ContinuousRangeStartExclusive,
    ContinuousRangeToExclusive, ContinuousRangeToInclusive,
};

mod range;

pub use range::Range;

#[cfg(test)]
mod tests;
