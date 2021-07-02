// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals};

#[macro_export]
macro_rules! fa_map_clip { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "Off",
            1  => "Clip",
            _  => "?",
        };
    write!($formatter, "{}", s)
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
    pub const offs : &'static str =
        "Map offs\nSignal input offset\nRange: (-1..1)\n";
    pub const imin : &'static str =
        "Map imin\n\nRange: (0..1)\n";
    pub const imax : &'static str =
        "Map imax\n\nRange: (0..1)\n";
    pub const min : &'static str =
        "Map min\n\nRange: (0..1)\n";
    pub const max : &'static str =
        "Map max\n\nRange: (0..1)\n";
    pub const clip : &'static str =
        "Map clip\n";
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

        let inp  = inp::Map::inp(inputs);
        let atv  = inp::Map::atv(inputs);
        let offs = inp::Map::offs(inputs);
        let imin = inp::Map::imin(inputs);
        let imax = inp::Map::imax(inputs);
        let min  = inp::Map::min(inputs);
        let max  = inp::Map::max(inputs);
        let out  = out::Map::sig(outputs);

        let clip = at::Map::clip(atoms);

        let mut last_val = 0.0;

        if clip.i() == 0 {
            for frame in 0..ctx.nframes() {
                let s =
                    (inp.read(frame) * atv.read(frame))
                    + offs.read(frame);

                let imin = imin.read(frame);
                let imax = imax.read(frame);
                let min  = min.read(frame);
                let max  = max.read(frame);

                let x =
                    if (imax - imin).abs() < std::f32::EPSILON {
                        1.0
                    } else {
                        ((s - imin) / (imax - imin)).abs()
                    };
                last_val = x;
                let s = min + (max - min) * x;

                out.write(frame, s);
            }
        } else {
            for frame in 0..ctx.nframes() {
                let s =
                    (inp.read(frame) * atv.read(frame))
                    + offs.read(frame);

                let imin = imin.read(frame);
                let imax = imax.read(frame);
                let min  = min.read(frame);
                let max  = max.read(frame);

                let x =
                    if (imax - imin).abs() < std::f32::EPSILON {
                        1.0
                    } else {
                        ((s - imin) / (imax - imin)).abs()
                    };
                last_val = x;
                let s = min + (max - min) * x;

                out.write(
                    frame,
                    if min < max { s.clamp(min, max) }
                    else         { s.clamp(max, min) });
            }
        }

        ctx_vals[0].set(last_val);
    }
}
