#![allow(dead_code)]

use super::notes;

pub type CustomChords = Vec<(String, Vec<i8>)>;

// TODO: remove chords? Allow user created chords only?
pub const AUGMENTED: [i8; 3] = [0, 4, 8];
pub const AUGMENTED_ELEVENTH: [i8; 6] = [0, 4, 7, 10, 2, 6];
pub const AUGMENTED_MAJOR_SEVENTH: [i8; 4] = [0, 4, 8, 11];
pub const AUGMENTED_SEVENTH: [i8; 4] = [0, 4, 8, 10];
pub const AUGMENTED_SIXTH: [i8; 3] = [0, 6, 8];
pub const DIMINISHED: [i8; 3] = [0, 3, 6];
pub const DIMINISHED_MAJOR_SEVENTH: [i8; 4] = [0, 3, 6, 11];
pub const DIMINISHED_SEVENTH: [i8; 4] = [0, 3, 6, 9];
pub const DOMINANT: [i8; 3] = [0, 4, 7];
pub const DOMINANT_ELEVENTH: [i8; 6] = [0, 4, 7, 10, 2, 5];
pub const DOMINANT_MINOR_NINTH: [i8; 5] = [0, 4, 7, 10, 1];
pub const DOMINANT_NINTH: [i8; 5] = [0, 4, 7, 10, 2];
pub const DOMINANT_PARALLEL: [i8; 3] = [0, 3, 7];
pub const DOMINANT_SEVENTH: [i8; 4] = [0, 4, 7, 10];
pub const DOMINANT_SEVENTH_FLAT_FIVE: [i8; 4] = [0, 4, 6, 10];
pub const DOMINANT_SEVENTH_RAISED_NINTH: [i8; 5] = [0, 4, 7, 10, 3];
pub const DOMINANT_THIRTEENTH: [i8; 7] = [0, 4, 7, 10, 2, 5, 9];
pub const DREAM: [i8; 4] = [0, 5, 6, 7];
pub const ELEKTRA: [i8; 5] = [0, 7, 9, 1, 4];
pub const FARBEN: [i8; 5] = [0, 8, 11, 4, 9];
pub const HALF_DIMINISHED_SEVENTH: [i8; 4] = [0, 3, 6, 10];
pub const HARMONIC_SEVENTH: [i8; 4] = [0, 4, 7, 10];
pub const AUGMENTED_NINTH: [i8; 5] = [0, 4, 7, 10, 3];
pub const LEADING_TONE: [i8; 3] = [0, 3, 6];
pub const LYDIAN: [i8; 5] = [0, 4, 7, 11, 6];
pub const MAGIC: [i8; 8] = [0, 1, 5, 6, 10, 0, 3, 5];
pub const MAJOR: [i8; 3] = [0, 4, 7];
pub const MAJOR_ELEVENTH: [i8; 6] = [0, 4, 7, 11, 2, 5];
pub const MAJOR_SEVENTH: [i8; 4] = [0, 4, 7, 11];
pub const MAJOR_SEVENTH_SHARP_ELEVENTH: [i8; 5] = [0, 4, 8, 11, 6];
pub const MAJOR_SIXTH: [i8; 4] = [0, 4, 7, 9];
pub const MAJOR_SIXTH_NINTH: [i8; 5] = [0, 4, 7, 9, 2];
pub const MAJOR_NINTH: [i8; 5] = [0, 4, 7, 11, 2];
pub const MAJOR_THIRTEENTH: [i8; 7] = [0, 4, 7, 11, 2, 6, 9];
pub const MEDIANT: [i8; 3] = [0, 3, 7];
pub const MINOR: [i8; 3] = [0, 3, 7];
pub const MINOR_ELEVENTH: [i8; 6] = [0, 3, 7, 10, 2, 5];
pub const MINOR_MAJOR_SEVENTH: [i8; 4] = [0, 3, 7, 11];
pub const MINOR_NINTH: [i8; 5] = [0, 3, 7, 10, 2];
pub const MINOR_SEVENTH: [i8; 4] = [0, 3, 7, 10];
pub const MINOR_SIXTH: [i8; 4] = [0, 3, 7, 9];
pub const MINOR_SIXTH_NINTH: [i8; 5] = [0, 3, 7, 9, 2];
pub const MINOR_THIRTEENTH: [i8; 7] = [0, 3, 7, 10, 2, 5, 9];
pub const MU: [i8; 4] = [0, 2, 4, 7];
pub const MYSTIC: [i8; 6] = [0, 6, 10, 4, 9, 2];
pub const NEAPOLITAN: [i8; 3] = [1, 5, 8];
pub const NINTH_AUGMENTED_FIFTH: [i8; 5] = [0, 4, 8, 10, 2];
pub const NINTH_FLAT_FIFTH: [i8; 5] = [0, 4, 6, 10, 2];
pub const NORTHERN_LIGHTS: [i8; 11] = [1, 2, 8, 0, 3, 6, 7, 10, 11, 4, 7];
pub const ODE_TO_NAPOLEON_HEXACHORD: [i8; 6] = [0, 1, 4, 5, 8, 9];
pub const PETRUSHKA: [i8; 6] = [0, 1, 4, 6, 7, 10];
pub const POWER: [i8; 2] = [0, 7];
pub const PSALMS: [i8; 3] = [0, 3, 7];
pub const SECONDARY_DOMINANT: [i8; 3] = [0, 4, 7];
pub const SECONDARY_LEADING_TONE: [i8; 3] = [0, 3, 6];
pub const SECONDARY_SUPERTONIC: [i8; 3] = [0, 3, 7];
pub const SEVEN_SIX: [i8; 5] = [0, 4, 7, 9, 10];
pub const SEVENTH_FLAT_NINE: [i8; 5] = [0, 4, 7, 10, 1];
pub const SEVENTH_SUSPENSION_FOUR: [i8; 4] = [0, 5, 7, 10];
pub const SO_WHAT: [i8; 5] = [0, 5, 10, 3, 7];
pub const SUSPENDED: [i8; 3] = [0, 5, 7];
pub const SUBDOMINANT: [i8; 3] = [0, 4, 7];
pub const SUBDOMINANT_PARALLEL: [i8; 3] = [0, 3, 7];
pub const SUBMEDIANT: [i8; 3] = [0, 3, 7];
pub const SUBTONIC: [i8; 3] = [0, 4, 7];
pub const SUPERTONIC: [i8; 3] = [0, 3, 7];
pub const THIRTEENTH_FLAT_NINTH: [i8; 7] = [0, 4, 7, 10, 1, 12, 9];
pub const THIRTEENTH_FLAT_NINTH_FLAT_FIFTH: [i8; 7] = [0, 4, 6, 10, 1, 12, 9];
pub const TONIC_COUNTER_PARALLEL: [i8; 3] = [0, 3, 7];
pub const TONIC: [i8; 3] = [0, 4, 7];
pub const TONIC_PARALLEL: [i8; 3] = [0, 3, 7];
pub const TRISTAN: [i8; 4] = [0, 3, 6, 10];
pub const VIENNESE_TRICHORD: [i8; 4] = [0, 1, 6, 7];

pub fn string_to_chord(name: &str) -> Vec<i8> {
  match name {
    "AUGMENTED" => AUGMENTED.to_vec(),
    "AUGMENTED_ELEVENTH" => AUGMENTED_ELEVENTH.to_vec(),
    "AUGMENTED_MAJOR_SEVENTH" => AUGMENTED_MAJOR_SEVENTH.to_vec(),
    "AUGMENTED_SEVENTH" => AUGMENTED_SEVENTH.to_vec(),
    "AUGMENTED_SIXTH" => AUGMENTED_SIXTH.to_vec(),
    "DIMINISHED" => DIMINISHED.to_vec(),
    "DIMINISHED_MAJOR_SEVENTH" => DIMINISHED_MAJOR_SEVENTH.to_vec(),
    "DIMINISHED_SEVENTH" => DIMINISHED_SEVENTH.to_vec(),
    "DOMINANT" => DOMINANT.to_vec(),
    "DOMINANT_ELEVENTH" => DOMINANT_ELEVENTH.to_vec(),
    "DOMINANT_MINOR_NINTH" => DOMINANT_MINOR_NINTH.to_vec(),
    "DOMINANT_NINTH" => DOMINANT_NINTH.to_vec(),
    "DOMINANT_PARALLEL" => DOMINANT_PARALLEL.to_vec(),
    "DOMINANT_SEVENTH" => DOMINANT_SEVENTH.to_vec(),
    "DOMINANT_SEVENTH_FLAT_FIVE" => DOMINANT_SEVENTH_FLAT_FIVE.to_vec(),
    "DOMINANT_SEVENTH_RAISED_NINTH" => DOMINANT_SEVENTH_RAISED_NINTH.to_vec(),
    "DOMINANT_THIRTEENTH" => DOMINANT_THIRTEENTH.to_vec(),
    "DREAM" => DREAM.to_vec(),
    "ELEKTRA" => ELEKTRA.to_vec(),
    "FARBEN" => FARBEN.to_vec(),
    "HALF_DIMINISHED_SEVENTH" => HALF_DIMINISHED_SEVENTH.to_vec(),
    "HARMONIC_SEVENTH" => HARMONIC_SEVENTH.to_vec(),
    "AUGMENTED_NINTH" => AUGMENTED_NINTH.to_vec(),
    "LEADING_TONE" => LEADING_TONE.to_vec(),
    "LYDIAN" => LYDIAN.to_vec(),
    "MAGIC" => MAGIC.to_vec(),
    "MAJOR" => MAJOR.to_vec(),
    "MAJOR_ELEVENTH" => MAJOR_ELEVENTH.to_vec(),
    "MAJOR_SEVENTH" => MAJOR_SEVENTH.to_vec(),
    "MAJOR_SEVENTH_SHARP_ELEVENTH" => MAJOR_SEVENTH_SHARP_ELEVENTH.to_vec(),
    "MAJOR_SIXTH" => MAJOR_SIXTH.to_vec(),
    "MAJOR_SIXTH_NINTH" => MAJOR_SIXTH_NINTH.to_vec(),
    "MAJOR_NINTH" => MAJOR_NINTH.to_vec(),
    "MAJOR_THIRTEENTH" => MAJOR_THIRTEENTH.to_vec(),
    "MEDIANT" => MEDIANT.to_vec(),
    "MINOR" => MINOR.to_vec(),
    "MINOR_ELEVENTH" => MINOR_ELEVENTH.to_vec(),
    "MINOR_MAJOR_SEVENTH" => MINOR_MAJOR_SEVENTH.to_vec(),
    "MINOR_NINTH" => MINOR_NINTH.to_vec(),
    "MINOR_SEVENTH" => MINOR_SEVENTH.to_vec(),
    "MINOR_SIXTH" => MINOR_SIXTH.to_vec(),
    "MINOR_SIXTH_NINTH" => MINOR_SIXTH_NINTH.to_vec(),
    "MINOR_THIRTEENTH" => MINOR_THIRTEENTH.to_vec(),
    "MU" => MU.to_vec(),
    "MYSTIC" => MYSTIC.to_vec(),
    "NEAPOLITAN" => NEAPOLITAN.to_vec(),
    "NINTH_AUGMENTED_FIFTH" => NINTH_AUGMENTED_FIFTH.to_vec(),
    "NINTH_FLAT_FIFTH" => NINTH_FLAT_FIFTH.to_vec(),
    "NORTHERN_LIGHTS" => NORTHERN_LIGHTS.to_vec(),
    "ODE_TO_NAPOLEON_HEXACHORD" => ODE_TO_NAPOLEON_HEXACHORD.to_vec(),
    "PETRUSHKA" => PETRUSHKA.to_vec(),
    "POWER" => POWER.to_vec(),
    "PSALMS" => PSALMS.to_vec(),
    "SECONDARY_DOMINANT" => SECONDARY_DOMINANT.to_vec(),
    "SECONDARY_LEADING_TONE" => SECONDARY_LEADING_TONE.to_vec(),
    "SECONDARY_SUPERTONIC" => SECONDARY_SUPERTONIC.to_vec(),
    "SEVEN_SIX" => SEVEN_SIX.to_vec(),
    "SEVENTH_FLAT_NINE" => SEVENTH_FLAT_NINE.to_vec(),
    "SEVENTH_SUSPENSION_FOUR" => SEVENTH_SUSPENSION_FOUR.to_vec(),
    "SO_WHAT" => SO_WHAT.to_vec(),
    "SUSPENDED" => SUSPENDED.to_vec(),
    "SUBDOMINANT" => SUBDOMINANT.to_vec(),
    "SUBDOMINANT_PARALLEL" => SUBDOMINANT_PARALLEL.to_vec(),
    "SUBMEDIANT" => SUBMEDIANT.to_vec(),
    "SUBTONIC" => SUBTONIC.to_vec(),
    "SUPERTONIC" => SUPERTONIC.to_vec(),
    "THIRTEENTH_FLAT_NINTH" => THIRTEENTH_FLAT_NINTH.to_vec(),
    "THIRTEENTH_FLAT_NINTH_FLAT_FIFTH" => THIRTEENTH_FLAT_NINTH_FLAT_FIFTH.to_vec(),
    "TONIC_COUNTER_PARALLEL" => TONIC_COUNTER_PARALLEL.to_vec(),
    "TONIC" => TONIC.to_vec(),
    "TONIC_PARALLEL" => TONIC_PARALLEL.to_vec(),
    "TRISTAN" => TRISTAN.to_vec(),
    "VIENNESE_TRICHORD" => VIENNESE_TRICHORD.to_vec(),
    _ => Vec::new(),
  }
}

pub fn chords_to_vector() -> Vec<Vec<i8>> {
  vec![
    AUGMENTED.to_vec(),
    AUGMENTED_ELEVENTH.to_vec(),
    AUGMENTED_MAJOR_SEVENTH.to_vec(),
    AUGMENTED_SEVENTH.to_vec(),
    AUGMENTED_SIXTH.to_vec(),
    DIMINISHED.to_vec(),
    DIMINISHED_MAJOR_SEVENTH.to_vec(),
    DIMINISHED_SEVENTH.to_vec(),
    DOMINANT.to_vec(),
    DOMINANT_ELEVENTH.to_vec(),
    DOMINANT_MINOR_NINTH.to_vec(),
    DOMINANT_NINTH.to_vec(),
    DOMINANT_PARALLEL.to_vec(),
    DOMINANT_SEVENTH.to_vec(),
    DOMINANT_SEVENTH_FLAT_FIVE.to_vec(),
    DOMINANT_SEVENTH_RAISED_NINTH.to_vec(),
    DOMINANT_THIRTEENTH.to_vec(),
    DREAM.to_vec(),
    ELEKTRA.to_vec(),
    FARBEN.to_vec(),
    HALF_DIMINISHED_SEVENTH.to_vec(),
    HARMONIC_SEVENTH.to_vec(),
    AUGMENTED_NINTH.to_vec(),
    LEADING_TONE.to_vec(),
    LYDIAN.to_vec(),
    MAGIC.to_vec(),
    MAJOR.to_vec(),
    MAJOR_ELEVENTH.to_vec(),
    MAJOR_SEVENTH.to_vec(),
    MAJOR_SEVENTH_SHARP_ELEVENTH.to_vec(),
    MAJOR_SIXTH.to_vec(),
    MAJOR_SIXTH_NINTH.to_vec(),
    MAJOR_NINTH.to_vec(),
    MAJOR_THIRTEENTH.to_vec(),
    MEDIANT.to_vec(),
    MINOR.to_vec(),
    MINOR_ELEVENTH.to_vec(),
    MINOR_MAJOR_SEVENTH.to_vec(),
    MINOR_NINTH.to_vec(),
    MINOR_SEVENTH.to_vec(),
    MINOR_SIXTH.to_vec(),
    MINOR_SIXTH_NINTH.to_vec(),
    MINOR_THIRTEENTH.to_vec(),
    MU.to_vec(),
    MYSTIC.to_vec(),
    NEAPOLITAN.to_vec(),
    NINTH_AUGMENTED_FIFTH.to_vec(),
    NINTH_FLAT_FIFTH.to_vec(),
    NORTHERN_LIGHTS.to_vec(),
    ODE_TO_NAPOLEON_HEXACHORD.to_vec(),
    PETRUSHKA.to_vec(),
    POWER.to_vec(),
    PSALMS.to_vec(),
    SECONDARY_DOMINANT.to_vec(),
    SECONDARY_LEADING_TONE.to_vec(),
    SECONDARY_SUPERTONIC.to_vec(),
    SEVEN_SIX.to_vec(),
    SEVENTH_FLAT_NINE.to_vec(),
    SEVENTH_SUSPENSION_FOUR.to_vec(),
    SO_WHAT.to_vec(),
    SUSPENDED.to_vec(),
    SUBDOMINANT.to_vec(),
    SUBDOMINANT_PARALLEL.to_vec(),
    SUBMEDIANT.to_vec(),
    SUBTONIC.to_vec(),
    SUPERTONIC.to_vec(),
    THIRTEENTH_FLAT_NINTH.to_vec(),
    THIRTEENTH_FLAT_NINTH_FLAT_FIFTH.to_vec(),
    TONIC_COUNTER_PARALLEL.to_vec(),
    TONIC.to_vec(),
    TONIC_PARALLEL.to_vec(),
    TRISTAN.to_vec(),
    VIENNESE_TRICHORD.to_vec(),
  ]
}

pub fn chord_to_string_array() -> Vec<&'static str> {
  vec![
    "AUGMENTED = [0, 4, 8]",
    "AUGMENTED_ELEVENTH = [0, 4, 7, 10, 2, 6]",
    "AUGMENTED_MAJOR_SEVENTH = [0, 4, 8, 11]",
    "AUGMENTED_SEVENTH = [0, 4, 8, 10]",
    "AUGMENTED_SIXTH = [0, 6, 8]",
    "DIMINISHED = [0, 3, 6]",
    "DIMINISHED_MAJOR_SEVENTH = [0, 3, 6, 11]",
    "DIMINISHED_SEVENTH = [0, 3, 6, 9]",
    "DOMINANT = [0, 4, 7]",
    "DOMINANT_ELEVENTH = [0, 4, 7, 10, 2, 5]",
    "DOMINANT_MINOR_NINTH = [0, 4, 7, 10, 1]",
    "DOMINANT_NINTH = [0, 4, 7, 10, 2]",
    "DOMINANT_PARALLEL = [0, 3, 7]",
    "DOMINANT_SEVENTH = [0, 4, 7, 10]",
    "DOMINANT_SEVENTH_FLAT_FIVE = [0, 4, 6, 10]",
    "DOMINANT_SEVENTH_RAISED_NINTH = [0, 4, 7, 10, 3]",
    "DOMINANT_THIRTEENTH = [0, 4, 7, 10, 2, 5, 9]",
    "DREAM = [0, 5, 6, 7]",
    "ELEKTRA = [0, 7, 9, 1, 4]",
    "FARBEN = [0, 8, 11, 4, 9]",
    "HALF_DIMINISHED_SEVENTH = [0, 3, 6, 10]",
    "HARMONIC_SEVENTH = [0, 4, 7, 10]",
    "AUGMENTED_NINTH = [0, 4, 7, 10, 3]",
    "LEADING_TONE = [0, 3, 6]",
    "LYDIAN = [0, 4, 7, 11, 6]",
    "MAGIC = [0, 1, 5, 6, 10, 0, 3, 5]",
    "MAJOR = [0, 4, 7]",
    "MAJOR_ELEVENTH = [0, 4, 7, 11, 2, 5]",
    "MAJOR_SEVENTH = [0, 4, 7, 11]",
    "MAJOR_SEVENTH_SHARP_ELEVENTH = [0, 4, 8, 11, 6]",
    "MAJOR_SIXTH = [0, 4, 7, 9]",
    "MAJOR_SIXTH_NINTH = [0, 4, 7, 9, 2]",
    "MAJOR_NINTH = [0, 4, 7, 11, 2]",
    "MAJOR_THIRTEENTH = [0, 4, 7, 11, 2, 6, 9]",
    "MEDIANT = [0, 3, 7]",
    "MINOR = [0, 3, 7]",
    "MINOR_ELEVENTH = [0, 3, 7, 10, 2, 5]",
    "MINOR_MAJOR_SEVENTH = [0, 3, 7, 11]",
    "MINOR_NINTH = [0, 3, 7, 10, 2]",
    "MINOR_SEVENTH = [0, 3, 7, 10]",
    "MINOR_SIXTH = [0, 3, 7, 9]",
    "MINOR_SIXTH_NINTH = [0, 3, 7, 9, 2]",
    "MINOR_THIRTEENTH = [0, 3, 7, 10, 2, 5, 9]",
    "MU = [0, 2, 4, 7]",
    "MYSTIC = [0, 6, 10, 4, 9, 2]",
    "NEAPOLITAN = [1, 5, 8]",
    "NINTH_AUGMENTED_FIFTH = [0, 4, 8, 10, 2]",
    "NINTH_FLAT_FIFTH = [0, 4, 6, 10, 2]",
    "NORTHERN_LIGHTS = [1, 2, 8, 0, 3, 6, 7, 10, 11, 4, 7]",
    "ODE_TO_NAPOLEON_HEXACHORD = [0, 1, 4, 5, 8, 9]",
    "PETRUSHKA = [0, 1, 4, 6, 7, 10]",
    "POWER = [0, 7]",
    "PSALMS = [0, 3, 7]",
    "SECONDARY_DOMINANT = [0, 4, 7]",
    "SECONDARY_LEADING_TONE = [0, 3, 6]",
    "SECONDARY_SUPERTONIC = [0, 3, 7]",
    "SEVEN_SIX = [0, 4, 7, 9, 10]",
    "SEVENTH_FLAT_NINE = [0, 4, 7, 10, 1]",
    "SEVENTH_SUSPENSION_FOUR = [0, 5, 7, 10]",
    "SO_WHAT = [0, 5, 10, 3, 7]",
    "SUSPENDED = [0, 5, 7]",
    "SUBDOMINANT = [0, 4, 7]",
    "SUBDOMINANT_PARALLEL = [0, 3, 7]",
    "SUBMEDIANT = [0, 3, 7]",
    "SUBTONIC = [0, 4, 7]",
    "SUPERTONIC = [0, 3, 7]",
    "THIRTEENTH_FLAT_NINTH = [0, 4, 7, 10, 1, 12, 9]",
    "THIRTEENTH_FLAT_NINTH_FLAT_FIFTH = [0, 4, 6, 10, 1, 12, 9]",
    "TONIC_COUNTER_PARALLEL = [0, 3, 7]",
    "TONIC = [0, 4, 7]",
    "TONIC_PARALLEL = [0, 3, 7]",
    "TRISTAN = [0, 3, 6, 10]",
    "VIENNESE_TRICHORD = [0, 1, 6, 7]",
  ]
}

pub struct IntervalChord {
  intervals: Vec<i8>,
  transpose: i8,
}

impl IntervalChord {
  pub fn from_string(interval_chord_string: &str) -> Self {
    Self {
      intervals: string_to_chord(interval_chord_string.trim()),
      transpose: 0,
    }
  }

  pub fn from_string_with_custom(
    interval_chord_string: &str,
    custom_chords: &CustomChords,
  ) -> Self {
    use rand::Rng;

    if interval_chord_string.trim() == "?" {
      let random_chord_intervals = {
        let index = rand::thread_rng().gen_range(0, custom_chords.len());
        custom_chords[index].1.clone()
      };

      Self {
        intervals: random_chord_intervals,
        transpose: 0,
      }
    } else if interval_chord_string.trim() == "??" {
      let random_chord_intervals = {
        let mut custom_chords_intervals = Vec::with_capacity(custom_chords.len());

        for (_name, intervals) in custom_chords {
          custom_chords_intervals.push(intervals.clone());
        }

        let mut all_chords = chords_to_vector();
        all_chords.append(&mut custom_chords_intervals);

        let index = rand::thread_rng().gen_range(0, all_chords.len());
        all_chords[index].clone()
      };

      Self {
        intervals: random_chord_intervals,
        transpose: 0,
      }
    } else {
      let mut chord = IntervalChord::from_string(interval_chord_string);

      let look_for_custom_chords = chord.intervals.is_empty();
      if look_for_custom_chords {
        for (name, intervals) in custom_chords {
          if name.trim() == interval_chord_string.trim() {
            chord.intervals = intervals.clone();
            break;
          }
        }
      }

      chord
    }
  }

  pub fn new(intervals: Vec<i8>, transpose: i8) -> Self {
    Self {
      intervals,
      transpose,
    }
  }

  pub fn to_midi(&self) -> Vec<u8> {
    let mut resolved_notes = Vec::with_capacity(self.intervals.len());
    for interval in &self.intervals {
      let midi_note = notes::to_midi_note(interval + self.transpose);
      resolved_notes.push(midi_note);
    }
    resolved_notes
  }

  pub fn transpose(&mut self, transpose_delta: i8) -> &mut Self {
    self.transpose += transpose_delta;
    self
  }

  pub fn transpose_octave(&mut self, octave_delta: i8) -> &mut Self {
    const NOTES_IN_OCTAVE_COUNT: i8 = 12;
    self.transpose += octave_delta * NOTES_IN_OCTAVE_COUNT;
    self
  }

  pub fn is_empty(&self) -> bool {
    self.intervals.is_empty()
  }

  pub fn get_interval(&self, index: usize) -> i8 {
    self.intervals[index] + self.transpose
  }

  pub fn len(&self) -> usize {
    self.intervals.len()
  }
}

mod tests {
  #[test]
  fn test_interval_chord() {
    use crate::theory::chords::*;
    let mut chord = IntervalChord::from_string("POWER");
    assert_eq!(chord.intervals, vec![0, 7]);
    chord.transpose(-1);
    chord.transpose_octave(1);
    assert_eq!(chord.transpose, 11);

    let chord = IntervalChord::from_string("  MINOR    ");
    assert_eq!(chord.intervals, vec![0, 3, 7]);

    let chord = IntervalChord::from_string(" POsdsdWER >");
    assert_eq!(chord.intervals.len(), 0);

    let chord = IntervalChord::from_string(" MINOR  -5 ");
    assert_eq!(chord.intervals.len(), 0);
    assert_eq!(chord.transpose, 0);
  }

  #[test]
  fn test_interval_random() {
    use crate::theory::chords::*;
    let custom_chords = vec![("customA".to_string(), vec![0, 1, 2])];

    let chord = IntervalChord::from_string_with_custom("?", &custom_chords);
    assert_eq!(chord.intervals, vec![0, 1, 2]);

    let custom_chords = vec![
      ("customA".to_string(), vec![0, 1, 2, 5, 6]),
      ("customB".to_string(), vec![3, 4, 5, 8, 9]),
    ];

    let chord = IntervalChord::from_string_with_custom("?", &custom_chords);
    assert_eq!(chord.intervals.len(), 5);

    let chord = IntervalChord::from_string_with_custom("??", &custom_chords);
    assert_ne!(chord.intervals.len(), 0);
  }

  #[test]
  fn test_interval_chord_custom() {
    use crate::theory::chords::*;
    let custom_chords = vec![
      ("customA".to_string(), vec![0, 1, 2]),
      ("customB".to_string(), vec![3, 4, 5]),
    ];

    let chord = IntervalChord::from_string_with_custom("POWER", &custom_chords);
    assert_eq!(chord.intervals, vec![0, 7]);

    let chord = IntervalChord::from_string_with_custom("  customA", &custom_chords);
    assert_eq!(chord.intervals, vec![0, 1, 2]);

    let chord = IntervalChord::from_string_with_custom("customB ", &custom_chords);
    assert_eq!(chord.intervals, vec![3, 4, 5]);
  }
}
