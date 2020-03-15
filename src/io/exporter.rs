use crate::composition;
use ghakuf::{
  messages::{Message, MetaEvent, MidiEvent},
  writer::Writer,
};
use music_timer::music_time;
use std::path;

pub fn export_composition(
  composition: &composition::Composition,
  parent_directory: &str,
) -> Result<crate::SuccessResult, crate::FailResult> {
  let mut pattern_midi = Vec::with_capacity(composition.len());
  for pattern in composition.get_patterns() {
    let pattern_messages = {
      let pattern_name = pattern.get_name().to_string();
      let mut messages = pattern_to_midi_meta(pattern);
      messages.append(&mut pattern_to_midi_messages(pattern));
      (pattern_name, messages)
    };

    pattern_midi.push(pattern_messages);
  }

  let name = composition.get_name();
  export_midi_patterns(name, &pattern_midi, parent_directory)
}

pub fn export_midi_patterns(
  composition_name: &str,
  patterns: &Vec<(String, Vec<Message>)>,
  parent_directory: &str,
) -> Result<crate::SuccessResult, crate::FailResult> {
  let target_dir = format!("{}/{}", parent_directory, composition_name);
  
  // Flush directory
  let _ = std::fs::remove_dir_all(&target_dir).is_ok();
  std::fs::create_dir(&target_dir).unwrap();

  // Export patterns
  let mut export_paths = Vec::with_capacity(patterns.len());
  for (pattern_name, midi_messages) in patterns {
    let path = format!("{}/{}.mid", target_dir, pattern_name);
    export_midi_messages(&path, midi_messages)?;
    export_paths.push(path);
  }

  Ok(crate::SuccessResult::Export(export_paths))
}

fn export_midi_messages(path: &str, midi_messages: &Vec<Message>) -> Result<(), crate::FailResult> {
  let path = path::Path::new(path);
  let mut writer = Writer::new();
  writer.running_status(false);

  for message in midi_messages {
    writer.push(message);
  }

  match writer.write(&path) {
    Err(_) => Err(crate::FailResult::ExportMIDI),
    _ => Ok(()),
  }
}

fn pattern_to_midi_meta(pattern: &composition::Pattern) -> Vec<Message> {
  let mut messages = Vec::with_capacity(4);
  let tempo = to_tempo_samples(pattern.get_bpm());
  messages.push(Message::MetaEvent {
    delta_time: 0,
    event: MetaEvent::SetTempo,
    data: vec![(tempo >> 16) as u8, (tempo >> 8) as u8, tempo as u8],
  });

  let time_signature = time_signature_to_data(pattern.get_time_signature().as_tuple());

  messages.push(Message::MetaEvent {
    delta_time: 0,
    event: MetaEvent::TimeSignature,
    data: time_signature,
  });

  messages.push(Message::MetaEvent {
    delta_time: 0,
    event: MetaEvent::EndOfTrack,
    data: Vec::new(),
  });

  messages.push(Message::TrackChange);

  messages
}

fn pattern_to_midi_messages(pattern: &composition::Pattern) -> Vec<Message> {
  let mut messages = Vec::new();
  let mut total_time = 0;

  messages.push(Message::MetaEvent {
    delta_time: 0,
    event: MetaEvent::SequenceOrTrackName,
    data: pattern.get_name().as_bytes().to_vec(),
  });

  for i in 0..pattern.get_events().len() {
    let (music_time, intervals) = pattern.get(i);

    // Skip empty intervals
    if intervals.is_empty() {
      continue;
    }

    let mut push_event = |smf_note: u8, time: u32, velocity: u8| {
      messages.push(Message::MidiEvent {
        delta_time: time,
        event: MidiEvent::NoteOn {
          ch: 0,
          note: smf_note,
          velocity: velocity,
        },
      });
    };

    let mut push_events = |velocity: u8, bar: u16, beat: u8, beat_interval: u8| {
      let delta_time = {
        let numerator = pattern.get_time_signature().get_numerator();
        let tick_time = to_tick_time(numerator, bar, beat, beat_interval);
        let delta_time = tick_time - total_time;
        total_time = to_tick_time(numerator, bar, beat, beat_interval);
        delta_time
      };
      // Push notes into the chord
      push_event(intervals[0] as u8, delta_time, velocity);
      for i in 1..intervals.len() {
        push_event(intervals[i], 0, velocity);
      }
    };

    // Note on
    const NOTE_ON: u8 = 64;
    push_events(
      NOTE_ON,
      music_time.get_bar(),
      music_time.get_beat(),
      music_time.get_beat_interval(),
    );

    // Note off
    let legato_end = {
      let is_last_element = i == pattern.get_events().len() - 1;
      if is_last_element {
        music_time::MusicTime::new(music_time.get_bar() + 1, 1, 1)
      } else {
        let (next_music_time, _next_intervals) = pattern.get(i + 1);
        next_music_time.clone()
      }
    };

    const NOTE_OFF: u8 = 0;
    push_events(
      NOTE_OFF,
      legato_end.get_bar(),
      legato_end.get_beat(),
      legato_end.get_beat_interval(),
    );
  }

  // End track
  messages.push(Message::MetaEvent {
    delta_time: 0,
    event: MetaEvent::EndOfTrack,
    data: Vec::new(),
  });

  messages
}
fn time_signature_to_data(time_signature: (u8, u8)) -> Vec<u8> {
  let (numerator, denominator) = time_signature;
  let dd = match denominator {
    2 => 1,
    4 => 2,
    8 => 3,
    16 => 4,
    32 => 5,
    64 => 6,
    _ => 0,
  };

  vec![numerator, dd, 8, 24]
}

fn to_tick_time(time_numerator: u8, bar: u16, beat: u8, beat_interval: u8) -> u32 {
  const BEAT_INTERVAL_SAMPLE: u32 = 60;
  const BEAT_SAMPLE: u32 = BEAT_INTERVAL_SAMPLE * 8;

  let bar_corrected = bar as u32 - 1;
  let beat_corrected = beat as u32 - 1;
  let beat_interval_corrected = beat_interval as u32 - 1;

  (bar_corrected * time_numerator as u32 * BEAT_SAMPLE)
    + (beat_corrected * BEAT_SAMPLE)
    + (beat_interval_corrected * BEAT_INTERVAL_SAMPLE)
}

fn to_tempo_samples(bpm: u8) -> u32 {
  (60.0 * 1000000.0 / bpm as f64) as u32
}

mod test {

  #[test]
  fn test_time_signature_convert() {
    use crate::io::exporter::*;
    let time_signature = time_signature_to_data((4, 4));
    assert_eq!(time_signature, [4, 2, 8, 24]);

    let time_signature = time_signature_to_data((2, 4));
    assert_eq!(time_signature, [2, 2, 8, 24]);
    let time_signature = time_signature_to_data((2, 2));
    assert_eq!(time_signature, [2, 1, 8, 24]);

    let time_signature = time_signature_to_data((2, 5));
    assert_eq!(time_signature, [2, 0, 8, 24]);

    let time_signature = time_signature_to_data((7, 8));
    assert_eq!(time_signature, [7, 3, 8, 24]);
  }

  #[test]
  fn test_correct_tick_time() {
    use crate::io::exporter::*;
    const BEAT_INTERVAL_SAMPLE: u32 = 60;
    const BEAT_SAMPLE: u32 = BEAT_INTERVAL_SAMPLE * 8;
    assert_eq!(to_tick_time(4, 1, 1, 1), 0);
    assert_eq!(to_tick_time(4, 1, 2, 1), BEAT_SAMPLE);
    assert_eq!(to_tick_time(4, 1, 3, 1), BEAT_SAMPLE * 2);
    assert_eq!(to_tick_time(4, 1, 4, 1), BEAT_SAMPLE * 3);
    assert_eq!(to_tick_time(4, 2, 1, 1), BEAT_SAMPLE * 4);
    assert_eq!(to_tick_time(4, 2, 2, 1), BEAT_SAMPLE * 5);
    assert_eq!(to_tick_time(4, 2, 3, 1), BEAT_SAMPLE * 6);
    assert_eq!(to_tick_time(4, 2, 4, 1), BEAT_SAMPLE * 7);
    assert_eq!(to_tick_time(4, 2, 5, 1), BEAT_SAMPLE * 8);

    assert_eq!(to_tick_time(3, 1, 1, 1), 0);
    assert_eq!(to_tick_time(3, 1, 2, 1), BEAT_SAMPLE);
    assert_eq!(to_tick_time(3, 1, 3, 1), BEAT_SAMPLE * 2);
    assert_eq!(to_tick_time(3, 2, 1, 1), BEAT_SAMPLE * 3);
    assert_eq!(to_tick_time(3, 2, 2, 1), BEAT_SAMPLE * 4);
    assert_eq!(to_tick_time(3, 2, 3, 1), BEAT_SAMPLE * 5);
    assert_eq!(to_tick_time(3, 3, 1, 1), BEAT_SAMPLE * 6);
    assert_eq!(to_tick_time(3, 3, 2, 1), BEAT_SAMPLE * 7);
    assert_eq!(to_tick_time(3, 3, 3, 1), BEAT_SAMPLE * 8);

    assert_eq!(to_tick_time(4, 1, 1, 1), 0);
    assert_eq!(to_tick_time(4, 1, 1, 2), BEAT_INTERVAL_SAMPLE);
    assert_eq!(to_tick_time(4, 1, 1, 3), BEAT_INTERVAL_SAMPLE * 2);
    assert_eq!(to_tick_time(4, 1, 1, 4), BEAT_INTERVAL_SAMPLE * 3);
    assert_eq!(to_tick_time(4, 1, 1, 5), BEAT_INTERVAL_SAMPLE * 4);
    assert_eq!(to_tick_time(4, 1, 1, 6), BEAT_INTERVAL_SAMPLE * 5);
    assert_eq!(to_tick_time(4, 1, 1, 7), BEAT_INTERVAL_SAMPLE * 6);
    assert_eq!(to_tick_time(4, 1, 1, 8), BEAT_INTERVAL_SAMPLE * 7);
    assert_eq!(to_tick_time(4, 1, 2, 1), BEAT_INTERVAL_SAMPLE * 8);
    assert_eq!(to_tick_time(4, 1, 2, 2), BEAT_INTERVAL_SAMPLE * 9);
    assert_eq!(to_tick_time(4, 1, 2, 3), BEAT_INTERVAL_SAMPLE * 10);
    assert_eq!(to_tick_time(4, 1, 2, 4), BEAT_INTERVAL_SAMPLE * 11);
    assert_eq!(to_tick_time(4, 1, 2, 5), BEAT_INTERVAL_SAMPLE * 12);
    assert_eq!(to_tick_time(4, 1, 2, 6), BEAT_INTERVAL_SAMPLE * 13);
    assert_eq!(to_tick_time(4, 1, 2, 7), BEAT_INTERVAL_SAMPLE * 14);
    assert_eq!(to_tick_time(4, 1, 2, 8), BEAT_INTERVAL_SAMPLE * 15);
  }

  #[test]
  fn test_event_delta() {
    use crate::io::exporter::*;
    let mut total_time = 0;

    let mut to_delta_time = |bar: u16, beat: u8, beat_interval: u8| -> u32 {
      let delta_time = to_tick_time(4, bar, beat, beat_interval) - total_time;
      total_time = to_tick_time(4, bar, beat, beat_interval);
      delta_time
    };

    assert_eq!(to_delta_time(1, 1, 1), 0);
    assert_eq!(to_delta_time(1, 2, 1), 480);
    assert_eq!(to_delta_time(2, 1, 1), 1440);
    assert_eq!(to_delta_time(2, 2, 1), 480);
    assert_eq!(to_delta_time(3, 1, 1), 1440);
    assert_eq!(to_delta_time(3, 1, 2), 60);
    assert_eq!(to_delta_time(3, 1, 3), 60);
    assert_eq!(to_delta_time(3, 1, 6), 180);
  }
}
