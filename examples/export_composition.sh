#!/bin/bash

# Build
cd ../
cargo build --release

# Run
clear
./target/release/chord_composer export ./examples/example_composition.yaml