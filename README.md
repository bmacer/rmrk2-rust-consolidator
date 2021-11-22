# Rust-based RMRK2.0 consolidator

This is a Rust-built tool that is built to be equivalent to Typescript-built [https://github.com/rmrk-team/rmrk-tools](rmrk-tools-consolidate).  It works for RMRK2.0 only.

## Requirements
Requires subkey commandline utility and jq.  If you can't run `subkey --help` and `jq --help` manually, this won't work.  Installation instructions are [here](https://docs.substrate.io/v3/tools/subkey/).

There will be a rmrk2-rust-consolidate executable in /dist, built for Mac.  Or, clone the repository, install with Cargo (install Rust first) with `cargo build --release`.  Then the executable will live at ./target/release/rmrk2-rust-consolidator.

