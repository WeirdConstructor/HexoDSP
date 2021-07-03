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
        "Map atv\nInput signal attenuverter, to attenuate or invert the input signal.\nRange: (0..1)\n";
    pub const offs : &'static str =
        "Map offs\nInput signal offset after 'atv' has been applied.\nRange: (-1..1)\n";
    pub const imin : &'static str =
        "Map imin\nMinimum of the input signal range, \
        it's mapped to the 'min' output signal range.\nRange: (0..1)\n";
    pub const imax : &'static str =
        "Map imax\nMaximum of the input signal range, \
        it's mapped to the 'max' output signal range.\nRange: (0..1)\n";
    pub const min : &'static str =
        "Map min\nMinimum of the output signal range.\nRange: (0..1)\n";
    pub const max : &'static str =
        "Map max\nMaximum of the output signal range.\nRange: (0..1)\n";
    pub const clip : &'static str =
        "Map clip\nThe 'clip' mode allows you to limit the output \
        exactly to the 'min'/'max' range. If this is off, the output \
        may be outside the output signal range if the input signal is \
        outside the input signal range.";
    pub const sig : &'static str =
        "Map sig\nMapped signal output\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"Signal Range Mapper

This node allows to map an input signal range to a precise output signal range.
It's mostly useful to map control signals to modulate inputs.

See also the 'SMap' node, which is a simplified version of this node.
"#;
    pub const HELP : &'static str =
r#"Map - Signal Range Mapper

This node allows to map an input signal range to a precise output signal
range. It's main use is for precise control of an input of another node.

It processes the input signal as follows. First the input is attenuverted
using the 'atv' paramter and then the 'offs' offset parameter is added:

    inp * atv + offs

The resulting signal is then processed by the mapping, that maps
the input signal range 'imin'/'imax' to the ouput signal range 'min/'max'.

The 'clip' mode allows you to limit the output exactly to the 'min'/'max'
range. If this is off, the output may be outside the output signal
range if the input signal is outside the input signal range.

This can also be used to invert the signal.

For a more simplified version of this node see also 'SMap'.
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
