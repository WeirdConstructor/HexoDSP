// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext};
use crate::dsp::helpers;

#[macro_export]
macro_rules! fa_comb_mode { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "FedBack",
            1  => "FedForw",
            _  => "?",
        };
    write!($formatter, "{}", s)
} } }


/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Comb {
    comb: Box<helpers::Comb>,
}

impl Comb {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            comb: Box::new(helpers::Comb::new()),
        }
    }

    pub const inp  : &'static str =
        "Comb inp\nThe signal input for the comb filter.\nRange: (-1..1)";
    pub const g : &'static str =
        "Comb g\nThe internal factor for the comb filter. Be careful with high 'g' \
        values (> 0.75) in feedback mode, you will probably have to attenuate \
        the output a bit yourself.\nRange: (-1..1)";
    pub const time : &'static str =
        "Comb time\nThe comb delay time.\nRange: (0..1)";
    pub const sig  : &'static str =
        "Comb sig\nThe output of comb filter.\nRange: (-1..1)";
    pub const mode  : &'static str =
        "Comb mode\nThe mode of the comb filter, whether it's a \
         feedback or feedforward comb filter.";
    pub const DESC : &'static str =
r#"Comb Filter

A very simple comb filter. It has interesting filtering effects
and can be used to build custom reverbs.
"#;
pub const HELP : &'static str =
r#"Comb - A Simple Comb Filter

This is a comb filter that can be used for filtering
as well as for building reverbs or anything you might
find it useful for.

Attention: Be careful with high 'g' values, you might need to
attenuate the output manually for the feedback combfilter mode,
because the feedback adds up quickly.

For typical arrangements in combination with allpass filters,
see the documentation of the 'Comb' node!
"#;
}

impl DspNode for Comb {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) {
        self.comb.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.comb.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, inp, denorm, at};

        let inp  = inp::Comb::inp(inputs);
        let time = inp::Comb::time(inputs);
        let g    = inp::Comb::g(inputs);
        let out  = out::Comb::sig(outputs);
        let mode = at::Comb::mode(atoms);

        let c = &mut *self.comb;

        if mode.i() == 0 {
            for frame in 0..ctx.nframes() {
                let v = inp.read(frame);

                out.write(frame,
                    c.next_feedback(
                        denorm::Comb::time(time, frame),
                        denorm::Comb::g(g, frame),
                        v));
            }
        } else {
            for frame in 0..ctx.nframes() {
                let v = inp.read(frame);

                out.write(frame,
                    c.next_feedforward(
                        denorm::Comb::time(time, frame),
                        denorm::Comb::g(g, frame),
                        v));
            }
        }

        let last_frame = ctx.nframes() - 1;
        ctx_vals[0].set(out.read(last_frame));
    }
}
