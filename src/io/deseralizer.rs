#![allow(dead_code)]

pub type CustomChords = Vec<(String, Vec<i8>)>;
pub type PatternObject = (u16, u8, u8, String, i8);

pub fn deserialize_file(file_name: &str) -> Result<CompositionParameters, crate::FailResult> {
  match std::fs::read_to_string(file_name) {
    Ok(stream) => Ok(deserialize_string(&stream)?),
    _ => Err(crate::FailResult::Deserialize),
  }
}

pub fn deserialize_string(string: &str) -> Result<CompositionParameters, crate::FailResult> {
  match serde_yaml::from_str(&string) {
    Ok(deserialize) => Ok(deserialize),
    _ => Err(crate::FailResult::Deserialize),
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CompositionParameters {
  name: Option<String>,
  master: Option<MasterParameters>,
  chords: Option<CustomChords>,
  patterns: Option<Vec<PatternParameters>>,
}

impl CompositionParameters {
  pub fn new() -> Self {
    Self {
      name: None,
      master: None,
      chords: None,
      patterns: None,
    }
  }

  pub fn get_name(&self) -> String {
    match &self.name {
      Some(name) => name.clone(),
      None => "Unnamed composition".to_string(),
    }
  }

  pub fn get_master(&self) -> &Option<MasterParameters> {
    &self.master
  }

  pub fn get_custom_chords(&self) -> &Option<CustomChords> {
    &self.chords
  }

  pub fn get_patterns(&self) -> &Option<Vec<PatternParameters>> {
    &self.patterns
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MasterParameters {
  key: Option<String>,
  time: Option<u8>,
  signature: Option<(u8, u8)>,
}

impl MasterParameters {
  const DEFAULT_KEY: &'static str = "C";
  const DEFAULT_TIME: u8 = 120;
  const DEFAULT_SIGNATURE: (u8, u8) = (4, 4);
  const DEFAULT_METRONOME: bool = true;
  const DEFAULT_VERBOSE: u8 = 4;

  pub fn from_overrides(defaults: &MasterParameters, overrides: &MasterParameters) -> Self {
    let key = Some(match &overrides.key {
      Some(key) => key.clone(),
      None => defaults.get_key_or_default(),
    });

    let time = Some(match overrides.time {
      Some(time) => time,
      None => defaults.get_time_or_default(),
    });

    let signature = Some(match overrides.signature {
      Some(signature) => signature,
      None => defaults.get_signature_or_default(),
    });

    Self {
      key,
      time,
      signature,
    }
  }

  pub fn get_key(&self) -> Option<String> {
    match &self.key {
      Some(key) => Some(key.clone()),
      None => None,
    }
  }
  pub fn get_key_or_default(&self) -> String {
    match &self.key {
      Some(key) => key.clone(),
      None => MasterParameters::DEFAULT_KEY.to_string(),
    }
  }
  pub fn get_time(&self) -> Option<u8> {
    match self.time {
      Some(time) => Some(time),
      None => None,
    }
  }
  pub fn get_time_or_default(&self) -> u8 {
    match self.time {
      Some(time) => time,
      None => MasterParameters::DEFAULT_TIME,
    }
  }
  pub fn get_signature(&self) -> Option<(u8, u8)> {
    match self.signature {
      Some(signature) => Some(signature),
      None => None,
    }
  }
  pub fn get_signature_or_default(&self) -> (u8, u8) {
    match self.signature {
      Some(signature) => signature,
      None => MasterParameters::DEFAULT_SIGNATURE,
    }
  }
  pub fn get_all_or_defaults(&self) -> (String, u8, (u8, u8)) {
    (
      self.get_key_or_default(),
      self.get_time_or_default(),
      self.get_signature_or_default(),
    )
  }
}
impl Default for MasterParameters {
  fn default() -> Self {
    Self {
      key: Some(MasterParameters::DEFAULT_KEY.to_string()),
      time: Some(MasterParameters::DEFAULT_TIME),
      signature: Some(MasterParameters::DEFAULT_SIGNATURE),
    }
  }
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PatternParameters {
  name: Option<String>,
  master: Option<MasterParameters>,
  pattern: Option<Vec<PatternObject>>,
}

impl PatternParameters {
  pub fn get_name(&self) -> &Option<String> {
    &self.name
  }

  pub fn get_pattern(&self) -> &Option<Vec<PatternObject>> {
    &self.pattern
  }

  pub fn get_master(&self) -> &Option<MasterParameters> {
    &self.master
  }
}

mod tests {

  #[test]
  fn test_yaml_import() {
    use crate::io::deseralizer::*;

    let params = deserialize_string(
      r#"
        name: bc_000_a

        # Can be overridden by patterns
        master:
            key: D
            time: 128
            signature: [3, 4]

        structure:
            - part_a
            - part_b

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
                  - [2,2,1, custom1, 0]
                  - [3,1,1, MAJOR_NINTH, -3]
                  - [3,2,1, custom1, 0]

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
                  - [2,2,1, custom1, 0]
                  - [3,1,1, MAJOR_NINTH, -3]
                  - [3,2,1, custom1, 0]

            "#,
    )
    .unwrap();

    assert_eq!(params.get_name(), "bc_000_a");

    match params.get_master() {
      Some(master) => {
        assert_eq!(master.get_key_or_default(), "D");
        assert_eq!(master.get_time_or_default(), 128);
        assert_eq!(master.get_signature_or_default(), (3, 4));
      }
      None => assert!(false),
    };

    match params.get_custom_chords() {
      Some(chords) => {
        assert_eq!(chords, &[("custom1".to_string(), vec![0, 3, 8])]);
      }
      None => assert!(false),
    };

    match params.get_patterns() {
      Some(patterns) => {
        let pattern = &patterns[0];
        assert_eq!(pattern.get_name(), &Some("part_a".to_string()));
        assert_eq!(pattern.get_master(), &None);
        match pattern.get_pattern() {
          Some(events) => assert_eq!(&events.len(), &8usize),
          None => assert!(false),
        }

        let pattern = &patterns[1];
        assert_eq!(pattern.get_name(), &Some("part_b".to_string()));

        match pattern.get_master() {
          Some(master) => {
            assert_eq!(master.get_key(), Some("C#".to_string()));
            assert_eq!(master.get_time(), Some(69));
            assert_eq!(master.get_signature(), Some((4, 8)));
          }
          None => assert!(false),
        };

        match pattern.get_pattern() {
          Some(events) => assert_eq!(&events.len(), &8usize),
          None => assert!(false),
        }
      }
      None => assert!(false),
    };
  }

  #[test]
  fn test_yaml_import_missing() {
    use crate::io::deseralizer::*;

    // Parse parameters
    let params = deserialize_string(
      r#"
        # Empty
        empty: empty 
        "#,
    )
    .unwrap();

    assert_eq!(params.get_name(), "Unnamed composition");
    assert_eq!(params.get_master(), &None);
    assert_eq!(params.get_custom_chords(), &None);
    assert_eq!(params.get_patterns(), &None);
  }

  #[test]
  fn test_yaml_import_none() {
    use crate::io::deseralizer::*;

    assert_eq!(
      deserialize_file("not a path"),
      Err(crate::FailResult::Deserialize)
    );
    assert_eq!(
      deserialize_string("not a path"),
      Err(crate::FailResult::Deserialize)
    );
  }

  #[test]
  fn test_master_defaults() {
    use crate::io::deseralizer::MasterParameters;

    let defaults = MasterParameters::default();

    const DEFAULT_KEY: &'static str = "C";
    const DEFAULT_TIME: u8 = 120;
    const DEFAULT_SIGNATURE: (u8, u8) = (4, 4);

    assert_eq!(defaults.get_key_or_default(), DEFAULT_KEY);
    assert_eq!(defaults.get_time_or_default(), DEFAULT_TIME);
    assert_eq!(defaults.get_signature_or_default(), DEFAULT_SIGNATURE);

    let (key, time, signature) = defaults.get_all_or_defaults();
    assert_eq!(key, DEFAULT_KEY);
    assert_eq!(time, DEFAULT_TIME);
    assert_eq!(signature, DEFAULT_SIGNATURE);
  }

  #[test]
  fn test_master_overrides() {
    use crate::io::deseralizer::MasterParameters;

    let defaults = MasterParameters {
      key: None,
      time: Some(130),
      signature: Some((4, 4)),
    };

    assert_eq!(defaults.get_key_or_default(), "C");
    assert_eq!(defaults.get_time_or_default(), 130);
    assert_eq!(defaults.get_signature_or_default(), (4, 4));

    let overrides = MasterParameters {
      key: Some("E".to_string()),
      time: None,
      signature: Some((3, 4)),
    };

    assert_eq!(overrides.get_key(), Some("E".to_string()));
    assert_eq!(overrides.get_time(), None);
    assert_eq!(overrides.get_signature(), Some((3, 4)));

    let master = MasterParameters::from_overrides(&defaults, &overrides);
    assert_eq!(master.get_key_or_default(), "E");
    assert_eq!(master.get_time_or_default(), 130);
    assert_eq!(master.get_signature_or_default(), (3, 4));
  }
}
