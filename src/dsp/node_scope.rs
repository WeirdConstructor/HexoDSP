// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.
//
// This code was inspired by VCV Rack's scope:
// https://github.com/VCVRack/Fundamental/blob/v2/src/Scope.cpp
// Which is/was under the license GPL-3.0-or-later.
// Copyright by Andrew Belt, 2021

//use super::helpers::{sqrt4_to_pow4, TrigSignal, Trigger};
use crate::dsp::helpers::CustomTrigger;
use crate::dsp::{DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::SCOPE_SAMPLES;
use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::ScopeHandle;
use std::sync::Arc;

#[macro_export]
macro_rules! fa_scope_tsrc {
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
    frame_time: f32,
    srate_ms: f32,
    cur_mm: Box<[(f32, f32); 3]>,
    trig: CustomTrigger,
}

impl Scope {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            handle: ScopeHandle::new_shared(),
            idx: 0,
            srate_ms: 44.1,
            frame_time: 0.0,
            cur_mm: Box::new([(0.0, 0.0); 3]),
            trig: CustomTrigger::new(0.0, 0.0001),
        }
    }
    pub const in1: &'static str = "Scope in1\nSignal input 1.\nRange: (-1..1)\n";
    pub const in2: &'static str = "Scope in2\nSignal input 2.\nRange: (-1..1)\n";
    pub const in3: &'static str = "Scope in3\nSignal input 3.\nRange: (-1..1)\n";
    pub const time: &'static str =
        "Scope time\nDisplayed time range of the oscilloscope view.\nRange: (0..1)\n";
    pub const trig: &'static str = "Scope trig\nExternal trigger input. Only active if 'tsrc' is set to 'Extern'. 'thrsh' applies also for external triggers.\nRange: (-1..1)\n";
    pub const thrsh: &'static str = "Scope thrsh\nTrigger threshold. If the threshold is passed by the signal from low to high the signal recording will be reset. Either for internal or for external triggering. Trigger is only active if 'tsrc' is not 'Off'.\nRange: (-1..1)\n";
    pub const off1: &'static str = "Scope off1\nVisual offset of signal input 1.\nRange: (-1..1)\n";
    pub const off2: &'static str = "Scope off2\nVisual offset of signal input 2.\nRange: (-1..1)\n";
    pub const off3: &'static str = "Scope off3\nVisual offset of signal input 3.\nRange: (-1..1)\n";
    pub const gain1: &'static str =
        "Scope gain1\nVisual amplification/attenuation of the signal input 1.\nRange: (0..1)\n";
    pub const gain2: &'static str =
        "Scope gain2\nVisual amplification/attenuation of the signal input 2.\nRange: (0..1)\n";
    pub const gain3: &'static str =
        "Scope gain3\nVisual amplification/attenuation of the signal input 3.\nRange: (0..1)\n";
    pub const tsrc: &'static str = "Scope tsrc\nTriggering allows you to capture fast signals or pinning fast waveforms into the scope view for better inspection. You can let the scope freeze and manually recapture waveforms by setting 'tsrc' to 'Extern' and hitting the 'trig' button manually.\nRange: (-1..1)\n";
    pub const DESC: &'static str = r#"Signal Oscilloscope Probe

This is a signal oscilloscope probe node, you can capture up to 3 signals. You can enable internal or external triggering for capturing signals or pinning fast waveforms.
"#;
    pub const HELP: &'static str = r#"Scope - Signal Oscilloscope Probe

You can have up to 8 different scopes in your patch. That means you can
in record up to 24 signals for displaying them in the scope view.
The received signal will be forwarded to the GUI and you can inspect
the waveform there.

You can enable an internal trigger with the 'tsrc' setting set to 'Intern'.
'Intern' here means that the signal input 1 'in1' is used as trigger signal.
The 'thrsh' parameter is the trigger detection parameter. That means, if your
signal passes that threshold in negative to positive direction, the signal
recording will be reset to that point.

You can also route in an external trigger to capture signals with the 'trig'
input and 'tsrc' set to 'Extern'. Of course you can also hit the 'trig' button
manually to recapture a waveform.

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

    fn set_sample_rate(&mut self, srate: f32) {
        self.srate_ms = srate / 1000.0;
    }

    fn reset(&mut self) {}

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self,
        ctx: &mut T,
        _ectx: &mut NodeExecContext,
        nctx: &NodeContext,
        atoms: &[SAtom],
        inputs: &[ProcBuf],
        _outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{at, denorm, inp};

        let in1 = inp::Scope::in1(inputs);
        let in2 = inp::Scope::in2(inputs);
        let in3 = inp::Scope::in3(inputs);
        let time = inp::Scope::time(inputs);
        let thrsh = inp::Scope::thrsh(inputs);
        let trig = inp::Scope::trig(inputs);
        let tsrc = at::Scope::tsrc(atoms);
        let inputs = [in1, in2, in3];

        self.handle.set_active_from_mask(nctx.in_connected);

        let time = denorm::Scope::time(time, 0).clamp(0.1, 1000.0 * 300.0);
        let samples_per_block = (time * self.srate_ms) / SCOPE_SAMPLES as f32;
        let time_per_block = time / SCOPE_SAMPLES as f32;
        let sample_time = 1.0 / self.srate_ms;
        let threshold = denorm::Scope::thrsh(thrsh, 0);

        self.trig.set_threshold(threshold, threshold + 0.001);

        let trigger_input = if tsrc.i() == 2 { trig } else { in1 };
        let trigger_disabled = tsrc.i() == 0;

        //d// println!("TIME time={}; st={}; tpb={}; frame_time={}", time, sample_time, time_per_block, self.frame_time);
        if samples_per_block < 1.0 {
            let copy_count = ((1.0 / samples_per_block) as usize).min(SCOPE_SAMPLES);

            for frame in 0..ctx.nframes() {
                if self.idx < SCOPE_SAMPLES {
                    for (i, input) in inputs.iter().enumerate() {
                        let in_val = input.read(frame);
                        self.handle.write_oversampled(i, self.idx, copy_count, in_val);
                    }

                    self.idx = self.idx.saturating_add(copy_count);
                }

                if self.idx >= SCOPE_SAMPLES {
                    if self.trig.check_trigger(trigger_input.read(frame)) {
                        self.frame_time = 0.0;
                        self.idx = 0;
                    } else if trigger_disabled {
                        self.frame_time = 0.0;
                        self.idx = 0;
                    }
                }
            }
        } else {
            let cur_mm = self.cur_mm.as_mut();
            //            let samples_per_block = samples_per_block as usize;

            for frame in 0..ctx.nframes() {
                if self.idx < SCOPE_SAMPLES {
                    for (i, input) in inputs.iter().enumerate() {
                        let in_val = input.read(frame);
                        cur_mm[i].0 = cur_mm[i].0.max(in_val);
                        cur_mm[i].1 = cur_mm[i].1.min(in_val);
                    }

                    if self.frame_time >= time_per_block {
                        for i in 0..inputs.len() {
                            self.handle.write(i, self.idx, cur_mm[i]);
                        }
                        *cur_mm = [(-99999.0, 99999.0); 3];
                        self.idx = self.idx.saturating_add(1);
                        self.frame_time -= time_per_block;
                    }

                    self.frame_time += sample_time;
                }

                if self.idx >= SCOPE_SAMPLES {
                    if self.trig.check_trigger(trigger_input.read(frame)) {
                        *cur_mm = [(-99999.0, 99999.0); 3];
                        self.frame_time = 0.0;
                        self.idx = 0;
                    } else if trigger_disabled {
                        *cur_mm = [(-99999.0, 99999.0); 3];
                        self.idx = 0;
                    }
                }
            }
        }

        let last_frame = ctx.nframes() - 1;
        ctx_vals[0].set(
            (in1.read(last_frame) + in2.read(last_frame) + in3.read(last_frame)).clamp(-1.0, 1.0),
        );
    }
}
