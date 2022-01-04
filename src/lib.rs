#![allow(clippy::bool_assert_comparison)]

mod range_structs;

pub use range_structs::{
    ContinuousRangeInclusive,
    ContinuousRangeExclusive,
    ContinuousRangeStartExclusive,
    ContinuousRangeEndExclusive,
    ContinuousRangeFromInclusive,
    ContinuousRangeFromExclusive,
    ContinuousRangeToInclusive,
    ContinuousRangeToExclusive
};

mod range;

pub use range::Range;

#[cfg(test)]
mod tests;
