// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    denorm, inp, out_idx, DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::SlewValue;


macro_rules! define_ext {
    ($name: ident, $p1: ident, $p2: ident, $p3: ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            slew1: SlewValue<f32>,
            slew2: SlewValue<f32>,
            slew3: SlewValue<f32>,
        }

        impl $name {
            pub fn new(_nid: &NodeId) -> Self {
                Self { slew1: SlewValue::new(), slew2: SlewValue::new(), slew3: SlewValue::new() }
            }

            pub const slew: &'static str = "ExtA-F slew\nSlew limiter for the 3 parameters\nRange: (0..1)";
            pub const atv1: &'static str = "ExtA-F atv1\nAttenuverter for the A1 parameter\nRange: (-1..1)";
            pub const atv2: &'static str = "ExtA-F atv2\nAttenuverter for the A2 parameter\nRange: (-1..1)";
            pub const atv3: &'static str = "ExtA-F atv3\nAttenuverter for the A3 parameter\nRange: (-1..1)";

            pub const sig1: &'static str = "ExtA-F sig1\nA1 output channel\nRange: (-1..1)";
            pub const sig2: &'static str = "ExtA-F sig2\nA2 output channel\nRange: (-1..1)";
            pub const sig3: &'static str = "ExtA-F sig3\nA3 output channel\nRange: (-1..1)";

            pub const DESC: &'static str = "External Parameter Set A-F Input\n\n\
                \
                \
                \
                ";
            pub const HELP: &'static str = r#"External Parameter Set A-F Input
                "#;
        }

        impl DspNode for $name {
            fn outputs() -> usize {
                0
            }

            fn set_sample_rate(&mut self, _srate: f32) {}
            fn reset(&mut self) {}

            #[inline]
            fn process<T: NodeAudioContext>(
                &mut self,
                ctx: &mut T,
                ectx: &mut NodeExecContext,
                _nctx: &NodeContext,
                _atoms: &[SAtom],
                inputs: &[ProcBuf],
                outputs: &mut [ProcBuf],
                ctx_vals: LedPhaseVals,
            ) {
                let slew = inp::$name::slew(inputs);
                let atv1 = inp::$name::atv1(inputs);
                let atv2 = inp::$name::atv2(inputs);
                let atv3 = inp::$name::atv3(inputs);
                let sig2_i = out_idx::$name::sig2();
                let (sig1, r) = outputs.split_at_mut(sig2_i);
                let (sig2, sig3) = r.split_at_mut(1);
                let sig1 = &mut sig1[0];
                let sig2 = &mut sig2[0];
                let sig3 = &mut sig3[0];

                if let Some(params) = &ectx.ext_param {
                    let p1 = params.$p1();
                    let p2 = params.$p2();
                    let p3 = params.$p3();

                    for frame in 0..ctx.nframes() {
                        let slew_ms = denorm::$name::slew(slew, frame);
                        sig1.write(
                            frame,
                            denorm::$name::atv1(atv1, frame) * self.slew1.next(p1, slew_ms),
                        );
                        sig2.write(
                            frame,
                            denorm::$name::atv2(atv2, frame) * self.slew2.next(p2, slew_ms),
                        );
                        sig3.write(
                            frame,
                            denorm::$name::atv3(atv3, frame) * self.slew3.next(p3, slew_ms),
                        );
                    }
                }

                let last_frame = ctx.nframes() - 1;
                ctx_vals[0].set(sig1.read(last_frame));
            }
        }

    }
}

define_ext! {ExtA, a1, a2, a3}
define_ext! {ExtB, b1, b2, b3}
define_ext! {ExtC, c1, c2, c3}
define_ext! {ExtD, d1, d2, d3}
define_ext! {ExtE, e1, e2, e3}
define_ext! {ExtF, f1, f2, f3}
