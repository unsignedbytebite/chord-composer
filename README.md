# chord-composer

> A music composition tool for structuring chord progressions and patterns, written in Rust.

The philosophy behind **Chord Composer** is to make a lightweight, portable, and accessible tool to structure chord patterns and progressions for music composition ideas. It must fit into common digital music writing workflows and not hinder the creative process of the user.

![](http://0x0a141e.co.uk/github/chord-composer/screen00.png)

## Features

- Describe compositions with patterns in `YAML`.
- Export _composition patterns_ to `MIDI` clips.
- Playback _composition patterns_ with a piano in the command line.
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

## Latest Binaries

- Linux: _TODO_
- Mac: _TODO_
- Window: _TODO_

## Command line arguments

> `./chord_composer --help`

```shell
Chord Composer 0.2.4
Cj <unsignedbytebite@gmail.com>
A music composition tool for structuring chord progressions and patterns.

USAGE:
    chord_composer.exe [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    chords      Print the list of supported chords and their intervals.
    export      Export composition patterns to .MID.
    help        Prints this message or the help of the given subcommand(s)
    play        Playback patterns in a composition.
    template    Export a composition arrangement YAML template
```

> `./chord_composer play --help`

```
Playback patterns in a composition.

USAGE:
    chord_composer.exe play [FLAGS] <COMPOSITION_FILE>

FLAGS:
    -h, --help         Prints help information
        --metronome    Play a metronome during playback.
    -V, --version      Prints version information

ARGS:
    <COMPOSITION_FILE>    The YAML composition arrangement file.
```


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

## Build Features

- `eng` - Build with English strings.
- `pt` - Build with Portugeuse strings.
- `zhn` - Build with Chinese(Simplified) strings.
- `no-audio` - Build without the support of audio playback.

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

