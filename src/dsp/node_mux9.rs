// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphFun, LedPhaseVals, NodeContext, NodeGlobalRef, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::Trigger;

#[macro_export]
macro_rules! fa_mux9_in_cnt {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "1",
            1 => "2",
            2 => "3",
            3 => "4",
            4 => "5",
            5 => "6",
            6 => "7",
            7 => "8",
            8 => "9",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

/// A 9 channel signal multiplexer
#[derive(Debug, Clone)]
pub struct Mux9 {
    trig_rst: Trigger,
    trig_up: Trigger,
    trig_down: Trigger,
    idx: u8,
}

impl Mux9 {
    pub fn new(_nid: &NodeId, _node_global: &NodeGlobalRef) -> Self {
        Self {
            trig_rst: Trigger::new(),
            trig_up: Trigger::new(),
            trig_down: Trigger::new(),
            idx: 0,
        }
    }
    pub const slct: &'static str = "Selects the input that is routed to the output ~~sig~~.\
        But only if this input is actually connected. If there is no \
        connection, the ~~t_rst~~, ~~t_up~~ and ~~t_down~~ trigger inputs are used to \
        control the current routing. The maximum routed input is determined \
        by the ~~in_cnt~~ setting.";
    pub const t_rst: &'static str =
        "Trigger resets the internal routing to the first input ~~in_1~~.\
        Keep in mind: This input is only used if ~~slct~~ is not connected.\
        ";
    pub const t_up: &'static str = "Trigger increases the internal routing to the next input port.\
        If the last input (depending on the ~~in_cnt~~ setting) was selected\
        if will wrap around to ~~in_1~~.\
        Keep in mind: This input is only used if ~~slct~~ is not connected.\
        ";
    pub const t_down: &'static str =
        "Trigger decreases the internal routing to the previous input \
        port (eg. ~~in_3~~ => ~~in_2~~). If ~~in_1~~ as selected, then it will \
        wrap around to the highest possible input port (depending on the \
        ~~in_cnt~~ setting).\
        Keep in mind: This input is only used if ~~slct~~ is not connected.\
        ";
    pub const in_1: &'static str = "Input port 1.";
    pub const in_2: &'static str = "Input port 2.";
    pub const in_3: &'static str = "Input port 3.";
    pub const in_4: &'static str = "Input port 4.";
    pub const in_5: &'static str = "Input port 5.";
    pub const in_6: &'static str = "Input port 6.";
    pub const in_7: &'static str = "Input port 7.";
    pub const in_8: &'static str = "Input port 8.";
    pub const in_9: &'static str = "Input port 9.";
    pub const in_cnt: &'static str = "The number of inputs that are routed to the output. \
        This will limit the number of maximally used inputs.\n";
    pub const sig: &'static str = "The currently selected input port will be presented on \
        this output port.";
    pub const DESC: &'static str = r#"9 Ch. Multiplexer

An up to 9 channel multiplexer aka switch or junction.
You can route one of the 9 (or fewer) inputs to the output.
The opposite of this node is the `Demux9`,
which demultiplexes or routes the one input signal to one of the 9 outputs.
"#;
    pub const HELP: &'static str = r#"9 Channel Multiplexer/Switch

This is an up to 9 channel multiplexer, also known as switch or junction.
You can route one of the 9 (or fewer) inputs to the one output.
Selection of the input is done either via a control signal to the
~~slct~~ input (range **0**..**1**) (exclusive) or via the ~~t_rst~~, ~~t_up~~ or
~~t_down~~ triggers.

If the ~~slct~~ input is not connected, the trigger inputs are active.
If you still prefer a knob for manually selecting the input, consider using
some constant signal source like an `Amp` node with an unconnected input.

The ~~in_cnt~~ parameter allows selecting the number of routed input channels.

The opposite of this node is the `Demux9`, which demultiplexes or routes
the one input signal to one of the 9 outputs.

Tip:
    An interesting use case for this node is to use it as (up to) 9 step
    control signal sequencer. Leave the ~~in_1~~ to ~~in_9~~ ports unconnected
    and dial in the desired value via the parameter knobs. This can lead to
    interesting results. Even more interesting it can become if you stack
    multiple `Demux9` in series and connect just some of the input ports
    for slightly changing sequences. Attach a slew limiter node (eg. `LSlew`
    or `ESlew`) if less harsh transitions between the input routings is
    desired.
"#;

    pub fn graph_fun() -> Option<GraphFun> {
        None
    }
}

impl DspNode for Mux9 {
    fn set_sample_rate(&mut self, _srate: f32) {}
    fn reset(&mut self) {}

    #[inline]
    fn process(
        &mut self,
        ctx: &mut dyn NodeAudioContext,
        _ectx: &mut NodeExecContext,
        nctx: &NodeContext,
        atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{at, denorm, inp, out};

        let in_1 = inp::Mux9::in_1(inputs);
        let in_2 = inp::Mux9::in_2(inputs);
        let in_3 = inp::Mux9::in_3(inputs);
        let in_4 = inp::Mux9::in_4(inputs);
        let in_5 = inp::Mux9::in_5(inputs);
        let in_6 = inp::Mux9::in_6(inputs);
        let in_7 = inp::Mux9::in_7(inputs);
        let in_8 = inp::Mux9::in_8(inputs);
        let in_9 = inp::Mux9::in_9(inputs);
        let slct = inp::Mux9::slct(inputs);
        let t_rst = inp::Mux9::t_rst(inputs);
        let t_up = inp::Mux9::t_up(inputs);
        let t_down = inp::Mux9::t_down(inputs);
        let out = out::Mux9::sig(outputs);

        let max: u8 = at::Mux9::in_cnt(atoms).i() as u8 + 1;
        self.idx %= max;

        if nctx.in_connected & 0x1 == 0x1 {
            for frame in 0..ctx.nframes() {
                self.idx =
                    (max as f32 * (denorm::Mux9::slct(slct, frame) - 0.00001)).floor() as u8 % max;

                out.write(
                    frame,
                    match self.idx {
                        0 => denorm::Mux9::in_1(in_1, frame),
                        1 => denorm::Mux9::in_2(in_2, frame),
                        2 => denorm::Mux9::in_3(in_3, frame),
                        3 => denorm::Mux9::in_4(in_4, frame),
                        4 => denorm::Mux9::in_5(in_5, frame),
                        5 => denorm::Mux9::in_6(in_6, frame),
                        6 => denorm::Mux9::in_7(in_7, frame),
                        7 => denorm::Mux9::in_8(in_8, frame),
                        _ => denorm::Mux9::in_9(in_9, frame),
                    },
                );
            }
        } else {
            for frame in 0..ctx.nframes() {
                if self.trig_rst.check_trigger(denorm::Mux9::t_rst(t_rst, frame)) {
                    self.idx = 0;
                }

                if self.trig_up.check_trigger(denorm::Mux9::t_up(t_up, frame)) {
                    self.idx = (self.idx + 1) % max;
                }

                if self.trig_down.check_trigger(denorm::Mux9::t_down(t_down, frame)) {
                    self.idx = (self.idx + max - 1) % max;
                }

                out.write(
                    frame,
                    match self.idx {
                        0 => denorm::Mux9::in_1(in_1, frame),
                        1 => denorm::Mux9::in_2(in_2, frame),
                        2 => denorm::Mux9::in_3(in_3, frame),
                        3 => denorm::Mux9::in_4(in_4, frame),
                        4 => denorm::Mux9::in_5(in_5, frame),
                        5 => denorm::Mux9::in_6(in_6, frame),
                        6 => denorm::Mux9::in_7(in_7, frame),
                        7 => denorm::Mux9::in_8(in_8, frame),
                        _ => denorm::Mux9::in_9(in_9, frame),
                    },
                );
            }
        }

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
    }
}
