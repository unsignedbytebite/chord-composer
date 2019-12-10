#!/bin/bash

# Build
cd ../
cargo build --release

# Run
clear
./target/release/chord_composer chords