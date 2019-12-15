#[cfg(feature = "with-sound")]
use crate::audio::basic_sampler;

use crate::{theory::composition, FailResult};
use music_timer::{music_time, music_timer_engine};
use std::{thread, time::Duration};

pub trait PerformanceState {
  fn on_ready(&mut self, composition: &composition::Composition);
  fn on_beat_interval_change(&mut self, current_time: &music_time::MusicTime);
  fn on_beat_change(&mut self, current_time: &music_time::MusicTime);
  fn on_bar_change(&mut self, current_time: &music_time::MusicTime);
  fn on_event(&mut self, event: &composition::PatternEvent);
  fn on_pattern_playback_begin(&mut self, pattern: &composition::Pattern);
  fn on_pattern_playback_end(&mut self, pattern: &composition::Pattern);
  fn on_completed(&mut self, composition: &composition::Composition);
}

pub struct PerformanceEngine<'a, State: PerformanceState> {
  #[cfg(feature = "with-sound")]
  sampler_metronome: basic_sampler::SamplerPlayer,
  #[cfg(feature = "with-sound")]
  sampler_piano: basic_sampler::SamplerPlayer,
  event_head: usize,
  current_pattern: &'a composition::Pattern,
  is_playing: bool,
  composition: &'a composition::Composition,
  state: &'a mut State,
  is_metronome_enabled: bool,
}

impl<'a, State: PerformanceState> PerformanceEngine<'a, State> {
  pub fn new(
    composition: &'a composition::Composition,
    state: &'a mut State,
  ) -> Result<Self, FailResult> {
    if composition.len() == 0 {
      // This should never panic IRL, the parsing should have picked up this error beforehand.
      panic!("PerformanceEngine cannot be created with no patterns in the composition!");
    }

    #[cfg(feature = "with-sound")]
    let sampler_metronome = basic_sampler::SamplerPlayer::new(&vec![
      "./audio_assets/metronome/tock.ogg".to_string(),
      "./audio_assets/metronome/tick.ogg".to_string(),
    ]);

    #[cfg(feature = "with-sound")]
    let sampler_piano = {
      let mut sample_paths = Vec::new();
      for i in 1..61 {
        let path = format!("audio_assets/piano_instrument/piano ({}).ogg", i);
        sample_paths.push(path);
      }

      basic_sampler::SamplerPlayer::new(&sample_paths)
    };

    #[cfg(feature = "with-sound")]
    let error_loading = sampler_metronome.is_err() || sampler_piano.is_err();

    #[cfg(not(feature = "with-sound"))]
    let error_loading = false;

    if error_loading {
      Err(FailResult::LoadSampler)
    } else {
      Ok(PerformanceEngine {
        #[cfg(feature = "with-sound")]
        sampler_metronome: sampler_metronome.unwrap(),
        #[cfg(feature = "with-sound")]
        sampler_piano: sampler_piano.unwrap(),
        event_head: 0,
        current_pattern: &composition.get(0),
        is_playing: false,
        composition,
        state,
        is_metronome_enabled: false,
      })
    }
  }

  pub fn run(&mut self) {
    self.state.on_ready(self.composition);
    for pattern in self.composition.get_patterns() {
      self.state.on_pattern_playback_begin(pattern);
      self.is_playing = true;
      self.event_head = 0;

      let mut music_timer = music_timer::create_performance_engine(
        pattern.get_time_signature().get_numerator(),
        pattern.get_time_signature().get_denominator(),
        pattern.get_bpm() as f32,
      );

      self.current_pattern = pattern;

      while self.is_playing {
        music_timer.pulse(self);
        const PULSE_RESOLUTION: Duration = Duration::from_millis(16);
        thread::sleep(PULSE_RESOLUTION);
      }
      self.state.on_pattern_playback_end(pattern);
    }

    self.state.on_completed(self.composition);
  }

  pub fn set_metronome_enabled(&mut self, is_enabled: bool) {
    self.is_metronome_enabled = is_enabled;
  }
}

impl<'a, State: PerformanceState> music_timer_engine::MusicTimerState
  for PerformanceEngine<'a, State>
{
  fn on_beat_interval(&mut self, current_time: &music_time::MusicTime) {
    let events_complete = self.event_head == self.current_pattern.len();

    const MAX_BEAT_INTERVALS: u8 = 8;
    self.is_playing = !(events_complete
      && current_time.get_beat() == 3
      && current_time.get_beat_interval() == MAX_BEAT_INTERVALS);

    self.state.on_beat_interval_change(current_time);
    if !events_complete {
      let current_event = self.current_pattern.get(self.event_head);

      #[cfg(feature = "with-sound")]
      let (event_time, event_notes) = current_event;

      #[cfg(not(feature = "with-sound"))]
      let (event_time, _event_notes) = current_event;

      let is_event_trigger_time = current_time == event_time;
      if is_event_trigger_time {
        self.state.on_event(&current_event);
        self.event_head += 1;

        #[cfg(feature = "with-sound")]
        for note in event_notes {
          let sample_index = {
            const MIDI_OFFSET: usize = 24;
            *note as usize - MIDI_OFFSET
          };

          self.sampler_piano.play(sample_index);
        }
      }
    }
  }

  fn on_beat(&mut self, current_time: &music_time::MusicTime) {
    if !self.is_playing {
      return;
    }
    self.state.on_beat_change(current_time);
    if self.is_metronome_enabled && current_time.get_beat() != 1 {
      #[cfg(feature = "with-sound")]
      self.sampler_metronome.play(0);
    }
  }

  fn on_bar(&mut self, current_time: &music_time::MusicTime) {
    if !self.is_playing {
      return;
    }
    self.state.on_bar_change(current_time);

    if self.is_metronome_enabled {
      #[cfg(feature = "with-sound")]
      self.sampler_metronome.play(1);
    }
  }
}
