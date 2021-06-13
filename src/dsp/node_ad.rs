// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals};

#[macro_export]
macro_rules! fa_ad_mult { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "x1",
            1  => "x10",
            2  => "x100",
            _  => "?",
        };
    write!($formatter, "{}", s)
} } }

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Ad {
}

impl Ad {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
        }
    }
    pub const inp : &'static str =
        "Ad inp\nSignal input. If you don't connect this, and set this to 1.0 \
        this will act as envelope signal generator. But you can also just \
        route a signal directly through this of course.\nRange: (-1..1)\n";
    pub const atk : &'static str =
        "Ad atk\nAttack time of the envelope. You can extend the maximum \
        range of this with the 'mult' setting.\nRange: (0..1)\n";
    pub const dcy : &'static str =
        "Ad atk\nDecay time of the envelope. You can extend the maximum \
        range of this with the 'mult' setting.\nRange: (0..1)\n";
    pub const ashp : &'static str =
        "Ad ashp\nAttack shape. This allows you to change the shape \
        of the attack stage from a logarithmic, to a linear and to an \
        exponential shape.\nRange: (0..1)\n";
    pub const dshp : &'static str =
        "Ad dshp\nDecay shape. This allows you to change the shape \
        of the decay stage from a logarithmic, to a linear and to an \
        exponential shape.\nRange: (0..1)\n";
    pub const mult : &'static str =
        "Ad mult\nAttack and Decay time range multiplier. \
        This will extend the maximum range of the 'atk' and 'dcy' parameters.";
    pub const sig : &'static str =
        "Ad sig\nEnvelope signal output. If a signal is sent to the 'inp' port, \
        you will receive an attenuated signal here. If you set 'inp' to a \
        fixed value (for instance 1.0), this will output an envelope signal \
        in the range 0.0 to 'inp' (1.0).\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"Attack-Decay Envelope

This is a simple envelope offering an attack time and decay time with shape parameter.
"#;
    pub const HELP : &'static str =
r#"Ad - Attack-Decay Envelope

"#;

}

impl DspNode for Ad {
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
