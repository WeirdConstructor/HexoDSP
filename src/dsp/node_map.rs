// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals};

#[macro_export]
macro_rules! fa_map_k { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    write!($formatter, "{}", $v)
} } }

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Map {
}

impl Map {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
        }
    }
    pub const inp : &'static str =
        "Map inp\nSignal input\nRange: (-1..1)\n";
    pub const atv : &'static str =
        "Map atv\n\nRange: (0..1)\n";
    pub const imin : &'static str =
        "Map imin\n\nRange: (0..1)\n";
    pub const imax : &'static str =
        "Map imax\n\nRange: (0..1)\n";
    pub const omin : &'static str =
        "Map omin\n\nRange: (0..1)\n";
    pub const omax : &'static str =
        "Map omax\n\nRange: (0..1)\n";
    pub const sig : &'static str =
        "Map sig\nMapped signal output\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"Signal Range Mapper

"#;
    pub const HELP : &'static str =
r#"Map - Signal Range Mapper

"#;

}

impl DspNode for Map {
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

//        let gain = inp::Amp::gain(inputs);
//        let att  = inp::Amp::att(inputs);
//        let inp  = inp::Amp::inp(inputs);
//        let out  = out::Amp::sig(outputs);
//        let neg  = at::Amp::neg_att(atoms);
//
//        let last_frame   = ctx.nframes() - 1;
//
//        let last_val =
//            if neg.i() > 0 {
//                for frame in 0..ctx.nframes() {
//                    out.write(frame,
//                        inp.read(frame)
//                        * denorm_v::Amp::att(
//                            inp_dir::Amp::att(att, frame)
//                            .max(0.0))
//                        * denorm::Amp::gain(gain, frame));
//                }
//
//                inp.read(last_frame)
//                * denorm_v::Amp::att(
//                    inp_dir::Amp::att(att, last_frame)
//                    .max(0.0))
//                * denorm::Amp::gain(gain, last_frame)
//
//            } else {
//                for frame in 0..ctx.nframes() {
//                    out.write(frame,
//                        inp.read(frame)
//                        * denorm_v::Amp::att(
//                            inp_dir::Amp::att(att, frame).abs())
//                        * denorm::Amp::gain(gain, frame));
//                }
//
//                inp.read(last_frame)
//                * denorm_v::Amp::att(
//                    inp_dir::Amp::att(att, last_frame).abs())
//                * denorm::Amp::gain(gain, last_frame)
//            };
//
//        ctx_vals[0].set(last_val);
    }
}
