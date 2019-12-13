#!/bin/bash

# Build
cd ../
cargo build --release

# Run
clear
./target/release/chord_composer play ./examples/example_composition.yaml --metronome --ticker-beat
