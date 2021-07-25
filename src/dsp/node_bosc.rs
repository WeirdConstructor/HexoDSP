// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext};
use crate::dsp::helpers::{rand_01, PolyBlepOscillator};

#[macro_export]
macro_rules! fa_bosc_wtype { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "Sin",
            1  => "Tri",
            2  => "Saw",
            3  => "Pulse",
            _  => "?",
        };
    write!($formatter, "{}", s)
} } }

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct BOsc {
    osc: PolyBlepOscillator,
    israte: f32,
}

impl BOsc {
    pub fn new(nid: &NodeId) -> Self {
        let init_phase =
            if nid.instance() > 0 {
                // 0.5 just to protect against sine cancellation
                rand_01() * 0.5
            } else {
                0.0
            };

        Self {
            osc: PolyBlepOscillator::new(init_phase),
            israte: 1.0 / 44100.0,
        }
    }

    pub const freq : &'static str =
        "BOsc freq\nBase frequency of the oscillator.\n\nRange: (-1..1)\n";
    pub const det : &'static str =
        "BOsc det\nDetune the oscillator in semitones and cents. \
         the input of this value is rounded to semitones on coarse input. \
         Fine input lets you detune in cents (rounded). \
         A signal sent to this port is not rounded.\n\
         Note: The signal input allows detune +-10 octaves.\
         \nRange: (Knob -0.2 .. 0.2) / (Signal -1.0 .. 1.0)\n";
    pub const pw : &'static str =
        "BOsc pw\n\nRange: (0..1)\n";
    pub const wtype : &'static str =
        "BOsc wtype\nWaveform type\nAvailable waveforms:\n\
            Sin   - Sine Waveform\n\
            Tri   - Triangle Waveform\n\
            Saw   - Sawtooth Waveform\n\
            Pulse - Pulse Waveform with configurable pulse width";
    pub const sig : &'static str =
        "BOsc sig\nOscillator output\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"Basic Oscillator

A very basic oscillator with a sine, triangle, pulse and sawtooth waveform.
"#;
    pub const HELP : &'static str =
r#"BOsc - Basic Waveform Oscillator

A very basic oscillator with a sine, triangle, pulse and sawtooth waveform.
The pulse width `pw` parameter only has an effect for the `Pulse` waveform.
"#;

}

impl DspNode for BOsc {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) {
        self.israte = 1.0 / srate;
    }

    fn reset(&mut self) {
        self.osc.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, inp, denorm, denorm_offs, at};

        let freq = inp::BOsc::freq(inputs);
        let det  = inp::BOsc::det(inputs);
        let pw   = inp::BOsc::pw(inputs);
        let out  = out::BOsc::sig(outputs);

        let wtype = at::BOsc::wtype(atoms);

        let israte = self.israte;

        match wtype.i() {
            0 => { // sin
                for frame in 0..ctx.nframes() {
                    let freq =
                        denorm_offs::BOsc::freq(
                            freq, det.read(frame), frame);
                    out.write(
                        frame,
                        self.osc.next_sin(freq, israte));
                }
            },
            1 => { // tri
                for frame in 0..ctx.nframes() {
                    let freq =
                        denorm_offs::BOsc::freq(
                            freq, det.read(frame), frame);
                    out.write(
                        frame,
                        self.osc.next_tri(freq, israte));
                }
            },
            2 => { // saw
                for frame in 0..ctx.nframes() {
                    let freq =
                        denorm_offs::BOsc::freq(
                            freq, det.read(frame), frame);
                    out.write(
                        frame,
                        self.osc.next_saw(freq, israte));
                }
            },
            3 | _ => { // pulse
                for frame in 0..ctx.nframes() {
                    let freq =
                        denorm_offs::BOsc::freq(
                            freq, det.read(frame), frame);
                    let pw = denorm::BOsc::pw(pw, frame);
                    out.write(
                        frame,
                        self.osc.next_pulse(freq, israte, pw));
                }
            }
        }

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
    }
}
