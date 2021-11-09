# Rust-based RMRK2.0 consolidator

This is an attempt to rebuild the rmrk-tools-consolidate script.

Phase one is to create a positive case, where this consolidator produces the same results in the Chunky example.
Phase two is to figure out the positive cases not included in the Chunky example.
Phase three is figuring out the failure cases.

at this point, to run, just `cargo run` inside the main folder.  the input is hard-coded as `chunky-unconsolidated.txt` and will output to `chunky-unconsolidated.json`.  the test for phase one is comparing this output to `chunky-perfect.json`.