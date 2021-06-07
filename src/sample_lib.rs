// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::dsp::SAtom;

use hound;
use std::collections::HashMap;

#[derive(Debug)]
pub enum SampleLoadError {
    LoadError(hound::Error),
    UnsupportedFormat,
}

impl From<hound::Error> for SampleLoadError {
    fn from(err: hound::Error) -> Self {
        SampleLoadError::LoadError(err)
    }
}

/// Loads and stores samples, for use as SAtom parameters for
/// nodes.
pub struct SampleLibrary {
    loaded_samples: HashMap<String, SAtom>,
}

impl SampleLibrary {
    pub fn new() -> Self {
        Self {
            loaded_samples: HashMap::new(),
        }
    }

    /// Synchronous/blocking loading of a sample from `path`.
    /// Returns an SAtom reference that you can clone and send directly
    /// to the sampling node of your choice.
    ///
    /// The maximum length of the sample is `44100 * 10` samples, which
    /// is the equivalent of roughly 1.7 MB.
    ///
    /// Keep in mind that blocking on I/O in the UI might not be desireable.
    pub fn load<'a>(&'a mut self, path: &str) -> Result<&'a SAtom, SampleLoadError> {
        if self.loaded_samples.get(path).is_some() {
            return Ok(self.loaded_samples.get(path).unwrap());
        }

        let mut rd =
            match hound::WavReader::open(path) {
                Err(e) => return Err(SampleLoadError::LoadError(e)),
                Ok(rd) => rd,
            };

        let channels = rd.spec().channels as usize;

        let mut v = vec![rd.spec().sample_rate as f32];

        match rd.spec().sample_format {
            hound::SampleFormat::Float => {
                for s in rd.samples::<f32>().step_by(channels) {
                    v.push(s?);
                }
            },
            hound::SampleFormat::Int => {
                for s in rd.samples::<i16>().step_by(channels) {
                    let s = s?;
                    let s = s as f32 / (i16::MAX as f32);
                    v.push(s);
                }
            },
        };

        let atom = SAtom::audio(path, std::sync::Arc::new(v));

        self.loaded_samples.insert(path.to_string(), atom);
        Ok(self.loaded_samples.get(path).unwrap())
    }
}

impl Default for SampleLibrary {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn save_wav(name: &str, buf: &[f32]) {
        let spec = hound::WavSpec {
            channels:        1,
            sample_rate:     44100,
            bits_per_sample: 16,
            sample_format:   hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(name, spec).unwrap();
        for s in buf.iter() {
            let amp = i16::MAX as f32;
            writer.write_sample((amp * s) as i16).unwrap();
        }
    }

    #[test]
    fn check_sample_lib() {
        let mut sl = SampleLibrary::new();

        save_wav("check_sample_lib_test.wav", &[0.1, -1.0, 1.0, -0.1]);

        let sat = sl.load("check_sample_lib_test.wav").unwrap();

        //d// println!("sa: {:?}", sat);

        if let SAtom::AudioSample((_n, Some(v))) = sat {
            assert_eq!(v[0], 44100.0);
            assert_eq!((v[1] * 1000.0).round() as i32, 100);
            assert_eq!((v[2] * 1000.0).round() as i32, -1000);
            assert_eq!((v[3] * 1000.0).round() as i32, 1000);
            assert_eq!((v[4] * 1000.0).round() as i32, -100);
        } else {
            assert!(false);
        }
    }
}
