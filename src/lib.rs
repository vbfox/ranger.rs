#![allow(
    clippy::bool_assert_comparison,
    clippy::match_same_arms,
    clippy::module_name_repetitions,
    // To reactivate when the fix for not matching invalid cases is in
    // https://github.com/rust-lang/rust-clippy/issues/11403
    clippy::ignored_unit_patterns
)]

mod continuous;
mod range;
mod relation;

pub use continuous::ContinuousRange;
pub use range::Range;
pub use relation::RangesRelation;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod continuous_tests;
