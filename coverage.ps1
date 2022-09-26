param ([String] $testName="")

Set-StrictMode -Version Latest

$ErrorActionPreference = "Stop"
function ThrowOnNativeFailure {
    if (-not $?)
    {
        throw 'Native Failure'
    }
}

if (Test-Path './target/coverage/') {
    Remove-Item './target/coverage/' -Recurse -Force
}

# Export the flags needed to instrument the program to collect code coverage.
$env:RUSTFLAGS = "-Cinstrument-coverage"

# Ensure each test runs gets its own profile information by defining the LLVM_PROFILE_FILE environment variable (%p will be replaced by the process ID, and %m by the binary signature):
$env:LLVM_PROFILE_FILE = "target/coverage/ranger-%p-%m.profraw"

# Build & test
cargo build
ThrowOnNativeFailure

cargo test $testName
ThrowOnNativeFailure

# Generate a HTML report in the coverage/ directory.
grcov ./target/coverage/ --binary-path ./target/debug/ -s ./src/ -t html --branch --ignore-not-existing -o ./coverage/
ThrowOnNativeFailure

Invoke-Item -Path 'coverage/index.html'
