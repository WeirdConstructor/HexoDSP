// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext};
use crate::dsp::helpers::{
    process_1pole_lowpass,
    process_1pole_highpass,
    process_1pole_tpt_lowpass,
    process_1pole_tpt_highpass,
};

#[macro_export]
macro_rules! fa_sfilter_type { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "LP(1p)",
            1  => "LP(1pt)",
            2  => "HP(1p)",
            3  => "HP(1pt)",
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
            0 => {
                for frame in 0..ctx.nframes() {
                    let input = inp.read(frame) as f64;
                    let freq = denorm::SFilter::freq(freq, frame) as f64;
                    let freq = freq.clamp(10.0, 22050.0);
                    out.write(frame,
                        process_1pole_lowpass(
                            input, freq, self.israte, &mut self.z)
                        as f32);
                }
            },
            1 => {
                for frame in 0..ctx.nframes() {
                    let input = inp.read(frame) as f64;
                    let freq = denorm::SFilter::freq(freq, frame) as f64;
                    let freq = freq.clamp(10.0, 18000.0);
                    out.write(frame,
                        process_1pole_tpt_lowpass(
                            input, freq, self.israte, &mut self.z)
                        as f32);
                }
            },
            2 => {
                for frame in 0..ctx.nframes() {
                    let input = inp.read(frame) as f64;
                    let freq = denorm::SFilter::freq(freq, frame) as f64;
                    let freq = freq.clamp(10.0, 22050.0);
                    out.write(frame,
                        process_1pole_highpass(
                            input, freq, self.israte, &mut self.z, &mut self.y)
                        as f32);
                }
            },
            3 => {
                for frame in 0..ctx.nframes() {
                    let input = inp.read(frame) as f64;
                    let freq = denorm::SFilter::freq(freq, frame) as f64;
                    let freq = freq.clamp(10.0, 18000.0);
                    out.write(frame,
                        process_1pole_tpt_highpass(
                            input, freq, self.israte, &mut self.z)
                        as f32);
                }
            },
            _ => {},
        }
    }
}
