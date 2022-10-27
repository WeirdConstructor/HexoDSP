// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphFun, LedPhaseVals, NodeContext, NodeGlobalRef, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};

#[macro_export]
macro_rules! fa_smap_clip {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "Off",
            1 => "Clip",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

#[macro_export]
macro_rules! fa_smap_mode {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "Unipolar",
            1 => "Bipolar",
            2 => "UniInv",
            3 => "BiInv",
            _ => "",
        };
        write!($formatter, "{}", s)
    }};
}

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct SMap {}

impl SMap {
    pub fn new(_nid: &NodeId, _node_global: &NodeGlobalRef) -> Self {
        Self {}
    }
    pub const inp: &'static str = "Signal input";
    pub const min: &'static str = "Minimum of the output signal range.";
    pub const max: &'static str = "Maximum of the output signal range.";
    pub const clip: &'static str = "The **Clip** mode allows you to limit the output \
        exactly to the ~~min~~/~~max~~ range. If this is **Off**, the output \
        may be outside the output signal range.";
    pub const mode: &'static str = "This mode defines what kind of input signal is expected \
        and how it will be mapped to the output ~~min~~/~~max~~ range. \
        These modes are available:\n\n\
        - **Unipolar** (**0**..**1**)\n\
        - **Bipolar**  (**-1**..**1**)\n\
        - **UniInv**   (**1**..**0**)\n\
        - **BiInv**    (**1**..**-1**)\n";
    pub const sig: &'static str = "Mapped signal output";
    pub const DESC: &'static str = r#"Simple Range Mapper

This node allows to map an unipolar (**0**..**1**) or bipolar signal (**-1**..**1**) to a defined
~~min~~/~~max~~ signal range.

See also the 'Map' node for a more sophisticated version of this.
"#;
    pub const HELP: &'static str = r#"Simple Range Mapper

This node allows to map an unipolar (**0**..**1**) or bipolar signal (**-1**..**1**)
to a defined ~~min~~/~~max~~ signal range.

The **Clip** mode allows you to limit the output exactly to the ~~min~~/~~max~~
range. If this is **Off**, the output may be outside the output signal
range if the input signal is outside the input signal range.

The ~~input~~ mode allows you to choose between 4 options:

- **Unipolar** (**0**..**1**)
- **Bipolar**  (**-1**..**1**)
- **UniInv**   (**1**..**0**)
- **BiInv**    (**1**..**-1**)

The inverse settings will map **1** to ~~min~~ and **0** to ~~max~~ for **UniInv**.
And **1** to ~~min~~ and **-1** to ~~max~~ for **BiInv**.

For a more sophisticated version of this node see also `Map`.
"#;

    pub fn graph_fun() -> Option<GraphFun> {
        None
    }
}

impl DspNode for SMap {
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

        let inp = inp::SMap::inp(inputs);
        let min = inp::SMap::min(inputs);
        let max = inp::SMap::max(inputs);
        let out = out::SMap::sig(outputs);

        let clip = at::SMap::clip(atoms);
        let mode = at::SMap::mode(atoms);

        let mut last_val = 0.0;

        match (mode.i(), clip.i()) {
            (0, 0) => {
                for frame in 0..ctx.nframes() {
                    let s = inp.read(frame);
                    let min = min.read(frame);
                    let max = max.read(frame);
                    last_val = s;
                    out.write(frame, min + (max - min) * s);
                }
            }
            (0, 1) => {
                for frame in 0..ctx.nframes() {
                    let s = inp.read(frame);
                    let min = min.read(frame);
                    let max = max.read(frame);
                    let s = s.clamp(0.0, 1.0);
                    last_val = s;
                    out.write(frame, min + (max - min) * s);
                }
            }
            (1, 0) => {
                for frame in 0..ctx.nframes() {
                    let s = inp.read(frame);
                    let min = min.read(frame);
                    let max = max.read(frame);
                    let s = (s + 1.0) * 0.5;
                    out.write(frame, min + (max - min) * s);
                }
            }
            (1, 1) => {
                for frame in 0..ctx.nframes() {
                    let s = inp.read(frame);
                    let min = min.read(frame);
                    let max = max.read(frame);
                    let s = ((s + 1.0) * 0.5).clamp(0.0, 1.0);
                    out.write(frame, min + (max - min) * s);
                }
            }
            (2, 0) => {
                for frame in 0..ctx.nframes() {
                    let s = inp.read(frame);
                    let min = min.read(frame);
                    let max = max.read(frame);
                    let s = 1.0 - s;
                    last_val = s;
                    out.write(frame, min + (max - min) * s);
                }
            }
            (2, 1) => {
                for frame in 0..ctx.nframes() {
                    let s = inp.read(frame);
                    let min = min.read(frame);
                    let max = max.read(frame);
                    let s = 1.0 - s.clamp(0.0, 1.0);
                    last_val = s;
                    out.write(frame, min + (max - min) * s);
                }
            }
            (3, 0) => {
                for frame in 0..ctx.nframes() {
                    let s = inp.read(frame);
                    let min = min.read(frame);
                    let max = max.read(frame);
                    let s = 1.0 - ((s + 1.0) * 0.5);
                    out.write(frame, min + (max - min) * s);
                }
            }
            (3, 1) => {
                for frame in 0..ctx.nframes() {
                    let s = inp.read(frame);
                    let min = min.read(frame);
                    let max = max.read(frame);
                    let s = 1.0 - ((s + 1.0) * 0.5).clamp(0.0, 1.0);
                    out.write(frame, min + (max - min) * s);
                }
            }
            _ => {}
        }

        ctx_vals[0].set(last_val);
    }
}
