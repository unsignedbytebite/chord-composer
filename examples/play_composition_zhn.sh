#!/bin/bash

# Build
cd ../
cargo build --release --features zhn

# Run
clear
./target/release/chord_composer play ./examples/example_composition.yaml