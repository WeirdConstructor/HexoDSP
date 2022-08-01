// Copyright (c) 2021-2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::tracker::PatternData;
use crate::dsp::{NodeId, NodeInfo, ParamId, SAtom};
use crate::matrix_repr::*;
pub use crate::monitor::MON_SIG_CNT;
pub use crate::nodes::MinMaxMonitorSamples;
use crate::nodes::{NodeConfigurator, NodeGraphOrdering, NodeProg, MAX_ALLOCATED_NODES};
pub use crate::CellDir;
use crate::ScopeHandle;
use crate::blocklang::BlockFun;
use crate::block_compiler::BlkJITCompileError;

use std::collections::{HashMap, HashSet};

/// This is a cell/tile of the hexagonal [Matrix].
///
/// The [Matrix] stores it to keep track of the graphical representation
/// of the hexagonal tilemap. Using [Matrix::place] you can place new cells.
///
///```
/// use hexodsp::*;
///
/// let (node_conf, mut node_exec) = new_node_engine();
/// let mut matrix = Matrix::new(node_conf, 3, 3);
///
/// matrix.place(
///     2, 2,
///     Cell::empty(NodeId::Sin(0))
///     .input(Some(0), None, None)
///     .out(None, None, Some(0)));
///
/// matrix.sync().unwrap();
///```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cell {
    node_id: NodeId,
    x: u8,
    y: u8,
    /// Top-Right output
    out1: Option<u8>,
    /// Bottom-Right output
    out2: Option<u8>,
    /// Bottom output
    out3: Option<u8>,
    /// Top input
    in1: Option<u8>,
    /// Top-Left input
    in2: Option<u8>,
    /// Bottom-Left input
    in3: Option<u8>,
}

impl Cell {
    /// This is the main contructor of a [Cell].
    /// Empty means that there is no associated position of this cell
    /// and no inputs/outputs have been assigned. Use the methods [Cell::input] and [Cell::out]
    /// to assign inputs / outputs.
    ///
    ///```
    /// use hexodsp::*;
    ///
    /// let some_cell =
    ///     Cell::empty(NodeId::Sin(0))
    ///     .input(None, Some(0), Some(0))
    ///     .out(None, Some(0), Some(0));
    ///```
    pub fn empty(node_id: NodeId) -> Self {
        Self::empty_at(node_id, 0, 0)
    }

    /// This is an alternative constructor, in case you know the position of the
    /// cell before you got it from the Matrix.
    pub fn empty_at(node_id: NodeId, x: u8, y: u8) -> Self {
        Self { node_id, x, y, out1: None, out2: None, out3: None, in1: None, in2: None, in3: None }
    }

    /// Returns a serializable representation of this [Matrix] [Cell].
    ///
    /// See also [CellRepr].
    ///
    ///```
    /// use hexodsp::*;
    ///
    /// let some_cell =
    ///     Cell::empty(NodeId::Sin(0))
    ///     .input(None, Some(0), Some(0))
    ///     .out(None, Some(0), Some(0));
    ///
    /// let repr = some_cell.to_repr();
    /// assert_eq!(
    ///     repr.serialize().to_string(),
    ///     "[\"sin\",0,0,0,[-1,\"freq\",\"freq\"],[-1,\"sig\",\"sig\"]]");
    ///```
    pub fn to_repr(&self) -> CellRepr {
        CellRepr {
            node_id: self.node_id,
            x: self.x as usize,
            y: self.y as usize,
            out: [
                self.out1.map(|v| v as i16).unwrap_or(-1),
                self.out2.map(|v| v as i16).unwrap_or(-1),
                self.out3.map(|v| v as i16).unwrap_or(-1),
            ],
            inp: [
                self.in1.map(|v| v as i16).unwrap_or(-1),
                self.in2.map(|v| v as i16).unwrap_or(-1),
                self.in3.map(|v| v as i16).unwrap_or(-1),
            ],
        }
    }

    pub fn from_repr(repr: &CellRepr) -> Self {
        Self {
            node_id: repr.node_id,
            x: repr.x as u8,
            y: repr.y as u8,
            out1: if repr.out[0] < 0 { None } else { Some(repr.out[0] as u8) },
            out2: if repr.out[1] < 0 { None } else { Some(repr.out[1] as u8) },
            out3: if repr.out[2] < 0 { None } else { Some(repr.out[2] as u8) },
            in1: if repr.inp[0] < 0 { None } else { Some(repr.inp[0] as u8) },
            in2: if repr.inp[1] < 0 { None } else { Some(repr.inp[1] as u8) },
            in3: if repr.inp[2] < 0 { None } else { Some(repr.inp[2] as u8) },
        }
    }

    pub fn with_pos_of(&self, other: Cell) -> Self {
        let mut new = *self;
        new.x = other.x;
        new.y = other.y;
        new
    }

    pub fn is_empty(&self) -> bool {
        self.node_id == NodeId::Nop
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn set_node_id(&mut self, new_id: NodeId) {
        self.node_id = new_id;
        // With a new node id, we also need new I/Os:
        self.in1 = None;
        self.in2 = None;
        self.in3 = None;
        self.out1 = None;
        self.out2 = None;
        self.out3 = None;
    }

    pub fn set_node_id_keep_ios(&mut self, node_id: NodeId) {
        self.node_id = node_id;
    }

    pub fn label<'a>(&self, buf: &'a mut [u8]) -> Option<&'a str> {
        use std::io::Write;
        let mut cur = std::io::Cursor::new(buf);

        if self.node_id == NodeId::Nop {
            return None;
        }

        //        let node_info = infoh.from_node_id(self.node_id);

        match write!(cur, "{}", self.node_id) {
            Ok(_) => {
                let len = cur.position() as usize;
                Some(std::str::from_utf8(&(cur.into_inner())[0..len]).unwrap())
            }
            Err(_) => None,
        }
    }

    pub fn pos(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }

    pub fn offs_dir(&mut self, dir: CellDir) -> bool {
        if let Some(new_pos) = dir.offs_pos((self.x as usize, self.y as usize)) {
            self.x = new_pos.0 as u8;
            self.y = new_pos.1 as u8;
            true
        } else {
            false
        }
    }

    pub fn has_dir_set(&self, dir: CellDir) -> bool {
        match dir {
            CellDir::TR => self.out1.is_some(),
            CellDir::BR => self.out2.is_some(),
            CellDir::B => self.out3.is_some(),
            CellDir::BL => self.in3.is_some(),
            CellDir::TL => self.in2.is_some(),
            CellDir::T => self.in1.is_some(),
            CellDir::C => false,
        }
    }

    pub fn local_port_idx(&self, dir: CellDir) -> Option<u8> {
        match dir {
            CellDir::TR => self.out1,
            CellDir::BR => self.out2,
            CellDir::B => self.out3,
            CellDir::BL => self.in3,
            CellDir::TL => self.in2,
            CellDir::T => self.in1,
            CellDir::C => None,
        }
    }

    pub fn clear_io_dir(&mut self, dir: CellDir) {
        match dir {
            CellDir::TR => {
                self.out1 = None;
            }
            CellDir::BR => {
                self.out2 = None;
            }
            CellDir::B => {
                self.out3 = None;
            }
            CellDir::BL => {
                self.in3 = None;
            }
            CellDir::TL => {
                self.in2 = None;
            }
            CellDir::T => {
                self.in1 = None;
            }
            CellDir::C => {
                self.out1 = None;
                self.out2 = None;
                self.out3 = None;
                self.in1 = None;
                self.in2 = None;
                self.in3 = None;
            }
        }
    }

    pub fn set_io_dir(&mut self, dir: CellDir, idx: usize) {
        match dir {
            CellDir::TR => {
                self.out1 = Some(idx as u8);
            }
            CellDir::BR => {
                self.out2 = Some(idx as u8);
            }
            CellDir::B => {
                self.out3 = Some(idx as u8);
            }
            CellDir::BL => {
                self.in3 = Some(idx as u8);
            }
            CellDir::TL => {
                self.in2 = Some(idx as u8);
            }
            CellDir::T => {
                self.in1 = Some(idx as u8);
            }
            CellDir::C => {}
        }
    }

    /// This is a helper function to quickly set an input by name and direction.
    ///
    ///```
    /// use hexodsp::*;
    ///
    /// let mut cell = Cell::empty(NodeId::Sin(0));
    /// cell.set_input_by_name("freq", CellDir::T).unwrap();
    ///```
    pub fn set_input_by_name(&mut self, name: &str, dir: CellDir) -> Result<(), ()> {
        if let Some(idx) = self.node_id.inp(name) {
            self.set_io_dir(dir, idx as usize);
            Ok(())
        } else {
            Err(())
        }
    }

    /// This is a helper function to quickly set an output by name and direction.
    ///
    ///```
    /// use hexodsp::*;
    ///
    /// let mut cell = Cell::empty(NodeId::Sin(0));
    /// cell.set_output_by_name("sig", CellDir::B).unwrap();
    ///```
    pub fn set_output_by_name(&mut self, name: &str, dir: CellDir) -> Result<(), ()> {
        if let Some(idx) = self.node_id.out(name) {
            self.set_io_dir(dir, idx as usize);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn input(mut self, i1: Option<u8>, i2: Option<u8>, i3: Option<u8>) -> Self {
        self.in1 = i1;
        self.in2 = i2;
        self.in3 = i3;
        self
    }

    pub fn out(mut self, o1: Option<u8>, o2: Option<u8>, o3: Option<u8>) -> Self {
        self.out1 = o1;
        self.out2 = o2;
        self.out3 = o3;
        self
    }

    /// Finds the first free input or output (one without an adjacent cell). If any free input/output
    /// has an assigned input, that edge is returned before any else.
    /// With `dir` you can specify input with `CellDir::T`, output with `CellDir::B`
    /// and any with `CellDir::C`.
    pub fn find_first_adjacent_free(
        &self,
        m: &Matrix,
        dir: CellDir,
    ) -> Option<(CellDir, Option<u8>)> {
        let mut free_ports = vec![];

        let options: &[CellDir] = if dir == CellDir::C {
            &[CellDir::T, CellDir::TL, CellDir::BL, CellDir::TR, CellDir::BR, CellDir::B]
        } else if dir.is_input() {
            &[CellDir::T, CellDir::TL, CellDir::BL]
        } else {
            &[CellDir::TR, CellDir::BR, CellDir::B]
        };

        for dir in options {
            if let Some(pos) = dir.offs_pos((self.x as usize, self.y as usize)) {
                if m.get(pos.0, pos.1).map(|c| c.is_empty()).unwrap_or(false) {
                    free_ports.push(dir);
                }
            }
        }

        for in_dir in &free_ports {
            if self.has_dir_set(**in_dir) {
                return Some((**in_dir, self.local_port_idx(**in_dir)));
            }
        }

        if free_ports.len() > 0 {
            Some((*free_ports[0], None))
        } else {
            None
        }
    }

    /// Finds the all adjacent free places around the current cell.
    /// With `dir` you can specify input with `CellDir::T`, output with `CellDir::B`
    /// and any with `CellDir::C`.
    pub fn find_all_adjacent_free(
        &self,
        m: &Matrix,
        dir: CellDir,
    ) -> Vec<(CellDir, (usize, usize))> {
        let mut free_ports = vec![];

        let options: &[CellDir] = if dir == CellDir::C {
            &[CellDir::T, CellDir::TL, CellDir::BL, CellDir::TR, CellDir::BR, CellDir::B]
        } else if dir.is_input() {
            &[CellDir::T, CellDir::TL, CellDir::BL]
        } else {
            &[CellDir::TR, CellDir::BR, CellDir::B]
        };

        for dir in options {
            if let Some(pos) = dir.offs_pos((self.x as usize, self.y as usize)) {
                if m.get(pos.0, pos.1).map(|c| c.is_empty()).unwrap_or(false) {
                    free_ports.push((*dir, pos));
                }
            }
        }

        free_ports
    }

    /// Finds all dangling ports in the specified direction.
    /// With `dir` you can specify input with `CellDir::T`, output with `CellDir::B`
    /// and any with `CellDir::C`.
    pub fn find_unconnected_ports(&self, m: &Matrix, dir: CellDir) -> Vec<CellDir> {
        let mut unused_ports = vec![];

        let options: &[CellDir] = if dir == CellDir::C {
            &[CellDir::T, CellDir::TL, CellDir::BL, CellDir::TR, CellDir::BR, CellDir::B]
        } else if dir.is_input() {
            &[CellDir::T, CellDir::TL, CellDir::BL]
        } else {
            &[CellDir::TR, CellDir::BR, CellDir::B]
        };

        for dir in options {
            if self.is_port_dir_connected(m, *dir).is_none() {
                unused_ports.push(*dir);
            }
        }

        unused_ports
    }

    /// If the port is connected, it will return the position of the other cell.
    pub fn is_port_dir_connected(&self, m: &Matrix, dir: CellDir) -> Option<(usize, usize)> {
        if self.has_dir_set(dir) {
            if let Some(new_pos) = dir.offs_pos((self.x as usize, self.y as usize)) {
                if let Some(dst_cell) = m.get(new_pos.0, new_pos.1) {
                    if dst_cell.has_dir_set(dir.flip()) {
                        return Some(new_pos);
                    }
                }
            }
        }

        None
    }
}

use std::sync::{Arc, Mutex};

/// To report back cycle errors from [Matrix::check] and [Matrix::sync].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MatrixError {
    CycleDetected,
    DuplicatedInput { output1: (NodeId, u8), output2: (NodeId, u8) },
    NonEmptyCell { cell: Cell },
    PosOutOfRange,
}

/// An intermediate data structure to store a single edge in the [Matrix].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    from: NodeId,
    from_out: u8,
    to: NodeId,
    to_input: u8,
}

/// This trait can be passed into [Matrix] as trait object
/// to get feedback when things change.
pub trait MatrixObserver {
    /// Called when a property is changing eg. via [Matrix::set_prop]
    /// or some other yet unknown method.
    /// Not called, when [MatrixObserver::update_all] tells you that
    /// everything has changed.
    fn update_prop(&self, key: &str);
    /// Called when a new cell is monitored via [Matrix::monitor_cell].
    /// Not called, when [MatrixObserver::update_all] tells you that
    /// everything has changed.
    fn update_monitor(&self, cell: &Cell);
    /// Called when a parameter or it's modulation amount is changing.
    /// Not called, when [MatrixObserver::update_all] tells you that
    /// everything has changed.
    fn update_param(&self, param_id: &ParamId);
    /// Called when the matrix graph was changed, usually called
    /// when [Matrix::sync] is called.
    /// Usually also called when [MatrixObserver::update_all] was called.
    fn update_matrix(&self);
    /// Called when the complete matrix has been changing.
    /// The called then needs up update all it's internal state it knows
    /// about [Matrix].
    fn update_all(&self);
}

pub struct Matrix {
    /// The node configurator to control the backend.
    config: NodeConfigurator,
    /// Holds the actual 2 dimensional matrix cells in one big vector.
    matrix: Vec<Cell>,
    /// Width of the matrix.
    w: usize,
    /// Height of the matrix.
    h: usize,

    /// The retained data structure of the graph topology.
    /// This is used by `sync()` and `check()` to determine the
    /// order and cycle freeness of the graph.
    /// We store it in this field, so we don't have to reallocate it
    /// all the time.
    graph_ordering: NodeGraphOrdering,

    /// Holds a saved version of the `matrix` field
    /// to roll back changes that might introduce cycles or
    /// other invalid topology.
    saved_matrix: Option<Vec<Cell>>,

    /// Stores the edges which are extracted from the `matrix` field
    /// by [Matrix::update_graph_ordering_and_edges], which is used
    /// by [Matrix::sync] and [Matrix::check].
    edges: Vec<Edge>,

    /// Holds custom user defined properties. They are saved with
    /// the [MatrixRepr] and you can set and retrieve these properties
    /// using [Matrix::set_prop] and [Matrix::get_prop].
    properties: HashMap<String, SAtom>,

    /// Stores the [crate::dsp::ParamId] of the inputs that have an output
    /// assigned to them. It's updates when [Matrix::edges] is updated and used
    /// by [Matrix::param_input_is_used] to return whether a parameter is
    /// controlled by some output port.
    assigned_inputs: HashSet<ParamId>,

    /// Holds the currently monitored cell.
    monitored_cell: Cell,

    /// A counter that increases for each sync(), it can be used
    /// by other components of the application to detect changes in
    /// the matrix to resync their own data.
    gen_counter: usize,

    /// A trait object that tracks changed on the [Matrix].
    observer: Option<Arc<dyn MatrixObserver>>,
}

unsafe impl Send for Matrix {}

impl Matrix {
    pub fn new(config: NodeConfigurator, w: usize, h: usize) -> Self {
        let mut matrix: Vec<Cell> = Vec::new();
        matrix.resize(w * h, Cell::empty(NodeId::Nop));

        Self {
            monitored_cell: Cell::empty(NodeId::Nop),
            gen_counter: 0,
            saved_matrix: None,
            graph_ordering: NodeGraphOrdering::new(),
            edges: Vec::with_capacity(MAX_ALLOCATED_NODES * 2),
            assigned_inputs: HashSet::new(),
            properties: HashMap::new(),
            observer: None,
            config,
            w,
            h,
            matrix,
        }
    }

    /// Assigns the [MatrixObserver] to observe changes on the [Matrix].
    pub fn set_observer(&mut self, obs: Arc<dyn MatrixObserver>) {
        self.observer = Some(obs);
    }

    pub fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    pub fn unique_index_for(&self, node_id: &NodeId) -> Option<usize> {
        self.config.unique_index_for(node_id)
    }

    pub fn info_for(&self, node_id: &NodeId) -> Option<NodeInfo> {
        Some(self.config.node_by_id(&node_id)?.0.clone())
    }

    pub fn phase_value_for(&self, node_id: &NodeId) -> f32 {
        self.config.phase_value_for(node_id)
    }

    pub fn led_value_for(&self, node_id: &NodeId) -> f32 {
        self.config.led_value_for(node_id)
    }

    pub fn update_filters(&mut self) {
        self.config.update_filters();
    }

    pub fn filtered_led_for(&mut self, ni: &NodeId) -> (f32, f32) {
        self.config.filtered_led_for(ni)
    }

    pub fn filtered_out_fb_for(&mut self, ni: &NodeId, out: u8) -> (f32, f32) {
        self.config.filtered_out_fb_for(ni, out)
    }

    /// Retrieve the oscilloscope handle for the scope index `scope`.
    pub fn get_pattern_data(&self, tracker_id: usize) -> Option<Arc<Mutex<PatternData>>> {
        self.config.get_pattern_data(tracker_id)
    }

    /// Retrieve a handle to the tracker pattern data of the tracker `tracker_id`.
    pub fn get_scope_handle(&self, scope: usize) -> Option<Arc<ScopeHandle>> {
        self.config.get_scope_handle(scope)
    }

    /// Checks if there are any updates to send for the pattern data that belongs to the
    /// tracker `tracker_id`. Call this repeatedly, eg. once per frame in a GUI, in case the user
    /// modified the pattern data. It will make sure that the modifications are sent to the
    /// audio thread.
    pub fn check_pattern_data(&mut self, tracker_id: usize) {
        self.config.check_pattern_data(tracker_id)
    }

    /// Checks the block function for the id `id`. If the block function did change,
    /// updates are then sent to the audio thread.
    /// See also [get_block_function].
    pub fn check_block_function(&mut self, id: usize) -> Result<(), BlkJITCompileError> {
        self.config.check_block_function(id)
    }

    /// Retrieve a handle to the block function `id`. In case you modify the block function,
    /// make sure to call [check_block_function].
    pub fn get_block_function(&mut self, id: usize) -> Option<Arc<Mutex<BlockFun>>> {
        self.config.get_block_function(id)
    }

    /// Saves the state of the hexagonal grid layout.
    /// This is usually used together with [Matrix::check]
    /// and [Matrix::restore_matrix] to try if changes on
    /// the matrix using [Matrix::place] (or other grid changing
    /// functions).
    ///
    /// It is advised to use convenience functions such as [Matrix::change_matrix].
    ///
    /// See also [Matrix::change_matrix], [Matrix::check] and [Matrix::sync].
    pub fn save_matrix(&mut self) {
        let matrix = self.matrix.clone();
        self.saved_matrix = Some(matrix);
    }

    /// Restores the previously via [Matrix::save_matrix] saved matrix.
    ///
    /// It is advised to use convenience functions such as [Matrix::change_matrix].
    ///
    /// See also [Matrix::change_matrix], [Matrix::check].
    pub fn restore_matrix(&mut self) {
        if let Some(matrix) = self.saved_matrix.take() {
            self.matrix = matrix;
        }
    }

    /// Helps encapsulating changes of the matrix and wraps them into
    /// a [Matrix::save_matrix], [Matrix::check] and [Matrix::restore_matrix].
    ///
    ///```
    /// use hexodsp::*;
    ///
    /// let (node_conf, mut node_exec) = new_node_engine();
    /// let mut matrix = Matrix::new(node_conf, 3, 3);
    ///
    /// let res = matrix.change_matrix(|matrix| {
    ///     matrix.place(0, 1,
    ///         Cell::empty(NodeId::Sin(1))
    ///         .input(Some(0), None, None));
    ///     matrix.place(0, 0,
    ///         Cell::empty(NodeId::Sin(1))
    ///         .out(None, None, Some(0)));
    /// });
    ///
    /// // In this examples case there is an error, as we created
    /// // a cycle:
    /// assert!(res.is_err());
    ///```
    pub fn change_matrix<F>(&mut self, mut f: F) -> Result<(), MatrixError>
    where
        F: FnMut(&mut Self),
    {
        self.save_matrix();

        f(self);

        if let Err(e) = self.check() {
            self.restore_matrix();
            Err(e)
        } else {
            Ok(())
        }
    }

    /// Like [Matrix::change_matrix] but the function passed to this
    /// needs to return a `Result<(), MatrixError>`.
    pub fn change_matrix_err<F>(&mut self, mut f: F) -> Result<(), MatrixError>
    where
        F: FnMut(&mut Self) -> Result<(), MatrixError>,
    {
        self.save_matrix();

        if let Err(e) = f(self) {
            self.restore_matrix();
            return Err(e);
        }

        if let Err(e) = self.check() {
            self.restore_matrix();
            Err(e)
        } else {
            Ok(())
        }
    }

    /// Tries to place all `cells` at once, if they are placed in empty
    /// cells only! Returns an error of the destination cell is not empty
    /// or out of range, or if the placement of the cluster results in any
    /// other inconsistencies.
    ///
    /// This action must be wrapped with [Matrix::change_matrix_err]!
    ///
    /// Restores the matrix to the previous state if placing fails.
    pub fn place_multiple(&mut self, cells: &[Cell]) -> Result<(), MatrixError> {
        for cell in cells {
            let x = cell.pos().0;
            let y = cell.pos().1;

            if let Some(existing) = self.get(x, y) {
                if !existing.is_empty() {
                    return Err(MatrixError::NonEmptyCell { cell: *existing });
                }

                self.place(x, y, *cell);
            } else {
                return Err(MatrixError::PosOutOfRange);
            }
        }

        if let Err(e) = self.check() {
            Err(e)
        } else {
            Ok(())
        }
    }

    /// Inserts a cell into the hexagonal grid of the matrix.
    /// You have to make sure that the resulting DSP graph topology
    /// does not have cycles, otherwise an upload to the DSP thread via
    /// [Matrix::sync] will fail.
    ///
    /// If you try to place a cell outside the grid, it will not be placed
    /// and just silently ignored.
    ///
    /// You can safely check the DSP topology of changes using
    /// the convenience function [Matrix::change_matrix]
    /// or alternatively: [Matrix::save_matrix], [Matrix::restore_matrix]
    /// and [Matrix::check].
    ///
    /// See also the example in [Matrix::change_matrix] and [Matrix::check].
    pub fn place(&mut self, x: usize, y: usize, mut cell: Cell) {
        cell.x = x as u8;
        cell.y = y as u8;

        if x >= self.w || y >= self.h {
            return;
        }

        self.matrix[x * self.h + y] = cell;
    }

    /// Set the cell at it's assigned position. This is basically a shorthand
    /// for [Matrix::place]. As if you would call:
    /// `m.place(cell.pos().0, cell.pos().1, cell)`.
    pub fn place_cell(&mut self, cell: Cell) {
        self.place(cell.pos().0, cell.pos().1, cell);
    }

    /// Clears the contents of the matrix. It's completely empty after this.
    pub fn clear(&mut self) {
        for cell in self.matrix.iter_mut() {
            *cell = Cell::empty(NodeId::Nop);
        }

        self.graph_ordering.clear();
        self.edges.clear();
        self.assigned_inputs.clear();
        self.saved_matrix = None;
        self.properties.clear();

        self.config.delete_nodes();
        self.monitor_cell(Cell::empty(NodeId::Nop));
        let _ = self.sync();

        if let Some(obs) = &self.observer {
            obs.update_all();
        }
    }

    /// Iterates through all atoms. This is useful for reading
    /// all the atoms after a [MatrixRepr] has been loaded with [Matrix::from_repr].
    pub fn for_each_atom<F: FnMut(usize, ParamId, &SAtom, Option<f32>)>(&self, f: F) {
        self.config.for_each_param(f);
    }

    /// Returns the DSP graph generation, which is increased
    /// after each call to [Matrix::sync].
    ///
    /// This can be used by external components to track if they
    /// should update their knowledge of the nodes in the DSP
    /// graph. Such as parameter values.
    ///
    /// HexoSynth for instance updates the UI by tracking this value.
    pub fn get_generation(&self) -> usize {
        self.gen_counter
    }

    /// Returns a serializable representation of the matrix.
    /// This representation contains all parameters,
    /// created nodes, connections and the tracker's pattern data.
    ///
    ///```
    /// use hexodsp::*;
    ///
    /// let (node_conf, mut _node_exec) = new_node_engine();
    /// let mut matrix = Matrix::new(node_conf, 3, 3);
    ///
    /// let sin = NodeId::Sin(2);
    ///
    /// matrix.place(0, 0,
    ///     Cell::empty(sin)
    ///     .out(None, Some(0), None));
    ///
    /// let freq_param = sin.inp_param("freq").unwrap();
    /// matrix.set_param(freq_param, SAtom::param(-0.1));
    ///
    /// let mut serialized = matrix.to_repr().serialize().to_string();
    ///
    /// assert!(serialized.find("\"sin\",2,0,0,[-1,-1,-1],[-1,\"sig\",-1]").is_some());
    /// assert!(serialized.find("\"freq\",220.0").is_some());
    ///```
    ///
    /// See also [MatrixRepr::serialize].
    pub fn to_repr(&self) -> MatrixRepr {
        let (params, atoms) = self.config.dump_param_values();

        let mut cells: Vec<CellRepr> = vec![];
        self.for_each(|_x, _y, cell| {
            if cell.node_id() != NodeId::Nop {
                cells.push(cell.to_repr())
            }
        });

        let mut patterns: Vec<Option<PatternRepr>> = vec![];
        let mut tracker_id = 0;
        while let Some(pdata) = self.get_pattern_data(tracker_id) {
            patterns.push(if pdata.lock().unwrap().is_unset() {
                None
            } else {
                Some(pdata.lock().unwrap().to_repr())
            });

            tracker_id += 1;
        }

        let properties = self.properties.iter().map(|(k, v)| (k.to_string(), v.clone())).collect();

        MatrixRepr { cells, params, atoms, patterns, properties, version: 2 }
    }

    /// Loads the matrix from a previously my [Matrix::to_repr]
    /// generated matrix representation.
    ///
    /// This function will call [Matrix::sync] after loading and
    /// overwriting the current matrix contents.
    pub fn from_repr(&mut self, repr: &MatrixRepr) -> Result<(), MatrixError> {
        self.clear();

        let normalize_params = repr.version > 1;

        self.config.load_dumped_param_values(&repr.params[..], &repr.atoms[..], normalize_params);

        for (key, val) in repr.properties.iter() {
            self.properties.insert(key.to_string(), val.clone());
        }

        for cell_repr in repr.cells.iter() {
            let cell = Cell::from_repr(cell_repr);
            self.place(cell.x as usize, cell.y as usize, cell);
        }

        for (tracker_id, pat) in repr.patterns.iter().enumerate() {
            if let Some(pat) = pat {
                if let Some(pd) = self.get_pattern_data(tracker_id) {
                    pd.lock().unwrap().from_repr(pat);
                }
            }
        }

        let ret = self.sync();

        if let Some(obs) = &self.observer {
            obs.update_all();
        }

        ret
    }

    /// Saves a property in the matrix, these can be retrieved
    /// using [Matrix::get_prop] and are saved/loaded along with
    /// the [MatrixRepr]. See also [Matrix::to_repr] and [Matrix::from_repr].
    ///
    ///```
    /// use hexodsp::*;
    ///
    /// let repr = {
    ///     let (node_conf, mut _node_exec) = new_node_engine();
    ///     let mut matrix = Matrix::new(node_conf, 3, 3);
    ///
    ///     matrix.set_prop("test", SAtom::setting(31337));
    ///
    ///     matrix.to_repr()
    /// };
    ///
    /// let (node_conf, mut _node_exec) = new_node_engine();
    /// let mut matrix2 = Matrix::new(node_conf, 3, 3);
    ///
    /// matrix2.from_repr(&repr).unwrap();
    /// assert_eq!(matrix2.get_prop("test").unwrap().i(), 31337);
    ///```
    pub fn set_prop(&mut self, key: &str, val: SAtom) {
        self.gen_counter += 1;
        self.properties.insert(key.to_string(), val);
        if let Some(obs) = &self.observer {
            obs.update_prop(key);
        }
    }

    /// Retrieves a matrix property. See also [Matrix::set_prop] for an
    /// example and more information.
    pub fn get_prop(&mut self, key: &str) -> Option<&SAtom> {
        self.properties.get(key)
    }

    /// Receives the most recent data for the monitored signal at index `idx`.
    /// Might introduce a short wait, because internally a mutex is still locked.
    /// If this leads to stuttering in the UI, we need to change the internal
    /// handling to a triple buffer.
    pub fn get_minmax_monitor_samples(&mut self, idx: usize) -> &MinMaxMonitorSamples {
        self.config.get_minmax_monitor_samples(idx)
    }

    /// Returns the currently monitored cell.
    pub fn monitored_cell(&self) -> &Cell {
        &self.monitored_cell
    }

    /// Sets the cell to monitor next. Please bear in mind, that you need to
    /// call `sync` before retrieving the cell from the matrix, otherwise
    /// the node instance might not have been created in the backend yet and
    /// we can not start monitoring the cell.
    pub fn monitor_cell(&mut self, cell: Cell) {
        self.monitored_cell = cell;

        let inputs = [cell.in1, cell.in2, cell.in3];
        let outputs = [cell.out1, cell.out2, cell.out3];

        self.config.monitor(&cell.node_id, &inputs, &outputs);

        if let Some(obs) = &self.observer {
            obs.update_monitor(&self.monitored_cell);
        }
    }

    /// Is called by [Matrix::sync] to refresh the monitored cell.
    /// In case the matrix has changed (inputs/outputs of a cell)
    /// we show the current state.
    ///
    /// Note, that if the UI actually moved a cell, it needs to
    /// monitor the newly moved cell anyways.
    fn remonitor_cell(&mut self) {
        let m = self.monitored_cell();
        if let Some(cell) = self.get(m.x as usize, m.y as usize).copied() {
            self.monitor_cell(cell);
        }
    }

    pub fn pop_error(&mut self) -> Option<String> {
        self.config.pop_error()
    }

    /// Retrieve [SAtom] values for input parameters and atoms.
    pub fn get_param(&self, param: &ParamId) -> Option<SAtom> {
        self.config.get_param(param)
    }

    /// Assign [SAtom] values to input parameters and atoms.
    pub fn set_param(&mut self, param: ParamId, at: SAtom) {
        self.config.set_param(param.clone(), at);
        self.gen_counter += 1;
        if let Some(obs) = &self.observer {
            obs.update_param(&param);
        }
    }

    /// Retrieve the modulation amount of the input parameter.
    pub fn get_param_modamt(&self, param: &ParamId) -> Option<f32> {
        self.config.get_param_modamt(param)
    }

    /// Assign or remove modulation of an input parameter.
    pub fn set_param_modamt(
        &mut self,
        param: ParamId,
        modamt: Option<f32>,
    ) -> Result<(), MatrixError> {
        if self.config.set_param_modamt(param.clone(), modamt) {
            if let Some(obs) = &self.observer {
                obs.update_param(&param);
            }

            // XXX: Remove the observer from the matrix, so the sync() does not
            //      generate a matrix graph update! There is no structural change!
            let obs = self.observer.take();
            // XXX: sync implicitly increases gen_counter!
            let ret = self.sync();
            self.observer = obs;
            ret
        } else {
            self.gen_counter += 1;
            Ok(())
        }
    }

    pub fn get_adjacent_output(&self, x: usize, y: usize, dir: CellDir) -> Option<(NodeId, u8)> {
        if dir.is_output() {
            return None;
        }

        let cell = self.get_adjacent(x, y, dir)?;

        if cell.node_id == NodeId::Nop {
            return None;
        }

        let cell_out = match dir {
            CellDir::T => cell.out3?,
            CellDir::TL => cell.out2?,
            CellDir::BL => cell.out1?,
            _ => {
                return None;
            }
        };

        Some((cell.node_id, cell_out))
    }

    pub fn get_adjacent(&self, x: usize, y: usize, dir: CellDir) -> Option<&Cell> {
        let offs: (i32, i32) = dir.as_offs(x);
        let x = x as i32 + offs.0;
        let y = y as i32 + offs.1;

        if x < 0 || y < 0 || (x as usize) >= self.w || (y as usize) >= self.h {
            return None;
        }

        Some(&self.matrix[(x as usize) * self.h + (y as usize)])
    }

    pub fn adjacent_edge_has_input(&self, x: usize, y: usize, edge: CellDir) -> bool {
        if let Some(cell) = self.get_adjacent(x, y, edge) {
            //d// println!("       ADJ CELL: {},{} ({})", cell.x, cell.y, cell.node_id());
            match edge {
                CellDir::TR => cell.in3.is_some(),
                CellDir::BR => cell.in2.is_some(),
                CellDir::B => cell.in1.is_some(),
                _ => false,
            }
        } else {
            false
        }
    }

    /// Retrieves the immediate connections to adjacent cells and returns a list.
    /// Returns none if there is no cell at the given position.
    ///
    /// Returns a vector with pairs of this content:
    ///
    ///    (
    ///        (center_cell, center_connection_dir, center_node_io_index),
    ///        (
    ///            other_cell,
    ///            other_connection_dir,
    ///            other__node_io_index,
    ///            (other_cell_x, other_cell_y)
    ///        )
    ///    )
    pub fn get_connections(
        &self,
        x: usize,
        y: usize,
    ) -> Option<Vec<((Cell, CellDir, u8), (Cell, CellDir, u8, (usize, usize)))>> {
        let this_cell = self.get(x, y)?;

        let mut ret = vec![];

        for edge in 0..6 {
            let dir = CellDir::from(edge);

            if let Some(node_io_idx) = this_cell.local_port_idx(dir) {
                if let Some((nx, ny)) = dir.offs_pos((x, y)) {
                    if !(nx < self.w && ny < self.h) {
                        continue;
                    }

                    if let Some(other_cell) = self.get(nx, ny) {
                        if let Some(other_node_io_idx) = other_cell.local_port_idx(dir.flip()) {
                            ret.push((
                                (*this_cell, dir, node_io_idx),
                                (*other_cell, dir.flip(), other_node_io_idx, (nx, ny)),
                            ));
                        }
                    }
                }
            }
        }

        Some(ret)
    }

    pub fn for_each<F: FnMut(usize, usize, &Cell)>(&self, mut f: F) {
        for x in 0..self.w {
            for y in 0..self.h {
                let cell = &self.matrix[x * self.h + y];
                f(x, y, cell);
            }
        }
    }

    pub fn edge_label<'a>(
        &self,
        cell: &Cell,
        edge: CellDir,
        buf: &'a mut [u8],
    ) -> Option<(&'a str, bool)> {
        use std::io::Write;
        let mut cur = std::io::Cursor::new(buf);

        if cell.node_id == NodeId::Nop {
            return None;
        }

        let out_idx = match edge {
            CellDir::TR => Some(cell.out1),
            CellDir::BR => Some(cell.out2),
            CellDir::B => Some(cell.out3),
            _ => None,
        };
        let in_idx = match edge {
            CellDir::BL => Some(cell.in3),
            CellDir::TL => Some(cell.in2),
            CellDir::T => Some(cell.in1),
            _ => None,
        };

        let info = self.info_for(&cell.node_id)?;

        let mut is_connected_edge = false;

        let edge_str = if let Some(out_idx) = out_idx {
            //d// println!("    CHECK ADJ EDGE {},{} @ {:?}", cell.x, cell.y, edge);
            is_connected_edge =
                self.adjacent_edge_has_input(cell.x as usize, cell.y as usize, edge);

            info.out_name(out_idx? as usize)
        } else if let Some(in_idx) = in_idx {
            info.in_name(in_idx? as usize)
        } else {
            None
        };

        let edge_str = edge_str?;

        match write!(cur, "{}", edge_str) {
            Ok(_) => {
                let len = cur.position() as usize;
                Some((std::str::from_utf8(&(cur.into_inner())[0..len]).unwrap(), is_connected_edge))
            }
            Err(_) => None,
        }
    }

    pub fn get_copy(&self, x: usize, y: usize) -> Option<Cell> {
        if x >= self.w || y >= self.h {
            return None;
        }

        let mut cell = self.matrix[x * self.h + y];
        cell.x = x as u8;
        cell.y = y as u8;
        Some(cell)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= self.w || y >= self.h {
            return None;
        }

        Some(&self.matrix[x * self.h + y])
    }

    pub fn param_input_is_used(&self, p: ParamId) -> bool {
        self.assigned_inputs.contains(&p)
    }

    pub fn get_unused_instance_node_id(&self, id: NodeId) -> NodeId {
        self.config.unused_instance_node_id(id)
    }

    fn create_intermediate_nodes(&mut self) {
        // Scan through the matrix and check if (backend) nodes need to be created
        // for new unknown nodes:
        for x in 0..self.w {
            for y in 0..self.h {
                let cell = &mut self.matrix[x * self.h + y];

                if cell.node_id == NodeId::Nop {
                    continue;
                }

                // - check if each NodeId has a corresponding entry in NodeConfigurator
                //   - if not, create a new one on the fly
                if self.config.unique_index_for(&cell.node_id).is_none() {
                    // - check if the previous node exist, if not,
                    //   create them on the fly now:
                    for inst in 0..cell.node_id.instance() {
                        let new_hole_filler_node_id = cell.node_id.to_instance(inst);

                        if self.config.unique_index_for(&new_hole_filler_node_id).is_none() {
                            self.config
                                .create_node(new_hole_filler_node_id)
                                .expect("NodeInfo existent in Matrix");
                        }
                    }

                    self.config.create_node(cell.node_id).expect("NodeInfo existent in Matrix");
                }
            }
        }
    }

    fn update_graph_ordering_and_edges(&mut self) {
        self.graph_ordering.clear();
        self.edges.clear();
        self.assigned_inputs.clear();

        for x in 0..self.w {
            for y in 0..self.h {
                let cell = self.matrix[x * self.h + y];
                if cell.node_id == NodeId::Nop {
                    continue;
                }

                self.graph_ordering.add_node(cell.node_id);

                let in1_output = self.get_adjacent_output(x, y, CellDir::T);
                let in2_output = self.get_adjacent_output(x, y, CellDir::TL);
                let in3_output = self.get_adjacent_output(x, y, CellDir::BL);

                match (cell.in1, in1_output) {
                    (Some(in1_idx), Some(in1_output)) => {
                        self.edges.push(Edge {
                            to: cell.node_id,
                            to_input: in1_idx,
                            from: in1_output.0,
                            from_out: in1_output.1,
                        });
                        self.graph_ordering.add_edge(in1_output.0, cell.node_id);
                    }
                    _ => {}
                }

                match (cell.in2, in2_output) {
                    (Some(in2_idx), Some(in2_output)) => {
                        self.edges.push(Edge {
                            to: cell.node_id,
                            to_input: in2_idx,
                            from: in2_output.0,
                            from_out: in2_output.1,
                        });
                        self.graph_ordering.add_edge(in2_output.0, cell.node_id);
                    }
                    _ => {}
                }

                match (cell.in3, in3_output) {
                    (Some(in3_idx), Some(in3_output)) => {
                        self.edges.push(Edge {
                            to: cell.node_id,
                            to_input: in3_idx,
                            from: in3_output.0,
                            from_out: in3_output.1,
                        });
                        self.graph_ordering.add_edge(in3_output.0, cell.node_id);
                    }
                    _ => {}
                }
            }
        }

        for edge in self.edges.iter() {
            if let Some(pid) = edge.to.param_by_idx(edge.to_input as usize) {
                self.assigned_inputs.insert(pid);
            }
        }
    }

    /// Compiles a [NodeProg] from the data collected by the previous
    /// call to [Matrix::update_graph_ordering_and_edges].
    ///
    /// May return an error if the graph topology is invalid (cycles)
    /// or something else happened.
    fn build_prog(&mut self) -> Result<NodeProg, MatrixError> {
        let mut ordered_nodes = vec![];
        if !self.graph_ordering.calculate_order(&mut ordered_nodes) {
            return Err(MatrixError::CycleDetected);
        }

        let mut prog = self.config.rebuild_node_ports();

        for node_id in ordered_nodes.iter() {
            self.config.add_prog_node(&mut prog, node_id);
        }

        for edge in self.edges.iter() {
            self.config.set_prog_node_exec_connection(
                &mut prog,
                (edge.to, edge.to_input),
                (edge.from, edge.from_out),
            );
        }

        Ok(prog)
    }

    /// Checks the topology of the DSP graph represented by the
    /// hexagonal matrix.
    ///
    /// Use [Matrix::save_matrix] and [Matrix::restore_matrix]
    /// for trying out changes before committing them to the
    /// DSP thread using [Matrix::sync].
    ///
    /// Note that there is a convenience function with [Matrix::change_matrix]
    /// to make it easier to test and rollback changes if they are faulty.
    ///
    ///```
    /// use hexodsp::*;
    ///
    /// let (node_conf, mut node_exec) = new_node_engine();
    /// let mut matrix = Matrix::new(node_conf, 3, 3);
    ///
    /// matrix.save_matrix();
    ///
    /// // ...
    /// matrix.place(0, 1,
    ///     Cell::empty(NodeId::Sin(1))
    ///     .input(Some(0), None, None));
    /// matrix.place(0, 0,
    ///     Cell::empty(NodeId::Sin(1))
    ///     .out(None, None, Some(0)));
    /// // ...
    ///
    /// let error =
    ///     if let Err(_) = matrix.check() {
    ///        matrix.restore_matrix();
    ///        true
    ///     } else {
    ///        matrix.sync().unwrap();
    ///        false
    ///     };
    ///
    /// // In this examples case there is an error, as we created
    /// // a cycle:
    /// assert!(error);
    ///```
    pub fn check(&mut self) -> Result<(), MatrixError> {
        self.update_graph_ordering_and_edges();

        let mut edge_map = std::collections::HashMap::new();
        for edge in self.edges.iter() {
            if let Some((out1_node_id, out1_idx)) = edge_map.get(&(edge.to, edge.to_input)) {
                return Err(MatrixError::DuplicatedInput {
                    output1: (*out1_node_id, *out1_idx),
                    output2: (edge.from, edge.from_out),
                });
            } else {
                edge_map.insert((edge.to, edge.to_input), (edge.from, edge.from_out));
            }
        }

        let mut ordered_nodes = vec![];
        if !self.graph_ordering.calculate_order(&mut ordered_nodes) {
            return Err(MatrixError::CycleDetected);
        }

        Ok(())
    }

    /// Synchronizes the matrix with the DSP thread.
    /// Call this everytime you changed any of the matrix [Cell]s
    /// eg. with [Matrix::place] and want to publish the
    /// changes to the DSP thread.
    ///
    /// This method might return an error, for instance if the
    /// DSP graph topology contains cycles or has other errors.
    ///
    /// You can check any changes and roll them back
    /// using the method [Matrix::change_matrix].
    pub fn sync(&mut self) -> Result<(), MatrixError> {
        self.create_intermediate_nodes();

        self.update_graph_ordering_and_edges();
        let prog = self.build_prog()?;

        self.config.upload_prog(prog, true); // true => copy_old_out

        // Update the generation counter which is used
        // by external data structures to sync their state with
        // the Matrix.
        self.gen_counter += 1;

        // Refresh the input/outputs of the monitored cell,
        // just in case something has changed with that monitored cell.
        self.remonitor_cell();

        if let Some(obs) = &self.observer {
            obs.update_matrix();
        }

        Ok(())
    }

    /// Retrieves the output port feedback for a specific output
    /// of the given [NodeId].
    ///
    /// See also [NodeConfigurator::out_fb_for].
    pub fn out_fb_for(&self, node_id: &NodeId, out: u8) -> Option<f32> {
        self.config.out_fb_for(node_id, out)
    }

    /// Updates the output port feedback. Call this every UI frame
    /// or whenever you want to get the most recent values from
    /// [Matrix::out_fb_for].
    ///
    /// See also [NodeConfigurator::update_output_feedback].
    pub fn update_output_feedback(&mut self) {
        self.config.update_output_feedback();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_matrix_3_sine() {
        use crate::nodes::new_node_engine;

        let (node_conf, mut node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        matrix.place(0, 0, Cell::empty(NodeId::Sin(0)).out(None, Some(0), None));
        matrix.place(
            1,
            0,
            Cell::empty(NodeId::Sin(1)).input(None, Some(0), None).out(None, None, Some(0)),
        );
        matrix.place(1, 1, Cell::empty(NodeId::Sin(2)).input(Some(0), None, None));
        matrix.sync().unwrap();

        node_exec.process_graph_updates();

        let nodes = node_exec.get_nodes();
        assert!(nodes[0].to_id(0) == NodeId::Sin(0));
        assert!(nodes[1].to_id(1) == NodeId::Sin(1));
        assert!(nodes[2].to_id(2) == NodeId::Sin(2));

        let prog = node_exec.get_prog();
        assert_eq!(prog.prog[0].to_string(), "Op(i=0 out=(0-1|1) in=(0-2|0) at=(0-0) mod=(0-0))");
        assert_eq!(
            prog.prog[1].to_string(),
            "Op(i=1 out=(1-2|1) in=(2-4|1) at=(0-0) mod=(0-0) cpy=(o0 => i2))"
        );
        assert_eq!(
            prog.prog[2].to_string(),
            "Op(i=2 out=(2-3|0) in=(4-6|1) at=(0-0) mod=(0-0) cpy=(o1 => i4))"
        );
    }

    #[test]
    fn check_matrix_get_connections() {
        use crate::nodes::new_node_engine;

        let (node_conf, _node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        matrix.place(0, 0, Cell::empty(NodeId::Sin(0)).out(None, Some(0), None));
        matrix.place(
            1,
            0,
            Cell::empty(NodeId::Sin(1)).input(None, Some(0), None).out(None, None, Some(0)),
        );
        matrix.place(1, 1, Cell::empty(NodeId::Sin(2)).input(Some(0), None, None));
        matrix.sync().unwrap();

        let res = matrix.get_connections(1, 0);
        let res = res.expect("Found connected cells");

        let (_src_cell, src_dir, src_io_idx) = res[0].0;
        let (_dst_cell, dst_dir, dst_io_idx, (nx, ny)) = res[0].1;

        assert_eq!(src_dir, CellDir::B, "Found first connection at bottom");
        assert_eq!(src_io_idx, 0, "Correct output port");
        assert_eq!(dst_dir, CellDir::T, "Found first connection at bottom");
        assert_eq!(dst_io_idx, 0, "Correct output port");
        assert_eq!((nx, ny), (1, 1), "Correct other position");

        let (_src_cell, src_dir, src_io_idx) = res[1].0;
        let (_dst_cell, dst_dir, dst_io_idx, (nx, ny)) = res[1].1;

        assert_eq!(src_dir, CellDir::TL, "Found first connection at bottom");
        assert_eq!(src_io_idx, 0, "Correct output port");
        assert_eq!(dst_dir, CellDir::BR, "Found first connection at bottom");
        assert_eq!(dst_io_idx, 0, "Correct output port");
        assert_eq!((nx, ny), (0, 0), "Correct other position");
    }

    #[test]
    fn check_matrix_param_is_used() {
        use crate::nodes::new_node_engine;

        let (node_conf, _node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        matrix.place(0, 0, Cell::empty(NodeId::Sin(0)).out(None, Some(0), None));
        matrix.place(
            1,
            0,
            Cell::empty(NodeId::Sin(1)).input(None, Some(0), None).out(None, None, Some(0)),
        );
        matrix.place(1, 1, Cell::empty(NodeId::Sin(2)).input(Some(0), None, None));
        matrix.sync().unwrap();

        assert!(matrix.param_input_is_used(NodeId::Sin(1).inp_param("freq").unwrap()));
        assert!(!matrix.param_input_is_used(NodeId::Sin(0).inp_param("freq").unwrap()));

        matrix.place(1, 0, Cell::empty(NodeId::Nop));
        matrix.sync().unwrap();

        assert!(!matrix.param_input_is_used(NodeId::Sin(1).inp_param("freq").unwrap()));
        assert!(!matrix.param_input_is_used(NodeId::Sin(2).inp_param("freq").unwrap()));
    }

    #[test]
    fn check_matrix_filled() {
        use crate::dsp::{Node, NodeId};
        use crate::nodes::new_node_engine;

        let (node_conf, mut node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 9, 9);

        let mut i = 1;
        for x in 0..9 {
            for y in 0..9 {
                matrix.place(x, y, Cell::empty(NodeId::Sin(i)));
                i += 1;
            }
        }
        matrix.sync().unwrap();

        node_exec.process_graph_updates();

        let nodes = node_exec.get_nodes();
        let ex_nodes: Vec<&Node> = nodes.iter().filter(|n| n.to_id(0) != NodeId::Nop).collect();
        assert_eq!(ex_nodes.len(), 9 * 9 + 1);
    }

    #[test]
    fn check_matrix_into_output() {
        use crate::nodes::new_node_engine;

        let (node_conf, mut node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        matrix.place(0, 0, Cell::empty(NodeId::Sin(0)).out(None, Some(0), None));
        matrix.place(
            1,
            0,
            Cell::empty(NodeId::Out(0)).input(None, Some(0), None).out(None, None, Some(0)),
        );
        matrix.sync().unwrap();

        node_exec.set_sample_rate(44100.0);
        node_exec.process_graph_updates();

        let nodes = node_exec.get_nodes();
        assert!(nodes[0].to_id(0) == NodeId::Sin(0));
        assert!(nodes[1].to_id(0) == NodeId::Out(0));

        let prog = node_exec.get_prog();
        assert_eq!(prog.prog.len(), 2);
        assert_eq!(prog.prog[0].to_string(), "Op(i=0 out=(0-1|1) in=(0-2|0) at=(0-0) mod=(0-0))");
        assert_eq!(
            prog.prog[1].to_string(),
            "Op(i=1 out=(1-1|0) in=(2-5|1) at=(0-1) mod=(0-0) cpy=(o0 => i2))"
        );
    }

    #[test]
    fn check_matrix_skip_instance() {
        use crate::nodes::new_node_engine;

        let (node_conf, mut node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        matrix.place(0, 0, Cell::empty(NodeId::Sin(2)).out(None, Some(0), None));
        matrix.place(
            1,
            0,
            Cell::empty(NodeId::Out(0)).input(None, Some(0), None).out(None, None, Some(0)),
        );
        matrix.sync().unwrap();

        node_exec.set_sample_rate(44100.0);
        node_exec.process_graph_updates();

        let nodes = node_exec.get_nodes();
        assert!(nodes[0].to_id(0) == NodeId::Sin(0));
        assert!(nodes[1].to_id(0) == NodeId::Sin(0));
        assert!(nodes[2].to_id(0) == NodeId::Sin(0));
        assert!(nodes[3].to_id(0) == NodeId::Out(0));

        let prog = node_exec.get_prog();
        assert_eq!(prog.prog.len(), 2);
        assert_eq!(prog.prog[0].to_string(), "Op(i=2 out=(2-3|1) in=(4-6|0) at=(0-0) mod=(0-0))");
        assert_eq!(
            prog.prog[1].to_string(),
            "Op(i=3 out=(3-3|0) in=(6-9|1) at=(0-1) mod=(0-0) cpy=(o2 => i6))"
        );
    }

    #[test]
    fn check_matrix_check_cycle() {
        use crate::nodes::new_node_engine;

        let (node_conf, _node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        matrix.save_matrix();
        matrix.place(0, 1, Cell::empty(NodeId::Sin(1)).input(Some(0), None, None));
        matrix.place(0, 0, Cell::empty(NodeId::Sin(1)).out(None, None, Some(0)));
        let error = if let Err(_) = matrix.check() {
            matrix.restore_matrix();
            true
        } else {
            matrix.sync().unwrap();
            false
        };

        // In this examples case there is an error, as we created
        // a cycle:
        assert!(error);
    }

    #[test]
    fn check_matrix_check_duplicate_input() {
        use crate::nodes::new_node_engine;

        let (node_conf, _node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 5, 5);

        matrix.save_matrix();
        matrix.place(0, 1, Cell::empty(NodeId::Sin(0)).input(Some(0), None, None));
        matrix.place(0, 0, Cell::empty(NodeId::Sin(1)).out(None, None, Some(0)));

        matrix.place(0, 3, Cell::empty(NodeId::Sin(0)).input(Some(0), None, None));
        matrix.place(0, 2, Cell::empty(NodeId::Sin(2)).out(None, None, Some(0)));

        assert_eq!(
            matrix.check(),
            Err(MatrixError::DuplicatedInput {
                output1: (NodeId::Sin(1), 0),
                output2: (NodeId::Sin(2), 0),
            })
        );
    }

    #[test]
    fn check_matrix_mod_amt_pre_sync() {
        use crate::nodes::new_node_engine;

        let (node_conf, mut node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        matrix.place(0, 0, Cell::empty(NodeId::Sin(0)).out(None, Some(0), None));
        matrix.place(
            1,
            0,
            Cell::empty(NodeId::Sin(1)).input(None, Some(0), None).out(None, None, Some(0)),
        );
        matrix.place(
            0,
            1,
            Cell::empty(NodeId::Sin(3)).input(None, None, None).out(None, Some(0), None),
        );
        matrix.place(1, 1, Cell::empty(NodeId::Sin(2)).input(Some(0), Some(1), None));
        matrix.set_param_modamt(NodeId::Sin(1).param_by_idx(0).unwrap(), Some(0.5)).unwrap();
        matrix.set_param_modamt(NodeId::Sin(1).param_by_idx(1).unwrap(), Some(0.33)).unwrap();
        matrix.set_param_modamt(NodeId::Sin(0).param_by_idx(0).unwrap(), Some(0.25)).unwrap();
        matrix.set_param_modamt(NodeId::Sin(2).param_by_idx(0).unwrap(), Some(0.75)).unwrap();
        matrix.set_param_modamt(NodeId::Sin(2).param_by_idx(1).unwrap(), Some(-0.75)).unwrap();
        matrix.sync().unwrap();

        node_exec.process_graph_updates();

        let prog = node_exec.get_prog();
        assert_eq!(prog.prog[0].to_string(), "Op(i=0 out=(0-1|1) in=(0-2|0) at=(0-0) mod=(0-1))");
        assert_eq!(prog.prog[1].to_string(), "Op(i=3 out=(3-4|1) in=(6-8|0) at=(0-0) mod=(5-5))");
        assert_eq!(
            prog.prog[2].to_string(),
            "Op(i=1 out=(1-2|1) in=(2-4|1) at=(0-0) mod=(1-3) cpy=(o0 => i2) mod=1)"
        );
        assert_eq!(
            prog.prog[3].to_string(),
            "Op(i=2 out=(2-3|0) in=(4-6|3) at=(0-0) mod=(3-5) cpy=(o1 => i4) cpy=(o3 => i5) mod=3 mod=4)");
    }

    #[test]
    fn check_matrix_mod_amt_post_sync() {
        use crate::nodes::new_node_engine;

        let (node_conf, mut node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        matrix.place(0, 0, Cell::empty(NodeId::Sin(0)).out(None, Some(0), None));
        matrix.place(
            1,
            0,
            Cell::empty(NodeId::Sin(1)).input(None, Some(0), None).out(None, None, Some(0)),
        );
        matrix.place(1, 1, Cell::empty(NodeId::Sin(2)).input(Some(0), None, None));
        matrix.sync().unwrap();
        matrix.set_param_modamt(NodeId::Sin(1).param_by_idx(0).unwrap(), Some(0.5)).unwrap();
        matrix.set_param_modamt(NodeId::Sin(1).param_by_idx(1).unwrap(), Some(0.33)).unwrap();
        matrix.set_param_modamt(NodeId::Sin(0).param_by_idx(0).unwrap(), Some(0.25)).unwrap();

        node_exec.process_graph_updates();

        let prog = node_exec.get_prog();
        assert_eq!(prog.prog[0].to_string(), "Op(i=0 out=(0-1|1) in=(0-2|0) at=(0-0) mod=(0-1))");
        assert_eq!(
            prog.prog[1].to_string(),
            "Op(i=1 out=(1-2|1) in=(2-4|1) at=(0-0) mod=(1-3) cpy=(o0 => i2) mod=1)"
        );
        assert_eq!(
            prog.prog[2].to_string(),
            "Op(i=2 out=(2-3|0) in=(4-6|1) at=(0-0) mod=(3-3) cpy=(o1 => i4))"
        );
    }

    #[test]
    fn check_matrix_set_get() {
        use crate::nodes::new_node_engine;

        let (node_conf, _node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        let pa1 = NodeId::Sin(1).param_by_idx(0).unwrap();
        let pa2 = NodeId::Sin(1).param_by_idx(1).unwrap();
        let pb1 = NodeId::Sin(2).param_by_idx(0).unwrap();
        let pb2 = NodeId::Sin(2).param_by_idx(1).unwrap();
        let px1 = NodeId::BOsc(1).param_by_idx(0).unwrap();
        let px2 = NodeId::BOsc(1).param_by_idx(1).unwrap();

        let gen1 = matrix.get_generation();
        matrix.set_param(pa1, (0.75).into());
        matrix.set_param(pa2, (0.50).into());
        matrix.set_param(pb1, (0.25).into());
        matrix.set_param(pb2, (0.20).into());
        matrix.set_param(px1, (0.10).into());
        matrix.set_param(px2, (0.13).into());

        assert_eq!(matrix.get_generation(), gen1 + 6);

        assert_eq!(matrix.get_param(&pa1), Some((0.75).into()));

        let _ = matrix.set_param_modamt(pa2, Some(0.4));
        let _ = matrix.set_param_modamt(pa1, Some(0.4));
        let _ = matrix.set_param_modamt(pa1, None);

        assert_eq!(matrix.get_generation(), gen1 + 9);

        assert_eq!(matrix.get_param_modamt(&pa2), Some(0.4));
        assert_eq!(matrix.get_param_modamt(&pa1), None);
    }
}
