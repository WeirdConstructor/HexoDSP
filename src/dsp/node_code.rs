// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::{NodeAudioContext, NodeExecContext};
#[cfg(feature = "synfx-dsp-jit")]
use crate::wblockdsp::CodeEngineBackend;

//use crate::dsp::MAX_BLOCK_SIZE;

/// A WBlockDSP code execution node for JIT'ed DSP code
pub struct Code {
    #[cfg(feature = "synfx-dsp-jit")]
    backend: Option<Box<CodeEngineBackend>>,
    srate: f64,
}

impl std::fmt::Debug for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Code")
    }
}

impl Clone for Code {
    fn clone(&self) -> Self {
        Self::new(&NodeId::Nop)
    }
}

impl Code {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            #[cfg(feature = "synfx-dsp-jit")]
            backend: None,
            srate: 48000.0,
        }
    }

    #[cfg(feature = "synfx-dsp-jit")]
    pub fn set_backend(&mut self, backend: CodeEngineBackend) {
        self.backend = Some(Box::new(backend));
    }

    pub const in1: &'static str = "Code in1\nInput Signal 1\nRange: (-1..1)\n";
    pub const in2: &'static str = "Code in2\nInput Signal 1\nRange: (-1..1)\n";
    pub const alpha: &'static str = "Code alpha\nInput Parameter Alpha\nRange: (-1..1)\n";
    pub const beta: &'static str = "Code alpha\nInput Parameter Alpha\nRange: (-1..1)\n";
    pub const delta: &'static str = "Code alpha\nInput Parameter Alpha\nRange: (-1..1)\n";
    pub const gamma: &'static str = "Code alpha\nInput Parameter Alpha\nRange: (-1..1)\n";
    pub const sig: &'static str = "Code sig\nReturn output\nRange: (-1..1)\n";
    pub const sig1: &'static str = "Code sig1\nSignal channel 1 output\nRange: (-1..1)\n";
    pub const sig2: &'static str = "Code sig2\nSignal channel 2 output\nRange: (-1..1)\n";

    pub const DESC: &'static str = "WBlockDSP Code Execution\n\n\
        This node executes just in time compiled code as fast as machine code. \
        Use this to implement real time DSP code yourself.";
    pub const HELP: &'static str = r#"WBlockDSP Code Execution

Do it!
"#;
}

impl DspNode for Code {
    fn outputs() -> usize {
        3
    }

    fn set_sample_rate(&mut self, srate: f32) {
        self.srate = srate as f64;
        #[cfg(feature = "synfx-dsp-jit")]
        if let Some(backend) = self.backend.as_mut() {
            backend.set_sample_rate(srate);
        }
    }

    fn reset(&mut self) {
        #[cfg(feature = "synfx-dsp-jit")]
        if let Some(backend) = self.backend.as_mut() {
            backend.clear();
        }
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self,
        ctx: &mut T,
        _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        _atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{inp, out_idx};
        let in1 = inp::Code::in1(inputs);
        let in2 = inp::Code::in2(inputs);
        let a = inp::Code::alpha(inputs);
        let b = inp::Code::beta(inputs);
        let d = inp::Code::delta(inputs);
        let g = inp::Code::gamma(inputs);
        let out_i = out_idx::Code::sig1();

        let (sig, sig1) = outputs.split_at_mut(out_i);
        let (sig1, sig2) = sig1.split_at_mut(1);
        let sig = &mut sig[0];
        let sig1 = &mut sig1[0];
        let sig2 = &mut sig2[0];

        #[cfg(feature = "synfx-dsp-jit")]
        {
            let backend = if let Some(backend) = &mut self.backend {
                backend
            } else {
                return;
            };

            backend.process_updates();

            let mut ret = 0.0;
            let mut s1 = 0.0;
            #[allow(unused_assignments)]
            let mut s2 = 0.0;
            for frame in 0..ctx.nframes() {
                (s1, s2, ret) = backend.process(
                    in1.read(frame),
                    in2.read(frame),
                    a.read(frame),
                    b.read(frame),
                    d.read(frame),
                    g.read(frame),
                );
                sig.write(frame, ret);
                sig1.write(frame, s1);
                sig2.write(frame, s2);
            }

            ctx_vals[0].set(ret);
            ctx_vals[1].set(s1);
        }
    }
}
