// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals};

#[macro_export]
macro_rules! fa_amp_neg_att { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "Allow",
            1  => "Clip",
            _  => "?",
        };
    write!($formatter, "{}", s)
} } }

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Amp {
}

impl Amp {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
        }
    }
    pub const inp : &'static str =
        "Amp inp\nSignal input\nRange: (-1..1)\n";
    pub const att : &'static str =
        "Amp att\nAttenuate input. Does only attenuate the signal, not amplify it.\n\
         Use this for envelope input.\nRange: (0..1)\n";
    pub const gain : &'static str =
        "Amp gain\nGain input. This control can actually amplify the signal.\nRange: (0..1)\n";
    pub const neg_att : &'static str =
        "Amp neg\nIf this is set to 'Clip', only positive inputs to 'att' are used.\nRange: (0..1)\n";
    pub const sig : &'static str =
        "Amp sig\nAmplified signal output\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"Signal Amplifier

This is a simple amplifier to amplify or attenuate a signal.
See also nodes like 'atv' for an Attenuverter or 'mix' for mixing signals.
"#;
    pub const HELP : &'static str =
r#"Amp - Signal Amplifier

It serves the simple purpose of taking an input signal and attenuate (either
with the 'att' or the 'gain' parameter) or just amplifying it with
the 'gain' parameter.

You can even use it as simple fixed control signal source if you leave the
'inp' port unconnected and just dial in the desired output value with the
parameter.

The main idea with the 'gain' and 'att' parameters is, that you can set
the desired amplification with the 'gain' parameter and automate it using
the 'att' parameter. The 'neg' setting then defines what happens with
negative inputs on the 'att' port.
"#;

}

impl DspNode for Amp {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, _srate: f32) { }
    fn reset(&mut self) { }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        atoms: &[SAtom], _params: &[ProcBuf], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, inp, denorm, denorm_v, inp_dir, at};

        let gain = inp::Amp::gain(inputs);
        let att  = inp::Amp::att(inputs);
        let inp  = inp::Amp::inp(inputs);
        let out  = out::Amp::sig(outputs);
        let neg  = at::Amp::neg_att(atoms);

        let last_frame   = ctx.nframes() - 1;

        let last_val =
            if neg.i() > 0 {
                for frame in 0..ctx.nframes() {
                    out.write(frame,
                        inp.read(frame)
                        * denorm_v::Amp::att(
                            inp_dir::Amp::att(att, frame)
                            .max(0.0))
                        * denorm::Amp::gain(gain, frame));
                }

                inp.read(last_frame)
                * denorm_v::Amp::att(
                    inp_dir::Amp::att(att, last_frame)
                    .max(0.0))
                * denorm::Amp::gain(gain, last_frame)

            } else {
                for frame in 0..ctx.nframes() {
                    out.write(frame,
                        inp.read(frame)
                        * denorm_v::Amp::att(
                            inp_dir::Amp::att(att, frame).abs())
                        * denorm::Amp::gain(gain, frame));
                }

                inp.read(last_frame)
                * denorm_v::Amp::att(
                    inp_dir::Amp::att(att, last_frame).abs())
                * denorm::Amp::gain(gain, last_frame)
            };

        ctx_vals[0].set(last_val);
    }
}
