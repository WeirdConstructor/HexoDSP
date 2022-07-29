// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use super::{
    FeedbackFilter, GraphMessage, NodeOp, NodeProg, MAX_ALLOCATED_NODES, MAX_AVAIL_TRACKERS,
    MAX_INPUTS, MAX_SCOPES, UNUSED_MONITOR_IDX, MAX_AVAIL_CODE_ENGINES
};
use crate::dsp::tracker::{PatternData, Tracker};
use crate::dsp::{node_factory, Node, NodeId, NodeInfo, ParamId, SAtom};
use crate::monitor::{new_monitor_processor, MinMaxMonitorSamples, Monitor, MON_SIG_CNT};
use crate::nodes::drop_thread::DropThread;
use crate::util::AtomicFloat;
use crate::SampleLibrary;
use crate::ScopeHandle;
#[cfg(feature = "wblockdsp")]
use crate::wblockdsp::CodeEngine;

use ringbuf::{Producer, RingBuffer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use triple_buffer::Output;

/// A NodeInstance describes the input/output/atom ports of a Node
/// and holds other important house keeping information for the [NodeConfigurator].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeInstance {
    id: NodeId,
    in_use: bool,
    prog_idx: usize,
    out_start: usize,
    out_end: usize,
    in_start: usize,
    in_end: usize,
    at_start: usize,
    at_end: usize,
    mod_start: usize,
    mod_end: usize,
    /// A mapping array, to map from input index of the node
    /// to the modulator index. Because not every input has an
    /// associated modulator.
    /// This is used later to send [GraphMessage::ModamtUpdate].
    /// The input index into this array is the index returned from
    /// routines like [NodeId::inp_param].
    in2mod_map: [Option<usize>; MAX_INPUTS],
}

impl NodeInstance {
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            in_use: false,
            prog_idx: 0,
            out_start: 0,
            out_end: 0,
            in_start: 0,
            in_end: 0,
            at_start: 0,
            at_end: 0,
            mod_start: 0,
            mod_end: 0,
            in2mod_map: [None; MAX_INPUTS],
        }
    }

    pub fn mark_used(&mut self) {
        self.in_use = true;
    }
    pub fn is_used(&self) -> bool {
        self.in_use
    }

    pub fn as_op(&self) -> NodeOp {
        NodeOp {
            idx: self.prog_idx as u8,
            out_idxlen: (self.out_start, self.out_end),
            in_idxlen: (self.in_start, self.in_end),
            at_idxlen: (self.at_start, self.at_end),
            mod_idxlen: (self.mod_start, self.mod_end),
            out_connected: 0x0,
            in_connected: 0x0,
            inputs: vec![],
        }
    }

    pub fn mod_in_local2global(&self, idx: u8) -> Option<usize> {
        if (idx as usize) > self.in2mod_map.len() {
            return None;
        }
        self.in2mod_map[idx as usize]
    }

    pub fn in_local2global(&self, idx: u8) -> Option<usize> {
        let idx = self.in_start + idx as usize;
        if idx < self.in_end {
            Some(idx)
        } else {
            None
        }
    }

    pub fn out_local2global(&self, idx: u8) -> Option<usize> {
        let idx = self.out_start + idx as usize;
        if idx < self.out_end {
            Some(idx)
        } else {
            None
        }
    }

    pub fn set_index(&mut self, idx: usize) -> &mut Self {
        self.prog_idx = idx;
        self
    }

    pub fn set_output(&mut self, s: usize, e: usize) -> &mut Self {
        self.out_start = s;
        self.out_end = e;
        self
    }

    pub fn set_input(&mut self, s: usize, e: usize) -> &mut Self {
        self.in_start = s;
        self.in_end = e;
        self
    }

    pub fn set_mod(&mut self, s: usize, e: usize) -> &mut Self {
        self.mod_start = s;
        self.mod_end = e;
        self
    }

    /// Sets the modulator index mapping: `idx` is the
    /// index of the parameter like in [NodeId::inp_param_by_idx],
    /// and `i` is the absolute index of the modulator that belongs
    /// to this parameter.
    pub fn set_mod_idx(&mut self, idx: usize, i: usize) -> &mut Self {
        self.in2mod_map[idx] = Some(i);
        self
    }

    pub fn set_atom(&mut self, s: usize, e: usize) -> &mut Self {
        self.at_start = s;
        self.at_end = e;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct NodeInputParam {
    param_id: ParamId,
    input_idx: usize,
    value: f32,
    modamt: Option<(usize, f32)>,
}

#[derive(Debug, Clone)]
struct NodeInputAtom {
    param_id: ParamId,
    at_idx: usize,
    value: SAtom,
}

/// This struct holds the frontend node configuration.
///
/// It stores which nodes are allocated and where.
/// Allocation of new nodes is done here, and parameter management
/// and synchronization is also done by this. It generally acts
/// as facade for the executed node graph in the backend.
pub struct NodeConfigurator {
    /// Holds all the nodes, their parameters and type.
    pub(crate) nodes: Vec<(NodeInfo, Option<NodeInstance>)>,
    /// An index of all nodes ever instanciated.
    /// Be aware, that currently there is no cleanup implemented.
    /// That means, any instanciated NodeId will persist throughout
    /// the whole runtime. A garbage collector might be implemented
    /// when saving presets.
    pub(crate) node2idx: HashMap<NodeId, usize>,
    /// Holding the tracker sequencers
    pub(crate) trackers: Vec<Tracker>,
    /// Holding the scope buffers:
    pub(crate) scopes: Vec<Arc<ScopeHandle>>,
    /// Holding the WBlockDSP code engine backends:
    #[cfg(feature = "wblockdsp")]
    pub(crate) code_engines: Vec<CodeEngine>,
    /// The shared parts of the [NodeConfigurator]
    /// and the [crate::nodes::NodeExecutor].
    pub(crate) shared: SharedNodeConf,

    feedback_filter: FeedbackFilter,

    /// Loads and Caches audio samples that are set as parameters
    /// for nodes.
    sample_lib: SampleLibrary,

    /// Error messages:
    errors: Vec<String>,

    /// Contains (automateable) parameters
    params: std::collections::HashMap<ParamId, NodeInputParam>,
    /// Stores the most recently set parameter values
    param_values: std::collections::HashMap<ParamId, f32>,
    /// Stores the modulation amount of a parameter
    param_modamt: std::collections::HashMap<ParamId, Option<f32>>,
    /// Contains non automateable atom data for the nodes
    atoms: std::collections::HashMap<ParamId, NodeInputAtom>,
    /// Stores the most recently set atoms
    atom_values: std::collections::HashMap<ParamId, SAtom>,

    /// Holds a copy of the most recently updated output port feedback
    /// values. Update this by calling [NodeConfigurator::update_output_feedback].
    output_fb_values: Vec<f32>,

    /// Holds the channel to the backend that sends output port feedback.
    /// This is queried by [NodeConfigurator::update_output_feedback].
    output_fb_cons: Option<Output<Vec<f32>>>,
}

pub(crate) struct SharedNodeConf {
    /// Holds the LED values of the nodes
    pub(crate) node_ctx_values: Vec<Arc<AtomicFloat>>,
    /// For updating the NodeExecutor with graph updates.
    pub(crate) graph_update_prod: Producer<GraphMessage>,
    /// For receiving monitor data from the backend thread.
    pub(crate) monitor: Monitor,
    /// Handles deallocation of dead nodes from the backend.
    #[allow(dead_code)]
    pub(crate) drop_thread: DropThread,
}

use super::node_exec::SharedNodeExec;

impl SharedNodeConf {
    pub(crate) fn new() -> (Self, SharedNodeExec) {
        let rb_graph = RingBuffer::new(MAX_ALLOCATED_NODES * 2);
        let rb_drop = RingBuffer::new(MAX_ALLOCATED_NODES * 2);

        let (rb_graph_prod, rb_graph_con) = rb_graph.split();
        let (rb_drop_prod, rb_drop_con) = rb_drop.split();

        let drop_thread = DropThread::new(rb_drop_con);

        let (monitor_backend, monitor) = new_monitor_processor();

        let mut node_ctx_values = Vec::new();
        node_ctx_values.resize_with(2 * MAX_ALLOCATED_NODES, || Arc::new(AtomicFloat::new(0.0)));

        let mut exec_node_ctx_vals = Vec::new();
        for ctx_val in node_ctx_values.iter() {
            exec_node_ctx_vals.push(ctx_val.clone());
        }

        (
            Self { node_ctx_values, graph_update_prod: rb_graph_prod, monitor, drop_thread },
            SharedNodeExec {
                node_ctx_values: exec_node_ctx_vals,
                graph_update_con: rb_graph_con,
                graph_drop_prod: rb_drop_prod,
                monitor_backend,
            },
        )
    }
}

impl NodeConfigurator {
    pub(crate) fn new() -> (Self, SharedNodeExec) {
        let mut nodes = Vec::new();
        nodes.resize_with(MAX_ALLOCATED_NODES, || (NodeInfo::from_node_id(NodeId::Nop), None));

        let (shared, shared_exec) = SharedNodeConf::new();

        let mut scopes = vec![];
        scopes.resize_with(MAX_SCOPES, || ScopeHandle::new_shared());

        (
            NodeConfigurator {
                nodes,
                shared,
                errors: vec![],
                sample_lib: SampleLibrary::new(),
                feedback_filter: FeedbackFilter::new(),
                output_fb_values: vec![],
                output_fb_cons: None,
                params: std::collections::HashMap::new(),
                param_values: std::collections::HashMap::new(),
                param_modamt: std::collections::HashMap::new(),
                atoms: std::collections::HashMap::new(),
                atom_values: std::collections::HashMap::new(),
                node2idx: HashMap::new(),
                trackers: vec![Tracker::new(); MAX_AVAIL_TRACKERS],
                #[cfg(feature = "wblockdsp")]
                code_engines: vec![CodeEngine::new(); MAX_AVAIL_CODE_ENGINES],
                scopes,
            },
            shared_exec,
        )
    }
    // FIXME: We can't drop nodes at runtime!
    //        We need to reinitialize the whole engine for this.
    //        There are too many things relying on the node index (UI).
    //
    //    pub fn drop_node(&mut self, idx: usize) {
    //        if idx >= self.nodes.len() {
    //            return;
    //        }
    //
    //        match self.nodes[idx] {
    //            NodeInfo::Nop => { return; },
    //            _ => {},
    //        }
    //
    //        self.nodes[idx] = NodeInfo::Nop;
    //        let _ =
    //            self.graph_update_prod.push(
    //                GraphMessage::NewNode {
    //                    index: idx as u8,
    //                    node: Node::Nop,
    //                });
    //    }

    pub fn for_each<F: FnMut(&NodeInfo, NodeId, usize)>(&self, mut f: F) {
        for (i, n) in self.nodes.iter().enumerate() {
            let nid = n.0.to_id();
            if NodeId::Nop == nid {
                break;
            }

            f(&n.0, nid, i);
        }
    }

    pub fn pop_error(&mut self) -> Option<String> {
        self.errors.pop()
    }

    pub fn unique_index_for(&self, ni: &NodeId) -> Option<usize> {
        self.node2idx.get(&ni).copied()
    }

    pub fn node_by_id(&self, ni: &NodeId) -> Option<&(NodeInfo, Option<NodeInstance>)> {
        let idx = self.unique_index_for(ni)?;
        self.nodes.get(idx)
    }

    pub fn node_by_id_mut(&mut self, ni: &NodeId) -> Option<&mut (NodeInfo, Option<NodeInstance>)> {
        let idx = self.unique_index_for(ni)?;
        self.nodes.get_mut(idx)
    }

    /// Returns the current modulation amount of the given parameter.
    /// Returns `None` if no modulation amount if set and thus no
    /// implicit attenuverter is set.
    pub fn get_param_modamt(&self, param: &ParamId) -> Option<f32> {
        self.param_modamt.get(&param).copied().flatten()
    }

    /// Set the modulation amount of a parameter.
    /// Returns true if a new [NodeProg] needs to be created, which can be
    /// necessary if there was no modulation amount assigned to this parameter
    /// yet.
    pub fn set_param_modamt(&mut self, param: ParamId, v: Option<f32>) -> bool {
        if param.is_atom() {
            return false;
        }

        let mut mod_idx = None;

        if let Some(nparam) = self.params.get_mut(&param) {
            if let Some(modamt) = &mut nparam.modamt {
                mod_idx = Some(modamt.0);
                modamt.1 = v.unwrap_or(0.0);
            }
        }

        // Check if the modulation amount was already set, if not, the caller
        // needs to reconstruct the graph and upload an updated NodeProg.
        if let Some(_old_modamt) = self.param_modamt.get(&param).copied().flatten() {
            if v.is_none() {
                self.param_modamt.insert(param, v);
                true
            } else {
                let modamt = v.unwrap();
                self.param_modamt.insert(param, v);

                if let Some(mod_idx) = mod_idx {
                    let _ = self
                        .shared
                        .graph_update_prod
                        .push(GraphMessage::ModamtUpdate { mod_idx, modamt });
                }

                false
            }
        } else {
            self.param_modamt.insert(param, v);
            true
        }
    }

    /// Retrieve [SAtom] values for input parameters and atoms.
    pub fn get_param(&self, param: &ParamId) -> Option<SAtom> {
        if param.is_atom() {
            self.atom_values.get(param).cloned()
        } else {
            self.param_values.get(param).map(|v| (*v).into())
        }
    }

    /// Assign [SAtom] values to input parameters and atoms.
    ///
    /// Only updates the DSP backend if [NodeConfigurator::rebuild_node_ports] was called
    /// before calling this. If no graph or the corresponding parameter is not active yet,
    /// then the value will be remembered until [NodeConfigurator::rebuild_node_ports] is called.
    pub fn set_param(&mut self, param: ParamId, at: SAtom) {
        if param.is_atom() {
            let at = if let SAtom::AudioSample((path, None)) = at.clone() {
                if !path.is_empty() {
                    match self.sample_lib.load(&path) {
                        Ok(sample) => sample.clone(),
                        Err(e) => {
                            self.errors.push(format!(
                                "Sample Loading Error\n\
                                        Couldn't load sample '{}':\n{:?}",
                                path, e
                            ));
                            at
                        }
                    }
                } else {
                    at
                }
            } else {
                at
            };

            self.atom_values.insert(param, at.clone());

            if let Some(nparam) = self.atoms.get_mut(&param) {
                nparam.value = at.clone();

                let at_idx = nparam.at_idx;
                let _ = self
                    .shared
                    .graph_update_prod
                    .push(GraphMessage::AtomUpdate { at_idx, value: at });
            }
        } else {
            self.param_values.insert(param, at.f());

            if let Some(nparam) = self.params.get_mut(&param) {
                let value = at.f();
                nparam.value = value;

                let input_idx = nparam.input_idx;
                let _ = self
                    .shared
                    .graph_update_prod
                    .push(GraphMessage::ParamUpdate { input_idx, value });
            }
        }
    }

    /// Dumps all set parameters (inputs and atoms).
    /// Most useful for serialization and saving patches.
    #[allow(clippy::type_complexity)]
    pub fn dump_param_values(&self) -> (Vec<(ParamId, f32, Option<f32>)>, Vec<(ParamId, SAtom)>) {
        let params: Vec<(ParamId, f32, Option<f32>)> = self
            .param_values
            .iter()
            .map(|(param_id, value)| {
                (
                    *param_id,
                    param_id.denorm(*value),
                    self.param_modamt.get(param_id).copied().flatten(),
                )
            })
            .collect();

        let atoms: Vec<(ParamId, SAtom)> =
            self.atom_values.iter().map(|(param_id, value)| (*param_id, value.clone())).collect();

        (params, atoms)
    }

    /// Loads parameter values from a dump. You will still need to upload
    /// a new [NodeProg] which contains these values.
    pub fn load_dumped_param_values(
        &mut self,
        params: &[(ParamId, f32, Option<f32>)],
        atoms: &[(ParamId, SAtom)],
        normalize_params: bool,
    ) {
        for (param_id, val, modamt) in params.iter() {
            let val = if normalize_params { param_id.norm(*val) } else { *val };
            self.set_param(*param_id, val.into());
            self.set_param_modamt(*param_id, *modamt);
        }

        for (param_id, val) in atoms.iter() {
            self.set_param(*param_id, val.clone());
        }
    }

    /// Iterates over every parameter and calls the given function with
    /// it's current value.
    pub fn for_each_param<F: FnMut(usize, ParamId, &SAtom, Option<f32>)>(&self, mut f: F) {
        for (_, node_input) in self.atoms.iter() {
            if let Some(unique_idx) = self.unique_index_for(&node_input.param_id.node_id()) {
                f(unique_idx, node_input.param_id, &node_input.value, None);
            }
        }

        for (_, node_input) in self.params.iter() {
            if let Some(unique_idx) = self.unique_index_for(&node_input.param_id.node_id()) {
                let modamt = self.param_modamt.get(&node_input.param_id).copied().flatten();

                f(unique_idx, node_input.param_id, &SAtom::param(node_input.value), modamt);
            }
        }
    }

    /// Returns the current phase value of the given node.
    ///
    /// It usually returns something like the position of a sequencer
    /// or the phase of an oscillator.
    pub fn phase_value_for(&self, ni: &NodeId) -> f32 {
        if let Some(idx) = self.unique_index_for(ni) {
            self.shared.node_ctx_values[(idx * 2) + 1].get()
        } else {
            0.0
        }
    }

    /// Returns the current status LED value of the given node.
    ///
    /// A status LED might be anything a specific node deems the most
    /// important value. Often it might be just the current value
    /// of the primary signal output.
    pub fn led_value_for(&self, ni: &NodeId) -> f32 {
        if let Some(idx) = self.unique_index_for(ni) {
            self.shared.node_ctx_values[idx * 2].get()
        } else {
            0.0
        }
    }

    /// Triggers recalculation of the filtered values from the
    /// current LED values and output feedback.
    ///
    /// This function internally calls [NodeConfigurator::update_output_feedback]
    /// for you, so you don't need to call it yourself.
    ///
    /// See also [NodeConfigurator::filtered_led_for]
    /// and [NodeConfigurator::filtered_out_fb_for].
    pub fn update_filters(&mut self) {
        self.update_output_feedback();
        self.feedback_filter.trigger_recalc();
    }

    /// Returns a filtered LED value that is smoothed a bit
    /// and provides a min and max value.
    ///
    /// Make sure to call [NodeConfigurator::update_filters]
    /// before calling this function, or the values won't be up to date.
    ///
    ///```
    /// use hexodsp::*;
    ///
    /// let (mut node_conf, mut node_exec) = new_node_engine();
    ///
    /// node_conf.create_node(NodeId::Sin(0));
    /// node_conf.create_node(NodeId::Amp(0));
    ///
    /// let mut prog = node_conf.rebuild_node_ports();
    ///
    /// node_conf.add_prog_node(&mut prog, &NodeId::Sin(0));
    /// node_conf.add_prog_node(&mut prog, &NodeId::Amp(0));
    ///
    /// node_conf.set_prog_node_exec_connection(
    ///     &mut prog,
    ///     (NodeId::Amp(0), NodeId::Amp(0).inp("inp").unwrap()),
    ///     (NodeId::Sin(0), NodeId::Sin(0).out("sig").unwrap()));
    ///
    /// node_conf.upload_prog(prog, true);
    ///
    /// node_exec.test_run(0.1, false);
    /// assert!((node_conf.led_value_for(&NodeId::Sin(0)) - (-0.062522)).abs() < 0.001);
    /// assert!((node_conf.led_value_for(&NodeId::Amp(0)) - (-0.062522)).abs() < 0.001);
    ///
    /// for _ in 0..10 {
    ///     node_exec.test_run(0.1, false);
    ///     node_conf.update_filters();
    ///     node_conf.filtered_led_for(&NodeId::Sin(0));
    ///     node_conf.filtered_led_for(&NodeId::Amp(0));
    /// }
    ///
    /// assert_eq!((node_conf.filtered_led_for(&NodeId::Sin(0)).0 * 1000.0).floor() as i64, 62);
    /// assert_eq!((node_conf.filtered_led_for(&NodeId::Amp(0)).0 * 1000.0).floor() as i64, 62);
    ///```
    pub fn filtered_led_for(&mut self, ni: &NodeId) -> (f32, f32) {
        let led_value = self.led_value_for(ni);
        self.feedback_filter.get_led(ni, led_value)
    }

    /// Returns a filtered output port value that is smoothed
    /// a bit and provides a min and max value.
    ///
    /// Make sure to call [NodeConfigurator::update_filters]
    /// before calling this function, or the values won't be up to date.
    /// That function also calls [NodeConfigurator::update_output_feedback]
    /// for you conveniently.
    ///
    /// For an example on how to use see [NodeConfigurator::filtered_led_for]
    /// which has the same semantics as this function.
    pub fn filtered_out_fb_for(&mut self, node_id: &NodeId, out: u8) -> (f32, f32) {
        let out_value = self.out_fb_for(node_id, out).unwrap_or(0.0);
        self.feedback_filter.get_out(node_id, out, out_value)
    }

    /// Monitor the given inputs and outputs of a specific node.
    ///
    /// The monitor data can be retrieved using
    /// [NodeConfigurator::get_minmax_monitor_samples].
    pub fn monitor(&mut self, node_id: &NodeId, inputs: &[Option<u8>], outputs: &[Option<u8>]) {
        let mut bufs = [UNUSED_MONITOR_IDX; MON_SIG_CNT];

        if let Some((_node_info, Some(node_instance))) = self.node_by_id(node_id) {
            let mut i = 0;
            for inp_idx in inputs.iter().take(MON_SIG_CNT / 2) {
                if let Some(inp_idx) = inp_idx {
                    if let Some(global_idx) = node_instance.in_local2global(*inp_idx) {
                        bufs[i] = global_idx;
                    }
                }

                i += 1;
            }

            for out_idx in outputs.iter().take(MON_SIG_CNT / 2) {
                if let Some(out_idx) = out_idx {
                    if let Some(global_idx) = node_instance.out_local2global(*out_idx) {
                        bufs[i] = global_idx;
                    }
                }

                i += 1;
            }

            let _ = self.shared.graph_update_prod.push(GraphMessage::SetMonitor { bufs });
        }
    }

    pub fn get_scope_handle(&self, scope: usize) -> Option<Arc<ScopeHandle>> {
        self.scopes.get(scope).cloned()
    }

    pub fn get_pattern_data(&self, tracker_id: usize) -> Option<Arc<Mutex<PatternData>>> {
        if tracker_id >= self.trackers.len() {
            return None;
        }

        Some(self.trackers[tracker_id].data())
    }

    pub fn check_pattern_data(&mut self, tracker_id: usize) {
        if tracker_id >= self.trackers.len() {
            return;
        }

        self.trackers[tracker_id].send_one_update();
    }

    pub fn delete_nodes(&mut self) {
        self.node2idx.clear();
        self.nodes.fill_with(|| (NodeInfo::from_node_id(NodeId::Nop), None));
        self.params.clear();
        self.param_values.clear();
        self.param_modamt.clear();
        self.atoms.clear();
        self.atom_values.clear();

        let _ = self.shared.graph_update_prod.push(GraphMessage::Clear { prog: NodeProg::empty() });
    }

    pub fn create_node(&mut self, ni: NodeId) -> Option<(&NodeInfo, u8)> {
        if let Some((mut node, info)) = node_factory(ni) {
            let mut index: Option<usize> = None;

            if let Node::TSeq { node } = &mut node {
                let tracker_idx = ni.instance();
                if let Some(trk) = self.trackers.get_mut(tracker_idx) {
                    node.set_backend(trk.get_backend());
                }
            }

            #[cfg(feature = "wblockdsp")]
            if let Node::Code { node } = &mut node {
                let code_idx = ni.instance();
                if let Some(cod) = self.code_engines.get_mut(code_idx) {
                    node.set_backend(cod.get_backend());
                }
            }

            if let Node::Scope { node } = &mut node {
                if let Some(handle) = self.scopes.get(ni.instance()) {
                    node.set_scope_handle(handle.clone());
                }
            }

            for i in 0..self.nodes.len() {
                if let NodeId::Nop = self.nodes[i].0.to_id() {
                    index = Some(i);
                    break;
                } else if ni == self.nodes[i].0.to_id() {
                    return Some((&self.nodes[i].0, i as u8));
                }
            }

            if let Some(index) = index {
                self.node2idx.insert(ni, index);

                self.nodes[index] = (info, None);

                let _ = self
                    .shared
                    .graph_update_prod
                    .push(GraphMessage::NewNode { index: index as u8, node });

                Some((&self.nodes[index].0, index as u8))
            } else {
                let index = self.nodes.len();
                self.node2idx.insert(ni, index);

                self.nodes.resize_with((self.nodes.len() + 1) * 2, || {
                    (NodeInfo::from_node_id(NodeId::Nop), None)
                });
                self.nodes[index] = (info, None);

                let _ = self
                    .shared
                    .graph_update_prod
                    .push(GraphMessage::NewNode { index: index as u8, node });

                Some((&self.nodes[index].0, index as u8))
            }
        } else {
            None
        }
    }

    /// Returns the first instance of the given [NodeId] (starting with the
    /// instance of the [NodeId]) that has not been used.
    ///
    /// Primarily used by the (G)UI when creating new nodes to be added to the
    /// graph.
    ///
    /// Should be called after the [NodeProg] has been created
    /// (and after [NodeConfigurator::rebuild_node_ports] was called).
    ///
    /// If new nodes were created/deleted/reordered in between this function
    /// might not work properly and assign already used instances.
    pub fn unused_instance_node_id(&self, mut id: NodeId) -> NodeId {
        while let Some((_, Some(ni))) = self.node_by_id(&id) {
            if !ni.is_used() {
                return ni.id;
            }

            id = id.to_instance(id.instance() + 1);
        }

        id
    }

    /// Rebuilds Input/Output/Atom indices for the nodes, which is necessary
    /// if nodes were created/deleted or reordered. It also assigns
    /// input parameter and atom values for new nodes.
    ///
    /// Returns a new NodeProg with space for all allocated nodes
    /// inputs, outputs and atoms.
    ///
    /// Execute this after a [NodeConfigurator::create_node].
    pub fn rebuild_node_ports(&mut self) -> NodeProg {
        // Regenerating the params and atoms in the next step:
        self.params.clear();
        self.atoms.clear();

        let mut out_len = 0;
        let mut in_len = 0;
        let mut at_len = 0;
        let mut mod_len = 0;

        for (i, (node_info, node_instance)) in self.nodes.iter_mut().enumerate() {
            let id = node_info.to_id();

            // - calculate size of output vector.
            let out_idx = out_len;
            out_len += node_info.out_count();

            // - calculate size of input vector.
            let in_idx = in_len;
            in_len += node_info.in_count();

            // - calculate size of atom vector.
            let at_idx = at_len;
            at_len += node_info.at_count();

            // - hold the mod start index of this node.
            let mod_idx = mod_len;

            if id == NodeId::Nop {
                break;
            }

            let mut ni = NodeInstance::new(id);
            ni.set_index(i)
                .set_output(out_idx, out_len)
                .set_input(in_idx, in_len)
                .set_atom(at_idx, at_len);

            // - save offset and length of each node's
            //   allocation in the output vector.
            *node_instance = Some(ni);

            //d// println!("INSERT[{}]: {:?} outidx: {},{} inidx: {},{} atidx: {},{}",
            //d//          i, id, out_idx, out_len, in_idx, in_len, at_idx, at_len);

            // Create new parameters and initialize them if they did not
            // already exist previously
            for param_idx in in_idx..in_len {
                let input_idx = param_idx - in_idx;

                if let Some(param_id) = id.inp_param_by_idx(input_idx) {
                    let value = if let Some(value) = self.param_values.get(&param_id) {
                        *value
                    } else {
                        param_id.norm_def()
                    };

                    // If we have a modulation, store the absolute
                    // index of it in the [NodeProg::modops] vector later:
                    let ma = self.param_modamt.get(&param_id).copied().flatten();
                    let modamt = if ma.is_some() {
                        let mod_idx = mod_len;
                        node_instance.as_mut().unwrap().set_mod_idx(input_idx, mod_idx);
                        mod_len += 1;
                        Some((mod_idx, ma.unwrap()))
                    } else {
                        None
                    };

                    self.param_values.insert(param_id, value);
                    self.params.insert(
                        param_id,
                        NodeInputParam { param_id, value, input_idx: param_idx, modamt },
                    );
                }
            }

            // After iterating through the parameters we can
            // store the range of the indices of this node.
            node_instance.as_mut().unwrap().set_mod(mod_idx, mod_len);

            // Create new atom data and initialize it if it did not
            // already exist from a previous matrix instance.
            for atom_idx in at_idx..at_len {
                // XXX: See also the documentation of atom_param_by_idx about the
                // little param_id for an Atom weirdness here.
                if let Some(param_id) = id.atom_param_by_idx(atom_idx - at_idx) {
                    let value = if let Some(atom) = self.atom_values.get(&param_id) {
                        atom.clone()
                    } else {
                        param_id.as_atom_def()
                    };

                    self.atom_values.insert(param_id, value.clone());
                    self.atoms
                        .insert(param_id, NodeInputAtom { param_id, value, at_idx: atom_idx });
                }
            }
        }

        NodeProg::new(out_len, in_len, at_len, mod_len)
    }

    /// Creates a new [NodeOp] and add it to the [NodeProg].
    ///
    /// It will fail silently if the nodes have not been created yet or
    /// [NodeConfigurator::rebuild_node_ports] was not called before. So make sure this is the
    /// case or don't expect the node and input to be executed.
    pub fn add_prog_node(&mut self, prog: &mut NodeProg, node_id: &NodeId) {
        if let Some((_node_info, Some(node_instance))) = self.node_by_id_mut(node_id) {
            node_instance.mark_used();
            let op = node_instance.as_op();
            prog.append_op(op);
        }
    }

    /// Adds an adjacent output connection to the given node input.
    /// Will either create a new [NodeOp] in the [NodeProg] or append to an
    /// existing one. This means the order you set the to be executed node
    /// connections, is the order the [NodeProg] is going to be executed by the
    /// DSP thread later.
    ///
    /// It will fail silently if the nodes have not been created yet or
    /// [NodeConfigurator::rebuild_node_ports] was not called before. So make sure this is the
    /// case or don't expect the node and input to be executed.
    pub fn set_prog_node_exec_connection(
        &mut self,
        prog: &mut NodeProg,
        node_input: (NodeId, u8),
        adjacent_output: (NodeId, u8),
    ) {
        let output_index =
            if let Some((_, Some(node_instance))) = self.node_by_id(&adjacent_output.0) {
                node_instance.out_local2global(adjacent_output.1)
            } else {
                return;
            };

        if let Some((_node_info, Some(node_instance))) = self.node_by_id_mut(&node_input.0) {
            node_instance.mark_used();
            let op = node_instance.as_op();

            let input_index = node_instance.in_local2global(node_input.1);
            let mod_index = node_instance.mod_in_local2global(node_input.1);
            if let (Some(input_index), Some(output_index)) = (input_index, output_index) {
                prog.append_edge(op, input_index, output_index, mod_index);
            }
        }
    }

    /// Uploads a new NodeProg instance.
    ///
    /// Create a new NodeProg instance with [NodeConfigurator::rebuild_node_ports]
    /// for each call to this function. Otherwise things like the
    /// [NodeConfigurator::out_fb_for] might not work properly!
    ///
    /// The `copy_old_out` parameter should be set if there are only
    /// new nodes appended to the end of the node instances.
    /// It helps to prevent clicks when there is a feedback path somewhere.
    ///
    /// It must not be set when a completely new set of node instances
    /// was created, for instance when a completely new patch was loaded.
    ///
    /// Here is an example on how to use the [NodeConfigurator]
    /// directly to setup and upload a [NodeProg]:
    ///
    ///```
    /// use hexodsp::*;
    ///
    /// let (mut node_conf, mut node_exec) = new_node_engine();
    ///
    /// node_conf.create_node(NodeId::Sin(0));
    /// node_conf.create_node(NodeId::Amp(0));
    ///
    /// let mut prog = node_conf.rebuild_node_ports();
    ///
    /// node_conf.add_prog_node(&mut prog, &NodeId::Sin(0));
    /// node_conf.add_prog_node(&mut prog, &NodeId::Amp(0));
    ///
    /// node_conf.set_prog_node_exec_connection(
    ///     &mut prog,
    ///     (NodeId::Amp(0), NodeId::Amp(0).inp("inp").unwrap()),
    ///     (NodeId::Sin(0), NodeId::Sin(0).out("sig").unwrap()));
    ///
    /// node_conf.upload_prog(prog, true);
    ///```
    pub fn upload_prog(&mut self, mut prog: NodeProg, copy_old_out: bool) {
        // Copy the parameter values and atom data into the program:
        // They are extracted by process_graph_updates() later to
        // reset the inp[] input value vector.
        for (_param_id, param) in self.params.iter() {
            prog.params_mut()[param.input_idx] = param.value;

            if let Some((mod_idx, amt)) = param.modamt {
                prog.modops_mut()[mod_idx].set_amt(amt);
            }
        }

        // The atoms are referred to directly on process() call.
        for (_param_id, param) in self.atoms.iter() {
            prog.atoms_mut()[param.at_idx] = param.value.clone();
        }

        self.output_fb_cons = prog.take_feedback_consumer();

        let _ = self.shared.graph_update_prod.push(GraphMessage::NewProg { prog, copy_old_out });
    }

    /// Retrieves the feedback value for a specific output port of the
    /// given [NodeId]. You need to call [NodeConfigurator::update_output_feedback]
    /// before this, or otherwise your output values might be outdated
    /// or not available at all.
    ///
    /// See also [NodeConfigurator::filtered_out_fb_for] for a
    /// filtered variant suitable for UI usage.
    pub fn out_fb_for(&self, node_id: &NodeId, out: u8) -> Option<f32> {
        if let Some((_, Some(node_instance))) = self.node_by_id(node_id) {
            self.output_fb_values.get(node_instance.out_local2global(out)?).copied()
        } else {
            None
        }
    }

    /// Checks if the backend has new output feedback values.
    /// Call this function for each frame of the UI to get the most
    /// up to date output feedback values that are available.
    ///
    /// Retrieve the output value by calling [NodeConfigurator::out_fb_for].
    pub fn update_output_feedback(&mut self) {
        if let Some(out_fb_output) = &mut self.output_fb_cons {
            out_fb_output.update();
            let out_vec = out_fb_output.output_buffer();

            self.output_fb_values.clear();
            self.output_fb_values.resize(out_vec.len(), 0.0);
            self.output_fb_values.copy_from_slice(&out_vec[..]);
        }
    }

    pub fn get_minmax_monitor_samples(&mut self, idx: usize) -> &MinMaxMonitorSamples {
        self.shared.monitor.get_minmax_monitor_samples(idx)
    }
}
