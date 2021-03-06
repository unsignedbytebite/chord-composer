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
  sampler_metronome: basic_sampler::SamplerPlayer,
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
    sample_paths_metronome: &Vec<String>,
    sample_paths_piano: &Vec<String>,
  ) -> Result<Self, FailResult> {
    if composition.len() > 0 {
      let sampler_metronome = basic_sampler::SamplerPlayer::new(sample_paths_metronome);
      let sampler_piano = basic_sampler::SamplerPlayer::new(&sample_paths_piano);
      let error_loading = sampler_metronome.is_err() || sampler_piano.is_err();
      if error_loading {
        Err(FailResult::LoadSampler)
      } else {
        Ok(PerformanceEngine {
          sampler_metronome: sampler_metronome.unwrap(),
          sampler_piano: sampler_piano.unwrap(),
          event_head: 0,
          current_pattern: &composition.get(0),
          is_playing: false,
          composition,
          state,
          is_metronome_enabled: false,
        })
      }
    } else {
      Err(FailResult::NoPatterns)
    }
  }

  pub fn run(&mut self) {
    self.run_from(&music_time::MusicTime::default(), 0);
  }

  pub fn run_from(&mut self, start_time: &music_time::MusicTime, starting_pattern_index: usize) {
    self.state.on_ready(self.composition);
    for pattern_index in starting_pattern_index..self.composition.get_patterns().len() {
      let pattern = &self.composition.get_patterns()[pattern_index];
      self.state.on_pattern_playback_begin(pattern);
      self.is_playing = true;

      // Create new music timer
      let mut music_timer = music_timer::create_performance_engine(
        pattern.get_time_signature().get_numerator(),
        pattern.get_time_signature().get_denominator(),
        pattern.get_bpm() as f32,
      );

      // Set the current time for playback and
      // advance events to that time
      self.event_head = pattern.find_next_event_index(start_time);
      music_timer.set_music_timer(*start_time);

      // Assign current pattern
      self.current_pattern = pattern;

      // Loop while playback enabled
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
      && current_time.get_beat() == self.current_pattern.get_time_signature().get_numerator()
      && current_time.get_beat_interval() == MAX_BEAT_INTERVALS);

    self.state.on_beat_interval_change(current_time);
    if !events_complete {
      let current_event = self.current_pattern.get(self.event_head);

      let (event_time, event_notes) = current_event;

      let is_event_trigger_time = current_time == event_time;
      if is_event_trigger_time {
        self.state.on_event(&current_event);
        self.event_head += 1;

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
      self.sampler_metronome.play(0);
    }
  }

  fn on_bar(&mut self, current_time: &music_time::MusicTime) {
    if !self.is_playing {
      return;
    }

    self.state.on_bar_change(current_time);
    if self.is_metronome_enabled {
      self.sampler_metronome.play(1);
    }
  }
}
