# Website Status Checker

A concurrent command-line tool to check website availability written in Rust.

## Build Instructions

```bash
cargo build --release 
cargo run --release -- --file sites.txt --workers 4 --timeout 5 --retries 2
