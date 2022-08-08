// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::{
    process_1pole_highpass, process_1pole_lowpass, process_1pole_tpt_highpass,
    process_1pole_tpt_lowpass, process_hal_chamberlin_svf, process_simper_svf,
    process_stilson_moog,
};

#[macro_export]
macro_rules! fa_sfilter_type {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "LP 1p",
            1 => "LP 1pt",
            2 => "HP 1p",
            3 => "HP 1pt",
            4 => "LP 12c",
            5 => "HP 12c",
            6 => "BP 12c",
            7 => "NO 12c",
            8 => "LP 12s",
            9 => "HP 12s",
            10 => "BP 12s",
            11 => "NO 12s",
            12 => "PK 12s",
            13 => "LP 24m",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct SFilter {
    israte: f32,
    z: f32,
    y: f32,
    k: f32,
    h: f32,
    delay: [f32; 4],
    otype: i8,
}

impl SFilter {
    pub fn new(_nid: &NodeId) -> Self {
        Self { israte: 1.0 / 44100.0, z: 0.0, y: 0.0, k: 0.0, h: 0.0, delay: [0.0; 4], otype: -1 }
    }
    pub const inp: &'static str = "SFilter inp\nSignal input\nRange: (-1..1)\n";
    pub const freq: &'static str = "SFilter freq\nFilter cutoff frequency.\nRange: (-1..1)\n";
    pub const res: &'static str = "SFilter res\nFilter resonance.\nRange: (0..1)\n";
    pub const ftype: &'static str = "SFilter ftype\nThe filter type, there are varying types of \
        filters available. Please consult the node documentation for \
        a complete list.\n\
        Types: 1p/1pt=one poles, 12c=Hal Chamberlin SVF,\n\
        12s=Simper SVF, 24m=Moog\n\
        Outputs: LP=Low-,HP=High-,BP=Band-Pass,NO=Notch,PK=Peak";
    pub const sig: &'static str = "SFilter sig\nFiltered signal output.\nRange: (-1..1)\n";
    pub const DESC: &'static str = r#"Simple Filter

This is a collection of more or less simple filters.
There are only two parameters: Filter cutoff 'freq' and the 'res'onance.
"#;
    pub const HELP: &'static str = r#"SFilter - Simple Audio Filter Collection

This is a collection of a few more or less simple filters
of varying types. There are only few parameters for you to change: 'freq'
and 'res'onance. You can switch between the types with the 'ftype'.
There are currently following filters available:

    HP 1p     - One pole low-pass filter (6db)
    HP 1pt    - One pole low-pass filter (6db) (TPT form)
    LP 1p     - One pole high-pass filter (6db)
    LP 1pt    - One pole high-pass filter (6db) (TPT form)

The Hal Chamberlin filters are an older state variable filter design,
that is limited to max cutoff frequency of 16kHz. For a more stable
filter use the "12s" variants.

    LP 12c    - Low-pass Hal Chamberlin state variable filter (12dB)
    HP 12c    - High-pass Hal Chamberlin state variable filter (12dB)
    BP 12c    - Band-pass Hal Chamberlin state variable filter (12dB)
    NO 12c    - Notch Hal Chamberlin state variable filter (12dB)

The (Andrew) Simper state variable filter is a newer design
and stable up to 22kHz at 44.1kHz sampling rate. It's overall more precise
and less quirky than the Hal Chamberlin SVF.

    LP 12s    - Low-pass Simper state variable filter (12dB)
    HP 12s    - High-pass Simper state variable filter (12dB)
    BP 12s    - Band-pass Simper state variable filter (12dB)
    NO 12s    - Notch Simper state variable filter (12dB)
    PK 12s    - Peak Simper state variable filter (12dB)

Next page: more filters (eg. Moog)
---page---
SFilter - Simple Audio Filter Collection

For a more colored filter reach for the Stilson/Moog filter with a 24dB
fall off per octave. Beware high cutoff frequencies for this filter,
as it can become quite unstable.

    LP 24m    - Low-pass Stilson/Moog filter (24dB)

"#;
}

macro_rules! process_filter_fun32 {
    ($nframes: expr, $inp: expr, $out: ident, $freq: ident, $res: ident,
     $input: ident, $minfreq: expr, $maxfreq: expr, $block: block) => {{
        for frame in 0..$nframes {
            let $input = $inp.read(frame);
            let $freq = denorm::SFilter::freq($freq, frame);
            let $freq = $freq.clamp($minfreq, $maxfreq);
            let $res = denorm::SFilter::res($res, frame);
            let $res = $res.clamp(0.0, 0.99);
            let s = $block;
            $out.write(frame, s);
        }
    }};
    ($nframes: expr, $inp: expr, $out: ident, $freq: ident, $res: ident,
     $input: ident, $maxfreq: expr, $block: block) => {{
        for frame in 0..$nframes {
            let $input = $inp.read(frame);
            let $freq = denorm::SFilter::freq($freq, frame);
            let $freq = $freq.clamp(1.0, $maxfreq);
            let $res = denorm::SFilter::res($res, frame);
            let $res = $res.clamp(0.0, 0.99);
            let s = $block;
            $out.write(frame, s);
        }
    }};
    ($nframes: expr, $inp: expr, $out: ident, $freq: ident, $res: ident,
     $maxres: expr, $input: ident, $maxfreq: expr, $block: block) => {{
        for frame in 0..$nframes {
            let $input = $inp.read(frame);
            let $freq = denorm::SFilter::freq($freq, frame);
            let $freq = $freq.clamp(1.0, $maxfreq);
            let $res = denorm::SFilter::res($res, frame);
            let $res = $res.clamp(0.0, $maxres);
            let s = $block;
            $out.write(frame, s);
        }
    }};
    ($nframes: expr, $inp: expr, $out: ident, $freq: ident,
     $input: ident, $maxfreq: expr, $block: block) => {{
        for frame in 0..$nframes {
            let $input = $inp.read(frame);
            let $freq = denorm::SFilter::freq($freq, frame);
            let $freq = $freq.clamp(1.0, $maxfreq);
            let s = $block;
            $out.write(frame, s);
        }
    }};
}

macro_rules! process_filter_fun {
    ($nframes: expr, $inp: expr, $out: ident, $freq: ident, $res: ident,
     $input: ident, $minfreq: expr, $maxfreq: expr, $block: block) => {{
        for frame in 0..$nframes {
            let $input = $inp.read(frame) as f64;
            let $freq = denorm::SFilter::freq($freq, frame) as f64;
            let $freq = $freq.clamp($minfreq, $maxfreq);
            let $res = denorm::SFilter::res($res, frame) as f64;
            let $res = $res.clamp(0.0, 0.99);
            let s = $block;
            $out.write(frame, s as f32);
        }
    }};
    ($nframes: expr, $inp: expr, $out: ident, $freq: ident, $res: ident,
     $input: ident, $maxfreq: expr, $block: block) => {{
        for frame in 0..$nframes {
            let $input = $inp.read(frame) as f64;
            let $freq = denorm::SFilter::freq($freq, frame) as f64;
            let $freq = $freq.clamp(1.0, $maxfreq);
            let $res = denorm::SFilter::res($res, frame) as f64;
            let $res = $res.clamp(0.0, 0.99);
            let s = $block;
            $out.write(frame, s as f32);
        }
    }};
    ($nframes: expr, $inp: expr, $out: ident, $freq: ident, $res: ident,
     $maxres: expr, $input: ident, $maxfreq: expr, $block: block) => {{
        for frame in 0..$nframes {
            let $input = $inp.read(frame) as f64;
            let $freq = denorm::SFilter::freq($freq, frame) as f64;
            let $freq = $freq.clamp(1.0, $maxfreq);
            let $res = denorm::SFilter::res($res, frame) as f64;
            let $res = $res.clamp(0.0, $maxres);
            let s = $block;
            $out.write(frame, s as f32);
        }
    }};
    ($nframes: expr, $inp: expr, $out: ident, $freq: ident,
     $input: ident, $maxfreq: expr, $block: block) => {{
        for frame in 0..$nframes {
            let $input = $inp.read(frame) as f64;
            let $freq = denorm::SFilter::freq($freq, frame) as f64;
            let $freq = $freq.clamp(1.0, $maxfreq);
            let s = $block;
            $out.write(frame, s as f32);
        }
    }};
}

impl DspNode for SFilter {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, srate: f32) {
        self.israte = 1.0 / srate;
    }
    fn reset(&mut self) {
        self.z = 0.0;
        self.y = 0.0;
        self.k = 0.0;
        self.h = 0.0;
        self.delay = [0.0; 4];
        self.otype = -1;
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

        let inp = inp::SFilter::inp(inputs);
        let freq = inp::SFilter::freq(inputs);
        let res = inp::SFilter::res(inputs);
        let ftype = at::SFilter::ftype(atoms);
        let out = out::SFilter::sig(outputs);

        let ftype = ftype.i() as i8;

        if ftype != self.otype {
            self.y = 0.0;
            self.z = 0.0;
            self.k = 0.0;
            self.h = 0.0;
            self.delay = [0.0; 4];
            self.otype = ftype;
        }

        match ftype {
            0 => {
                // Lowpass
                process_filter_fun32!(ctx.nframes(), inp, out, freq, input, 22000.0, {
                    process_1pole_lowpass(input, freq, self.israte, &mut self.z)
                })
            }
            1 => {
                // Lowpass TPT
                process_filter_fun32!(ctx.nframes(), inp, out, freq, input, 22000.0, {
                    process_1pole_tpt_lowpass(input, freq, self.israte, &mut self.z)
                })
            }
            2 => {
                // Highpass
                process_filter_fun32!(ctx.nframes(), inp, out, freq, input, 22000.0, {
                    process_1pole_highpass(input, freq, self.israte, &mut self.z, &mut self.y)
                })
            }
            3 => {
                // Highpass TPT
                process_filter_fun32!(ctx.nframes(), inp, out, freq, input, 22000.0, {
                    process_1pole_tpt_highpass(input, freq, self.israte, &mut self.z)
                })
            }
            4 => {
                // Low Pass Hal Chamberlin SVF
                process_filter_fun32!(ctx.nframes(), inp, out, freq, res, input, 2.0, 16000.0, {
                    let (_high, _notch) = process_hal_chamberlin_svf(
                        input,
                        freq,
                        res,
                        self.israte,
                        &mut self.z,
                        &mut self.y,
                    );
                    self.y
                });
            }
            5 => {
                // High Pass Hal Chamberlin SVF
                process_filter_fun32!(ctx.nframes(), inp, out, freq, res, input, 16000.0, {
                    let (high, _notch) = process_hal_chamberlin_svf(
                        input,
                        freq,
                        res,
                        self.israte,
                        &mut self.z,
                        &mut self.y,
                    );
                    high
                });
            }
            6 => {
                // Band Pass Hal Chamberlin SVF
                process_filter_fun32!(ctx.nframes(), inp, out, freq, res, input, 16000.0, {
                    let (_high, _notch) = process_hal_chamberlin_svf(
                        input,
                        freq,
                        res,
                        self.israte,
                        &mut self.z,
                        &mut self.y,
                    );
                    self.z
                });
            }
            7 => {
                // Notch Hal Chamberlin SVF
                process_filter_fun32!(ctx.nframes(), inp, out, freq, res, input, 16000.0, {
                    let (_high, notch) = process_hal_chamberlin_svf(
                        input,
                        freq,
                        res,
                        self.israte,
                        &mut self.z,
                        &mut self.y,
                    );
                    notch
                });
            }
            8 => {
                // Simper SVF Low Pass
                process_filter_fun32!(ctx.nframes(), inp, out, freq, res, 1.0, input, 22000.0, {
                    let (low, _band, _high) =
                        process_simper_svf(input, freq, res, self.israte, &mut self.k, &mut self.h);
                    low
                });
            }
            9 => {
                // Simper SVF High Pass
                process_filter_fun32!(ctx.nframes(), inp, out, freq, res, 1.0, input, 22000.0, {
                    let (_low, _band, high) =
                        process_simper_svf(input, freq, res, self.israte, &mut self.k, &mut self.h);
                    high
                });
            }
            10 => {
                // Simper SVF Band Pass
                process_filter_fun32!(ctx.nframes(), inp, out, freq, res, 1.0, input, 22000.0, {
                    let (_low, band, _high) =
                        process_simper_svf(input, freq, res, self.israte, &mut self.k, &mut self.h);
                    band
                });
            }
            11 => {
                // Simper SVF Notch
                process_filter_fun32!(ctx.nframes(), inp, out, freq, res, 1.0, input, 22000.0, {
                    let (low, _band, high) =
                        process_simper_svf(input, freq, res, self.israte, &mut self.k, &mut self.h);
                    low + high
                });
            }
            12 => {
                // Simper SVF Peak
                process_filter_fun32!(ctx.nframes(), inp, out, freq, res, 1.0, input, 22000.0, {
                    let (low, _band, high) =
                        process_simper_svf(input, freq, res, self.israte, &mut self.k, &mut self.h);
                    low - high
                });
            }
            13 => {
                // Stilson/Moog Low Pass
                process_filter_fun32!(ctx.nframes(), inp, out, freq, res, 1.0, input, 20000.0, {
                    // Clip here, to prevent blowups, because the
                    // moog filter is quite touchy...
                    let input = input.clamp(-1.0, 1.0);
                    process_stilson_moog(
                        input,
                        freq,
                        res,
                        self.israte,
                        &mut self.z,
                        &mut self.y,
                        &mut self.k,
                        &mut self.h,
                        &mut self.delay,
                    )
                });
            }
            _ => {}
        }

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
    }
}
