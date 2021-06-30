// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals};
use crate::dsp::helpers::{DelayBuffer, crossfade, TriggerSampleClock};

#[macro_export]
macro_rules! fa_delay_mode { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
        let s =
            match ($v.round() as usize) {
                0  => "Time",
                1  => "Sync",
                _  => "?",
            };
        write!($formatter, "{}", s)
} } }

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Delay {
    buffer:             Box<DelayBuffer>,
    clock:              TriggerSampleClock,
}

impl Delay {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            buffer:            Box::new(DelayBuffer::new()),
            clock:             TriggerSampleClock::new(),
        }
    }

    pub const inp  : &'static str =
        "Delay inp\nThe signal input for the delay. You can mix in this \
         input to the output with the 'mix' parameter.\nRange: (-1..1)";
    pub const trig : &'static str =
        "Delay trig\nIf you set 'mode' to 'Sync', the delay time will be \
         synchronized to the trigger signals received on this input.\nRange: (-1..1)";
    pub const time : &'static str =
        "Delay time\nThe delay time. It can be freely modulated to your \
         likings.\nRange: (0..1)";
    pub const fb   : &'static str =
        "Delay fb\nThe feedback amount of the delay output to it's input. \
        \nRange: (-1..1)";
    pub const mix  : &'static str =
        "Delay mix\nThe dry/wet mix of the delay.\nRange: (0..1)";
    pub const mode : &'static str =
        "Delay mode\nAllows different operating modes of the delay. \
        'Time' is the default, and means that the 'time' input \
        specifies the delay time. 'Sync' will synchronize the delay time \
        with the trigger signals on the 'trig' input.";
    pub const sig  : &'static str =
        "Delay sig\nThe output of the dry/wet mix.\nRange: (-1..1)";

    pub const DESC : &'static str =
r#"Simple Delay Line

This is a very simple single buffer delay node.
It provides an internal feedback and dry/wet mix.
"#;
pub const HELP : &'static str =
r#"Delay - A Simple Delay Line

This node provides a very simple delay line with the bare minimum of
parameters. Most importantly a freely modulateable 'time' parameter
and a feedback 'fb' parameter.

Via the 'mix' parameter you can mix in the input signal to the output.

You can use this node to delay any kind of signal, from a simple control
signal to an audio signal.

For other kinds of delay/feedback please see also the 'FbWr'/'FbRd' nodes.
"#;
}

impl DspNode for Delay {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) {
        self.buffer.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.buffer.reset();
        self.clock.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        atoms: &[SAtom], _params: &[ProcBuf], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{at, out, inp, denorm};

        let buffer  = &mut *self.buffer;

        let mode = at::Delay::mode(atoms);
        let inp  = inp::Delay::inp(inputs);
        let trig = inp::Delay::trig(inputs);
        let time = inp::Delay::time(inputs);
        let fb   = inp::Delay::fb(inputs);
        let mix  = inp::Delay::mix(inputs);
        let out  = out::Delay::sig(outputs);

        if mode.i() == 0 {
            for frame in 0..ctx.nframes() {
                let dry = inp.read(frame);

                let out_sample =
                    buffer.cubic_interpolate_at(
                        denorm::Delay::time(time, frame));

                buffer.feed(dry + out_sample * denorm::Delay::fb(fb, frame));

                out.write(frame,
                    crossfade(dry, out_sample,
                        denorm::Delay::mix(mix, frame).clamp(0.0, 1.0)));
            }
        } else {
            for frame in 0..ctx.nframes() {
                let dry = inp.read(frame);

                let clock_samples =
                    self.clock.next(denorm::Delay::trig(trig, frame));
                let out_sample = buffer.at(clock_samples as usize);

                buffer.feed(dry + out_sample * denorm::Delay::fb(fb, frame));

                out.write(frame,
                    crossfade(dry, out_sample,
                        denorm::Delay::mix(mix, frame).clamp(0.0, 1.0)));
            }
        }

        let last_frame = ctx.nframes() - 1;
        ctx_vals[0].set(out.read(last_frame));
    }
}
