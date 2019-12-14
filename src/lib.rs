#[macro_use]
extern crate serde_derive;

mod audio;
mod io;
pub mod performance;
pub mod theory;

use music_timer::{music_time, time_signature};
use performance::performance_engine;
use std::{io::Write, path::Path};
use theory::{chords, composition, notes};

/// Enum for differing resulting failures.
#[derive(Debug, PartialEq)]
pub enum FailResult {
  Deserialize,
  ExportMIDI,
  ExportTemplate,
  NoPatterns,
  EmptyPatterns,
  TimeReverse(music_time::MusicTime, usize, String),
  UnreachableTime(music_time::MusicTime, usize, String),
  TimeSignature(time_signature::TimeSignature),
  LoadSampler,
}

/// Enum for differing resulting successes.
#[derive(Debug, PartialEq)]
pub enum SuccessResult {
  Export(Vec<String>),
  ExportTemplate,
  Playback,
}

/// Returns all the internal chord keywords supported in composition yaml files.
pub fn get_chord_keywords() -> Vec<&'static str> {
  theory::chords::chord_to_string_array()
}

/// Export composition patterns to midi files.
///
/// # Arguments
/// * `composition_path` - The file path to the composition yaml file.
pub fn export_composition_to_midi(composition_path: &str) -> Result<SuccessResult, FailResult> {
  let composition_parameters = io::deseralizer::deserialize_file(composition_path)?;

  let parent_directory = Path::new(composition_path)
    .parent()
    .unwrap_or(Path::new("./"))
    .to_str()
    .unwrap_or("./");

  let composition = parameters_to_composition(&composition_parameters)?;
  io::exporter::export_composition(&composition, parent_directory)
}

/// Play composition patterns.
///
/// # Argument
/// * `composition_path` - The file path to the composition yaml file.
/// * `performance_state` - The composition playback performance state.
/// * `is_metronome_enabled` - If `true` a metronome will be played on playback.
pub fn play<State: performance_engine::PerformanceState>(
  composition_path: &str,
  performance_state: &mut State,
  is_metronome_enabled: bool,
) -> Result<SuccessResult, FailResult> {
  let composition_parameters = io::deseralizer::deserialize_file(composition_path)?;
  let composition = parameters_to_composition(&composition_parameters)?;

  let mut performance_engine =
    performance_engine::PerformanceEngine::new(&composition, performance_state)?;

  performance_engine.set_metronome_enabled(is_metronome_enabled);
  performance_engine.run();

  Ok(SuccessResult::Playback)
}

/// Export a template composition yaml file.
///
/// # Arguments
/// * `path` - The path to export the template to.
pub fn export_template(path: &str) -> Result<SuccessResult, FailResult> {
  let mut file = match std::fs::File::create(path) {
    Ok(file) => Ok(file),
    _ => Err(FailResult::ExportTemplate),
  }?;

  let write_out = file.write_all(
    br#"
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
          signature: [3, 4]
          key: C#
          time: 69
      # Each pattern event = [bar, beat, beat interval, chord name, chord transpose].
      pattern:
          - [1, 1, 1, MAJOR_SEVENTH, 0]
          - [1, 2, 1, custom1, 0]
          - [2, 1, 5, MAJOR_NINTH, 0]
          - [2, 2, 1, custom1, 0]
          - [3, 1, 5, MAJOR_SEVENTH, 3]
          - [3, 2, 1, custom1, 0]
          - [4, 1, 1, MAJOR_NINTH, -3]
          - [4, 2, 1, ??, 0] #?? = Select a random chord from user defined and internal defined chord.
  "#,
  );

  match write_out {
    Ok(()) => Ok(SuccessResult::ExportTemplate),
    _ => Err(FailResult::ExportTemplate),
  }
}

/// Convert deserialized composition parameters to a `Composition` data type that.
///
/// # Arguments
/// * `params` - The `CompositionParameters` to convert into a `Composition`.
fn parameters_to_composition(
  params: &io::deseralizer::CompositionParameters,
) -> Result<composition::Composition, crate::FailResult> {
  let default_master: io::deseralizer::MasterParameters = match params.get_master() {
    Some(master) => master.clone(),
    None => io::deseralizer::MasterParameters::default(),
  };

  let mut composition_result = Ok(composition::Composition::new(&params.get_name()));
  let mut count = 0;

  match params.get_patterns() {
    None => Err(crate::FailResult::NoPatterns),
    Some(patterns) => {
      if patterns.is_empty() {
        Err(crate::FailResult::EmptyPatterns)
      } else {
        for pattern in patterns {
          if composition_result.is_err() {
            break;
          }

          match pattern.get_pattern() {
            Some(pattern_pattern) => {
              let pattern_master = match pattern.get_master() {
                Some(pattern_master) => {
                  io::deseralizer::MasterParameters::from_overrides(&default_master, pattern_master)
                }
                _ => default_master.clone(),
              };

              let name = match pattern.get_name() {
                Some(name) => name.to_string(),
                _ => format!("unnamed_pattern_{}", count),
              };
              let composition_pattern = {
                let mut pattern = {
                  let bpm = pattern_master.get_time_or_default();
                  let (numerator, denominator) = pattern_master.get_signature_or_default();
                  let time_signature = time_signature::TimeSignature::new(numerator, denominator);

                  if !time_signature.is_valid() {
                    composition_result = Err(FailResult::TimeSignature(time_signature));
                    break;
                  }

                  composition::Pattern::new(name, bpm, time_signature)
                };

                let additional_chords = match params.get_custom_chords() {
                  Some(custom_chords) => custom_chords.clone(),
                  None => Vec::new(),
                };

                for (bar, beat, beat_interval, chord_string, transpose) in pattern_pattern {
                  let chord_notes = {
                    let mut chord_intervals = chords::IntervalChord::from_string_with_custom(
                      chord_string,
                      &additional_chords,
                    );

                    let key_offset = {
                      let key_string = pattern_master.get_key_or_default();
                      let key = notes::string_to_key(&key_string);
                      notes::key_to_index(key) as i8
                    };
                    const PLAYBACK_OCTAVE: i8 = 3;
                    chord_intervals
                      .transpose(key_offset)
                      .transpose(*transpose)
                      .transpose_octave(PLAYBACK_OCTAVE - 1)
                      .to_midi()
                  };

                  let time = music_time::MusicTime::new(*bar, *beat, *beat_interval);

                  const INTERVAL_RESOLUTION: u8 = 16;
                  let unreachable_beat_interval = time.get_beat_interval()
                    > INTERVAL_RESOLUTION / 2
                    || time.get_beat_interval() == 0;
                  let unreachable_beat = time.get_beat()
                    > pattern.get_time_signature().get_numerator()
                    || time.get_beat() == 0;
                  let unreachable_bar = time.get_bar() == 0;

                  if unreachable_bar || unreachable_beat_interval || unreachable_beat {
                    composition_result = Err(FailResult::UnreachableTime(
                      time,
                      pattern.len(),
                      chord_string.to_string(),
                    ));
                  } else if pattern.len() != 0 {
                    let previous_time = pattern.get(pattern.len() - 1).0;
                    let time_does_not_advance = time == previous_time;

                    if time_does_not_advance {
                      composition_result = Err(FailResult::UnreachableTime(
                        time,
                        pattern.len(),
                        chord_string.to_string(),
                      ));
                    } else {
                      let reverse_time_flow = time < previous_time;
                      if reverse_time_flow {
                        composition_result = Err(FailResult::TimeReverse(
                          time,
                          pattern.len(),
                          chord_string.to_string(),
                        ));
                      }
                    }
                  }

                  pattern.push_event(time, chord_notes);
                }
                pattern
              };

              match &mut composition_result {
                Ok(composition) => composition.push_pattern(composition_pattern),
                _ => break,
              }

              count += 1;
            }
            None => composition_result = Err(FailResult::NoPatterns),
          }
        }

        composition_result
      }
    }
  }
}

#[test]
fn test_new_composition() {
  let params = io::deseralizer::deserialize_string(
    r#"
      name: bc_000_a

      # Can be overridden by patterns
      master:
          key: D             
          time: 128
          signature: [3, 4]
     
      chords:
          - [custom1, [0, 3, 8]]

      patterns:
          - name: part_a
            pattern:
                - [1,1,1, MAJOR_SEVENTH, 0]
                - [1,3,1, custom1, 0]
                - [2,1,1, MAJOR_NINTH, 0]
                - [2,3,1, custom1, 0]
                - [3,1,1, MAJOR_SEVENTH, 3]
                - [3,2,1, custom1, 0]
                - [4,1,1, MAJOR_NINTH, -3]
                - [4,2,1, custom1, -3]

          - name: part_b
            master:
                signature: [4, 8]
                key: C#
                time: 69
            pattern:
                - [1,1,1, MAJOR_SEVENTH, 0]
                - [1,2,1, custom1, 0]
                - [2,1,1, MAJOR_NINTH, 0]
                - [2,2,1, custom1, 0]
                - [3,1,1, MAJOR_SEVENTH, 3]
                - [3,2,1, custom1, 0]
                - [4,1,1, MAJOR_NINTH, -3]
                - [4,2,1, custom1, 0]

        "#,
  );

  assert_ne!(params, Err(crate::FailResult::Deserialize));

  let compo = parameters_to_composition(&params.unwrap()).unwrap();

  assert_eq!(compo.len(), 2);
  assert_eq!(compo.get(0).len(), 8);
  assert_eq!(compo.get(1).len(), 8);

  assert_eq!(compo.get(0).get_bpm(), 128);
  assert_eq!(
    compo.get(0).get_time_signature(),
    time_signature::TimeSignature::new(3, 4)
  );

  assert_eq!(compo.get(1).get_bpm(), 69);
  assert_eq!(
    compo.get(1).get_time_signature(),
    time_signature::TimeSignature::new(4, 8)
  );

  let (time, notes) = compo.get(0).get(0);
  assert_eq!(time, &music_time::MusicTime::new(1, 1, 1));
  assert_eq!(notes, &vec![50, 54, 57, 61]);

  let (time, notes) = compo.get(0).get(1);
  assert_eq!(time, &music_time::MusicTime::new(1, 3, 1));
  assert_eq!(notes, &vec![50, 53, 58]);

  let (time, notes) = compo.get(0).get(2);
  assert_eq!(time, &music_time::MusicTime::new(2, 1, 1));
  assert_eq!(notes, &vec![50, 54, 57, 61, 52]);

  let (time, notes) = compo.get(0).get(7);
  assert_eq!(time, &music_time::MusicTime::new(4, 2, 1));
  assert_eq!(notes, &vec![47, 50, 55]);

  let (time, notes) = compo.get(1).get(0);
  assert_eq!(time, &music_time::MusicTime::new(1, 1, 1));
  assert_eq!(notes, &vec![49, 53, 56, 60]);

  let (time, notes) = compo.get(1).get(1);
  assert_eq!(time, &music_time::MusicTime::new(1, 2, 1));
  assert_eq!(notes, &vec![49, 52, 57]);

  let (time, notes) = compo.get(1).get(2);
  assert_eq!(time, &music_time::MusicTime::new(2, 1, 1));
  assert_eq!(notes, &vec![49, 53, 56, 60, 51]);

  let (time, notes) = compo.get(1).get(7);
  assert_eq!(time, &music_time::MusicTime::new(4, 2, 1));
  assert_eq!(notes, &vec![49, 52, 57]);
}

#[test]
fn test_flow_reverse() {
  let params = io::deseralizer::deserialize_string(
    r#"
      patterns:
          - name: part_a
            pattern:
                - [1,3,1, MAJOR_SEVENTH, 0]
                - [1,2,1, custom1, 0]
        "#,
  );
  assert_ne!(params, Err(crate::FailResult::Deserialize));

  let compo = parameters_to_composition(&params.unwrap());

  match compo {
    Err(FailResult::TimeReverse(music_time, index, chord)) => {
      assert_eq!(music_time, music_time::MusicTime::new(1, 2, 1));
      assert_eq!(index, 1);
      assert_eq!(chord, "custom1".to_string());
    }
    _ => assert_eq!(false, true),
  }
}

#[test]
fn test_unreachable_time() {
  {
    let params = io::deseralizer::deserialize_string(
      r#"
      patterns:
          - name: part_a
            pattern:
                - [1,3,1, MAJOR_SEVENTH, 0]
                - [1,2,9, custom1, 0]
        "#,
    );
    assert_ne!(params, Err(crate::FailResult::Deserialize));

    let compo = parameters_to_composition(&params.unwrap());

    match compo {
      Err(FailResult::UnreachableTime(music_time, index, chord)) => {
        assert_eq!(music_time, music_time::MusicTime::new(1, 2, 9));
        assert_eq!(index, 1);
        assert_eq!(chord, "custom1".to_string());
      }
      _ => assert_eq!(false, true),
    }
  }
  {
    let params = io::deseralizer::deserialize_string(
      r#"
      patterns:
          - name: part_a
            pattern:
                - [1,3,1, MAJOR_SEVENTH, 0]
                - [1,9,1, custom1, 0]
        "#,
    );
    assert_ne!(params, Err(crate::FailResult::Deserialize));

    let compo = parameters_to_composition(&params.unwrap());

    match compo {
      Err(FailResult::UnreachableTime(music_time, index, chord)) => {
        assert_eq!(music_time, music_time::MusicTime::new(1, 9, 1));
        assert_eq!(index, 1);
        assert_eq!(chord, "custom1".to_string());
      }
      _ => assert_eq!(false, true),
    }
  }
  {
    let params = io::deseralizer::deserialize_string(
      r#"
      patterns:
          - name: part_a
            pattern:
                - [1,3,1, MAJOR_SEVENTH, 0]
                - [1,9,0, custom1, 0]
        "#,
    );
    assert_ne!(params, Err(crate::FailResult::Deserialize));

    let compo = parameters_to_composition(&params.unwrap());

    match compo {
      Err(FailResult::UnreachableTime(music_time, index, chord)) => {
        assert_eq!(music_time, music_time::MusicTime::new(1, 9, 0));
        assert_eq!(index, 1);
        assert_eq!(chord, "custom1".to_string());
      }
      _ => assert_eq!(false, true),
    }
  }
  {
    let params = io::deseralizer::deserialize_string(
      r#"
      patterns:
          - name: part_a
            pattern:
                - [1,0,1, custom1, 0]
        "#,
    );
    assert_ne!(params, Err(crate::FailResult::Deserialize));

    let compo = parameters_to_composition(&params.unwrap());

    match compo {
      Err(FailResult::UnreachableTime(music_time, index, chord)) => {
        assert_eq!(music_time, music_time::MusicTime::new(1, 0, 1));
        assert_eq!(index, 0);
        assert_eq!(chord, "custom1".to_string());
      }
      _ => assert_eq!(false, true),
    }
  }
  {
    let params = io::deseralizer::deserialize_string(
      r#"
      patterns:
          - name: part_a
            pattern:
                - [0,1,1, custom1, 0]
        "#,
    );
    assert_ne!(params, Err(crate::FailResult::Deserialize));

    let compo = parameters_to_composition(&params.unwrap());

    match compo {
      Err(FailResult::UnreachableTime(music_time, index, chord)) => {
        assert_eq!(music_time, music_time::MusicTime::new(0, 1, 1));
        assert_eq!(index, 0);
        assert_eq!(chord, "custom1".to_string());
      }
      _ => assert!(false, true),
    }
  }
  {
    let params = io::deseralizer::deserialize_string(
      r#"
      patterns:
          - name: part_a
            pattern:
                - [1,1,1, MINOR, 0]
                - [1,1,1, custom1, 0]
        "#,
    );
    assert_ne!(params, Err(crate::FailResult::Deserialize));

    let compo = parameters_to_composition(&params.unwrap());

    match compo {
      Err(FailResult::UnreachableTime(music_time, index, chord)) => {
        assert_eq!(music_time, music_time::MusicTime::new(1, 1, 1));
        assert_eq!(index, 1);
        assert_eq!(chord, "custom1".to_string());
      }
      _ => assert!(false, true),
    }
  }
}
