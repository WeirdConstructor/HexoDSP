// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

//use super::helpers::{sqrt4_to_pow4, TrigSignal, Trigger};
use crate::dsp::{DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::SCOPE_SAMPLES;
use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::ScopeHandle;
use std::sync::Arc;

#[macro_export]
macro_rules! fa_scope_tsrc  {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "Off",
            1 => "Intern",
            2 => "Extern",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

/// A simple signal scope
#[derive(Debug, Clone)]
pub struct Scope {
    handle: Arc<ScopeHandle>,
    idx: usize,
}

impl Scope {
    pub fn new(_nid: &NodeId) -> Self {
        Self { handle: ScopeHandle::new_shared(), idx: 0 }
    }
    pub const in1: &'static str = "Scope in1\nSignal input 1.\nRange: (-1..1)\n";
    pub const in2: &'static str = "Scope in2\nSignal input 2.\nRange: (-1..1)\n";
    pub const in3: &'static str = "Scope in3\nSignal input 3.\nRange: (-1..1)\n";
    pub const time: &'static str = "Scope time\nDisplayed time range of the oscilloscope view.\nRange: (0..1)\n";
    pub const trig: &'static str = "Scope trig\nExternal trigger input. Only active if 'tsrc' is set to 'Extern'. 'thrsh' applies also for external triggers.\nRange: (-1..1)\n";
    pub const thrsh: &'static str = "Scope thrsh\nTrigger threshold. If the threshold is passed by the signal from low to high the signal recording will be reset. Either for internal or for external triggering. Trigger is only active if 'tsrc' is not 'Off'.\nRange: (-1..1)\n";
    pub const off1: &'static str = "Scope off1\nVisual offset of signal input 1.\nRange: (-1..1)\n";
    pub const off2: &'static str = "Scope off2\nVisual offset of signal input 2.\nRange: (-1..1)\n";
    pub const off3: &'static str = "Scope off3\nVisual offset of signal input 3.\nRange: (-1..1)\n";
    pub const gain1: &'static str = "Scope gain1\nVisual amplification/attenuation of the signal input 1.\nRange: (0..1)\n";
    pub const gain2: &'static str = "Scope gain2\nVisual amplification/attenuation of the signal input 2.\nRange: (0..1)\n";
    pub const gain3: &'static str = "Scope gain3\nVisual amplification/attenuation of the signal input 3.\nRange: (0..1)\n";
    pub const tsrc: &'static str = "Scope tsrc\nTriggering allows you to capture fast signals or pinning fast waveforms into the scope view for better inspection.\nRange: (-1..1)\n";
    pub const DESC: &'static str = r#"Signal Oscilloscope Probe

This is a signal oscilloscope probe node, you can capture up to 3 signals. You can enable internal or external triggering for capturing signals or pinning fast waveforms.
"#;
    pub const HELP: &'static str = r#"Scope - Signal Oscilloscope Probe

You can have up to 8 different scopes in your patch. That means you can
in record up to 24 signals for displaying them in the scope view.
The received signal will be forwarded to the GUI and you can inspect
the waveform there.

You can enable an internal trigger with the 'tsrc'. The 'thrsh' parameter
is the trigger detection parameter. That means, if your signal passes that
trigger from negative to positive, the signal recording will be
reset to that point.

You can also route in an external trigger to capture signals with the 'trig'
input and 'tsrc' set to 'Extern'.

The inputs 'off1', 'off2' and 'off3' define a vertical offset of the signal
waveform in the scope view. Use 'gain1', 'gain2' and 'gain3' for scaling
the input signals up/down.
"#;

    pub fn set_scope_handle(&mut self, handle: Arc<ScopeHandle>) {
        self.handle = handle;
    }
}

impl DspNode for Scope {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, _srate: f32) {}

    fn reset(&mut self) {}

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self,
        ctx: &mut T,
        _ectx: &mut NodeExecContext,
        nctx: &NodeContext,
        _atoms: &[SAtom],
        inputs: &[ProcBuf],
        _outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::inp;

        let in1 = inp::Scope::in1(inputs);
        let in2 = inp::Scope::in2(inputs);
        let in3 = inp::Scope::in3(inputs);
        let inputs = [in1, in2, in3];

        self.handle.set_active_from_mask(nctx.in_connected);

        for frame in 0..ctx.nframes() {
            for (i, input) in inputs.iter().enumerate() {
                let in_val = input.read(frame);
                self.handle.write(i, self.idx, in_val);
            }

            self.idx = (self.idx + 1) % SCOPE_SAMPLES;
        }

        let last_frame = ctx.nframes() - 1;
        ctx_vals[0].set(
            (in1.read(last_frame) + in2.read(last_frame) + in3.read(last_frame)).clamp(-1.0, 1.0),
        );
    }
}
