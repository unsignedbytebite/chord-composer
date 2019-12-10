#!/bin/bash

# Run from `chord_composer/examples`

# Build
cd ../
cargo build --release

# Prepare directory
rm -r ./examples/generative_exports -f
mkdir ./examples/generative_exports

# Run
#clear
for i in {0..10}; do
  echo 'Generation:' $i
  ./target/release/chord_composer export ./examples/example_generative_composition.yaml
  mv ./examples/example_generative_composition ./examples/generative_exports/
  mv ./examples/generative_exports/example_generative_composition ./examples/generative_exports/generation_$i
  echo ''
done
