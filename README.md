# chord-composer

> A music composition tool for structuring chord progressions and patterns, written in Rust.

The philosophy behind **Chord Composer** is to make a lightweight, portable, and accessible tool to structure chord patterns and progressions for music composition ideas. It must fit into common digital music writing workflows and not hinder the creative process of the user.

## Features

- Describe compositions with patterns in `YAML`.
- Export _composition patterns_ to `MIDI` clips.
- Playback _composition patterns_ in the command line.
- Command line interface.
- Cross-platform.
- Languages:
  - English
  - Português
  - 简体中文

## Future Work

- Build a terminal user interface.
- Develop better audio engine and instrument sampler.
- Support `MIDI` routing.
- Explore the need to support common trackers.
- Support more languages (including pirate).

## Command line arguments

- `play <COMPOSITION_FILE>` : Playback a _composition_.
  - Options:
    - `--metronome` : Play a metronome during playback.
  - Example:
    ```shell
    ./chord_composer play ./my_idea.yaml --metronome
    ```
- `export <COMPOSITION_FILE>` : Export a _composition_ to _pattern_ `MIDI` clips.
  - Example:
    ```shell
    ./chord_composer export ./my_idea.yaml
    ```
- `chords` : Prints to the terminal the internal supported _chord keywords_ and their intervals.
  - Example:
    ```shell
    ./chord_composer chords
    ```
- `template <EXPORT_PATH>` : Export a template of the _composition parameters_ `YAML` file.
  - Example:
    ```shell
    ./chord_composer template ./my_new_idea.yaml
    ```
- `--help` : Prints to the terminal the list of supported commands.
  - Example:
    ```shell
    ./chord_composer --help
    ```

## Latest Binaries

- Linux: _TODO_
- Windows: _TODO_
- Mac: _TODO_

## Build steps

1. Install [Rust](https://www.rust-lang.org/).
2. Clone this repo with [Git](https://git-scm.com/). _e.g_
    ```shell
    git clone https://github.com/unsignedbytebite/chord-composer.git 
    ```
3. Run `cargo test` in the repo's directory to ensure it's working. _e.g_
    ```shell
    cargo test
    ```
4. Open `./examples/` and run/look at the example scripts to see how to use **Chord Composer** _e.g_
    ```shell
    cd ./examples/
    ./play_composition.sh
    ```

## Composition Parameters `YAML` file

The schema for the _composition parameters_ `YAML` file are outlined below in this template.

```yaml
# Name of the composition
name: default_composition

# The default master parameters of the composition. 
# New master pattern can be assigned to a pattern that overrides
# the default master values.
master:
    # The musical key to transpose the chords. 
    # Supported values: C, C#, D, D#, E, F, F#, G, G#, A, A#, B
    key: F# 

    # The beats per minute of the composition.
    time: 120

    # The time signature of the composition.
    # Beat numerator supported values: must be > 0.
    # Beat denominator supported values: 2, 4, 8, 16, 32, 64 
    # e.g 3/8 is supported, 0/7 is not supported.
    signature: [4, 4]

# Composition defined chords.
chords:
    # [chord_name, [chord intervals]].
    - [custom1, [0, 3, 8]]
    - [custom2, [0, 5]]

# The composition's chord patterns/progressions.
patterns:
    - name: part_a
      # Each pattern event = [bar, beat, beat interval, chord name, chord transpose].
      pattern:
          - [1, 1, 1, MAJOR_SEVENTH, 0]
          - [1, 3, 1, custom1, 0]
          - [2, 1, 1, MAJOR_NINTH, 0]
          - [2, 3, 1, custom1, 0]
          - [3, 1, 1, MAJOR_SEVENTH, 3]
          - [3, 2, 1, custom1, 0]
          - [4, 1, 1, MAJOR_NINTH, -3]
          - [4, 2, 1, ?, 0] # ? = Select a random user defined chord.

    - name: part_b
      master:
          signature: [4, 8]
          key: C#
          time: 69
      # Each pattern event = [bar, beat, beat interval, chord name, chord transpose].
      pattern:
          - [1, 1, 1, MAJOR_SEVENTH, 0]
          - [1, 2, 1, custom1, 0]
          - [2, 1, 1, MAJOR_NINTH, 0]
          - [2, 2, 1, custom1, 0]
          - [3, 1, 1, MAJOR_SEVENTH, 3]
          - [3, 2, 1, custom1, 0]
          - [4, 1, 1, MAJOR_NINTH, -3]
          - [4, 2, 1, ??, 0] #?? = Select a random chord from user defined and internal defined chord.
```

