// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals};
//use crate::dsp::helpers::};

#[macro_export]
macro_rules! fa_noise_mode { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
        let s =
            match ($v.round() as usize) {
                0  => "Bipolar",
                1  => "Unipolar",
                _  => "?",
            };
        write!($formatter, "{}", s)
} } }

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Noise {
}

impl Noise {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
        }
    }

    pub const atv  : &'static str =
        "Noise atv\n...\nRange: (-1..1)";
    pub const offs : &'static str =
        "Noise offs\n...\nRange: (-1..1)";
    pub const mode : &'static str =
        "Noise mode\n...";
    pub const sig  : &'static str =
        "Noise sig\nThe output of the dry/wet mix.\nRange: (-1..1)";

    pub const DESC : &'static str =
r#"A Simple Noise Oscillator

This is a very simple noise oscillator, which can be used for any kind of audio rate noise.
And as a source for sample & hold like nodes to generate low frequency modulation.
"#;
pub const HELP : &'static str =
r#"Noise - A Simple Noise Oscillator

This is a very simple noise oscillator, which can be used for
any kind of audio rate noise. And as a source for sample & hold
like nodes to generate low frequency modulation.

The 'atv' attenuverter and 'offs' parameters control the value range
of the noise, and the 'mode' allows to switch the oscillator between
unipolar and bipolar output.
"#;
}

impl DspNode for Noise {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) {
    }

    fn reset(&mut self) {
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        atoms: &[SAtom], _params: &[ProcBuf], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{at, out, inp, denorm};

//        let buffer  = &mut *self.buffer;
//
//        let mode = at::Delay::mode(atoms);
//        let inp  = inp::Delay::inp(inputs);
//        let trig = inp::Delay::trig(inputs);
//        let time = inp::Delay::time(inputs);
//        let fb   = inp::Delay::fb(inputs);
//        let mix  = inp::Delay::mix(inputs);
//        let out  = out::Delay::sig(outputs);

//        if mode.i() == 0 {
//            for frame in 0..ctx.nframes() {
//                let dry = inp.read(frame);
//
//                let out_sample =
//                    buffer.cubic_interpolate_at(
//                        denorm::Delay::time(time, frame));
//
//                buffer.feed(dry + out_sample * denorm::Delay::fb(fb, frame));
//
//                out.write(frame,
//                    crossfade(dry, out_sample,
//                        denorm::Delay::mix(mix, frame).clamp(0.0, 1.0)));
//            }
//        } else {
//            for frame in 0..ctx.nframes() {
//                let dry = inp.read(frame);
//
//                let clock_samples =
//                    self.clock.next(denorm::Delay::trig(trig, frame));
//                let out_sample = buffer.at(clock_samples as usize);
//
//                buffer.feed(dry + out_sample * denorm::Delay::fb(fb, frame));
//
//                out.write(frame,
//                    crossfade(dry, out_sample,
//                        denorm::Delay::mix(mix, frame).clamp(0.0, 1.0)));
//            }
//        }

//        let last_frame = ctx.nframes() - 1;
//        ctx_vals[0].set(out.read(last_frame));
    }
}
