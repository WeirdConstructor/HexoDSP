// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom, GraphFun};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::Rng;

#[macro_export]
macro_rules! fa_noise_mode {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "Bipolar",
            1 => "Unipolar",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

/// A simple noise generator
#[derive(Debug, Clone)]
pub struct Noise {
    seed: u64,
    rng: Rng,
}

impl Noise {
    pub fn new(nid: &NodeId) -> Self {
        let mut rng = Rng::new();
        rng.seed((0x193a67f4a8a6d769_u64).wrapping_add(0x131415 * (nid.instance() as u64 + 1)));

        Self { seed: nid.instance() as u64, rng }
    }

    pub const atv: &'static str = "Attenuverter input, to attenuate or invert \
        the noise";
    pub const offs: &'static str = "Offset input, that is added to the output \
        signal after attenuvertig it.";
    pub const mode: &'static str = "You can switch between **Bipolar** noise, which \
         uses the full range from **-1** to **1**, or **Unipolar** noise that \
         only uses the range from **0** to **1**.";
    pub const sig: &'static str = "The noise output.";

    pub const DESC: &'static str = r#"Noise Oscillator

This is a very simple noise oscillator, which can be used for any kind of audio rate noise.
And as a source for sample & hold like nodes to generate low frequency modulation. The white
noise is uniformly distributed and not normal distributed (which could be a bit more natural
in some contexts). See also the `XNoise` node for more noise alternatives.
"#;
    pub const HELP: &'static str = r#"A Simple Noise Oscillator

This is a very simple noise oscillator, which can be used for
any kind of audio rate noise. And as a source for sample & hold
like nodes to generate low frequency modulation.

The noise follows a uniform distribution. That means all amplitudes are equally likely to occur.
While it might sound similar, white noise is usually following a normal distribution, which makes
some amplitudes more likely to occur than others.
See also the `XNoise` node for more noise alternatives.

The ~~atv~~ attenuverter and ~~offs~~ parameters control the value range
of the noise, and the ~~mode~~ allows to switch the oscillator between
unipolar and bipolar output.
"#;

    fn graph_fun() -> Option<GraphFun> { None }
}

impl DspNode for Noise {
    fn set_sample_rate(&mut self, _srate: f32) {}

    fn reset(&mut self) {
        self.rng.seed((0x193a67f4a8a6d769_u64).wrapping_add(0x131415 * (self.seed + 1)));
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

        let mode = at::Noise::mode(atoms);
        let atv = inp::Noise::atv(inputs);
        let offs = inp::Noise::offs(inputs);
        let out = out::Noise::sig(outputs);

        let rng = &mut self.rng;

        if mode.i() == 0 {
            for frame in 0..ctx.nframes() {
                let s = (rng.next() * 2.0) - 1.0;
                let s = s * denorm::Noise::atv(atv, frame) + denorm::Noise::offs(offs, frame);
                out.write(frame, s);
            }
        } else {
            for frame in 0..ctx.nframes() {
                let s =
                    rng.next() * denorm::Noise::atv(atv, frame) + denorm::Noise::offs(offs, frame);
                out.write(frame, s);
            }
        }

        let last_frame = ctx.nframes() - 1;
        ctx_vals[0].set(out.read(last_frame));
    }
}
