// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphFun, LedPhaseVals, NodeContext, NodeGlobalRef, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use std::simd::f32x4;
use std::sync::Arc;
use synfx_dsp::fh_va::{FilterParams, LadderFilter, LadderSlope, SallenKey, Svf};
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

#[derive(Debug, Clone)]
struct VaFiltState {
    ladder: LadderFilter,
    svf: Svf,
    sallenkey: SallenKey,
    oversample: (PolyIIRHalfbandFilter, PolyIIRHalfbandFilter),
    dc_filter: DCFilterX4,
}

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct FVaFilt {
    params: Arc<FilterParams>,
    old_params: Box<(f32, f32, f32, i8, i8)>,
    state: Box<VaFiltState>,
    //    ladder: Box<LadderFilter>,
    //    svf: Box<Svf>,
    //    sallenkey: Box<SallenKey>,
    //    oversample: Box<(PolyIIRHalfbandFilter, PolyIIRHalfbandFilter)>,
}

impl FVaFilt {
    pub fn new(_nid: &NodeId, _node_global: &NodeGlobalRef) -> Self {
        let params = Arc::new(FilterParams::new());
        Self {
            state: Box::new(VaFiltState {
                ladder: LadderFilter::new(params.clone()),
                svf: Svf::new(params.clone()),
                sallenkey: SallenKey::new(params.clone()),
                oversample: (
                    PolyIIRHalfbandFilter::new(8, true),
                    PolyIIRHalfbandFilter::new(8, true),
                ),
                dc_filter: DCFilterX4::default(),
            }),
            params,
            old_params: Box::new((0.0, 0.0, 0.0, 0, -1)),
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

    pub fn graph_fun() -> Option<GraphFun> {
        None
    }
}

macro_rules! on_param_change {
    ($self: ident, $freq: ident, $res: ident, $drive: ident, $ftype: ident, $lslope: ident, $frame: ident,
     $on_change: block) => {
        unsafe {
            let params = Arc::get_mut_unchecked(&mut $self.params);
            let new_params = (
                denorm::FVaFilt::freq($freq, $frame).clamp(1.0, 20000.0),
                denorm::FVaFilt::res($res, $frame).clamp(0.0, 1.0),
                denorm::FVaFilt::drive($drive, $frame).max(0.0),
                $ftype,
                $lslope,
            );

            if new_params != *$self.old_params {
                params.set_frequency(new_params.0);
                params.set_resonance(new_params.1);
                params.drive = new_params.2;
                params.slope = match new_params.4 {
                    0 => LadderSlope::LP6,
                    1 => LadderSlope::LP12,
                    2 => LadderSlope::LP18,
                    _ => LadderSlope::LP24,
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
        self.state.ladder.reset();
        self.state.sallenkey.reset();
        self.state.svf.reset();
        self.state.dc_filter.reset();
        self.state.oversample =
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
        let _smode = smode.i() as i8;
        let lslope = lslope.i() as i8;

        let state = self.state.as_mut();

        let oversample = &mut state.oversample;
        let mut _old_params = &mut self.old_params;

        match ftype {
            2 => {
                // SallenKey
                let sallenkey = &mut state.sallenkey;
                for frame in 0..ctx.nframes() {
                    on_param_change!(self, freq, res, drive, ftype, lslope, frame, {
                        sallenkey.update();
                    });

                    let sig_l = denorm::FVaFilt::inp(inp, frame);

                    // TODO: Read in second channel!
                    let vframe = f32x4::from_array([sig_l, 0.0, 0.0, 0.0]);
                    let vframe = state.dc_filter.process(vframe);
                    let input = [vframe, f32x4::splat(0.)];
                    let mut output = f32x4::splat(0.);

                    for inp in &input {
                        let vframe = oversample.0.process(f32x4::splat(2.) * inp);
                        let out = sallenkey.process(vframe);
                        output = oversample.1.process(out);
                    }

                    let output = output.as_array();

                    // TODO: Add output[1] to second output!
                    out.write(frame, output[0]);
                }
            }
            1 => {
                // SVF
                let svf = &mut state.svf;
                for frame in 0..ctx.nframes() {
                    on_param_change!(self, freq, res, drive, ftype, lslope, frame, {
                        svf.update();
                    });

                    let sig_l = denorm::FVaFilt::inp(inp, frame);

                    // TODO: Read in second channel!
                    let vframe = f32x4::from_array([sig_l, 0.0, 0.0, 0.0]);
                    let vframe = state.dc_filter.process(vframe);
                    let input = [vframe, f32x4::splat(0.)];
                    let mut output = f32x4::splat(0.);

                    for inp in &input {
                        let vframe = oversample.0.process(f32x4::splat(2.) * inp);
                        let out = svf.process(vframe);
                        output = oversample.1.process(out);
                    }

                    let output = output.as_array();

                    // TODO: Add output[1] to second output!
                    out.write(frame, output[0]);
                }
            }
            _ => {
                // Ladder
                let ladder = &mut state.ladder;
                for frame in 0..ctx.nframes() {
                    on_param_change!(self, freq, res, drive, ftype, lslope, frame, {});
                    let sig_l = denorm::FVaFilt::inp(inp, frame);

                    // TODO: Read in second channel!
                    let vframe = f32x4::from_array([sig_l, 0.0, 0.0, 0.0]);
                    let vframe = state.dc_filter.process(vframe);
                    let input = [vframe, f32x4::splat(0.)];
                    let mut output = f32x4::splat(0.);

                    for inp in &input {
                        let vframe = oversample.0.process(f32x4::splat(2.) * inp);
                        let out = ladder.tick_newton(vframe);
                        output = oversample.1.process(out);
                    }

                    let output = output.as_array();

                    // TODO: Add output[1] to second output!
                    out.write(frame, output[0]);
                }
            }
        }

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
    }
}
