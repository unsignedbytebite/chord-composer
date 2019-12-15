pub mod performance_engine;

#[test]
fn test_performance_engine() {
  use crate::theory::composition;
  use music_timer::{music_time, time_signature};

  struct MyState {
    callback_calls: u8,
    current_time: music_time::MusicTime,
  }

  impl performance_engine::PerformanceState for MyState {
    fn on_ready(&mut self, composition: &composition::Composition) {
      self.callback_calls += 1;
      assert_eq!(composition.get_name(), "test compo");
    }
    fn on_beat_interval_change(&mut self, current_time: &music_time::MusicTime) {
      self.callback_calls += 1;
      self.current_time = current_time.clone();
    }
    fn on_beat_change(&mut self, _current_time: &music_time::MusicTime) {
      self.callback_calls += 1;
    }
    fn on_bar_change(&mut self, _current_time: &music_time::MusicTime) {
      self.callback_calls += 1;
    }
    fn on_event(&mut self, event: &composition::PatternEvent) {
      self.callback_calls += 1;

      let (time, notes) = event;

      if time == &music_time::MusicTime::new(1, 3, 1) {
        let test_notes: Vec<u8> = Vec::new();
        assert_eq!(*notes, test_notes);
      }
    }
    fn on_pattern_playback_begin(&mut self, pattern: &composition::Pattern) {
      self.callback_calls += 1;

      println!(">{}", self.callback_calls);
      if self.callback_calls < 80 {
        assert_eq!(pattern.get_name(), "pattern_z");
        assert_eq!(
          pattern.get_time_signature(),
          time_signature::TimeSignature::default()
        );
        assert_eq!(pattern.get_bpm(), 140);
        assert_eq!(pattern.len(), 2);
      } else {
        assert_eq!(pattern.get_name(), "pattern_y");
        assert_eq!(
          pattern.get_time_signature(),
          time_signature::TimeSignature::default()
        );
        assert_eq!(pattern.get_bpm(), 130);
        assert_eq!(pattern.len(), 2);
      }
    }
    fn on_pattern_playback_end(&mut self, pattern: &composition::Pattern) {
      self.callback_calls += 1;
      if self.callback_calls < 109 {
        assert_eq!(pattern.get_name(), "pattern_z");
      } else {
        assert_eq!(pattern.get_name(), "pattern_y");
      }
    }
    fn on_completed(&mut self, composition: &composition::Composition) {
      self.callback_calls += 1;
      assert_eq!(composition.get_name(), "test compo");
    }
  }

  let composition = {
    let mut composition = composition::Composition::new("test compo");
    let mut pattern = composition::Pattern::new(
      "pattern_z".to_string(),
      140,
      time_signature::TimeSignature::default(),
    );

    pattern.push_event(music_time::MusicTime::new(1, 3, 1), Vec::new());
    pattern.push_event(music_time::MusicTime::new(3, 1, 1), Vec::new());
    composition.push_pattern(pattern);

    let mut pattern = composition::Pattern::new(
      "pattern_y".to_string(),
      130,
      time_signature::TimeSignature::default(),
    );

    pattern.push_event(music_time::MusicTime::new(1, 3, 1), Vec::new());
    pattern.push_event(music_time::MusicTime::new(3, 1, 1), Vec::new());
    composition.push_pattern(pattern);

    composition
  };

  let mut my_state = MyState {
    callback_calls: 0,
    current_time: music_time::MusicTime::default(),
  };

  let performance_engine =
    performance_engine::PerformanceEngine::new(&composition, &mut my_state);

  match performance_engine {
    Ok(mut performance) => {
      performance.set_metronome_enabled(true);
      performance.run();
    }
    _ => assert!(false, "Cannot create performance engine"),
  }
  assert_eq!(my_state.callback_calls, 216);
  assert_eq!(my_state.current_time, music_time::MusicTime::new(3, 3, 8));
}
