# Range ranger

[![Build](https://github.com/vbfox/ranger.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/vbfox/ranger.rs/actions/workflows/ci.yml)

**This library is a WORK IN PROGRESS experiment into rust APIs for me**

Range ranger is a range manipulation library supporting multiple type of ranges under a single type:

* Empty
* Continuous (containing all values between min and max)
  * Bounds can be inclusive or exclusive
  * Bounds can be finite or infinite
* Full ranges
* List of values
* Single value
* Composite ranges (Union of multiple of theses)

With support for the following operations:

* Union
* Intersection
* Difference
* Contains range
* Contains value
* Overlaps
* Simplification

The range type is an enum of all the possible range subtypes.
The default behaviour for ranges constructed via methods is to be simplified and sorted but non-simplified ranges can be constructed by creating the enum members directly.

## Running coverage

```powershell
# Requirements
cargo install grcov to install
rustup component add llvm-tools-preview

# Running
./coverage.ps1
```

## See also

* https://gitlab.com/bit-refined/ranges/ Another rust range library
* https://www.postgresql.org/docs/current/functions-range.html PostgreSQL operation on ranges
* https://www.postgresql.org/docs/14/rangetypes.html PostgreSQL range types
