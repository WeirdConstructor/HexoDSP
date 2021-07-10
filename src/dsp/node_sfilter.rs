// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext};
use crate::dsp::helpers::{process_1pole_lowpass, process_1pole_tpt_lowpass};

#[macro_export]
macro_rules! fa_sfilter_type { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "LP(1p)",
            1  => "LP(1p,TPT)",
            2  => "HP(1p)",
            3  => "HP(1p,TPT)",
            _  => "?",
        };
    write!($formatter, "{}", s)
} } }

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct SFilter {
    israte: f64,
    z:      f64,
    y:      f64,
}

impl SFilter {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            israte: 1.0 / 44100.0,
            z:      0.0,
            y:      0.0,
        }
    }
    pub const inp : &'static str =
        "SFilter inp\nSignal input\nRange: (-1..1)\n";
    pub const freq : &'static str =
        "SFilter freq\nFilter cutoff frequency.\nRange: (-1..1)\n";
    pub const ftype : &'static str =
        "SFilter ftype\nFilter type.";
    pub const sig : &'static str =
        "SFilter sig\nFiltered signal output.\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"Simple Audio Filter

This is a very simple collection of filters.
"#;
    pub const HELP : &'static str =
r#"SFilter - Simple Audio Filter

"#;
}

impl DspNode for SFilter {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) {
        self.israte = 1.0 / (srate as f64);
    }
    fn reset(&mut self) {
        self.z = 0.0;
        self.y = 0.0;
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, inp, denorm, at};

        let inp   = inp::SFilter::inp(inputs);
        let freq  = inp::SFilter::freq(inputs);
        let ftype = at::SFilter::ftype(atoms);
        let out   = out::SFilter::sig(outputs);

        match ftype.i() {
            // one pole lp from valley rack free:
            // https://github.com/ValleyAudio/ValleyRackFree/blob/v1.0/src/Common/DSP/OnePoleFilters.cpp
            0 => {
                for frame in 0..ctx.nframes() {
                    let input = inp.read(frame) as f64;
                    let freq = (denorm::SFilter::freq(freq, frame) as f64);
                    out.write(frame,
                        process_1pole_lowpass(input, freq, self.israte, &mut self.z)
                        as f32);
                }
            },
            // one pole from:
            // http://www.willpirkle.com/Downloads/AN-4VirtualAnalogFilters.pdf
            // (page 5)
            1 => {
                for frame in 0..ctx.nframes() {
                    let input = inp.read(frame) as f64;
                    let freq = (denorm::SFilter::freq(freq, frame) as f64);
                    out.write(frame,
                        process_1pole_tpt_lowpass(input, freq, self.israte, &mut self.z)
                        as f32);
                }
            },
//            // From https://en.wikipedia.org/wiki/RC_circuit
//            // has the same output as the SVF variant, takes the same amount of time.
//            3 => {
//                for frame in 0..ctx.nframes() {
//                    let input = inp.read(frame) as f64;
//                    let c =
//                        2.0
//                        / (std::f64::consts::TAU
//                         * (denorm::SFilter::freq(freq, frame) as f64)
//                         * self.israte);
//
//                    let y = (input + self.z - self.y * (1.0 - c)) / (1.0 + c);
//                    self.z = input;
//                    self.y = y;
//
//                    // highpass: self.z - self.y
//                    out.write(frame, y as f32);
//                }
//            },
            _ => {},
        }
    }
}
