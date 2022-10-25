// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphAtomData, GraphFun, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::{apply_distortion, Oversampling, VPSOscillator};

#[macro_export]
macro_rules! fa_vosc_ovrsmpl {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "Off",
            1 => "On",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

const OVERSAMPLING: usize = 4;

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct VOsc {
    israte: f32,
    osc: VPSOscillator,
    oversampling: Box<Oversampling<OVERSAMPLING>>,
}

impl VOsc {
    pub fn new(nid: &NodeId) -> Self {
        let init_phase = nid.init_phase();

        Self {
            israte: 1.0 / 44100.0,
            osc: VPSOscillator::new(init_phase),
            oversampling: Box::new(Oversampling::new()),
        }
    }

    pub const freq: &'static str = "Base frequency of the oscillator.\n";
    pub const det: &'static str = "Detune the oscillator in semitones and cents. \
         the input of this value is rounded to semitones on coarse input. \
         Fine input lets you detune in cents (rounded). \
         A signal sent to this port is not rounded.\n\
         Note: The signal input allows detune +-10 octaves.\
         ";
    pub const d: &'static str = "This is the horzontal bending point of the waveform. \
        It has a similar effect that pulse width settings have on other \
        oscillators. Make sure to try modulating this parameter at audio rate!\
        ";
    pub const v: &'static str = "This is the vertical bending point of the waveform. \
        You can adjust the effect that ~~d~~ has on the waveform with this \
        parameter. Make sure to try to modulate this parameter at audio rate!\
        ";
    pub const vs: &'static str = "Scaling factor for ~~v~~. If you increase this beyond **1.0**, \
        you will hear formant like sounds from the oscillator. Try adjusting \
        ~~d~~ to move the formants around.";
    pub const dist: &'static str = "A collection of waveshaper/distortions to choose from.";
    pub const damt: &'static str = "Distortion amount.";
    pub const ovrsmpl: &'static str = "Enable/Disable oversampling.";
    pub const sig: &'static str = "Oscillator output";
    pub const DESC: &'static str = r#"V Oscillator

A vector phase shaping oscillator, to create interesting waveforms and ways to manipulate them.
It has two parameters (~~v~~ and ~~d~~) to shape the phase of the sinusoid wave,
and a ~~vs~~ parameter to add extra spice.
Distortion can beef up the oscillator output and you can apply oversampling.
"#;
    pub const HELP: &'static str = r#"Vector Phase Shaping Oscillator

A vector phase shaping oscillator, to create interesting waveforms and
ways to manipulate them. It has two parameters (~~v~~ and ~~d~~) to shape the
phase of the sinusoid wave, and a third parameter ~~vs~~ to add extra spice.
With distortion you can beef up the oscillator output even more and to
make it more harmonic you can apply oversampling.
"#;

    pub fn graph_fun() -> Option<GraphFun> {
        let mut osc = VPSOscillator::new(0.0);
        let israte = 1.0 / 128.0;

        Some(Box::new(move |gd: &dyn GraphAtomData, init: bool, _x: f32, _xn: f32| -> f32 {
            if init {
                osc.reset();
            }

            let v = NodeId::VOsc(0).inp_param("v").unwrap().inp();
            let vs = NodeId::VOsc(0).inp_param("vs").unwrap().inp();
            let d = NodeId::VOsc(0).inp_param("d").unwrap().inp();
            let damt = NodeId::VOsc(0).inp_param("damt").unwrap().inp();
            let dist = NodeId::VOsc(0).inp_param("dist").unwrap().inp();

            let v = gd.get_denorm(v as u32).clamp(0.0, 1.0);
            let d = gd.get_denorm(d as u32).clamp(0.0, 1.0);
            let vs = gd.get_denorm(vs as u32).clamp(0.0, 20.0);
            let damt = gd.get_denorm(damt as u32);
            let dist = gd.get(dist as u32).map(|a| a.i()).unwrap_or(0);

            let v = VPSOscillator::limit_v(d, v + vs);
            let s = osc.next(1.0, israte, d, v);
            let s = apply_distortion(s, damt, dist as u8);

            (s + 1.0) * 0.5
        }))
    }
}

impl DspNode for VOsc {
    fn set_sample_rate(&mut self, srate: f32) {
        self.israte = 1.0 / (srate * (OVERSAMPLING as f32));
        self.oversampling.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.oversampling.reset();
        self.osc.reset();
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
        use crate::dsp::{at, denorm, denorm_offs, inp, out};

        let freq = inp::VOsc::freq(inputs);
        let det = inp::VOsc::det(inputs);
        let d = inp::VOsc::d(inputs);
        let v = inp::VOsc::v(inputs);
        let vs = inp::VOsc::vs(inputs);
        let damt = inp::VOsc::damt(inputs);
        let out = out::VOsc::sig(outputs);
        let ovrsmpl = at::VOsc::ovrsmpl(atoms);
        let dist = at::VOsc::dist(atoms);

        let israte = self.israte;

        let dist = dist.i() as u8;
        let oversample = ovrsmpl.i() == 1;

        let osc = &mut self.osc;

        if oversample {
            for frame in 0..ctx.nframes() {
                let freq = denorm_offs::VOsc::freq(freq, det.read(frame), frame);
                let v = denorm::VOsc::v(v, frame).clamp(0.0, 1.0);
                let d = denorm::VOsc::d(d, frame).clamp(0.0, 1.0);
                let vs = denorm::VOsc::vs(vs, frame).clamp(0.0, 20.0);
                let damt = denorm::VOsc::damt(damt, frame).clamp(0.0, 1.0);

                let v = VPSOscillator::limit_v(d, v + vs);

                let overbuf = self.oversampling.resample_buffer();
                for b in overbuf {
                    let s = osc.next(freq, israte, d, v);
                    *b = apply_distortion(s, damt, dist);
                }

                out.write(frame, self.oversampling.downsample());
            }
        } else {
            for frame in 0..ctx.nframes() {
                let freq = denorm_offs::VOsc::freq(freq, det.read(frame), frame);
                let v = denorm::VOsc::v(v, frame).clamp(0.0, 1.0);
                let d = denorm::VOsc::d(d, frame).clamp(0.0, 1.0);
                let vs = denorm::VOsc::vs(vs, frame).clamp(0.0, 20.0);
                let damt = denorm::VOsc::damt(damt, frame).clamp(0.0, 1.0);

                let v = VPSOscillator::limit_v(d, v + vs);
                let s = osc.next(freq, israte * (OVERSAMPLING as f32), d, v);
                let s = apply_distortion(s, damt, dist);

                out.write(frame, s);
            }
        }

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
    }
}
