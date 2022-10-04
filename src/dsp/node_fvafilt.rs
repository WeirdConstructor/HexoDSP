// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use std::simd::f32x4;
use std::sync::Arc;
use synfx_dsp::fh_va::{FilterParams, LadderFilter, LadderSlope, SallenKey, Svf};
use synfx_dsp::PolyIIRHalfbandFilter;

#[macro_export]
macro_rules! fa_fvafilt_type {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "Ladder",
            1 => "SVF",
            2 => "SallenKey",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

#[macro_export]
macro_rules! fa_fvafilt_svf_mode {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "LP",
            1 => "HP",
            2 => "BP1",
            3 => "BP2",
            4 => "Notch",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

#[macro_export]
macro_rules! fa_fvafilt_lslope {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "Ladder 6dB",
            1 => "Ladder 12dB",
            2 => "Ladder 18dB",
            3 => "Ladder 24dB",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct FVaFilt {
    params: Arc<FilterParams>,
    ladder: Box<LadderFilter>,
    oversample: Box<(PolyIIRHalfbandFilter, PolyIIRHalfbandFilter)>,
}

impl FVaFilt {
    pub fn new(nid: &NodeId) -> Self {
        let params = Arc::new(FilterParams::new());
        Self {
            ladder: Box::new(LadderFilter::new(params.clone())),
            params,
            oversample: Box::new((
                PolyIIRHalfbandFilter::new(8, true),
                PolyIIRHalfbandFilter::new(8, true),
            )),
        }
    }
    pub const inp: &'static str = "Signal input";
    pub const freq: &'static str = "Filter cutoff frequency.";
    pub const res: &'static str = "Filter resonance.";
    pub const drive: &'static str = "Filter (over) drive.";
    pub const ftype: &'static str = "The filter type, there are varying types of \
        filters available:\n\
        - **Ladder**\n\
        - **SVF**\n\
        - **Sallen Key**\n";
    pub const smode: &'static str = "SVF Filter Mode\n\
    - **LP** - Low pass\n\
    - **HP** - High pass\n\
    - **BP1** - Band pass 1\n\
    - **BP2** - Band pass 2\n\
    - **Notch** - Notch\n";
    pub const lslope: &'static str = "Ladder Slope\n\
    Available slopes: **6dB**, **12dB**, **18dB**, **24dB**";
    pub const sig: &'static str = "Filtered signal output.";
    pub const DESC: &'static str = r#"F's Virtual Analog (Stereo) Filter

This is a collection of virtual analog filters that were implemented
by Fredemus (aka Frederik Halkjær). They behave well when driven hard
but that comes with the price that they are more expensive.
"#;
    pub const HELP: &'static str = r#"Frederik Halkjær Virtual Analog Stereo Filters
"#;
}

//macro_rules! process_filter_fun32 {
//    ($nframes: expr, $inp: expr, $out: ident, $freq: ident, $res: ident,
//     $input: ident, $minfreq: expr, $maxfreq: expr, $block: block) => {{
//        for frame in 0..$nframes {
//            let $input = $inp.read(frame);
//            let $freq = denorm::FVaFilt::freq($freq, frame);
//            let $freq = $freq.clamp($minfreq, $maxfreq);
//            let $res = denorm::FVaFilt::res($res, frame);
//            let $res = $res.clamp(0.0, 0.99);
//            let s = $block;
//            $out.write(frame, s);
//        }
//    }};
//    ($nframes: expr, $inp: expr, $out: ident, $freq: ident, $res: ident,
//     $input: ident, $maxfreq: expr, $block: block) => {{
//        for frame in 0..$nframes {
//            let $input = $inp.read(frame);
//            let $freq = denorm::FVaFilt::freq($freq, frame);
//            let $freq = $freq.clamp(1.0, $maxfreq);
//            let $res = denorm::FVaFilt::res($res, frame);
//            let $res = $res.clamp(0.0, 0.99);
//            let s = $block;
//            $out.write(frame, s);
//        }
//    }};
//    ($nframes: expr, $inp: expr, $out: ident, $freq: ident, $res: ident,
//     $maxres: expr, $input: ident, $maxfreq: expr, $block: block) => {{
//        for frame in 0..$nframes {
//            let $input = $inp.read(frame);
//            let $freq = denorm::FVaFilt::freq($freq, frame);
//            let $freq = $freq.clamp(1.0, $maxfreq);
//            let $res = denorm::FVaFilt::res($res, frame);
//            let $res = $res.clamp(0.0, $maxres);
//            let s = $block;
//            $out.write(frame, s);
//        }
//    }};
//    ($nframes: expr, $inp: expr, $out: ident, $freq: ident,
//     $input: ident, $maxfreq: expr, $block: block) => {{
//        for frame in 0..$nframes {
//            let $input = $inp.read(frame);
//            let $freq = denorm::FVaFilt::freq($freq, frame);
//            let $freq = $freq.clamp(1.0, $maxfreq);
//            let s = $block;
//            $out.write(frame, s);
//        }
//    }};
//}
//
//macro_rules! process_filter_fun {
//    ($nframes: expr, $inp: expr, $out: ident, $freq: ident, $res: ident,
//     $input: ident, $minfreq: expr, $maxfreq: expr, $block: block) => {{
//        for frame in 0..$nframes {
//            let $input = $inp.read(frame) as f64;
//            let $freq = denorm::FVaFilt::freq($freq, frame) as f64;
//            let $freq = $freq.clamp($minfreq, $maxfreq);
//            let $res = denorm::FVaFilt::res($res, frame) as f64;
//            let $res = $res.clamp(0.0, 0.99);
//            let s = $block;
//            $out.write(frame, s as f32);
//        }
//    }};
//    ($nframes: expr, $inp: expr, $out: ident, $freq: ident, $res: ident,
//     $input: ident, $maxfreq: expr, $block: block) => {{
//        for frame in 0..$nframes {
//            let $input = $inp.read(frame) as f64;
//            let $freq = denorm::FVaFilt::freq($freq, frame) as f64;
//            let $freq = $freq.clamp(1.0, $maxfreq);
//            let $res = denorm::FVaFilt::res($res, frame) as f64;
//            let $res = $res.clamp(0.0, 0.99);
//            let s = $block;
//            $out.write(frame, s as f32);
//        }
//    }};
//    ($nframes: expr, $inp: expr, $out: ident, $freq: ident, $res: ident,
//     $maxres: expr, $input: ident, $maxfreq: expr, $block: block) => {{
//        for frame in 0..$nframes {
//            let $input = $inp.read(frame) as f64;
//            let $freq = denorm::FVaFilt::freq($freq, frame) as f64;
//            let $freq = $freq.clamp(1.0, $maxfreq);
//            let $res = denorm::FVaFilt::res($res, frame) as f64;
//            let $res = $res.clamp(0.0, $maxres);
//            let s = $block;
//            $out.write(frame, s as f32);
//        }
//    }};
//    ($nframes: expr, $inp: expr, $out: ident, $freq: ident,
//     $input: ident, $maxfreq: expr, $block: block) => {{
//        for frame in 0..$nframes {
//            let $input = $inp.read(frame) as f64;
//            let $freq = denorm::FVaFilt::freq($freq, frame) as f64;
//            let $freq = $freq.clamp(1.0, $maxfreq);
//            let s = $block;
//            $out.write(frame, s as f32);
//        }
//    }};
//}

impl DspNode for FVaFilt {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, srate: f32) {
        unsafe {
            let mut params = Arc::get_mut_unchecked(&mut self.params);
            // TODO: Set oversampling dependent on the sample rate, and not pass 2.0*sr here!
            params.set_sample_rate(srate * 2.0);
        }
    }
    fn reset(&mut self) {
        self.ladder.reset();
        self.oversample =
            Box::new((PolyIIRHalfbandFilter::new(8, true), PolyIIRHalfbandFilter::new(8, true)));
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self,
        ctx: &mut T,
        _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{at, denorm, inp, out};

        let inp = inp::FVaFilt::inp(inputs);
        let freq = inp::FVaFilt::freq(inputs);
        let res = inp::FVaFilt::res(inputs);
        let drive = inp::FVaFilt::drive(inputs);
        let ftype = at::FVaFilt::ftype(atoms);
        let smode = at::FVaFilt::smode(atoms);
        let lslope = at::FVaFilt::lslope(atoms);
        let out = out::FVaFilt::sig(outputs);

        let ftype = ftype.i() as i8;
        let smode = smode.i() as i8;
        let lslope = lslope.i() as i8;

        unsafe {
            let params = Arc::get_mut_unchecked(&mut self.params);
            params.set_frequency(denorm::FVaFilt::freq(freq, 0).clamp(1.0, 20000.0));
            params.set_resonance(denorm::FVaFilt::res(res, 0).clamp(0.0, 1.0));
            params.drive = denorm::FVaFilt::drive(drive, 0).max(0.0);
            //d// println!("DRIVE={}", params.drive);
            params.slope = match lslope {
                0 => LadderSlope::LP6,
                1 => LadderSlope::LP12,
                2 => LadderSlope::LP18,
                _ => LadderSlope::LP24,
            };
        };

        for frame in 0..ctx.nframes() {
            let sig_l = denorm::FVaFilt::inp(inp, frame);

            // TODO: Read in second channel!
            let vframe = f32x4::from_array([sig_l, 0.0, 0.0, 0.0]);
            let input = [vframe, f32x4::splat(0.)];
            let mut output = f32x4::splat(0.);

            for i in 0..2 {
                let vframe = self.oversample.0.process(f32x4::splat(2.) * input[i]);
                let out = self.ladder.tick_newton(vframe);
                output = self.oversample.1.process(out);
            }

            let output = output.as_array();

            // TODO: Add output[1] to second output!
            out.write(frame, output[0]);
        }

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
    }
}
