// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

pub const MAX_ALLOCATED_NODES  : usize = 256;
pub const MAX_INPUTS           : usize = 32;
pub const MAX_SMOOTHERS        : usize = 36 + 4; // 6 * 6 modulator inputs + 4 UI Knobs
pub const MAX_AVAIL_TRACKERS   : usize = 128;
pub const MAX_FB_DELAYS        : usize = 256;   // 256 feedback delays, thats roughly 1.2MB RAM
pub const FB_DELAY_TIME_US     : usize = 3140;  // 3.14ms (should be enough for MAX_BLOCK_SIZE)
// This means, until 384000 sample rate the times are accurate.
pub const MAX_FB_DELAY_SRATE   : usize = 48000 * 8;
pub const MAX_FB_DELAY_SIZE    : usize =
    (MAX_FB_DELAY_SRATE * FB_DELAY_TIME_US) / 1000000;

mod node_prog;
mod node_exec;
mod node_conf;
mod drop_thread;
mod node_graph_ordering;
pub mod visual_sampling_filter;
mod feedback_filter;

pub(crate) use visual_sampling_filter::*;

pub use node_exec::*;
pub use node_prog::*;
pub use node_conf::*;
pub use feedback_filter::*;
pub use node_graph_ordering::NodeGraphOrdering;

pub use crate::monitor::MinMaxMonitorSamples;
use crate::monitor::MON_SIG_CNT;
use crate::dsp::{Node, SAtom};

#[derive(Debug)]
pub(crate) enum DropMsg {
    Node { node: Node },
    Prog { prog: NodeProg },
    Atom { atom: SAtom },
}

/// Big messages for updating the NodeExecutor thread.
/// Usually used for shoveling NodeProg and Nodes to and from
/// the NodeExecutor thread.
#[derive(Debug)]
pub enum GraphMessage {
    NewNode { index: u8, node: Node },
    NewProg { prog: NodeProg, copy_old_out: bool },
    Clear   { prog: NodeProg },
}

/// Messages for small updates between the NodeExecutor thread
/// and the NodeConfigurator.
#[derive(Debug)]
pub enum QuickMessage {
    AtomUpdate   { at_idx:    usize, value: SAtom },
    ParamUpdate  { input_idx: usize, value: f32 },
    ModamtUpdate { mod_idx:   usize, modamt: f32 },
    /// Sets the buffer indices to monitor with the FeedbackProcessor.
    SetMonitor   { bufs: [usize; MON_SIG_CNT], },
}

pub const UNUSED_MONITOR_IDX : usize = 99999;

/// Creates a NodeConfigurator and a NodeExecutor which are interconnected
/// by ring buffers.
pub fn new_node_engine() -> (NodeConfigurator, NodeExecutor) {
    let (nc, shared_exec) = NodeConfigurator::new();
    let ne = NodeExecutor::new(shared_exec);

    // XXX: This is one of the earliest and most consistent points
    //      in runtime to do this kind of initialization:
    crate::dsp::helpers::init_cos_tab();

    (nc, ne)
}

