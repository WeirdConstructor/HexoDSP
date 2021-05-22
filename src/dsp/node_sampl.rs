// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::NodeAudioContext;
use crate::dsp::{SAtom, ProcBuf, DspNode, LedPhaseVals};

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Sampl {
    sample_idx:     usize,
}

impl Sampl {
    pub fn new() -> Self {
        Self {
            sample_idx: 0,
        }
    }
    pub const freq : &'static str =
        "Sampl freq\nPitch input for the sampler, giving the playback speed of the\
        sample.\nRange: (-1..1)\n";
    pub const sample : &'static str =
        "Sampl sample\nThe audio sample that is played back.\nRange: (-1..1)\n";
    pub const sig : &'static str =
        "Sampl sig\nSampler audio output\nRange: (-1..1)\n";
}

impl DspNode for Sampl {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, _srate: f32) { }
    fn reset(&mut self) { }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, atoms: &[SAtom], _params: &[ProcBuf],
        inputs: &[ProcBuf], outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, at, inp, denorm}; //, inp, denorm, denorm_v, inp_dir, at};

        let sample = at::Sampl::sample(atoms);
        let freq   = inp::Sampl::freq(inputs);
        let out    = out::Sampl::sig(outputs);

        if let SAtom::AudioSample((_, Some(sample_data))) = sample {
            let sd_len = sample_data.len() - 1;

            for frame in 0..ctx.nframes() {
                let speed = denorm::Sampl::freq(freq, frame) / 440.0;

                let sd = sample_data[self.sample_idx % sd_len + 1];
                out.write(frame, sd);
                self.sample_idx += (1.0 * speed).ceil() as usize;
            }
        } else {
            for frame in 0..ctx.nframes() {
                out.write(frame, 0.0);
            }
        }

        ctx_vals[0].set(1.0);
//        let neg  = at::Amp::neg_att(atoms);
//
//        let last_frame   = ctx.nframes() - 1;
//
//        let last_val =
//            if neg.i() > 0 {
//                for frame in 0..ctx.nframes() {
//                    out.write(frame,
//                        inp.read(frame)
//                        * denorm_v::Amp::att(
//                            inp_dir::Amp::att(att, frame)
//                            .max(0.0))
//                        * denorm::Amp::gain(gain, frame));
//                }
//
//                inp.read(last_frame)
//                * denorm_v::Amp::att(
//                    inp_dir::Amp::att(att, last_frame)
//                    .max(0.0))
//                * denorm::Amp::gain(gain, last_frame)
//
//            } else {
//                for frame in 0..ctx.nframes() {
//                    out.write(frame,
//                        inp.read(frame)
//                        * denorm_v::Amp::att(
//                            inp_dir::Amp::att(att, frame).abs())
//                        * denorm::Amp::gain(gain, frame));
//                }
//
//                inp.read(last_frame)
//                * denorm_v::Amp::att(
//                    inp_dir::Amp::att(att, last_frame).abs())
//                * denorm::Amp::gain(gain, last_frame)
//            };
    }
}
