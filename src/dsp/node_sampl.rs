// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::NodeAudioContext;
use crate::dsp::{SAtom, ProcBuf, DspNode, LedPhaseVals};

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Sampl {
}

impl Sampl {
    pub fn new() -> Self {
        Self {
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
        use crate::dsp::{out}; //, inp, denorm, denorm_v, inp_dir, at};

//        let gain = inp::Amp::gain(inputs);
//        let att  = inp::Amp::att(inputs);
//        let inp  = inp::Amp::inp(inputs);
        let out  = out::Sampl::sig(outputs);

        for frame in 0..ctx.nframes() {
            out.write(frame, 0.0);
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
