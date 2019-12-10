#!/bin/bash

# Build
cd ../
cargo build --release --features zhn

# Run
clear
./target/release/chord_composer export ./examples/example_composition.yaml