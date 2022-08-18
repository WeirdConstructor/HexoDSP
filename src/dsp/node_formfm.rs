// Copyright (c) 2022 Dimas Leenman <skythedragon@outlook.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::{NodeAudioContext, NodeExecContext};

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct FormFM {
    inv_sample_rate: f32,
    phase: f32,
}

impl FormFM {
    pub fn new(_nid: &NodeId) -> Self {
        Self { inv_sample_rate: 1.0 / 44100.0, phase: 0.0 }
    }
    pub const freq: &'static str = "Formant freq\nBase frequency to oscillate at\n";
    pub const det: &'static str = "Formant det\nDetune the oscillator in semitones and cents.\n";
    pub const form: &'static str = "Formant form\nFrequency of the formant\nThis affects how much lower or higher tones the sound has.";
    pub const side: &'static str =
        "Formant side\nWhich side the peak of the wave is. Values more towards 0.0 or 1.0 make the base frequency more pronounced";
    pub const peak: &'static str =
        "Formant peak\nHow high the peak amplitude is. Lower values make the effect more pronounced";
    pub const sig: &'static str = "Formant sig\nGenerated formant signal";
    pub const DESC: &'static str = r#"Formant oscillator

Simple formant oscillator that generates a formant like sound.
Loosely based on the ModFM synthesis method.
"#;
    pub const HELP: &'static str = r#"formfm - Direct formant synthesizer

This is a formant synthesizer that directly generates 
the audio of a single formant.

This can be seen as passing a saw wave with frequency `freq` 
into a bandpass filter with the cutoff at `form`

`freq` controls the base frequency of the formant.
`form` controls the formant frequency. Lower values give more bass to the sound, 
and higher values give the high frequencies more sound.

`side` controls where the peak of the carrier wave is, 
and in turn controls the bandwidth of the effect. The more towards 0.0 or 1.0,
the more the formant is audible.

`peak` controls how high the peak of the carrier wave is.
This also controls the bandwidth of the effect, where lower means a higher 
bandwidth, and thus more audible formant.
"#;
}

impl DspNode for FormFM {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, srate: f32) {
        self.inv_sample_rate = 1.0 / srate;
    }

    fn reset(&mut self) {
        self.phase = 0.0;
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
        _ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{denorm, denorm_offs, inp, out};

        let base_freq = inp::FormFM::freq(inputs);
        let det = inp::FormFM::det(inputs);
        let formant_freq = inp::FormFM::form(inputs);
        let side_val = inp::FormFM::side(inputs);
        let peak_val = inp::FormFM::peak(inputs);
        let out = out::FormFM::sig(outputs);

        for frame in 0..ctx.nframes() {
            // get the inputs
            let base_freq = denorm_offs::FormFM::freq(base_freq, det.read(frame), frame);
            let formant_freq = denorm::FormFM::form(formant_freq, frame);
            let side_val = denorm::FormFM::side(side_val, frame).min(1.0 - 1e-6).max(1e-6);
            let peak_val = denorm::FormFM::peak(peak_val, frame);

            // make a triangle wave, with the peak at carrier center
            let carrier_base = (self.phase / side_val).min((1.0 - self.phase) / (1.0 - side_val));

            // smoothstep
            let carrier = 1.0
                - ((1.0 - peak_val) * (carrier_base * carrier_base * (3.0 - 2.0 * carrier_base)));

            // multiple of the frequency the modulators are at
            let multiple = formant_freq / base_freq.max(1e-6);

            // round them to the closest integer of the formant freq
            let freq_a = multiple.floor();
            let freq_b = freq_a + 1.0;

            // and how much to lerp between them
            let blend = multiple.fract();

            // get the true modulator
            let modulator = (1.0 - blend)
                * if multiple < 1.0 {
                    0.0
                } else {
                    (std::f32::consts::TAU * self.phase * freq_a).cos()
                }
                + blend * (std::f32::consts::TAU * self.phase * freq_b).cos();

            // entire wave
            let wave = carrier * modulator;

            // increment phase (very imporant)
            self.phase += base_freq * self.inv_sample_rate;

            // wrap around
            self.phase = self.phase.fract();

            out.write(frame, wave);
        }
    }
}
