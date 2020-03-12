#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum Key {
  C,
  Cs,
  D,
  Ds,
  E,
  F,
  Fs,
  G,
  Gs,
  A,
  As,
  B,
}

pub fn key_to_index(key: Key) -> u8 {
  match key {
    Key::C => 0,
    Key::Cs => 1,
    Key::D => 2,
    Key::Ds => 3,
    Key::E => 4,
    Key::F => 5,
    Key::Fs => 6,
    Key::G => 7,
    Key::Gs => 8,
    Key::A => 9,
    Key::As => 10,
    Key::B => 11,
  }
}

pub fn key_to_string(key: Key) -> &'static str {
  match key {
    Key::C => "C",
    Key::Cs => "C#",
    Key::D => "D",
    Key::Ds => "D#",
    Key::E => "E",
    Key::F => "F",
    Key::Fs => "F#",
    Key::G => "G",
    Key::Gs => "G#",
    Key::A => "A",
    Key::As => "A#",
    Key::B => "B",
  }
}

pub fn string_to_key(key: &str) -> Key {
  match key {
    "C" => Key::C,
    "C#" => Key::Cs,
    "D" => Key::D,
    "D#" => Key::Ds,
    "E" => Key::E,
    "F" => Key::F,
    "F#" => Key::Fs,
    "G" => Key::G,
    "G#" => Key::Gs,
    "A" => Key::A,
    "A#" => Key::As,
    "B" => Key::B,
    _ => Key::C,
  }
}

pub fn index_to_key(key: i8) -> Key {
  match key {
    0 => Key::C,
    1 => Key::Cs,
    2 => Key::D,
    3 => Key::Ds,
    4 => Key::E,
    5 => Key::F,
    6 => Key::Fs,
    7 => Key::G,
    8 => Key::Gs,
    9 => Key::A,
    10 => Key::As,
    11 => Key::B,
    _ => Key::C,
  }
}

pub fn to_midi_note(note_interval: i8) -> u8 {
  const MIN_VALUE: i8 = 24;
  const MAX_VALUE: i8 = 107;

  if MIN_VALUE + note_interval <= MIN_VALUE {
    MIN_VALUE as u8
  } else if MIN_VALUE + note_interval >= MAX_VALUE {
    MAX_VALUE as u8
  } else {
    (MIN_VALUE + note_interval) as u8
  }
}

pub fn midi_to_note(value: u8) -> (u8, Key) {
  const MIN_VALUE: u8 = 24;
  const NOTES_IN_OCTAVE_COUNT: u8 = 12;
  let corrected_value = value - MIN_VALUE;
  let note = corrected_value % NOTES_IN_OCTAVE_COUNT;
  let octave = corrected_value / NOTES_IN_OCTAVE_COUNT;

  (octave + 1, index_to_key(note as i8))
}

mod tests {
  #[test]
  fn test_midi_note() {
    use crate::theory::notes::*;
    let note = to_midi_note(0);
    assert_eq!(note, 24);
    assert_eq!(midi_to_note(note), (1, Key::C));

    assert_eq!(midi_to_note(60), (4, Key::C));
    assert_eq!(midi_to_note(60 - 12 + 3), (3, Key::Ds));
  }

  #[test]
  fn test_conversions() {
    use crate::theory::notes::*;
    let result = key_to_index(Key::G);
    assert_eq!(result, 7);

    let result = key_to_string(Key::As);
    assert_eq!(result, "A#");

    let result = string_to_key("B");
    assert_eq!(result, Key::B);

    let result = index_to_key(2);
    assert_eq!(result, Key::D);
  }
}
