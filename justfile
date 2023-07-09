set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

_default:
  @just --list --justfile {{justfile()}}

build:
    cargo build

clippy:
    cargo clippy --all-targets --all-features --tests --benches -- "-Dclippy::all" "-Dclippy::pedantic"
