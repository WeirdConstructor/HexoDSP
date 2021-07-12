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
    process_hal_chamberlin_svf,
};

#[macro_export]
macro_rules! fa_sfilter_type { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "LP 1p",
            1  => "LP 1pt",
            2  => "HP 1p",
            3  => "HP 1pt",
            4  => "LP 12s",
            5  => "HP 12s",
            6  => "BP 12s",
            7  => "NO 12s",
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
    otype:  i8,
}

impl SFilter {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            israte: 1.0 / 44100.0,
            z:      0.0,
            y:      0.0,
            otype:  -1,
        }
    }
    pub const inp : &'static str =
        "SFilter inp\nSignal input\nRange: (-1..1)\n";
    pub const freq : &'static str =
        "SFilter freq\nFilter cutoff frequency.\nRange: (-1..1)\n";
    pub const res : &'static str =
        "SFilter res\nFilter resonance.\nRange: (0..1)\n";
    pub const ftype : &'static str =
        "SFilter ftype\nThe filter type, there are varying types of \
        filters available. Please consult the node documentation for \
        a complete list.";
    pub const sig : &'static str =
        "SFilter sig\nFiltered signal output.\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"Simple Audio Filter

This is a collection of more or less simple filters.
There are only two parameters: Filter cutoff 'freq' and the 'res'onance.
"#;
    pub const HELP : &'static str =
r#"SFilter - Simple Audio Filter

This is a collection of a few more or less simple filters
of varying types. There are only few parameters for you to change: 'freq'
and 'res'onance. You can switch between the types with the 'ftype'.
There are currently following filters available:

    HP 1p     - One pole low-pass filter (6db)
    HP 1pt    - One pole low-pass filter (6db) (TPT form)
    LP 1p     - One pole high-pass filter (6db)
    LP 1pt    - One pole high-pass filter (6db) (TPT form)
    LP 12s    - Low-pass Hal Chamberlin state variable filter (12dB)
    HP 12s    - High-pass Hal Chamberlin state variable filter (12dB)
    BP 12s    - Band-pass Hal Chamberlin state variable filter (12dB)
    NO 12s    - Notch Hal Chamberlin state variable filter (12dB)
"#;
}

impl DspNode for SFilter {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) {
        self.israte = 1.0 / (srate as f64);
    }
    fn reset(&mut self) {
        self.z     = 0.0;
        self.y     = 0.0;
        self.otype = -1;
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
        let res   = inp::SFilter::res(inputs);
        let ftype = at::SFilter::ftype(atoms);
        let out   = out::SFilter::sig(outputs);

        let ftype = ftype.i() as i8;

        if ftype != self.otype {
            self.y = 0.0;
            self.z = 0.0;
            self.otype = ftype;
        }

        match ftype {
            0 => { // Lowpass
                for frame in 0..ctx.nframes() {
                    let input = inp.read(frame) as f64;
                    let freq = denorm::SFilter::freq(freq, frame) as f64;
                    let freq = freq.clamp(1.0, 22000.0);
                    out.write(frame,
                        process_1pole_lowpass(
                            input, freq, self.israte, &mut self.z)
                        as f32);
                }
            },
            1 => { // Lowpass TPT
                for frame in 0..ctx.nframes() {
                    let input = inp.read(frame) as f64;
                    let freq = denorm::SFilter::freq(freq, frame) as f64;
                    let freq = freq.clamp(1.0, 22000.0);
                    out.write(frame,
                        process_1pole_tpt_lowpass(
                            input, freq, self.israte, &mut self.z)
                        as f32);
                }
            },
            2 => { // Highpass
                for frame in 0..ctx.nframes() {
                    let input = inp.read(frame) as f64;
                    let freq = denorm::SFilter::freq(freq, frame) as f64;
                    let freq = freq.clamp(1.0, 22000.0);
                    out.write(frame,
                        process_1pole_highpass(
                            input, freq, self.israte, &mut self.z, &mut self.y)
                        as f32);
                }
            },
            3 => { // Highpass TPT
                for frame in 0..ctx.nframes() {
                    let input = inp.read(frame) as f64;
                    let freq = denorm::SFilter::freq(freq, frame) as f64;
                    let freq = freq.clamp(1.0, 22000.0);
                    out.write(frame,
                        process_1pole_tpt_highpass(
                            input, freq, self.israte, &mut self.z)
                        as f32);
                }
            },
            4 => { // Low Pass Hal Chamberlin SVF
                for frame in 0..ctx.nframes() {
                    let input = inp.read(frame) as f64;
                    let freq = denorm::SFilter::freq(freq, frame) as f64;
                    let freq = freq.clamp(2.0, 16000.0);
                    let res  = denorm::SFilter::res(res, frame) as f64;
                    let res  = res.clamp(0.0, 0.99);

                    let (_high, _notch) =
                        process_hal_chamberlin_svf(
                            input, freq, res, self.israte,
                            &mut self.z, &mut self.y);

                    out.write(frame, self.y as f32);
                }
            },
            5 => { // High Pass Hal Chamberlin SVF
                for frame in 0..ctx.nframes() {
                    let input = inp.read(frame) as f64;
                    let freq = denorm::SFilter::freq(freq, frame) as f64;
                    let freq = freq.clamp(1.0, 16000.0);
                    let res  = denorm::SFilter::res(res, frame) as f64;
                    let res  = res.clamp(0.0, 0.99);

                    let (high, _notch) =
                        process_hal_chamberlin_svf(
                            input, freq, res, self.israte,
                            &mut self.z, &mut self.y);

                    out.write(frame, high as f32);
                }
            },
            6 => { // Band Pass Hal Chamberlin SVF
                for frame in 0..ctx.nframes() {
                    let input = inp.read(frame) as f64;
                    let freq = denorm::SFilter::freq(freq, frame) as f64;
                    let freq = freq.clamp(1.0, 16000.0);
                    let res  = denorm::SFilter::res(res, frame) as f64;
                    let res  = res.clamp(0.0, 0.99);

                    let (_high, _notch) =
                        process_hal_chamberlin_svf(
                            input, freq, res, self.israte,
                            &mut self.z, &mut self.y);

                    out.write(frame, self.z as f32);
                }
            },
            7 => { // Notch Hal Chamberlin SVF
                for frame in 0..ctx.nframes() {
                    let input = inp.read(frame) as f64;
                    let freq = denorm::SFilter::freq(freq, frame) as f64;
                    let freq = freq.clamp(1.0, 16000.0);
                    let res  = denorm::SFilter::res(res, frame) as f64;
                    let res  = res.clamp(0.0, 0.99);

                    let (_high, notch) =
                        process_hal_chamberlin_svf(
                            input, freq, res, self.israte,
                            &mut self.z, &mut self.y);

                    out.write(frame, notch as f32);
                }
            },
            _ => {},
        }

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
    }
}
