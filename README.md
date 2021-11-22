[X] Accept input argument
[X] Accept --output argument
[X] Load --output arg if exists (for appending)

[X] ACCEPT
[X] BASE
[X] BURN
[X] BUY
[X] CHANGEISSUER
[X] CREATE
[X] EMOTE
[X] EQUIP
[X] EQUIPPABLE
[X] LIST
[N] LOCK
[X] MINT
[X] RESADD
[X] SEND
[X] SETPRIORITY
[X] SETPROPERTY
[N] THEMEADD

# Rust-based RMRK2.0 consolidator

Requires subkey commandline utility and jq.  If you can't run `subkey --help` and `jq --help` manually, this won't work.

This is an attempt to rebuild the rmrk-tools-consolidate script.

Phase one is to create a positive case, where this consolidator produces the same results in the Chunky example.
Phase two is to figure out the positive cases not included in the Chunky example.
Phase three is figuring out the failure cases.

at this point, to run, just `cargo run` inside the main folder.  the input is hard-coded as `chunky-unconsolidated.txt` and will output to `chunky-unconsolidated.json`.  the test for phase one is comparing this output to `chunky-perfect.json`.