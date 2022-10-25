// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{DspNode, GraphFun, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::{NodeAudioContext, NodeExecContext};

#[macro_export]
macro_rules! fa_map_clip {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "Off",
            1 => "Clip",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Map {}

impl Map {
    pub fn new(_nid: &NodeId) -> Self {
        Self {}
    }
    pub const inp: &'static str = "Signal input";
    pub const atv: &'static str =
        "Input signal attenuverter, to attenuate or invert the input signal.";
    pub const offs: &'static str = "Input signal offset after ~~atv~~ has been applied.";
    pub const imin: &'static str = "Minimum of the input signal range, \
        it's mapped to the ~~min~~ output signal range.";
    pub const imax: &'static str = "Maximum of the input signal range, \
        it's mapped to the ~~max~~ output signal range.";
    pub const min: &'static str = "Minimum of the output signal range.";
    pub const max: &'static str = "Maximum of the output signal range.";
    pub const clip: &'static str = "The ~~clip~~ mode allows you to limit the output \
        exactly to the ~~min~~/~~max~~ range. If this is off, the output \
        may be outside the output signal range if the input signal is \
        outside the input signal range.";
    pub const sig: &'static str = "Mapped signal output";
    pub const DESC: &'static str = r#"Range Mapper

This node allows to map an input signal range to a precise output signal range.
It's mostly useful to map control signals to modulate inputs.

See also the `SMap` node, which is a simplified version of this node.
"#;
    pub const HELP: &'static str = r#"Range Mapper

This node allows to map an input signal range to a precise output signal
range. It's main use is for precise control of an input of another node.

It processes the input signal as follows. First the input is attenuverted
using the ~~atv~~ parameter and then the ~~offs~~ offset parameter is added:

```text
    inp * atv + offs
```

The resulting signal is then processed by the mapping, that maps
the input signal range ~~imin~~/~~imax~~ to the ouput signal range ~~min~~/~~max~~.

The ~~clip~~ mode allows you to limit the output exactly to the ~~min~~/~~max~~
range. If this is off, the output may be outside the output signal
range if the input signal is outside the input signal range.

This can also be used to invert the signal.

For a more simplified version of this node see also `SMap`.
"#;

    pub fn graph_fun() -> Option<GraphFun> {
        None
    }
}

impl DspNode for Map {
    fn set_sample_rate(&mut self, _srate: f32) {}
    fn reset(&mut self) {}

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
        use crate::dsp::{at, inp, out};

        let inp = inp::Map::inp(inputs);
        let atv = inp::Map::atv(inputs);
        let offs = inp::Map::offs(inputs);
        let imin = inp::Map::imin(inputs);
        let imax = inp::Map::imax(inputs);
        let min = inp::Map::min(inputs);
        let max = inp::Map::max(inputs);
        let out = out::Map::sig(outputs);

        let clip = at::Map::clip(atoms);

        let mut last_val = 0.0;

        if clip.i() == 0 {
            for frame in 0..ctx.nframes() {
                let s = (inp.read(frame) * atv.read(frame)) + offs.read(frame);

                let imin = imin.read(frame);
                let imax = imax.read(frame);
                let min = min.read(frame);
                let max = max.read(frame);

                let x = if (imax - imin).abs() < std::f32::EPSILON {
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
                let s = (inp.read(frame) * atv.read(frame)) + offs.read(frame);

                let imin = imin.read(frame);
                let imax = imax.read(frame);
                let min = min.read(frame);
                let max = max.read(frame);

                let x = if (imax - imin).abs() < std::f32::EPSILON {
                    1.0
                } else {
                    ((s - imin) / (imax - imin)).abs()
                };
                last_val = x;
                let s = min + (max - min) * x;

                out.write(frame, if min < max { s.clamp(min, max) } else { s.clamp(max, min) });
            }
        }

        ctx_vals[0].set(last_val);
    }
}
