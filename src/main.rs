mod language;

use chord_composer::{
  performance::performance_engine::PerformanceState,
  theory::{
    composition::{Composition, Pattern, PatternEvent},
    notes,
  },
  FailResult, SuccessResult,
};
use clap::{App, Arg, SubCommand};
use language::strings;
use music_timer::music_time::MusicTime;

/// Parse the results of a chord composer interaction.
///
/// # Arguments
/// * `result` - The result from the interaction.
fn parse_result(result: &Result<SuccessResult, FailResult>) {
  match result {
    Ok(SuccessResult::Export(exported_files)) => {
      for exported_file in exported_files {
        println!("{}{}", strings::STRING_EXPORTED, exported_file);
      }
      println!("{}", strings::STRING_EXPORTED_COMPLETE)
    }
    Ok(SuccessResult::Playback) => println!("{}", strings::STRING_PLAYBACK_COMPLETE),
    Ok(SuccessResult::ExportTemplate) => println!("{}", strings::STRING_TEMPLATE_EXPORT_COMPLETE),
    Err(FailResult::Deserialize) => println!("{}", strings::STRING_FAIL_DESERIALIZE),
    Err(FailResult::EmptyPatterns) => println!("{}", strings::STRING_FAIL_EMPTY_PATTERNS),
    Err(FailResult::NoPatterns) => println!("{}", strings::STRING_FAIL_NO_PATTERNS),
    Err(FailResult::ExportMIDI) => println!("{}", strings::STRING_FAIL_EXPORT_MIDI),
    Err(FailResult::ExportTemplate) => println!("{}", strings::STRING_TEMPLATE_EXPORT_FAIL),
    Err(FailResult::LoadSampler) => println!("{}", strings::STRING_FAIL_LOAD_SAMPLER),
    Err(FailResult::TimeReverse(music_time, index, chord)) => {
      println!(
        "{} - [{}] {} @ {}.{}.{}",
        strings::STRING_TIME_REVERSE,
        index,
        chord,
        music_time.get_bar(),
        music_time.get_beat(),
        music_time.get_beat_interval()
      );
    }
    Err(FailResult::TimeSignature(time_signature)) => println!(
      "{} - {}/{}",
      strings::STRING_BAD_TIME_SIGNATURE,
      time_signature.get_numerator(),
      time_signature.get_denominator(),
    ),
    Err(FailResult::UnreachableTime(music_time, index, chord)) => {
      println!(
        "{} - [{}] {} @ {}.{}.{}",
        strings::STRING_UNREACHABLE_EVENT,
        index,
        chord,
        music_time.get_bar(),
        music_time.get_beat(),
        music_time.get_beat_interval()
      );
    }
  }
}

/// Export the patterns from a composition file to midi files.
///
/// # Arguments
/// * `file_path` - The path of the composition parameter yaml file to export.
fn export(file_path: &str) {
  println!("{}{}", strings::STRING_EXPORTING, file_path);
  parse_result(&chord_composer::export_composition_to_midi(file_path));
}

/// Load a composition from a file and then play it.
///
/// # Arguments
/// * `file_path` - The file path of the composition parameter yaml file.
/// * `play_with_metronome` - Playback the composition with a metronome.
fn load_and_play(
  file_path: &str,
  play_with_metronome: bool,
  ticker_bar: bool,
  ticker_beat: bool,
  ticker_interval: bool,
) {
  /// The composition playback performance state.
  struct State {
    ticker_bar: bool,
    ticker_beat: bool,
    ticker_interval: bool,
  }

  impl PerformanceState for State {
    /// Called when the composition is ready for playback.
    ///
    /// # Arguments
    /// * `composition` - The composition to play.
    fn on_ready(&mut self, composition: &Composition) {
      println!("[ {} ]", composition.get_name());
    }

    /// Called when playback has a change in beat interval.
    ///
    /// # Arguments
    /// * `current_time` - The `MusicTime` which the callback has been triggered.
    fn on_beat_interval_change(&mut self, current_time: &MusicTime) {
      if self.ticker_interval {
        println!(
          "| {:02}.{}.{}",
          current_time.get_bar(),
          current_time.get_beat(),
          current_time.get_beat_interval()
        );
      }
    }

    /// Called when playback has a change in beat.
    ///
    /// # Arguments
    /// * `current_time` - The `MusicTime` which the callback has been triggered.
    fn on_beat_change(&mut self, current_time: &MusicTime) {
      if self.ticker_beat {
        println!(
          "| {:02}.{}.{}",
          current_time.get_bar(),
          current_time.get_beat(),
          current_time.get_beat_interval()
        );
      }
    }

    /// Called when playback has a change in bar.
    ///
    /// # Arguments
    /// * `current_time` - The `MusicTime` which the callback has been triggered.
    fn on_bar_change(&mut self, current_time: &MusicTime) {
      if self.ticker_bar {
        println!(
          "| {:02}.{}.{}",
          current_time.get_bar(),
          current_time.get_beat(),
          current_time.get_beat_interval()
        );
      }
    }

    /// Called when playback is triggering an event in the performance.
    ///
    /// # Arguments
    /// * `event` - The `PatternEvent` been triggered at this event.
    fn on_event(&mut self, event: &PatternEvent) {
      let (time, notes) = event;

      let intervals_string = {
        let mut intervals_string = String::new();

        for note in notes {
          let (octave, interval_enum) = notes::midi_to_note(*note);
          let interval_string = notes::key_to_string(interval_enum);
          intervals_string.push_str(&format!("{}{} ", octave, interval_string));
        }
        intervals_string
      };

      println!(
        "| {:02}.{}.{} | {}",
        time.get_bar(),
        time.get_beat(),
        time.get_beat_interval(),
        intervals_string
      );
    }

    /// Called when playback is about the begin to play a pattern.
    ///
    /// # Arguments
    /// * `pattern` - The `Pattern` ready for playback.
    fn on_pattern_playback_begin(&mut self, pattern: &Pattern) {
      println!(
        "<< {} {}/{} @ {}bmp >>",
        pattern.get_name(),
        pattern.get_time_signature().get_numerator(),
        pattern.get_time_signature().get_denominator(),
        pattern.get_bpm()
      );
    }

    /// Called when playback of a pattern has ended.
    ///
    /// # Arguments
    /// * `pattern` - The `Pattern` concluding it's playback.
    fn on_pattern_playback_end(&mut self, _pattern: &Pattern) {}

    /// Called when playback has completed.
    ///
    /// # Arguments
    /// * `composition` - The `Composition` concluding it's playback.
    fn on_completed(&mut self, _composition: &Composition) {}
  }

  // Load and play a composition file
  let mut performance_state = State {
    ticker_bar,
    ticker_beat,
    ticker_interval,
  };

  parse_result(&chord_composer::play(
    file_path,
    &mut performance_state,
    play_with_metronome,
  ));
}

/// Print all the supported internal chords.
fn print_chords() {
  for chord in chord_composer::get_chord_keywords() {
    println!("> {}", chord);
  }
}

/// Export a template composition yaml file.
///
/// # Arguments
/// * `file_path` - The path to export the template to.
fn export_template(file_path: &str) {
  parse_result(&chord_composer::export_template(file_path));
}

fn main() {
  let matches = App::new(strings::STRING_TITLE)
    .version("0.2.6")
    .author("Cj <unsignedbytebite@gmail.com>")
    .about(strings::STRING_ABOUT)
    .subcommand(
      SubCommand::with_name("play")
        .about(strings::STRING_ABOUT_PLAY)
        .arg(
          Arg::with_name("COMPOSITION_FILE")
            .help(strings::STRING_HELP_COMPOSITION_FILE)
            .required(true),
        )
        .arg(
          Arg::with_name("metronome")
            .long("metronome")
            .value_name("metronome")
            .help(strings::STRING_HELP_METRONOME)
            .takes_value(false),
        )
        .arg(
          Arg::with_name("ticker-bar")
            .long("ticker-bar")
            .value_name("ticker-bar")
            .help(strings::STRING_HELP_TICKER_BAR)
            .takes_value(false),
        )
        .arg(
          Arg::with_name("ticker-beat")
            .long("ticker-beat")
            .value_name("ticker-beat")
            .help(strings::STRING_HELP_TICKER_BEAT)
            .takes_value(false),
        )
        .arg(
          Arg::with_name("ticker-interval")
            .long("ticker-interval")
            .value_name("ticker-interval")
            .help(strings::STRING_HELP_TICKER_INTERVAL)
            .takes_value(false),
        ),
    )
    .subcommand(
      SubCommand::with_name("template")
        .about(strings::STRING_ABOUT_TEMPLATE_EXPORT)
        .arg(
          Arg::with_name("EXPORT_PATH")
            .help(strings::STRING_PATH_TEMPLATE_EXPORT)
            .required(true),
        ),
    )
    .subcommand(
      SubCommand::with_name("export")
        .about(strings::STRING_ABOUT_EXPORT)
        .arg(
          Arg::with_name("COMPOSITION_FILE")
            .help(strings::STRING_HELP_COMPOSITION_FILE)
            .required(true),
        ),
    )
    .subcommand(SubCommand::with_name("chords").about(strings::STRING_ABOUT_CHORDS))
    .get_matches();

  if let Some(matches) = matches.subcommand_matches("play") {
    match matches.value_of("COMPOSITION_FILE") {
      Some(file_name) => load_and_play(
        file_name,
        matches.is_present("metronome"),
        matches.is_present("ticker-bar"),
        matches.is_present("ticker-beat"),
        matches.is_present("ticker-interval"),
      ),
      None => println!(
        "{} {}",
        strings::STRING_WARNING_ADDITIONAL,
        strings::STRING_HELP
      ),
    }
  } else if let Some(matches) = matches.subcommand_matches("export") {
    match matches.value_of("COMPOSITION_FILE") {
      Some(file_name) => export(file_name),
      None => println!(
        "COMPOSITION_FILE {} {}",
        strings::STRING_WARNING_NOT_FOUND,
        strings::STRING_HELP
      ),
    }
  } else if let Some(_matches) = matches.subcommand_matches("chords") {
    print_chords();
  } else if let Some(matches) = matches.subcommand_matches("template") {
    match matches.value_of("EXPORT_PATH") {
      Some(file_name) => export_template(file_name),
      None => println!(
        "EXPORT_PATH {} {}",
        strings::STRING_WARNING_NOT_FOUND,
        strings::STRING_HELP
      ),
    }
  } else {
    println!("{}", strings::STRING_HELP)
  }
}
