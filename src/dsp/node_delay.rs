// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, GraphFun, GraphAtomData, DspNode, LedPhaseVals};

#[macro_export]
macro_rules! fa_dly_s { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
        let s =
            match ($v.round() as usize) {
                0  => "Zero",
                1  => "One",
                2  => "Two",
                3  => "Three",
                4  => "Four",
                5  => "Five",
                6  => "Six",
                7  => "Seven",
                8  => "Eigth",
                9  => "Nine",
                10 => "Ten",
                _  => "?",
            };
        write!($formatter, "{}", s)
} } }

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Delay {
}

impl Delay {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
        }
    }

    pub const inp  : &'static str =
        "Delay inp\nThe signal input for the delay. You can mix in this \
         input to the output with the 'mix' parameter.\nRange: (-1..1)";
    pub const time : &'static str =
        "Delay time\nThe delay time. It can be freely modulated to your \
         likings.\nRange: (0..1)";
    pub const fb   : &'static str =
        "Delay fb\nThe feedback amount of the delay output to it's input. \
        \nRange: (0..1)";
    pub const mix  : &'static str =
        "Delay mix\nThe dry/wet mix of the delay.\nRange: (0..1)";
    pub const sig  : &'static str =
        "Delay sig\nThe output of the dry/wet mix.\nRange: (-1..1)";

    pub const DESC : &'static str =
r#"Simple Delay Line

This is a very simple single buffer delay node.
It provides an internal feedback and dry/wet mix.
"#;
pub const HELP : &'static str =
r#"Delay - A Simple Delay Line

This node provides a very simple delay line with the bare minimum of
parameters. Most importantly a freely modulateable 'time' parameter
and a feedback 'fb' parameter.

Via the 'mix' parameter you can mix in the input signal to the output.

You can use this node to delay any kind of signal, from a simple control
signal to an audio signal.

For other kinds of delay/feedback please see also the 'FbWr'/'FbRd' nodes.
"#;
}

impl DspNode for Delay {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, _srate: f32) { }
    fn reset(&mut self) { }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        atoms: &[SAtom], _params: &[ProcBuf], _inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], _led: LedPhaseVals)
    {
        use crate::dsp::{out, at};

//        let p    = at::Delay::p(atoms);
        let out  = out::Delay::sig(outputs);
        for frame in 0..ctx.nframes() {
            out.write(frame, 0.0);
        }
    }

    fn graph_fun() -> Option<GraphFun> {
        Some(Box::new(|_gd: &dyn GraphAtomData, _init: bool, x: f32| -> f32 {
            x
        }))
    }
}
