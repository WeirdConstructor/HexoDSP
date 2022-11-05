// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

pub const SCOPE_SAMPLES: usize = 512;
pub const MAX_DSP_NODE_INPUTS: usize = 32;
pub const MAX_SMOOTHERS: usize = 36 + 4; // 6 * 6 modulator inputs + 4 UI Knobs
pub const MAX_INJ_MIDI_EVENTS: usize = 64;

mod drop_thread;
mod feedback_filter;
mod midi;
mod node_conf;
mod node_exec;
mod node_graph_ordering;
mod node_prog;
pub mod visual_sampling_filter;

pub(crate) use visual_sampling_filter::*;

pub use feedback_filter::*;
pub use midi::{EventWindowing, HxMidiEvent, HxTimedEvent, MidiEventPointer};
pub use node_conf::*;
pub use node_exec::*;
pub use node_graph_ordering::NodeGraphOrdering;
pub use node_prog::*;

use crate::dsp::{Node, SAtom};
pub use crate::monitor::MinMaxMonitorSamples;
use crate::monitor::MON_SIG_CNT;

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum DropMsg {
    Node { node: Node },
    Prog { prog: NodeProg },
    Atom { atom: SAtom },
}

/// Messages for updating the [NodeExecutor] thread.
/// Usually used for shoveling NodeProg and Nodes to and from
/// the [NodeExecutor] thread. And also parameter updates of course.
#[derive(Debug)]
pub enum GraphMessage {
    NewProg {
        prog: NodeProg,
        copy_old_out: bool,
    },
    Clear {
        prog: NodeProg,
    },

    // XXX: Parameter updates used to be separate from the graph update, but this
    // became a race condition and I had to revert this premature optimization.
    AtomUpdate {
        at_idx: usize,
        value: SAtom,
    },
    ParamUpdate {
        input_idx: usize,
        value: f32,
    },
    ModamtUpdate {
        mod_idx: usize,
        modamt: f32,
    },
    InjectMidi {
        midi_ev: HxMidiEvent,
    },
    /// Sets the buffer indices to monitor with the FeedbackProcessor.
    SetMonitor {
        bufs: [usize; MON_SIG_CNT],
    },
}

/// Message from the DSP graph/backend to the frontend. Such as MIDI events
/// for MIDI learn for instance.
pub enum GraphEvent {
    MIDI(HxMidiEvent),
}

pub const UNUSED_MONITOR_IDX: usize = 99999;

/// Creates a [NodeConfigurator] and a [NodeExecutor] which are interconnected
/// by ring buffers.
pub fn new_node_engine() -> (NodeConfigurator, NodeExecutor) {
    let (nc, shared_exec) = NodeConfigurator::new();
    let ne = NodeExecutor::new(shared_exec);

    // XXX: This is one of the earliest and most consistent points
    //      in runtime to do this kind of initialization:
    synfx_dsp::init_cos_tab();

    (nc, ne)
}
