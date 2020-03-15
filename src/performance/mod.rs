pub mod performance_engine;

mod tests {
  #[test]
  fn test_performance_engine() {
    use crate::{
      performance_engine::{PerformanceEngine, PerformanceState},
      theory::composition,
    };
    use music_timer::{music_time, time_signature};

    struct MyState {
      callback_calls: u8,
      current_time: music_time::MusicTime,
    }

    impl PerformanceState for MyState {
      fn on_ready(&mut self, composition: &composition::Composition) {
        self.callback_calls += 1;
        assert_eq!(composition.get_name(), "test compo");
      }
      fn on_beat_interval_change(&mut self, current_time: &music_time::MusicTime) {
        self.callback_calls += 1;
        self.current_time = current_time.clone();
        println!("on_beat_interval_change: {:?}", current_time);
      }
      fn on_beat_change(&mut self, current_time: &music_time::MusicTime) {
        self.callback_calls += 1;
        println!("on_beat_change: {:?}", current_time);
      }
      fn on_bar_change(&mut self, current_time: &music_time::MusicTime) {
        self.callback_calls += 1;
        println!("on_bar_change: {:?}", current_time);
      }
      fn on_event(&mut self, event: &composition::PatternEvent) {
        self.callback_calls += 1;

        let (time, notes) = event;

        if time == &music_time::MusicTime::new(1, 3, 1) {
          let test_notes: Vec<u8> = Vec::new();
          assert_eq!(*notes, test_notes);
        }
        println!("event: {:?}", event);
      }
      fn on_pattern_playback_begin(&mut self, pattern: &composition::Pattern) {
        self.callback_calls += 1;

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
        if self.callback_calls < 119 {
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
      let mut pattern =
        composition::Pattern::new("pattern_z", 140, time_signature::TimeSignature::default());

      pattern.push_event(music_time::MusicTime::new(1, 3, 1), Vec::new());
      pattern.push_event(music_time::MusicTime::new(3, 1, 1), Vec::new());
      composition.push_pattern(pattern);

      let mut pattern =
        composition::Pattern::new("pattern_y", 130, time_signature::TimeSignature::default());

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
      PerformanceEngine::new(&composition, &mut my_state, &Vec::new(), &Vec::new());

    match performance_engine {
      Ok(mut performance) => {
        performance.set_metronome_enabled(true);
        performance.run();
      }
      _ => assert!(false, "Cannot create performance engine"),
    }
    assert_eq!(my_state.callback_calls, 232);
    assert_eq!(my_state.current_time, music_time::MusicTime::new(3, 4, 8));
  }

  #[test]
  fn test_performance_engine_run_from() {
    use crate::{
      performance_engine::{PerformanceEngine, PerformanceState},
      theory::composition,
    };
    use music_timer::{music_time, time_signature};

    struct MyState {
      beat_interval_count: u8,
      current_time: music_time::MusicTime,
    }

    impl PerformanceState for MyState {
      fn on_ready(&mut self, _composition: &composition::Composition) {}
      fn on_beat_interval_change(&mut self, current_time: &music_time::MusicTime) {
        self.beat_interval_count += 1;
        self.current_time = current_time.clone();
        println!("on_beat_interval_change: {:?}", current_time);
      }
      fn on_beat_change(&mut self, _current_time: &music_time::MusicTime) {}
      fn on_bar_change(&mut self, _current_time: &music_time::MusicTime) {}
      fn on_event(&mut self, _event: &composition::PatternEvent) {}
      fn on_pattern_playback_begin(&mut self, _pattern: &composition::Pattern) {}
      fn on_pattern_playback_end(&mut self, _pattern: &composition::Pattern) {}
      fn on_completed(&mut self, _composition: &composition::Composition) {}
    }

    let composition = {
      let mut composition = composition::Composition::new("test compo");
      let mut pattern =
        composition::Pattern::new("pattern_z", 140, time_signature::TimeSignature::default());

      pattern.push_event(music_time::MusicTime::new(1, 3, 1), Vec::new());
      pattern.push_event(music_time::MusicTime::new(5, 1, 1), Vec::new());
      composition.push_pattern(pattern);

      composition
    };

    let mut my_state = MyState {
      beat_interval_count: 0,
      current_time: music_time::MusicTime::default(),
    };

    let performance_engine =
      PerformanceEngine::new(&composition, &mut my_state, &Vec::new(), &Vec::new());

    match performance_engine {
      Ok(mut performance) => {
        performance.set_metronome_enabled(true);
        performance.run_from(&music_time::MusicTime::new(4, 3, 5), 0);
      }
      _ => assert!(false, "Cannot create performance engine"),
    }
    assert_eq!(my_state.beat_interval_count, 44);
    assert_eq!(my_state.current_time, music_time::MusicTime::new(5, 4, 8));
  }
}
