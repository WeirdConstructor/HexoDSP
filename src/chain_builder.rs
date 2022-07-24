// Copyright (c) 2021-2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.
use crate::{Cell, CellDir, Matrix, NodeId, ParamId, SAtom};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct MatrixChainLink {
    cell: Cell,
    x: i32,
    y: i32,
    params: Vec<(ParamId, SAtom)>,
}

/// A DSP chain builder for the [hexodsp::Matrix].
///
/// This is an extremely easy API to create and place new DSP chains into the [hexodsp::Matrix].
/// It can be used by frontends to place DSP chains on user request or it can be used
/// by test cases to quickly fill the hexagonal Matrix.
///
///```
/// use hexodsp::*;
/// let mut chain = MatrixCellChain::new(CellDir::BR);
/// chain.node_out("sin")
///     .set_denorm("freq", 220.0)
///     .node_io("amp", "inp", "sig")
///     .set_denorm("att", 0.5)
///     .node_inp("out", "ch1");
///
/// // use crate::nodes::new_node_engine;
/// let (node_conf, _node_exec) = new_node_engine();
/// let mut matrix = Matrix::new(node_conf, 7, 7);
///
/// chain.place(&mut matrix, 2, 2).expect("no error in this case");
///```
#[derive(Clone)]
pub struct MatrixCellChain {
    chain: Vec<MatrixChainLink>,
    error: Option<ChainError>,
    dir: CellDir,
    pos: (i32, i32),
    param_idx: usize,
}

#[derive(Debug, Clone)]
pub enum ChainError {
    UnknownOutput(NodeId, String),
    UnknownInput(NodeId, String),
}

impl MatrixCellChain {
    /// Create a new [MatrixCellChain] with the given placement direction.
    ///
    /// The direction is used to guide the placement of the cells.
    pub fn new(dir: CellDir) -> Self {
        Self {
            dir,
            chain: vec![],
            error: None,
            pos: (0, 0),
            param_idx: 0,
        }
    }

    fn output_dir(&self) -> CellDir {
        if self.dir.is_output() {
            self.dir
        } else {
            self.dir.flip()
        }
    }

    fn input_dir(&self) -> CellDir {
        if self.dir.is_input() {
            self.dir
        } else {
            self.dir.flip()
        }
    }

    /// Sets the current parameter cell by chain index.
    pub fn params_for_idx(&mut self, idx: usize) -> &mut Self {
        self.param_idx = idx;
        if self.param_idx >= self.chain.len() {
            self.param_idx = self.chain.len();
        }

        self
    }

    /// Sets the denormalized value of the current parameter cell's parameter.
    ///
    /// The current parameter cell is set automatically when a new node is added.
    /// Alternatively you can use [MatrixCellChain::params_for_idx] to set the current
    /// parameter cell.
    pub fn set_denorm(&mut self, param: &str, denorm: f32) -> &mut Self {
        let link = self.chain.get_mut(self.param_idx).expect("Correct parameter idx");

        if let Some(pid) = link.cell.node_id().inp_param(param) {
            link.params.push((pid, SAtom::param(pid.norm(denorm as f32))));
        } else {
            self.error = Some(ChainError::UnknownInput(link.cell.node_id(), param.to_string()));
        }

        self
    }

    /// Sets the atom value of the current parameter cell's parameter.
    ///
    /// The current parameter cell is set automatically when a new node is added.
    /// Alternatively you can use [MatrixCellChain::params_for_idx] to set the current
    /// parameter cell.
    pub fn set_atom(&mut self, param: &str, at: SAtom) -> &mut Self {
        let link = self.chain.get_mut(self.param_idx).expect("Correct parameter idx");

        if let Some(pid) = link.cell.node_id().inp_param(param) {
            link.params.push((pid, at));
        } else {
            self.error = Some(ChainError::UnknownInput(link.cell.node_id(), param.to_string()));
        }

        self
    }

    /// Utility function for creating [hexodsp::Cell] for this chain.
    pub fn spawn_cell_from_node_id_name(&mut self, node_id: &str) -> Cell {
        let node_id = NodeId::from_str(node_id);

        Cell::empty(node_id)
    }

    /// Utility function to add a pre-built [hexodsp::Cell] as next link.
    ///
    /// This also sets the current parameter cell.
    pub fn add_link(&mut self, cell: Cell) {
        self.chain.push(MatrixChainLink { x: self.pos.0, y: self.pos.1, cell, params: vec![] });

        let offs = self.dir.as_offs(self.pos.0 as usize);
        self.pos.0 += offs.0;
        self.pos.1 += offs.1;

        self.param_idx = self.chain.len() - 1;
    }

    /// Place a new node in the chain with the given output assigned.
    pub fn node_out(&mut self, node_id: &str, out: &str) -> &mut Self {
        let mut cell = self.spawn_cell_from_node_id_name(node_id);

        if let Err(()) = cell.set_output_by_name(out, self.output_dir()) {
            self.error = Some(ChainError::UnknownOutput(cell.node_id(), out.to_string()));
        }

        self.add_link(cell);

        self
    }

    /// Place a new node in the chain with the given input assigned.
    pub fn node_inp(&mut self, node_id: &str, inp: &str) -> &mut Self {
        let mut cell = self.spawn_cell_from_node_id_name(node_id);

        if let Err(()) = cell.set_input_by_name(inp, self.input_dir()) {
            self.error = Some(ChainError::UnknownInput(cell.node_id(), inp.to_string()));
        }

        self.add_link(cell);

        self
    }

    /// Place a new node in the chain with the given input and output assigned.
    pub fn node_io(&mut self, node_id: &str, inp: &str, out: &str) -> &mut Self {
        let mut cell = self.spawn_cell_from_node_id_name(node_id);

        if let Err(()) = cell.set_input_by_name(inp, self.input_dir()) {
            self.error = Some(ChainError::UnknownInput(cell.node_id(), inp.to_string()));
        }

        if let Err(()) = cell.set_output_by_name(out, self.output_dir()) {
            self.error = Some(ChainError::UnknownOutput(cell.node_id(), out.to_string()));
        }

        self.add_link(cell);

        self
    }

    /// Places the chain into the matrix at the given position.
    ///
    /// If any error occured while building the chain (such as bad input/output names
    /// or unknown parameters), it will be returned here.
    pub fn place(&mut self, matrix: &mut Matrix, at_x: usize, at_y: usize) -> Result<(), ChainError> {
        if let Some(err) = self.error.take() {
            return Err(err);
        }

        let mut last_unused = HashMap::new();

        for link in self.chain.iter() {
            let (x, y) = (link.x, link.y);
            let x = (x + (at_x as i32)) as usize;
            let y = (y + (at_y as i32)) as usize;

            let mut cell = link.cell.clone();

            let node_id = cell.node_id();
            let node_name = node_id.name();

            let node_id = if let Some(i) = last_unused.get(node_name).cloned() {
                last_unused.insert(node_name.to_string(), i + 1);
                node_id.to_instance(i + 1)
            } else {
                let node_id = matrix.get_unused_instance_node_id(node_id);
                last_unused.insert(node_name.to_string(), node_id.instance());
                node_id
            };

            cell.set_node_id_keep_ios(node_id);

            println!("PLACE: ({},{}) {:?}", x, y, cell);

            matrix.place(x, y, cell);
        }

        for link in self.chain.iter() {
            for (pid, at) in link.params.iter() {
                matrix.set_param(*pid, at.clone());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_matrix_chain_builder_1() {
        use crate::nodes::new_node_engine;

        let (node_conf, _node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 7, 7);

        let mut chain = MatrixCellChain::new(CellDir::B);

        chain
            .node_out("sin", "sig")
            .set_denorm("freq", 220.0)
            .node_io("amp", "inp", "sig")
            .set_denorm("att", 0.5)
            .node_inp("out", "ch1");

        chain.params_for_idx(0).set_atom("det", SAtom::param(0.1));

        chain.place(&mut matrix, 2, 2).expect("no error in this case");

        matrix.sync().expect("Sync ok");

        let cell_sin = matrix.get(2, 2).unwrap();
        assert_eq!(cell_sin.node_id(), NodeId::Sin(0));

        let cell_amp = matrix.get(2, 3).unwrap();
        assert_eq!(cell_amp.node_id(), NodeId::Amp(0));

        let cell_out = matrix.get(2, 4).unwrap();
        assert_eq!(cell_out.node_id(), NodeId::Out(0));

        assert_eq!(
            format!("{:?}", matrix.get_param(&NodeId::Sin(0).inp_param("freq").unwrap()).unwrap()),
            "Param(-0.1)"
        );
        assert_eq!(
            format!("{:?}", matrix.get_param(&NodeId::Sin(0).inp_param("det").unwrap()).unwrap()),
            "Param(0.1)"
        );
        assert_eq!(
            format!("{:?}", matrix.get_param(&NodeId::Amp(0).inp_param("att").unwrap()).unwrap()),
            "Param(0.70710677)"
        );
    }
}
