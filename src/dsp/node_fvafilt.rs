// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphFun, LedPhaseVals, NodeContext, NodeGlobalRef, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use std::simd::f32x4;
use std::sync::Arc;
use synfx_dsp::fh_va::{FilterParams, LadderFilter, LadderMode, SallenKey, Svf, SvfMode};
use synfx_dsp::{DCFilterX4, PolyIIRHalfbandFilter};

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
macro_rules! fa_fvafilt_lmode {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0  => "LP 6dB",
            1  => "LP 12dB",
            2  => "LP 18dB",
            3  => "LP 24dB",
            4  => "HP 6dB",
            5  => "HP 12dB",
            6  => "HP 18dB",
            7  => "HP 24dB",
            8  => "BP 12dB",
            9  => "BP 24dB",
            10 => "N 12dB",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct FVaFilt {
    params: Arc<FilterParams>,
    old_params: Box<(f32, f32, f32, i8, i8, i8)>,
    ladder: LadderFilter,
    svf: Svf,
    sallenkey: SallenKey,
    oversample: (PolyIIRHalfbandFilter, PolyIIRHalfbandFilter),
    dc_filter: DCFilterX4,
}

impl FVaFilt {
    pub fn new(_nid: &NodeId, _node_global: &NodeGlobalRef) -> Self {
        let params = Arc::new(FilterParams::new());
        Self {
            ladder: LadderFilter::new(params.clone()),
            svf: Svf::new(params.clone()),
            sallenkey: SallenKey::new(params.clone()),
            oversample: (
                PolyIIRHalfbandFilter::new(8, true),
                PolyIIRHalfbandFilter::new(8, true),
            ),
            dc_filter: DCFilterX4::default(),
            params,
            old_params: Box::new((0.0, 0.0, 0.0, 0, 0, -1)),
        }
    }
    pub const in_l: &'static str = "Signal left channel input";
    pub const in_r: &'static str = "Signal right channel input";
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
    pub const lmode: &'static str = "Ladder Slope\n\
        - **LP 6dB** - Low pass 6dB\n\
        - **LP 12dB** - Low pass 12dB\n\
        - **LP 18dB** - Low pass 18dB\n\
        - **LP 24dB** - Low pass 24dB\n\
        - **HP 6dB** - High pass 6dB\n\
        - **HP 12dB** - High pass 12dB\n\
        - **HP 18dB** - High pass 18dB\n\
        - **HP 24dB** - High pass 24dB\n\
        - **BP 12dB** - Band pass 12dB\n\
        - **BP 24dB** - Band pass 24dB\n\
        - **N 12dB** - Notch 12dB\n\
    ";
    pub const sig_l: &'static str = "Filtered signal left channel output.";
    pub const sig_r: &'static str = "Filtered signal right channel output.";
    pub const DESC: &'static str = r#"F's Virtual Analog (Stereo) Filter

This is a collection of virtual analog filters that were implemented
by Fredemus (aka Frederik Halkjær). They behave well when driven hard
but that comes with the price that they are more expensive.
"#;
    pub const HELP: &'static str = r#"Frederik Halkjær Virtual Analog Stereo Filters
"#;

    pub fn graph_fun() -> Option<GraphFun> {
        None
    }
}

macro_rules! on_param_change {
    ($self: ident, $freq: ident, $res: ident, $drive: ident, $ftype: ident, $smode: ident, $lmode: ident, $frame: ident,
     $ladder_mode_changed: ident,
     $on_change: block) => {
        unsafe {
            let params = Arc::get_mut_unchecked(&mut $self.params);
            let new_params = (
                denorm::FVaFilt::freq($freq, $frame).clamp(1.0, 20000.0),
                denorm::FVaFilt::res($res, $frame).clamp(0.0, 1.0),
                denorm::FVaFilt::drive($drive, $frame).max(0.0),
                $ftype,
                $lmode,
                $smode
            );

            if new_params != *$self.old_params {
                #[allow(unused_assignments)]
                if new_params.4 != $self.old_params.4 {
                    $ladder_mode_changed = true;
                }

                params.set_frequency(new_params.0);
                params.set_resonance(new_params.1);
                params.drive = new_params.2;
                params.ladder_mode = match new_params.4 {
                    0 => LadderMode::LP6,
                    1 => LadderMode::LP12,
                    2 => LadderMode::LP18,
                    3 => LadderMode::LP24,
                    4 => LadderMode::HP6,
                    5 => LadderMode::HP12,
                    6 => LadderMode::HP18,
                    7 => LadderMode::HP24,
                    8 => LadderMode::BP12,
                    9 => LadderMode::BP24,
                    _ => LadderMode::N12,
                };
                params.mode = match new_params.5 {
                    0 => SvfMode::LP,
                    1 => SvfMode::HP,
                    2 => SvfMode::BP1,
                    3 => SvfMode::BP2,
                    _ => SvfMode::Notch,
                };

                $on_change;

                *$self.old_params = new_params;
            }
        }
    };
}

impl DspNode for FVaFilt {
    fn set_sample_rate(&mut self, srate: f32) {
        unsafe {
            let params = Arc::get_mut_unchecked(&mut self.params);
            // TODO: Set oversampling dependent on the sample rate, and not pass 2.0*sr here!
            params.set_sample_rate(srate * 2.0);
        }
    }
    fn reset(&mut self) {
        self.ladder.reset();
        self.sallenkey.reset();
        self.svf.reset();
        self.dc_filter.reset();
        self.oversample =
            (PolyIIRHalfbandFilter::new(8, true), PolyIIRHalfbandFilter::new(8, true));
    }

    #[inline]
    fn process(
        &mut self,
        ctx: &mut dyn NodeAudioContext,
        _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{at, denorm, inp, out_idx};

        let in_l = inp::FVaFilt::in_l(inputs);
        let in_r = inp::FVaFilt::in_r(inputs);
        let freq = inp::FVaFilt::freq(inputs);
        let res = inp::FVaFilt::res(inputs);
        let drive = inp::FVaFilt::drive(inputs);
        let ftype = at::FVaFilt::ftype(atoms);
        let smode = at::FVaFilt::smode(atoms);
        let lmode = at::FVaFilt::lmode(atoms);

        let out_i = out_idx::FVaFilt::sig_r();
        let (out_l, out_r) = outputs.split_at_mut(out_i);
        let out_l = &mut out_l[0];
        let out_r = &mut out_r[0];

        let ftype = ftype.i() as i8;
        let smode = smode.i() as i8;
        let lmode = lmode.i() as i8;

        let oversample = &mut self.oversample;
        let mut _old_params = &mut self.old_params;
        let mut ladder_mode_changed = false;

        match ftype {
            2 => {
                // SallenKey
                let sallenkey = &mut self.sallenkey;
                for frame in 0..ctx.nframes() {
                    on_param_change!(self, freq, res, drive, ftype, smode, lmode, frame, ladder_mode_changed, {
                        sallenkey.update();
                    });

                    let sig_l = denorm::FVaFilt::in_l(in_l, frame);
                    let sig_r = denorm::FVaFilt::in_r(in_r, frame);

                    let vframe = f32x4::from_array([sig_l, sig_r, 0.0, 0.0]);
                    let vframe = self.dc_filter.process(vframe);
                    let input = [vframe, f32x4::splat(0.)];
                    let mut output = f32x4::splat(0.);

                    for inp in &input {
                        let vframe = oversample.0.process(f32x4::splat(2.) * inp);
                        let out = sallenkey.process(vframe);
                        output = oversample.1.process(out);
                    }

                    let output = output.as_array();

                    out_l.write(frame, output[0]);
                    out_r.write(frame, output[1]);
                }
            }
            1 => {
                // SVF
                let svf = &mut self.svf;
                for frame in 0..ctx.nframes() {
                    on_param_change!(self, freq, res, drive, ftype, smode, lmode, frame, ladder_mode_changed, {
                        svf.update();
                    });

                    let sig_l = denorm::FVaFilt::in_l(in_l, frame);
                    let sig_r = denorm::FVaFilt::in_r(in_r, frame);

                    let vframe = f32x4::from_array([sig_l, sig_r, 0.0, 0.0]);
                    let vframe = self.dc_filter.process(vframe);
                    let input = [vframe, f32x4::splat(0.)];
                    let mut output = f32x4::splat(0.);

                    for inp in &input {
                        let vframe = oversample.0.process(f32x4::splat(2.) * inp);
                        let out = svf.process(vframe);
                        output = oversample.1.process(out);
                    }

                    let output = output.as_array();

                    out_l.write(frame, output[0]);
                    out_r.write(frame, output[1]);
                }
            }
            _ => {
                // Ladder
                let ladder = &mut self.ladder;
                for frame in 0..ctx.nframes() {
                    on_param_change!(self, freq, res, drive, ftype, smode, lmode, frame, ladder_mode_changed, {
                        if ladder_mode_changed {
                            ladder.set_mix(self.params.ladder_mode);
                        }
                    });

                    let sig_l = denorm::FVaFilt::in_l(in_l, frame);
                    let sig_r = denorm::FVaFilt::in_r(in_r, frame);

                    let vframe = f32x4::from_array([sig_l, sig_r, 0.0, 0.0]);
                    let vframe = self.dc_filter.process(vframe);
                    let input = [vframe, f32x4::splat(0.)];
                    let mut output = f32x4::splat(0.);

                    for inp in &input {
                        let vframe = oversample.0.process(f32x4::splat(2.) * inp);
                        let out = ladder.tick_newton(vframe);
                        output = oversample.1.process(out);
                    }

                    let output = output.as_array();

                    out_l.write(frame, output[0]);
                    out_r.write(frame, output[1]);
                }
            }
        }

        let o_l = out_l.read(ctx.nframes() - 1);
        let o_r = out_r.read(ctx.nframes() - 1);
        if o_l.abs() > o_r.abs() {
            ctx_vals[0].set(o_l);
        } else {
            ctx_vals[0].set(o_r);
        }
    }
}
