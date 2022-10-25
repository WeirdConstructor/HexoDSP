// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{DspNode, GraphFun, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::{crossfade, DelayBuffer, TriggerSampleClock};

#[macro_export]
macro_rules! fa_delay_mode {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "Time",
            1 => "Sync",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Delay {
    buffer: Box<DelayBuffer<f32>>,
    clock: TriggerSampleClock,
}

impl Delay {
    pub fn new(_nid: &NodeId) -> Self {
        Self { buffer: Box::new(DelayBuffer::new()), clock: TriggerSampleClock::new() }
    }

    pub const inp: &'static str = "The signal input for the delay. You can mix in this \
         input to the output with the ~~mix~~ parameter.";
    pub const trig: &'static str = "If you set ~~mode~~ to **Sync** the delay time will be \
         synchronized to the trigger signals received on this input.";
    pub const time: &'static str = "The delay time. It can be freely modulated to your \
         likings.";
    pub const fb: &'static str = "The feedback amount of the delay output to it's input. \
        ";
    pub const mix: &'static str = "The dry/wet mix of the delay.";
    pub const mode: &'static str = "Allows different operating modes of the delay. \
        **Time** is the default, and means that the ~~time~~ input \
        specifies the delay time. **Sync** will synchronize the delay time \
        with the trigger signals on the ~~trig~~ input.";
    pub const sig: &'static str = "The output of the dry/wet mix.";

    pub const DESC: &'static str = r#"Simple Delay Line

This is a very simple single buffer delay node.
It provides an internal feedback and dry/wet mix.
"#;
    pub const HELP: &'static str = r#"A Simple Delay Line

This node provides a very simple delay line with the bare minimum of
parameters. Most importantly a freely modulateable ~~time~~ parameter
and a feedback ~~fb~~ parameter.

Via the ~~mix~~ parameter you can mix in the input signal to the output.

You can use this node to delay any kind of signal, from a simple control
signal to an audio signal.

For other kinds of delay/feedback please see also the `FbWr`/`FbRd` nodes.
"#;

    pub fn graph_fun() -> Option<GraphFun> {
        None
    }
}

impl DspNode for Delay {
    fn set_sample_rate(&mut self, srate: f32) {
        self.buffer.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.buffer.reset();
        self.clock.reset();
    }

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
        use crate::dsp::{at, denorm, inp, out};

        let buffer = &mut *self.buffer;

        let mode = at::Delay::mode(atoms);
        let inp = inp::Delay::inp(inputs);
        let trig = inp::Delay::trig(inputs);
        let time = inp::Delay::time(inputs);
        let fb = inp::Delay::fb(inputs);
        let mix = inp::Delay::mix(inputs);
        let out = out::Delay::sig(outputs);

        if mode.i() == 0 {
            for frame in 0..ctx.nframes() {
                let dry = inp.read(frame);

                let out_sample = buffer.cubic_interpolate_at(denorm::Delay::time(time, frame));

                buffer.feed(dry + out_sample * denorm::Delay::fb(fb, frame));

                out.write(
                    frame,
                    crossfade(dry, out_sample, denorm::Delay::mix(mix, frame).clamp(0.0, 1.0)),
                );
            }
        } else {
            for frame in 0..ctx.nframes() {
                let dry = inp.read(frame);

                let clock_samples = self.clock.next(denorm::Delay::trig(trig, frame));
                let out_sample = buffer.at(clock_samples as usize);

                buffer.feed(dry + out_sample * denorm::Delay::fb(fb, frame));

                out.write(
                    frame,
                    crossfade(dry, out_sample, denorm::Delay::mix(mix, frame).clamp(0.0, 1.0)),
                );
            }
        }

        let last_frame = ctx.nframes() - 1;
        ctx_vals[0].set(out.read(last_frame));
    }
}
