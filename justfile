set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

_default:
  @just --list --justfile {{justfile()}}

build:
    cargo build

test:
    cargo test

clippy *args='':
    cargo clippy --all-targets --all-features --tests --benches "$@" -- "-Dclippy::all" "-Dclippy::pedantic"
