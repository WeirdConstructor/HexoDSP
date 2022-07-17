// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::biquad::Biquad;
use crate::dsp::helpers::{DelayBuffer, FixedOnePole};
use crate::dsp::{
    denorm, denorm_offs, inp, out, DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};

// Bowed String instrument oscillator
// Bowed string model, a la Smith (1986),
// after McIntyre, Schumacher, Woodhouse (1983).
//
// This is a digital waveguide model, making its use possibly subject to
// patents held by Stanford University, Yamaha, and others.
//
// Implementation taken from tubonitaub / alec-deason
// from https://github.com/alec-deason/virtual_modular/blob/4025f1ef343c2eb9cd74eac07b5350c1e7ec9c09/src/simd_graph.rs#L3926
// or
// under MIT License
//
// Which is a reimplementation of this implementation:
// https://github.com/thestk/stk/blob/38970124ecda9d78a74a375426ed5fb9c09840a2/src/Bowed.cpp#L32
// By Perry R. Cook and Gary P. Scavone, 1995--2019.
// Contributions by Esteban Maestre, 2011.
#[derive(Debug, Clone)]
struct BowedString {
    srate: f32,
    nut_to_bow: DelayBuffer<f32>,
    bow_to_bridge: DelayBuffer<f32>,
    string_filter: FixedOnePole,
    body_filters: [Biquad; 6],
}

impl BowedString {
    pub fn new() -> Self {
        let mut s = Self {
            srate: 44100.0,
            nut_to_bow: DelayBuffer::new(),
            bow_to_bridge: DelayBuffer::new(),
            string_filter: FixedOnePole::new(0.0, 0.0),
            body_filters: [
                Biquad::new_with(1.0, 1.5667, 0.3133, -0.5509, -0.3925),
                Biquad::new_with(1.0, -1.9537, 0.9542, -1.6357, 0.8697),
                Biquad::new_with(1.0, -1.6683, 0.8852, -1.7674, 0.8735),
                Biquad::new_with(1.0, -1.8585, 0.9653, -1.8498, 0.9516),
                Biquad::new_with(1.0, -1.9299, 0.9621, -1.9354, 0.9590),
                Biquad::new_with(1.0, -1.9800, 0.9888, -1.9867, 0.9923),
            ],
        };
        s.set_sample_rate(s.srate);
        s
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.srate = sample_rate;
        self.string_filter = FixedOnePole::new(0.75 - (0.2 * (22050.0 / sample_rate)), 0.9);
    }

    pub fn reset(&mut self) {
        self.nut_to_bow.reset();
        self.bow_to_bridge.reset();
        self.string_filter.reset();

        for f in self.body_filters.iter_mut() {
            f.reset();
        }
    }

    #[inline]
    pub fn process(&mut self, freq: f32, bow_velocity: f32, bow_force: f32, pos: f32) -> f32 {
        let total_l = self.srate / freq.max(20.0);
        let total_l = if total_l <= 0.0 { 0.3 } else { total_l };
        let bow_position = ((pos + 1.0) / 2.0).clamp(0.01, 0.99);

        let bow_nut_l = total_l * (1.0 - bow_position);
        let bow_bridge_l = total_l * bow_position;

        let nut = -self.nut_to_bow.cubic_interpolate_at_s(bow_nut_l);
        let brid = self.bow_to_bridge.cubic_interpolate_at_s(bow_bridge_l);
        let bridge = -self.string_filter.process(brid);

        let dv = 0.25 * bow_velocity - (nut + bridge);

        let phat = ((dv + 0.001) * bow_force + 0.75).abs().powf(-4.0).clamp(0.01, 0.98);
        let phat = phat * dv;

        self.bow_to_bridge.feed(nut + phat);
        self.nut_to_bow.feed(bridge + phat);

        let mut output = bridge;
        for f in self.body_filters.iter_mut() {
            output = f.tick(output);
        }

        output
    }
}

/// A bowed string simulation oscillator
#[derive(Debug, Clone)]
pub struct BowStri {
    bstr: Box<BowedString>,
}

impl BowStri {
    pub fn new(_nid: &NodeId) -> Self {
        Self { bstr: Box::new(BowedString::new()) }
    }
    pub const freq: &'static str =
        "BowStri freq\nFrequency of the bowed string oscillator.\n\nRange: (-1..1)\n";
    pub const det: &'static str = "BowStri det\nDetune the oscillator in semitones and cents. \
         the input of this value is rounded to semitones on coarse input. \
         Fine input lets you detune in cents (rounded). \
         A signal sent to this port is not rounded.\n\
         Note: The signal input allows detune +-10 octaves.\
         \nRange: (Knob -0.2 .. 0.2) / (Signal -1.0 .. 1.0)\n";
    pub const vel: &'static str = "BowStri vel\n\n\nRange: (-1..1)\n";
    pub const force: &'static str = "BowStri force\n\n\nRange: (-1..1)\n";
    pub const pos: &'static str = "BowStri pos\n\n\nRange: (-1..1)\n";
    pub const sig: &'static str = "BowStri sig\nOscillator signal output.\n\nRange: (-1..1)\n";

    pub const DESC: &'static str = r#"Bowed String Oscillator

This is an oscillator that simulates a bowed string.
"#;

    pub const HELP: &'static str = r#"BowStri - A Bowed String Simulation Oscillator

This is an oscillator that simulates a bowed string.
It's a bit wonky, so play around with the parameters and see what
works and what doesn't. It plays find in the area from ~55Hz up to
~1760Hz, beyond that it might not produce a sound.

I can recommend to apply an envelope to the 'vel' parameter,
which is basically the bow's velocity.
"#;
}

impl DspNode for BowStri {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, srate: f32) {
        self.bstr.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.bstr.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self,
        ctx: &mut T,
        _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        _atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        let o = out::BowStri::sig(outputs);
        let freq = inp::BowStri::freq(inputs);
        let det = inp::BowStri::det(inputs);
        let vel = inp::BowStri::vel(inputs);
        let force = inp::BowStri::force(inputs);
        let pos = inp::BowStri::pos(inputs);

        let mut last_val = 0.0;
        for frame in 0..ctx.nframes() {
            // The BowStri oscillator is usually off by ~30 cent per octave,
            // that makes it off by 1 semitone at about 1760Hz and off by ~30c
            // at 440 Hz.
            // Calculate some tune correction here based on the
            // normalized value (-0.2 is 110Hz, 0.0 is 440Hz, ...):
            let tune_correction = (freq.read(frame).clamp(-0.2, 1.0) + 0.2) * 10.0 * 0.0012;

            let freq = denorm_offs::BowStri::freq(freq, tune_correction + det.read(frame), frame);

            let out = self.bstr.process(
                freq,
                denorm::BowStri::vel(vel, frame),
                denorm::BowStri::force(force, frame),
                denorm::BowStri::pos(pos, frame),
            );
            last_val = out;
            o.write(frame, out);
        }

        ctx_vals[0].set(last_val);
    }
}
