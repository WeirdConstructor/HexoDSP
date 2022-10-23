use crate::dsp::build::*;
use crate::nodes::{new_node_engine, NodeGraphOrdering};
use crate::{NodeConfigurator, NodeExecutor, NodeId, ParamId, SAtom};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum SynthError {
    CycleDetected,
    BadParamName(NodeId, String),
    BadOutputName(NodeId, String),
    UnknownParam(NodeId, String),
}

#[derive(Debug, Clone)]
pub struct NodeConfig {
    node_id: NodeId,
    edges: HashMap<String, (NodeId, String)>,
    params: HashMap<String, (SAtom, Option<f32>)>,
}

impl NodeConfig {
    pub fn new(node_id: NodeId) -> Self {
        Self { node_id, edges: HashMap::new(), params: HashMap::new() }
    }

    pub fn set_edge(&mut self, param_name: &str, node_id: NodeId, output: &str) {
        self.edges.insert(param_name.to_string(), (node_id, output.to_string()));
    }

    pub fn set_param(&mut self, param_name: &str, value: SAtom, modamt: Option<f32>) {
        self.params.insert(param_name.to_string(), (value, modamt));
    }
}

pub struct SynthConstructor {
    config: NodeConfigurator,
    exec: Option<NodeExecutor>,
    nodes: HashMap<NodeId, Box<NodeConfig>>,
    graph_ordering: NodeGraphOrdering,
}

impl SynthConstructor {
    pub fn new() -> Self {
        let (config, exec) = new_node_engine();

        Self {
            config,
            exec: Some(exec),
            nodes: HashMap::new(),
            graph_ordering: NodeGraphOrdering::new(),
        }
    }

    pub fn clear(&mut self) {
        self.graph_ordering.clear();
        self.nodes.clear();
        self.config.delete_nodes();
    }

    pub fn executor(&mut self) -> Option<NodeExecutor> {
        self.exec.take()
    }

    fn walk_upload(&mut self, node: &ConstructorNode, only_update_params: bool) -> Result<bool, SynthError> {
        let mut need_rebuild = false;

        if !self.nodes.contains_key(&node.node_id) {
            self.nodes.insert(node.node_id, Box::new(NodeConfig::new(node.node_id)));
        }

        let mut walk_afterwads = vec![];

        let mut changed_params = false;

        if let Some(node_config) = self.nodes.get_mut(&node.node_id) {
            for op in node.ops.borrow().iter() {
                match op {
                    ConstructorOp::SetDenormModAmt(name, v, ma) => {
                        node_config.set_param(&name, SAtom::param(*v), Some(*ma));
                        changed_params = true;
                    }
                    ConstructorOp::SetDenorm(name, v) => {
                        node_config.set_param(&name, SAtom::param(*v), None);
                        changed_params = true;
                    }
                    ConstructorOp::SetSetting(name, v) => {
                        node_config.set_param(&name, SAtom::setting(*v), None);
                        changed_params = true;
                    }
                    ConstructorOp::Input(param, node, out) => {
                        let id = node.node_id;
                        if !only_update_params {
                            node_config.set_edge(param, id, out);
                            need_rebuild = true;
                        }
                        walk_afterwads.push(node.clone());
                    }
                }
            }
        }

        if only_update_params && changed_params {
            if self.update_node_params(node.node_id)? {
                need_rebuild = true;
            }
        }

        for node in walk_afterwads.iter() {
            if self.walk_upload(&node, only_update_params)? {
                need_rebuild = true;
            }
        }

        Ok(need_rebuild)
    }

    fn update_node_params(&mut self, node_id: NodeId) -> Result<bool, SynthError> {
        let mut needs_graph_rebuild = false;

        if let Some(node_config) = self.nodes.get(&node_id) {
            for (param, (value, modamt)) in node_config.params.iter() {
                if let Some(param_id) = node_id.inp_param(&param) {
                    let mut changed_value = false;
                    let mut changed_modamt = false;
                    if let Some(old_val) = self.config.get_param(&param_id) {
                        if old_val != *value {
                            changed_value = true;
                        }
                    } else {
                        changed_value = true;
                    }

                    if !param_id.is_atom() {
                        let old_modamt = self.config.get_param_modamt(&param_id);
                        if old_modamt != *modamt {
                            changed_value = true;
                            changed_modamt = true;
                        }
                    }

                    if changed_value {
                        self.config.set_param(param_id, value.clone());
                        if changed_modamt && !param_id.is_atom() {
                            if self.config.set_param_modamt(param_id, *modamt) {
                                needs_graph_rebuild = true;
                            }
                        }
                    }
                } else {
                    return Err(SynthError::UnknownParam(node_id, param.to_string()));
                }
            }
        }

        Ok(needs_graph_rebuild)
    }

    pub fn update_params(&mut self, node: &dyn ConstructorNodeBuilder) -> Result<bool, SynthError> {
        let built_node = node.build();

        if self.walk_upload(&built_node, true)? {
            self.upload(node)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn upload(&mut self, node: &dyn ConstructorNodeBuilder) -> Result<(), SynthError> {
        let node = node.build();
        self.walk_upload(&node, false)?;

        self.graph_ordering.clear();

        for (node_id, node_conf) in self.nodes.iter() {
            self.graph_ordering.add_node(*node_id);

            for (_, (output_node_id, _)) in node_conf.edges.iter() {
                self.graph_ordering.add_edge(*node_id, *output_node_id);
            }
        }

        let mut ordered_nodes = vec![];
        if !self.graph_ordering.calculate_order(&mut ordered_nodes) {
            return Err(SynthError::CycleDetected);
        }

        for node_id in ordered_nodes.iter().rev() {
            if self.config.unique_index_for(node_id).is_none() {
                self.config.create_node(*node_id);
            }

            self.update_node_params(*node_id)?;
        }

        let mut prog = self.config.rebuild_node_ports();

        for node_id in ordered_nodes.iter().rev() {
            self.config.add_prog_node(&mut prog, node_id);
        }

        for node_id in ordered_nodes.iter().rev() {
            if let Some(node_config) = self.nodes.get(&node_id) {
                for (inp_param, (out_node_id, out_port)) in node_config.edges.iter() {
                    if let Some(idx) = node_id.inp(&inp_param) {
                        if let Some(out_idx) = out_node_id.out(&out_port) {
                            self.config.set_prog_node_exec_connection(
                                &mut prog,
                                (*node_id, idx),
                                (*out_node_id, out_idx),
                            );
                        } else {
                            return Err(SynthError::BadOutputName(
                                *out_node_id,
                                out_port.to_string(),
                            ));
                        }
                    } else {
                        return Err(SynthError::BadParamName(*node_id, inp_param.to_string()));
                    }
                }
            }
        }
        self.config.upload_prog(prog, true);

        Ok(())
    }
}
