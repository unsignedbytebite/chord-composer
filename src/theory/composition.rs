use music_timer::{music_time::MusicTime, time_signature::TimeSignature};

pub type PatternEvent = (MusicTime, Vec<u8>);

#[derive(Debug)]
pub struct Pattern {
  name: String,
  bpm: u8,
  signature: TimeSignature,
  events: Vec<PatternEvent>,
}

impl Pattern {
  pub fn new(name: String, bpm: u8, signature: TimeSignature) -> Self {
    Pattern {
      name,
      bpm,
      signature,
      events: Vec::new(),
    }
  }

  pub fn push_event(&mut self, time: MusicTime, notes: Vec<u8>) -> &Self {
    self.events.push((time, notes));
    self
  }

  pub fn len(&self) -> usize {
    self.events.len()
  }

  pub fn get_bpm(&self) -> u8 {
    self.bpm
  }

  pub fn get_time_signature(&self) -> TimeSignature {
    self.signature
  }

  pub fn get(&self, index: usize) -> &PatternEvent {
    &self.events[index]
  }

  pub fn get_events(&self) -> &Vec<PatternEvent> {
    &self.events
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }
}

#[derive(Debug)]
pub struct Composition {
  name: String,
  patterns: Vec<Pattern>,
}

impl Composition {
  pub fn new(name: &str) -> Self {
    Composition {
      name: name.to_string(),
      patterns: Vec::new(),
    }
  }

  pub fn push_new_pattern(&mut self, name: String, bpm: u8, signature: TimeSignature) {
    self.patterns.push(Pattern::new(name, bpm, signature));
  }

  pub fn push_pattern(&mut self, pattern: Pattern) {
    self.patterns.push(pattern);
  }

  pub fn len(&self) -> usize {
    self.patterns.len()
  }
  pub fn get(&self, index: usize) -> &Pattern {
    &self.patterns[index]
  }

  pub fn get_mut(&mut self, index: usize) -> &mut Pattern {
    &mut self.patterns[index]
  }

  pub fn get_patterns(&self) -> &Vec<Pattern> {
    &self.patterns
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }
}

mod tests {
  #[test]
  fn test_create() {
    use crate::composition::*;
    let mut compo = Composition::new("test");
    assert_eq!(compo.get_name(), "test");

    compo.push_new_pattern("a".to_string(), 120, TimeSignature::default());
    compo
      .get_mut(0)
      .push_event(MusicTime::new(1, 1, 1), vec![0, 1, 2]);
    compo
      .get_mut(0)
      .push_event(MusicTime::new(2, 1, 1), vec![2, 3, 4]);

    compo.push_new_pattern("a".to_string(), 54, TimeSignature::new(3, 4));
    compo
      .get_mut(1)
      .push_event(MusicTime::new(1, 3, 1), vec![51, 51, 52]);
    compo
      .get_mut(1)
      .push_event(MusicTime::new(2, 4, 1), vec![52, 53, 54]);

    assert_eq!(compo.len(), 2);
    assert_eq!(compo.get(0).len(), 2);
    assert_eq!(compo.get(1).len(), 2);

    assert_eq!(compo.get(0).get_bpm(), 120);
    assert_eq!(compo.get(0).get_time_signature(), TimeSignature::new(4, 4));

    assert_eq!(compo.get(1).get_bpm(), 54);
    assert_eq!(compo.get(1).get_time_signature(), TimeSignature::new(3, 4));

    let (time, notes) = compo.get(0).get(0);
    assert_eq!(time, &MusicTime::new(1, 1, 1));
    assert_eq!(notes, &vec![0, 1, 2]);

    let (time, notes) = compo.get(0).get(1);
    assert_eq!(time, &MusicTime::new(2, 1, 1));
    assert_eq!(notes, &vec![2, 3, 4]);

    let (time, notes) = compo.get(1).get(0);
    assert_eq!(time, &MusicTime::new(1, 3, 1));
    assert_eq!(notes, &vec![51, 51, 52]);

    let (time, notes) = compo.get(1).get(1);
    assert_eq!(time, &MusicTime::new(2, 4, 1));
    assert_eq!(notes, &vec![52, 53, 54]);
  }
}
