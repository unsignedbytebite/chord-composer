extern crate chord_composer;

use chord_composer::{
  performance::performance_engine::PerformanceState,
  theory::composition::{Composition, Pattern, PatternEvent},
  FailResult, SuccessResult,
};
use music_timer::{music_time::MusicTime, time_signature::TimeSignature};

struct MiddleCState {
  callback_calls: u16,
  current_time: MusicTime,
}

impl PerformanceState for MiddleCState {
  fn on_ready(&mut self, composition: &Composition) {
    self.callback_calls += 1;
    assert_eq!(composition.get_name(), "middle_c");
    println!("on_ready");
  }
  fn on_beat_interval_change(&mut self, current_time: &MusicTime) {
    self.callback_calls += 1;
    self.current_time = current_time.clone();
    println!("on_beat_interval_change: {:?}", current_time);
  }
  fn on_beat_change(&mut self, current_time: &MusicTime) {
    self.callback_calls += 1;
    println!("on_beat_change: {:?}", current_time);
  }
  fn on_bar_change(&mut self, current_time: &MusicTime) {
    self.callback_calls += 1;
    println!("on_bar_change: {:?}", current_time);
  }
  fn on_event(&mut self, event: &PatternEvent) {
    self.callback_calls += 1;
    let (event_time, event_notes) = event;
    assert_eq!(event_time, &MusicTime::default());
    assert_eq!(event_notes, &vec![60]);
    println!("on_event");
  }
  fn on_pattern_playback_begin(&mut self, _pattern: &Pattern) {
    self.callback_calls += 1;
    println!("on_pattern_playback_begin");
  }
  fn on_pattern_playback_end(&mut self, _pattern: &Pattern) {
    self.callback_calls += 1;
    println!("on_pattern_playback_end");
  }
  fn on_completed(&mut self, composition: &Composition) {
    self.callback_calls += 1;
    assert_eq!(composition.get_name(), "middle_c");
    println!("on_completed");
  }
}

#[test]
fn chord_to_string_array() {
  let chords = chord_composer::get_chord_keywords();
  assert_eq!(chords.len(), 73);
  assert_eq!(chords[0], "AUGMENTED = [0, 4, 8]");
}

#[test]
fn export_midi_missing_file() {
  let file = "./tests/no_file.gone";

  assert_eq!(
    chord_composer::export_file_to_midi(file),
    Err(chord_composer::FailResult::Deserialize)
  );
}

#[test]
fn export_midi_no_patterns() {
  let file = "./tests/export_test_no_patterns.yaml";

  assert_eq!(
    chord_composer::export_file_to_midi(file),
    Err(chord_composer::FailResult::NoPatterns)
  );
}

#[test]
fn test_export_template() {
  assert_eq!(
    chord_composer::export_template("./tests/test_template_export.yaml"),
    Ok(chord_composer::SuccessResult::ExportTemplate)
  );
}

#[test]
fn export_midi() {
  let file = "./tests/export_test.yaml";

  let files = vec![
    "./tests/bc_000_a/part_a.mid".to_string(),
    "./tests/bc_000_a/part_b.mid".to_string(),
  ];

  assert_eq!(
    chord_composer::export_file_to_midi(file),
    Ok(chord_composer::SuccessResult::Export(files.clone())),
  );

  use std::fs::File;
  use std::io::prelude::*;

  let file1_bin = vec![
    0x4D, 0x54, 0x68, 0x64, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x02, 0x01, 0xE0, 0x4D, 0x54,
    0x72, 0x6B, 0x00, 0x00, 0x00, 0x13, 0x00, 0xFF, 0x51, 0x03, 0x07, 0x27, 0x0E, 0x00, 0xFF, 0x58,
    0x04, 0x03, 0x02, 0x08, 0x18, 0x00, 0xFF, 0x2F, 0x00, 0x4D, 0x54, 0x72, 0x6B, 0x00, 0x00, 0x01,
    0x07, 0x00, 0xFF, 0x03, 0x06, 0x70, 0x61, 0x72, 0x74, 0x5F, 0x61, 0x00, 0x90, 0x3E, 0x40, 0x00,
    0x90, 0x42, 0x40, 0x00, 0x90, 0x45, 0x40, 0x00, 0x90, 0x49, 0x40, 0x87, 0x40, 0x90, 0x3E, 0x00,
    0x00, 0x90, 0x42, 0x00, 0x00, 0x90, 0x45, 0x00, 0x00, 0x90, 0x49, 0x00, 0x00, 0x90, 0x3E, 0x40,
    0x00, 0x90, 0x41, 0x40, 0x00, 0x90, 0x46, 0x40, 0x83, 0x60, 0x90, 0x3E, 0x00, 0x00, 0x90, 0x41,
    0x00, 0x00, 0x90, 0x46, 0x00, 0x00, 0x90, 0x3E, 0x40, 0x00, 0x90, 0x42, 0x40, 0x00, 0x90, 0x45,
    0x40, 0x00, 0x90, 0x49, 0x40, 0x00, 0x90, 0x40, 0x40, 0x87, 0x40, 0x90, 0x3E, 0x00, 0x00, 0x90,
    0x42, 0x00, 0x00, 0x90, 0x45, 0x00, 0x00, 0x90, 0x49, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00, 0x90,
    0x3E, 0x40, 0x00, 0x90, 0x41, 0x40, 0x00, 0x90, 0x46, 0x40, 0x83, 0x60, 0x90, 0x3E, 0x00, 0x00,
    0x90, 0x41, 0x00, 0x00, 0x90, 0x46, 0x00, 0x00, 0x90, 0x41, 0x40, 0x00, 0x90, 0x45, 0x40, 0x00,
    0x90, 0x48, 0x40, 0x00, 0x90, 0x4C, 0x40, 0x83, 0x60, 0x90, 0x41, 0x00, 0x00, 0x90, 0x45, 0x00,
    0x00, 0x90, 0x48, 0x00, 0x00, 0x90, 0x4C, 0x00, 0x00, 0x90, 0x3E, 0x40, 0x00, 0x90, 0x41, 0x40,
    0x00, 0x90, 0x46, 0x40, 0x83, 0x60, 0x90, 0x3E, 0x00, 0x00, 0x90, 0x41, 0x00, 0x00, 0x90, 0x46,
    0x00, 0x83, 0x60, 0x90, 0x3B, 0x40, 0x00, 0x90, 0x3F, 0x40, 0x00, 0x90, 0x42, 0x40, 0x00, 0x90,
    0x46, 0x40, 0x00, 0x90, 0x3D, 0x40, 0x83, 0x60, 0x90, 0x3B, 0x00, 0x00, 0x90, 0x3F, 0x00, 0x00,
    0x90, 0x42, 0x00, 0x00, 0x90, 0x46, 0x00, 0x00, 0x90, 0x3D, 0x00, 0x00, 0x90, 0x3E, 0x40, 0x00,
    0x90, 0x41, 0x40, 0x00, 0x90, 0x46, 0x40, 0x87, 0x40, 0x90, 0x3E, 0x00, 0x00, 0x90, 0x41, 0x00,
    0x00, 0x90, 0x46, 0x00, 0x00, 0xFF, 0x2F, 0x00,
  ];

  let buffer = {
    let mut f = File::open(&files[0]).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    buffer
  };

  assert_eq!(buffer, file1_bin);

  let file2_bin = vec![
    0x4D, 0x54, 0x68, 0x64, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x02, 0x01, 0xE0, 0x4D, 0x54,
    0x72, 0x6B, 0x00, 0x00, 0x00, 0x13, 0x00, 0xFF, 0x51, 0x03, 0x0D, 0x44, 0xBD, 0x00, 0xFF, 0x58,
    0x04, 0x04, 0x03, 0x08, 0x18, 0x00, 0xFF, 0x2F, 0x00, 0x4D, 0x54, 0x72, 0x6B, 0x00, 0x00, 0x01,
    0x06, 0x00, 0xFF, 0x03, 0x06, 0x70, 0x61, 0x72, 0x74, 0x5F, 0x62, 0x00, 0x90, 0x3D, 0x40, 0x00,
    0x90, 0x41, 0x40, 0x00, 0x90, 0x44, 0x40, 0x00, 0x90, 0x48, 0x40, 0x83, 0x60, 0x90, 0x3D, 0x00,
    0x00, 0x90, 0x41, 0x00, 0x00, 0x90, 0x44, 0x00, 0x00, 0x90, 0x48, 0x00, 0x00, 0x90, 0x3D, 0x40,
    0x00, 0x90, 0x40, 0x40, 0x00, 0x90, 0x45, 0x40, 0x8B, 0x20, 0x90, 0x3D, 0x00, 0x00, 0x90, 0x40,
    0x00, 0x00, 0x90, 0x45, 0x00, 0x00, 0x90, 0x3D, 0x40, 0x00, 0x90, 0x41, 0x40, 0x00, 0x90, 0x44,
    0x40, 0x00, 0x90, 0x48, 0x40, 0x00, 0x90, 0x3F, 0x40, 0x83, 0x60, 0x90, 0x3D, 0x00, 0x00, 0x90,
    0x41, 0x00, 0x00, 0x90, 0x44, 0x00, 0x00, 0x90, 0x48, 0x00, 0x00, 0x90, 0x3F, 0x00, 0x00, 0x90,
    0x3D, 0x40, 0x00, 0x90, 0x40, 0x40, 0x00, 0x90, 0x45, 0x40, 0x8B, 0x20, 0x90, 0x3D, 0x00, 0x00,
    0x90, 0x40, 0x00, 0x00, 0x90, 0x45, 0x00, 0x00, 0x90, 0x40, 0x40, 0x00, 0x90, 0x44, 0x40, 0x00,
    0x90, 0x47, 0x40, 0x00, 0x90, 0x4B, 0x40, 0x83, 0x60, 0x90, 0x40, 0x00, 0x00, 0x90, 0x44, 0x00,
    0x00, 0x90, 0x47, 0x00, 0x00, 0x90, 0x4B, 0x00, 0x00, 0x90, 0x3D, 0x40, 0x00, 0x90, 0x40, 0x40,
    0x00, 0x90, 0x45, 0x40, 0x8B, 0x20, 0x90, 0x3D, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00, 0x90, 0x45,
    0x00, 0x00, 0x90, 0x3A, 0x40, 0x00, 0x90, 0x3E, 0x40, 0x00, 0x90, 0x41, 0x40, 0x00, 0x90, 0x45,
    0x40, 0x00, 0x90, 0x3C, 0x40, 0x83, 0x60, 0x90, 0x3A, 0x00, 0x00, 0x90, 0x3E, 0x00, 0x00, 0x90,
    0x41, 0x00, 0x00, 0x90, 0x45, 0x00, 0x00, 0x90, 0x3C, 0x00, 0x00, 0x90, 0x3D, 0x40, 0x00, 0x90,
    0x40, 0x40, 0x00, 0x90, 0x45, 0x40, 0x8B, 0x20, 0x90, 0x3D, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00,
    0x90, 0x45, 0x00, 0x00, 0xFF, 0x2F, 0x00,
  ];

  let buffer = {
    let mut f = File::open(&files[1]).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    buffer
  };

  assert_eq!(buffer, file2_bin);
}

#[test]
fn play_empty_composition() {
  let composition = Composition::new_with_patterns("middle_c", vec![]);

  let mut my_state = MiddleCState {
    callback_calls: 0,
    current_time: MusicTime::default(),
  };

  assert_eq!(
    chord_composer::play(&composition, &mut my_state, false, &Vec::new(), &Vec::new()),
    Err(FailResult::NoPatterns),
  );
}

// #[test]
// fn play_middle_c() {
//   let composition = Composition::new_with_patterns(
//     "middle_c",
//     vec![Pattern::new_with_events(
//       "part_a",
//       100,
//       TimeSignature::default(),
//       vec![chord_composer::build_event(1, 1, 1, vec![0], 0)],
//     )],
//   );

//   let mut my_state = MiddleCState {
//     callback_calls: 0,
//     current_time: MusicTime::default(),
//   };

//   assert_eq!(
//     chord_composer::play(&composition, &mut my_state, false, &Vec::new(), &Vec::new()),
//     Ok(SuccessResult::Playback),
//   );

//   assert_eq!(my_state.callback_calls, 42);
//   assert_eq!(my_state.current_time, MusicTime::new(1, 4, 8));
// }

#[test]
fn play_middle_c_yaml() {
  use music_timer::music_time::MusicTime;
  let yaml = r#"
  name: middle_c
  chords:
      - [single_note, [0]]
  patterns:
      - name: part_a
        pattern:
            - [1, 1, 1, single_note, 0]
  "#;

  let mut my_state = MiddleCState {
    callback_calls: 0,
    current_time: MusicTime::default(),
  };

  assert_eq!(
    chord_composer::play_yaml(yaml, &mut my_state, false, &Vec::new(), &Vec::new()),
    Ok(chord_composer::SuccessResult::Playback)
  );

  assert_eq!(my_state.callback_calls, 42);
  assert_eq!(my_state.current_time, MusicTime::new(1, 4, 8));
}

#[test]
fn play_middle_c_file() {
  use music_timer::music_time::MusicTime;
  let file = "./tests/middle_c.yaml";
  let mut my_state = MiddleCState {
    callback_calls: 0,
    current_time: MusicTime::default(),
  };

  assert_eq!(
    chord_composer::play_file(file, &mut my_state, false, &Vec::new(), &Vec::new()),
    Ok(chord_composer::SuccessResult::Playback)
  );

  assert_eq!(my_state.callback_calls, 42);
  assert_eq!(my_state.current_time, MusicTime::new(1, 4, 8));
}

#[test]
fn play_composition_from() {
  struct MyState {
    events: u16,
    current_time: MusicTime,
  }
  impl PerformanceState for MyState {
    fn on_ready(&mut self, _composition: &Composition) {
      println!("on_ready");
    }
    fn on_beat_interval_change(&mut self, current_time: &MusicTime) {
      self.current_time = current_time.clone();
      println!("on_beat_interval_change: {:?}", current_time);
    }
    fn on_beat_change(&mut self, current_time: &MusicTime) {
      println!("on_beat_change: {:?}", current_time);
    }
    fn on_bar_change(&mut self, current_time: &MusicTime) {
      println!("on_bar_change: {:?}", current_time);
    }
    fn on_event(&mut self, event: &PatternEvent) {
      self.events += 1;

      if self.events == 1 {
        assert_eq!(event, &chord_composer::build_event(2, 1, 1, vec![1], -1));
      } else if self.events == 2 {
        assert_eq!(event, &chord_composer::build_event(3, 5, 1, vec![2], -2));
      }
      println!("on_event");
    }
    fn on_pattern_playback_begin(&mut self, _pattern: &Pattern) {
      println!("on_pattern_playback_begin");
    }
    fn on_pattern_playback_end(&mut self, _pattern: &Pattern) {
      println!("on_pattern_playback_end");
    }
    fn on_completed(&mut self, composition: &Composition) {
      assert_eq!(composition.get_name(), "test compo");
      println!("on_completed");
    }
  }

  let composition = Composition::new_with_patterns(
    "test compo",
    vec![
      Pattern::new_with_events(
        "a",
        100,
        TimeSignature::default(),
        vec![
          chord_composer::build_event(1, 1, 1, vec![0, 3, 7], 0),
          chord_composer::build_event(2, 1, 1, vec![0, 3, 7], 1),
          chord_composer::build_event(3, 5, 1, vec![0, 3, 7], 2),
        ],
      ),
      Pattern::new_with_events(
        "b",
        150,
        TimeSignature::new(7, 4),
        vec![
          chord_composer::build_event(3, 5, 1, vec![2], -2),
          chord_composer::build_event(2, 1, 1, vec![1], -1),
          chord_composer::build_event(1, 1, 1, vec![0], 0),
        ],
      ),
    ],
  );

  let mut my_state = MyState {
    events: 0,
    current_time: MusicTime::default(),
  };

  assert_eq!(
    chord_composer::play_from(
      &composition,
      &mut my_state,
      false,
      &Vec::new(),
      &Vec::new(),
      &MusicTime::new(2, 1, 1),
      "b"
    ),
    Ok(SuccessResult::Playback),
  );

  assert_eq!(my_state.events, 2);
  assert_eq!(my_state.current_time, MusicTime::new(3, 7, 8));

  assert_eq!(
    chord_composer::play_from(
      &composition,
      &mut my_state,
      false,
      &Vec::new(),
      &Vec::new(),
      &MusicTime::new(2, 1, 1),
      "c"
    ),
    Err(FailResult::NoFoundPattern("c".to_owned())),
  );
}
