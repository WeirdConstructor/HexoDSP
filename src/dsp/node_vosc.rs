// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::biquad::Oversampling4x4;
use crate::dsp::{
    NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext,
    GraphAtomData, GraphFun,
};

#[macro_export]
macro_rules! fa_vosc_ovr { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "Off",
            1  => "On",
            _  => "?",
        };
    write!($formatter, "{}", s)
} } }

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct VOsc {
//    osc: PolyBlepOscillator,
    israte: f32,
    phase:  f32,
    oversampling: Box<Oversampling4x4>,
}

impl VOsc {
    pub fn new(nid: &NodeId) -> Self {
        let init_phase = nid.init_phase();

        Self {
            israte: 1.0 / 44100.0,
            phase: init_phase,
            oversampling: Box::new(Oversampling4x4::new()),
        }
    }

    pub const freq : &'static str =
        "VOsc freq\nBase frequency of the oscillator.\n\nRange: (-1..1)\n";
    pub const det : &'static str =
        "VOsc det\nDetune the oscillator in semitones and cents. \
         the input of this value is rounded to semitones on coarse input. \
         Fine input lets you detune in cents (rounded). \
         A signal sent to this port is not rounded.\n\
         Note: The signal input allows detune +-10 octaves.\
         \nRange: (Knob -0.2 .. 0.2) / (Signal -1.0 .. 1.0)\n";
    pub const d : &'static str =
        "VOsc d\n\nRange: (0..1)\n";
    pub const v : &'static str =
        "VOsc v\n\nRange: (0..1)\n";
    pub const vs : &'static str =
        "VOsc vs\nScaling factor for 'v'.\nRange: (0..1)\n";
    pub const ovr : &'static str =
        "VOsc ovr\nEnable/Disable oversampling.";
    pub const wtype : &'static str =
        "VOsc wtype\nWaveform type\nAvailable waveforms:\n\
            Sin   - Sine Waveform\n\
            Tri   - Triangle Waveform\n\
            Saw   - Sawtooth Waveform\n\
            Pulse - Pulse Waveform with configurable pulse width";
    pub const sig : &'static str =
        "VOsc sig\nOscillator output\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"V Oscillator

A vector phase shaping oscillator, to create interesting waveforms and
ways to manipulate them.
"#;
    pub const HELP : &'static str =
r#"VOsc - Vector Phase Shaping Oscillator

A vector phase shaping oscillator, to create interesting waveforms and
ways to manipulate them.
"#;

}

#[inline]
fn s(p: f32) -> f32 {
    -(std::f32::consts::TAU * p).cos()
}

#[inline]
fn phi_vps(x: f32, v: f32, d: f32) -> f32 {
    if x < d {
        (v * x) / d
    } else {
        v + ((1.0 - v) * (x - d))/(1.0 - d)
    }
}

impl DspNode for VOsc {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) {
        self.israte = 1.0 / (srate * 4.0);
        self.oversampling.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.phase = 0.0;
        self.oversampling.reset();
//        self.osc.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, inp, denorm, denorm_offs, at};

        let freq = inp::VOsc::freq(inputs);
        let det  = inp::VOsc::det(inputs);
        let d    = inp::VOsc::d(inputs);
        let v    = inp::VOsc::v(inputs);
        let vs   = inp::VOsc::vs(inputs);
        let out  = out::VOsc::sig(outputs);
        let ovr  = at::VOsc::ovr(atoms);

        let israte = self.israte;

        let oversample = ovr.i() == 1;

        if oversample {
            for frame in 0..ctx.nframes() {
                let freq = denorm_offs::VOsc::freq(freq, det.read(frame), frame);
                let v    = denorm::VOsc::v(v, frame).clamp(0.0, 1.0);
                let d    = denorm::VOsc::d(d, frame).clamp(0.0, 1.0);
                let vs   = denorm::VOsc::vs(vs, frame).clamp(0.0, 20.0);

                let overbuf = self.oversampling.resample_buffer();
                for i in 0..4 {
                    let s = s(phi_vps(self.phase, v + vs, d));

                    overbuf[i] = s;

                    self.phase += freq * israte;
                    self.phase = self.phase.fract();
                }

                out.write(frame, self.oversampling.downsample());
            }
        } else {
            for frame in 0..ctx.nframes() {
                let freq = denorm_offs::VOsc::freq(freq, det.read(frame), frame);
                let v    = denorm::VOsc::v(v, frame).clamp(0.0, 1.0);
                let d    = denorm::VOsc::d(d, frame).clamp(0.0, 1.0);
                let vs   = denorm::VOsc::vs(vs, frame).clamp(0.0, 20.0);

                let s = s(phi_vps(self.phase, v + vs, d));

                out.write(frame, s);

                self.phase += freq * (israte * 4.0);
                self.phase = self.phase.fract();
            }
        }

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
    }

    fn graph_fun() -> Option<GraphFun> {
        let israte = 1.0 / 128.0;

        Some(Box::new(move |gd: &dyn GraphAtomData, _init: bool, x: f32, _xn: f32| -> f32 {
            let v  = NodeId::VOsc(0).inp_param("v").unwrap().inp();
            let vs = NodeId::VOsc(0).inp_param("vs").unwrap().inp();
            let d  = NodeId::VOsc(0).inp_param("d").unwrap().inp();

            let v  = gd.get_denorm(v as u32).clamp(0.0, 1.0);
            let d  = gd.get_denorm(d as u32).clamp(0.0, 1.0);
            let vs = gd.get_denorm(vs as u32).clamp(0.0, 20.0);

            let s = s(phi_vps(x, v + vs, d));
            (s + 1.0) * 0.5
        }))
    }
}
