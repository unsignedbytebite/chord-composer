use rodio::Source; //TODO: replace rodio with something better
use std::{fs, io};

type AudioBuffers = Vec<rodio::source::Buffered<rodio::Decoder<io::BufReader<fs::File>>>>;

pub struct SamplerPlayer {
  device: Option<rodio::Device>,
  clip_buffers: AudioBuffers,
}

impl SamplerPlayer {
  pub fn new(audio_clip_paths: &Vec<String>) -> Result<SamplerPlayer, ()> {
    if cfg!(feature = "no-audio") {
      Ok(SamplerPlayer {
        device: None,
        clip_buffers: Vec::new(),
      })
    } else {
      let mut clip_buffers: AudioBuffers = Vec::new();

      for path in audio_clip_paths {
        clip_buffers.push({
          let buffer = {
            let file = match fs::File::open(path) {
              Ok(file) => file,
              Err(_) => return Err(()),
            };

            io::BufReader::new(file)
          };

          match rodio::Decoder::new(buffer) {
            Ok(decoder) => decoder.buffered(),
            Err(_) => return Err(()),
          }
        });
      }

      let device = match rodio::default_output_device() {
        Some(device) => device,
        _ => return Err(()),
      };

      Ok(SamplerPlayer {
        device: Some(device),
        clip_buffers,
      })
    }
  }

  pub fn play(&self, sample_index: usize) {
    if sample_index >= self.clip_buffers.len() {
      return;
    }

    match &self.device {
      Some(device) => {
        rodio::play_raw(
          device,
          self.clip_buffers[sample_index].clone().convert_samples(),
        );
      }
      _ => {}
    }
  }
}
