# Rust-based RMRK2.0 consolidator

This is a Rust-built tool that is built to be equivalent to Typescript-built [https://github.com/rmrk-team/rmrk-tools](rmrk-tools-consolidate).  It works for RMRK2.0 only.

## Requirements
Requires subkey commandline utility and jq.  If you can't run `subkey --help` and `jq --help` manually, this won't work.  Installation instructions are [here](https://docs.substrate.io/v3/tools/subkey/).

There will be a rmrk2-rust-consolidate executable in /dist, built for Mac.  Or, clone the repository, install with Cargo (install Rust first) with `cargo build --release`.  Then the executable will live at ./target/release/rmrk2-rust-consolidator.

# Running

Once the binary is built in ./target/release, you can run it directly.  `./target/release/rmrk2-rust-consolidator --help` to see the commands.

The only required argument is the raw input file.  An example input file is chunky-unconsolidated.txt, which is the result of rmrk-tools-fetch on the [Chunkies demo project](https://github.com/rmrk-team/rmrk2-examples).  The simplest execution is `./target/release/rmrk2-rust-consolidator chunky-unconsolidated.txt`.  This will create an output file consolidated-chunky-unconsolidated.txt.  Alternatively, pass an --append arg to either specify the output file name or, if the file already exists, to consolidate additional blocks onto an existing output file.  `./target/release/rmrk2-rust-consolidator chunky-unconsolidated.txt --append my-consolidated-output.json`.