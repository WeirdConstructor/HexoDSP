// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use serde_json::{json, Value};

pub trait BlockView {
    fn rows(&self) -> usize;
    fn contains(&self, idx: usize) -> Option<usize>;
    fn expanded(&self) -> bool;
    fn label(&self, buf: &mut [u8]) -> usize;
    fn has_input(&self, idx: usize) -> bool;
    fn has_output(&self, idx: usize) -> bool;
    fn input_label(&self, idx: usize, buf: &mut [u8]) -> usize;
    fn output_label(&self, idx: usize, buf: &mut [u8]) -> usize;
    fn custom_color(&self) -> Option<usize>;
}

pub trait BlockCodeView {
    fn area_header(&self, id: usize) -> Option<&str>;
    fn area_size(&self, id: usize) -> (usize, usize);
    fn block_at(&self, id: usize, x: i64, y: i64) -> Option<&dyn BlockView>;
    fn origin_at(&self, id: usize, x: i64, y: i64) -> Option<(i64, i64)>;
    fn generation(&self) -> u64;
}

#[derive(Debug, Clone)]
pub struct BlockIDGenerator {
    counter: Rc<RefCell<usize>>,
}

impl BlockIDGenerator {
    pub fn new() -> Self {
        Self { counter: Rc::new(RefCell::new(0)) }
    }

    pub fn new_with_id(id: usize) -> Self {
        Self { counter: Rc::new(RefCell::new(id)) }
    }

    pub fn current(&self) -> usize {
        *self.counter.borrow_mut()
    }

    pub fn next(&self) -> usize {
        let mut c = self.counter.borrow_mut();
        *c += 1;
        *c
    }
}

/// This structure represents a block inside the [BlockArea] of a [BlockFun].
/// It stores everything required for calculating a node of the AST.
///
/// A [BlockType::instanciate_block] is used to create a new instance of this
/// structure.
///
/// You usually don't use this structure directly, but you use the
/// position of it inside the [BlockFun]. The position of a block
/// is specified by the `area_id`, and the `x` and `y` coordinates.
#[derive(Debug, Clone)]
pub struct Block {
    /// An ID to track this block.
    id: usize,
    /// How many rows this block spans. A [Block] can only be 1 cell wide.
    rows: usize,
    /// Up to two sub [BlockArea] can be specified here by their ID.
    contains: (Option<usize>, Option<usize>),
    /// Whether the sub areas are visible/drawn.
    expanded: bool,
    /// The type of this block. It's just a string set by the [BlockType]
    /// and it should be everything that determines what this block is
    /// going to end up as in the AST.
    typ: String,
    /// The label of the block.
    lbl: String,
    /// The input ports, the index into the [Vec] is the row. The [String]
    /// is the label of the input port.
    inputs: Vec<Option<String>>,
    /// The output ports, the index into the [Vec] is the row. The [String]
    /// is the label of the output port.
    outputs: Vec<Option<String>>,
    /// The color index of this block.
    color: usize,
}

impl Block {
    pub fn clone_with_new_id(&self, new_id: usize) -> Self {
        Self {
            id: new_id,
            rows: self.rows,
            contains: self.contains.clone(),
            expanded: self.expanded,
            typ: self.typ.clone(),
            lbl: self.lbl.clone(),
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
            color: self.color,
        }
    }

    /// Takes the (input) port at row `idx` and pushed it one row further
    /// down, wrapping around at the end. If `output` is true, the
    /// output port at `idx` is shifted.
    pub fn shift_port(&mut self, idx: usize, output: bool) {
        if self.rows <= 1 {
            return;
        }

        let v = if output { &mut self.outputs } else { &mut self.inputs };

        if v.len() < self.rows {
            v.resize(self.rows, None);
        }

        let idx_from = idx;
        let idx_to = (idx + 1) % v.len();
        let elem = v.remove(idx_from);
        v.insert(idx_to, elem);
    }

    /// Calls `f` for every output port that is available.
    /// `f` gets passed the row index.
    pub fn for_output_ports<F: FnMut(usize, &str)>(&self, mut f: F) {
        for i in 0..self.rows {
            if let Some(p) = self.outputs.get(i) {
                if let Some(p) = p {
                    f(i, p);
                }
            }
        }
    }

    /// Returns the number of output ports of this [Block].
    pub fn count_outputs(&self) -> usize {
        let mut count = 0;

        for i in 0..self.rows {
            if let Some(o) = self.outputs.get(i) {
                if o.is_some() {
                    count += 1;
                }
            }
        }

        count
    }

    /// Calls `f` for every input port that is available.
    /// `f` gets passed the row index.
    pub fn for_input_ports<F: FnMut(usize, &str)>(&self, mut f: F) {
        for i in 0..self.rows {
            if let Some(p) = self.inputs.get(i) {
                if let Some(p) = p {
                    f(i, p);
                }
            }
        }
    }

    /// Calls `f` for every input port that is available.
    /// `f` gets passed the row index.
    pub fn for_input_ports_reverse<F: FnMut(usize, &str)>(&self, mut f: F) {
        for i in 1..=self.rows {
            let i = self.rows - i;
            if let Some(p) = self.inputs.get(i) {
                if let Some(p) = p {
                    f(i, p);
                }
            }
        }
    }

    /// Serializes this [Block] into a [Value]. Called by [BlockArea::serialize].
    pub fn serialize(&self) -> Value {
        let mut inputs = json!([]);
        let mut outputs = json!([]);

        if let Value::Array(inputs) = &mut inputs {
            for p in self.inputs.iter() {
                inputs.push(json!(p));
            }
        }

        if let Value::Array(outputs) = &mut outputs {
            for p in self.outputs.iter() {
                outputs.push(json!(p));
            }
        }

        let c0 = if let Some(c) = self.contains.0 { c.into() } else { Value::Null };
        let c1 = if let Some(c) = self.contains.1 { c.into() } else { Value::Null };
        let mut contains = json!([c0, c1]);
        json!({
            "id": self.id as i64,
            "rows": self.rows as i64,
            "contains": contains,
            "expanded": self.expanded,
            "typ": self.typ,
            "lbl": self.lbl,
            "color": self.color,
            "inputs": inputs,
            "outputs": outputs,

        })
    }

    /// Deserializes this [Block] from a [Value]. Called by [BlockArea::deserialize].
    pub fn deserialize(v: &Value) -> Result<Box<Block>, serde_json::Error> {
        let mut inputs = vec![];
        let mut outputs = vec![];

        let inps = &v["inputs"];
        if let Value::Array(inps) = inps {
            for v in inps.iter() {
                inputs.push(if v.is_string() {
                    Some(v.as_str().unwrap_or("").to_string())
                } else {
                    None
                })
            }
        }

        let outs = &v["outputs"];
        if let Value::Array(outs) = outs {
            for v in outs.iter() {
                outputs.push(if v.is_string() {
                    Some(v.as_str().unwrap_or("").to_string())
                } else {
                    None
                })
            }
        }

        Ok(Box::new(Block {
            id: v["id"].as_i64().unwrap_or(0) as usize,
            rows: v["rows"].as_i64().unwrap_or(0) as usize,
            contains: (
                if v["contains"][0].is_i64() {
                    Some(v["contains"][0].as_i64().unwrap_or(0) as usize)
                } else {
                    None
                },
                if v["contains"][1].is_i64() {
                    Some(v["contains"][1].as_i64().unwrap_or(0) as usize)
                } else {
                    None
                },
            ),
            expanded: v["expanded"].as_bool().unwrap_or(true),
            typ: v["typ"].as_str().unwrap_or("?").to_string(),
            lbl: v["lbl"].as_str().unwrap_or("?").to_string(),
            inputs,
            outputs,
            color: v["color"].as_i64().unwrap_or(0) as usize,
        }))
    }
}

impl BlockView for Block {
    fn rows(&self) -> usize {
        self.rows
    }
    fn contains(&self, idx: usize) -> Option<usize> {
        if idx == 0 {
            self.contains.0
        } else {
            self.contains.1
        }
    }
    fn expanded(&self) -> bool {
        true
    }
    fn label(&self, buf: &mut [u8]) -> usize {
        use std::io::Write;
        let mut bw = std::io::BufWriter::new(buf);
        match write!(bw, "{}", self.lbl) {
            Ok(_) => bw.buffer().len(),
            _ => 0,
        }
    }
    fn has_input(&self, idx: usize) -> bool {
        self.inputs.get(idx).map(|s| s.is_some()).unwrap_or(false)
    }
    fn has_output(&self, idx: usize) -> bool {
        self.outputs.get(idx).map(|s| s.is_some()).unwrap_or(false)
    }
    fn input_label(&self, idx: usize, buf: &mut [u8]) -> usize {
        use std::io::Write;
        if let Some(lbl_opt) = self.inputs.get(idx) {
            if let Some(lbl) = lbl_opt {
                let mut bw = std::io::BufWriter::new(buf);
                match write!(bw, "{}", lbl) {
                    Ok(_) => bw.buffer().len(),
                    _ => 0,
                }
            } else {
                0
            }
        } else {
            0
        }
    }
    fn output_label(&self, idx: usize, buf: &mut [u8]) -> usize {
        use std::io::Write;
        if let Some(lbl_opt) = self.outputs.get(idx) {
            if let Some(lbl) = lbl_opt {
                let mut bw = std::io::BufWriter::new(buf);
                match write!(bw, "{}", lbl) {
                    Ok(_) => bw.buffer().len(),
                    _ => 0,
                }
            } else {
                0
            }
        } else {
            0
        }
    }
    fn custom_color(&self) -> Option<usize> {
        Some(self.color)
    }
}

/// Represents a connected collection of blocks. Is created by
/// [BlockFun::retrieve_block_chain_at] or [BlockArea::chain_at].
///
/// After creating a [BlockChain] structure you can decide to
/// clone the blocks from the [BlockArea] with [BlockChain::clone_load]
/// or remove the blocks from the [BlockArea] and store them
/// inside this [BlockChain] via [BlockChain::remove_load].
///
/// The original positions of the _loaded_ blocks is stored too.
/// If you want to move the whole chain in the coordinate system
/// to the upper left most corner, you can use [BlockChain::normalize_load_pos].
#[derive(Debug)]
pub struct BlockChain {
    /// The area ID this BlockChain was created from.
    area_id: usize,
    /// Stores the positions of the blocks of the chain inside the [BlockArea].
    blocks: HashSet<(i64, i64)>,
    /// Stores the positions of blocks that only have output ports.
    sources: HashSet<(i64, i64)>,
    /// Stores the positions of blocks that only have input ports.
    sinks: HashSet<(i64, i64)>,
    /// This field stores _loaded_ blocks from the [BlockArea]
    /// into this [BlockChain] for inserting or analyzing them.
    ///
    /// Stores the blocks themself, with their position in the [BlockArea],
    /// which can be normalized (moved to the upper left) with
    /// [BlockChain::normalize_load_pos].
    ///
    /// The blocks in this [Vec] are stored in sorted order.
    /// They are stored in ascending order of their `x` coordinate,
    /// and for the same `x` coordinate in
    /// ascending order of their `y` coordinate.
    load: Vec<(Box<Block>, i64, i64)>,
}

impl BlockChain {
    pub fn move_by_offs(&mut self, xo: i64, yo: i64) {
        for (_, x, y) in &mut self.load {
            *x += xo;
            *y += yo;
            //d// println!("MOVE_BY_OFFS TO x={:3} y={:3}", *x, *y);
        }
    }

    /// Normalizes the position of all loaded blocks and returns
    /// the original top left most position of the chain.
    pub fn normalize_load_pos(&mut self) -> (i64, i64) {
        let mut min_x = 100000000;
        let mut min_y = 100000000;

        for (_, xo, yo) in &self.load {
            min_x = min_x.min(*xo);
            min_y = min_y.min(*yo);
        }

        for (_, xo, yo) in &mut self.load {
            *xo -= min_x;
            *yo -= min_y;
        }

        self.sort_load_pos();

        (min_x, min_y)
    }

    fn sort_load_pos(&mut self) {
        self.load.sort_by(|&(_, x0, y0), &(_, x1, y1)| x0.cmp(&x1).then(y0.cmp(&y1)));
    }

    pub fn get_connected_inputs_from_load_at_x(&self, x_split: i64) -> Vec<(i64, i64)> {
        let mut output_points = vec![];
        for (block, x, y) in &self.load {
            if *x == x_split {
                block.for_output_ports(|row, _| {
                    output_points.push(y + (row as i64));
                });
            }
        }

        let mut connection_pos = vec![];

        for (block, x, y) in &self.load {
            if *x == (x_split + 1) {
                block.for_input_ports(|row, _| {
                    if output_points.iter().find(|&&out_y| out_y == (y + (row as i64))).is_some() {
                        connection_pos.push((*x, y + (row as i64)));
                    }
                });
            }
        }

        connection_pos
    }

    //    pub fn join_load_after_x(&mut self, x_join: i64, y_split: i64) -> bool {
    //        let filler_pos : Vec<(i64, i64)> =
    //            self.get_connected_inputs_from_load_at_x(x_split);
    //        if filler_pos.len() > 1
    //           || (filler_pos.len() == 1 && filler_pos[0] != (x_join, y_split)
    //    }

    pub fn split_load_after_x(
        &mut self,
        x_split: i64,
        y_split: i64,
        filler: Option<&BlockType>,
        id_gen: BlockIDGenerator,
    ) {
        let filler_pos: Vec<(i64, i64)> = self.get_connected_inputs_from_load_at_x(x_split);

        for (_block, x, _y) in &mut self.load {
            if *x > x_split {
                *x += 1;
            }
        }

        if let Some(filler) = filler {
            for (x, y) in filler_pos {
                if y == y_split {
                    continue;
                }
                let filler_block = filler.instanciate_block(None, id_gen.clone());

                self.load.push((filler_block, x, y));
            }
        }

        self.sort_load_pos();
    }

    pub fn clone_load(&mut self, area: &mut BlockArea, id_gen: BlockIDGenerator) {
        self.load.clear();

        for b in &self.blocks {
            if let Some((block, xo, yo)) = area.ref_at_origin(b.0, b.1) {
                self.load.push((Box::new(block.clone_with_new_id(id_gen.next())), xo, yo));
            }
        }

        self.sort_load_pos();
    }

    pub fn remove_load(&mut self, area: &mut BlockArea) {
        self.load.clear();

        for b in &self.blocks {
            if let Some((block, xo, yo)) = area.remove_at(b.0, b.1) {
                self.load.push((block, xo, yo));
            }
        }

        self.sort_load_pos();
    }

    pub fn place_load(&mut self, area: &mut BlockArea) {
        let load = std::mem::replace(&mut self.load, vec![]);
        area.set_blocks_from(load);
    }

    pub fn try_fit_load_into_space(&mut self, area: &mut BlockArea) -> bool {
        for (xo, yo) in &[
            (0, 0), // where it currently is
            (0, -1),
            (0, -2),
            (0, -3),
            (-1, 0),
            (-1, -1),
            (-1, -2),
            (-1, -3),
            (1, 0),
            (1, -1),
            (1, -2),
            (1, -3),
            (0, 1),
            (0, 2),
            (0, 3),
            (-1, 1),
            (-1, 2),
            (-1, 3),
            (1, 1),
            (1, 2),
            (1, 3),
        ] {
            println!("TRY {},{}", *xo, *yo);
            if self.area_has_space_for_load(area, *xo, *yo) {
                self.move_by_offs(*xo, *yo);
                return true;
            }

            //d// println!("RETRY xo={}, yo={}", *xo, *yo);
        }

        return false;
    }

    pub fn area_has_space_for_load(
        &mut self,
        area: &mut BlockArea,
        xoffs: i64,
        yoffs: i64,
    ) -> bool {
        for (block, x, y) in self.load.iter() {
            if !area.check_space_at(*x + xoffs, *y + yoffs, block.rows) {
                return false;
            }
        }

        true
    }

    pub fn area_is_subarea_of_loaded(&mut self, area: usize, fun: &mut BlockFun) -> bool {
        let mut areas = vec![];

        for (block, _, _) in self.load.iter() {
            fun.all_sub_areas_of(block.as_ref(), &mut areas);
        }

        for a_id in areas.iter() {
            if *a_id == area {
                return true;
            }
        }

        return false;
    }
}

#[derive(Debug, Clone)]
pub struct BlockArea {
    blocks: HashMap<(i64, i64), Box<Block>>,
    origin_map: HashMap<(i64, i64), (i64, i64)>,
    size: (usize, usize),
    auto_shrink: bool,
    header: String,
}

impl BlockArea {
    fn new(w: usize, h: usize) -> Self {
        Self {
            blocks: HashMap::new(),
            origin_map: HashMap::new(),
            size: (w, h),
            auto_shrink: false,
            header: "".to_string(),
        }
    }

    pub fn set_header(&mut self, header: String) {
        self.header = header;
    }

    pub fn set_auto_shrink(&mut self, shrink: bool) {
        self.auto_shrink = shrink;
    }

    pub fn auto_shrink(&self) -> bool {
        self.auto_shrink
    }

    pub fn chain_at(&self, x: i64, y: i64) -> Option<Box<BlockChain>> {
        let (_block, xo, yo) = self.ref_at_origin(x, y)?;

        let mut dq: VecDeque<(i64, i64)> = VecDeque::new();
        dq.push_back((xo, yo));

        let mut blocks: HashSet<(i64, i64)> = HashSet::new();
        let mut sources: HashSet<(i64, i64)> = HashSet::new();
        let mut sinks: HashSet<(i64, i64)> = HashSet::new();

        let mut check_port_conns = vec![];

        while let Some((x, y)) = dq.pop_front() {
            check_port_conns.clear();

            // First we find all adjacent output/input port positions
            // and collect them in `check_port_conns`.
            //
            // While are at it, we also record which blocks are only
            // sinks and which are only sources. Might be important for
            // other algorithms that do things with this.
            if let Some((block, xo, yo)) = self.ref_at_origin(x, y) {
                if blocks.contains(&(xo, yo)) {
                    continue;
                }

                blocks.insert((xo, yo));

                let mut has_input = false;
                let mut has_output = false;

                block.for_input_ports(|idx, _| {
                    check_port_conns.push((xo - 1, yo + (idx as i64), true));
                    has_input = true;
                });

                block.for_output_ports(|idx, _| {
                    check_port_conns.push((xo + 1, yo + (idx as i64), false));
                    has_output = true;
                });

                if !has_input {
                    sources.insert((xo, yo));
                }

                if !has_output {
                    sinks.insert((xo, yo));
                }
            }

            // Then we look if there is a block at that position, with
            // a corresponding input or output port at the right
            // row inside the block.
            for (x, y, is_output) in &check_port_conns {
                if let Some((_block, xo, yo, _row)) = self.find_port_at(*x, *y, *is_output) {
                    dq.push_back((xo, yo));
                }
            }
        }

        Some(Box::new(BlockChain { area_id: 0, blocks, sources, sinks, load: vec![] }))
    }

    pub fn find_last_unconnected_output(&self) -> Option<(i64, i64, String)> {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut port: Option<(i64, i64, String)> = None;

        for ((x, y), block) in &self.blocks {
            let (x, y) = (*x, *y);

            block.for_output_ports(|row, _| {
                let y = y + (row as i64);

                if self.find_port_at(x + 1, y, false).is_none() {
                    if y > max_y {
                        max_y = y;
                        max_x = x;

                        port = Some((
                            max_x,
                            max_y,
                            block
                                .outputs
                                .get(row)
                                .cloned()
                                .flatten()
                                .unwrap_or_else(|| "".to_string()),
                        ));
                    } else if y == max_y && x > max_x {
                        max_x = x;

                        port = Some((
                            max_x,
                            max_y,
                            block
                                .outputs
                                .get(row)
                                .cloned()
                                .flatten()
                                .unwrap_or_else(|| "".to_string()),
                        ));
                    }
                }
            })
        }

        port
    }

    /// Collects the sinks in this area.
    /// It returns a list of [Block] positions inside the
    /// area. For unconnected outputs, which are also evaluated
    /// and returned as possible last value of an [BlockArea],
    /// the output row is also given.
    ///
    /// The result is sorted so, that the bottom right most element
    /// is the first one in the result list.
    pub fn collect_sinks(&self) -> Vec<(i64, i64, Option<usize>)> {
        let mut sinks_out = vec![];

        for ((x, y), block) in &self.blocks {
            if block.count_outputs() == 0 {
                sinks_out.push((*x, *y, None));
            } else {
                block.for_output_ports(|row, _| {
                    if self.find_port_at(*x + 1, *y + (row as i64), false).is_none() {
                        sinks_out.push((*x, *y + (row as i64), Some(row)));
                    }
                });
            }
        }

        sinks_out.sort_by(|&(x0, y0, _), &(x1, y1, _)| y1.cmp(&y0).then(x1.cmp(&x0)));

        sinks_out
    }

    fn ref_at(&self, x: i64, y: i64) -> Option<&Block> {
        let (xo, yo) = self.origin_map.get(&(x, y))?;
        self.blocks.get(&(*xo, *yo)).map(|b| b.as_ref())
    }

    fn ref_at_origin(&self, x: i64, y: i64) -> Option<(&Block, i64, i64)> {
        let (xo, yo) = self.origin_map.get(&(x, y))?;
        let (xo, yo) = (*xo, *yo);
        self.blocks.get(&(xo, yo)).map(|b| (b.as_ref(), xo, yo))
    }

    fn ref_mut_at(&mut self, x: i64, y: i64) -> Option<&mut Block> {
        let (xo, yo) = self.origin_map.get(&(x, y))?;
        self.blocks.get_mut(&(*xo, *yo)).map(|b| b.as_mut())
    }

    fn ref_mut_at_origin(&mut self, x: i64, y: i64) -> Option<(&mut Block, i64, i64)> {
        let (xo, yo) = self.origin_map.get(&(x, y))?;
        let (xo, yo) = (*xo, *yo);
        self.blocks.get_mut(&(xo, yo)).map(|b| (b.as_mut(), xo, yo))
    }

    fn find_port_at(
        &self,
        x: i64,
        y: i64,
        expect_output: bool,
    ) -> Option<(&Block, i64, i64, usize)> {
        let (block, xo, yo) = self.ref_at_origin(x, y)?;

        let port_y = (y - yo).max(0) as usize;

        if expect_output {
            if let Some(o) = block.outputs.get(port_y) {
                if o.is_some() {
                    return Some((block, xo, yo, port_y));
                }
            }
        } else {
            if let Some(i) = block.inputs.get(port_y) {
                if i.is_some() {
                    return Some((block, xo, yo, port_y));
                }
            }
        }

        None
    }

    fn set_blocks_from(&mut self, list: Vec<(Box<Block>, i64, i64)>) {
        for (block, x, y) in list.into_iter() {
            self.blocks.insert((x, y), block);
        }

        self.update_origin_map();
    }

    fn set_block_at(&mut self, x: i64, y: i64, block: Box<Block>) {
        self.blocks.insert((x, y), block);
        self.update_origin_map();
    }

    fn remove_at(&mut self, x: i64, y: i64) -> Option<(Box<Block>, i64, i64)> {
        let (xo, yo) = self.origin_map.get(&(x, y))?;
        if let Some(block) = self.blocks.remove(&(*xo, *yo)) {
            let (xo, yo) = (*xo, *yo);
            self.update_origin_map();
            Some((block, xo, yo))
        } else {
            None
        }
    }

    fn set_size(&mut self, w: usize, h: usize) {
        self.size = (w, h);
    }

    fn get_direct_sub_areas(&self, out: &mut Vec<usize>) {
        for ((_x, _y), block) in &self.blocks {
            if let Some(sub_area) = block.contains.0 {
                out.push(sub_area);
            }

            if let Some(sub_area) = block.contains.1 {
                out.push(sub_area);
            }
        }
    }

    /// Calculates only the size of the area in the +x/+y quadrant.
    /// The negative areas are not counted in.
    fn resolve_size<F: Fn(usize) -> (usize, usize)>(&self, resolve_sub_areas: F) -> (usize, usize) {
        let mut min_w = 1;
        let mut min_h = 1;

        for ((ox, oy), _) in &self.origin_map {
            let (ox, oy) = ((*ox).max(0) as usize, (*oy).max(0) as usize);

            if min_w < (ox + 1) {
                min_w = ox + 1;
            }
            if min_h < (oy + 1) {
                min_h = oy + 1;
            }
        }

        for ((x, y), block) in &self.blocks {
            let (x, y) = ((*x).max(0) as usize, (*y).max(0) as usize);

            let mut prev_h = 1; // one for the top block

            if let Some(sub_area) = block.contains.0 {
                let (sub_w, mut sub_h) = resolve_sub_areas(sub_area);
                sub_h += prev_h;
                prev_h += sub_h;
                if min_w < (x + sub_w + 1) {
                    min_w = x + sub_w + 1;
                }
                if min_h < (y + sub_h + 1) {
                    min_h = y + sub_h + 1;
                }
            }

            if let Some(sub_area) = block.contains.1 {
                let (sub_w, mut sub_h) = resolve_sub_areas(sub_area);
                sub_h += prev_h;
                if min_w < (x + sub_w + 1) {
                    min_w = x + sub_w + 1;
                }
                if min_h < (y + sub_h + 1) {
                    min_h = y + sub_h + 1;
                }
            }
        }

        if self.auto_shrink {
            (min_w, min_h)
        } else {
            (
                if self.size.0 < min_w { min_w } else { self.size.0 },
                if self.size.1 < min_h { min_h } else { self.size.1 },
            )
        }
    }

    fn update_origin_map(&mut self) {
        self.origin_map.clear();

        for ((ox, oy), block) in &self.blocks {
            for r in 0..block.rows {
                self.origin_map.insert((*ox, *oy + (r as i64)), (*ox, *oy));
            }
        }
    }

    fn check_space_at(&self, x: i64, y: i64, rows: usize) -> bool {
        for i in 0..rows {
            let yo = y + (i as i64);

            if self.origin_map.get(&(x, yo)).is_some() {
                return false;
            }
        }

        true
    }

    /// Serializes this [BlockArea] to a JSON [Value].
    /// Usually called by [BlockFunSnapshot::serialize].
    pub fn serialize(&self) -> Value {
        let mut v = json!({
            "size": [self.size.0 as i64, self.size.1 as i64],
            "header": self.header,
            "auto_shrink": self.auto_shrink,
        });

        let mut blks = json!([]);
        if let Value::Array(blks) = &mut blks {
            for ((x, y), b) in self.blocks.iter() {
                blks.push(json!({
                    "x": x,
                    "y": y,
                    "block": b.serialize(),
                }));
            }
        }

        v["blocks"] = blks;

        v
    }

    /// Deserializes a from a JSON [Value].
    /// Usually called by [BlockFunSnapshot::deserialize].
    pub fn deserialize(v: &Value) -> Result<Box<BlockArea>, serde_json::Error> {
        let mut blocks = HashMap::new();

        let blks = &v["blocks"];
        if let Value::Array(blks) = blks {
            for b in blks.iter() {
                let x = b["x"].as_i64().unwrap_or(0);
                let y = b["y"].as_i64().unwrap_or(0);
                blocks.insert((x, y), Block::deserialize(&b["block"])?);
            }
        }

        let size = (
            v["size"][0].as_i64().unwrap_or(0) as usize,
            v["size"][1].as_i64().unwrap_or(0) as usize,
        );
        let auto_shrink = v["auto_shrink"].as_bool().unwrap_or(true);
        let header = v["header"].as_str().unwrap_or("").to_string();

        let mut ba =
            Box::new(BlockArea { blocks, origin_map: HashMap::new(), size, auto_shrink, header });

        ba.update_origin_map();

        Ok(ba)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockUserInput {
    None,
    Float,
    Integer,
    Identifier,
    ClientDecision,
}

impl Default for BlockUserInput {
    fn default() -> Self {
        Self::None
    }
}

impl BlockUserInput {
    pub fn needs_input(&self) -> bool {
        *self != BlockUserInput::None
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlockType {
    pub category: String,
    pub name: String,
    pub rows: usize,
    pub inputs: Vec<Option<String>>,
    pub outputs: Vec<Option<String>>,
    pub area_count: usize,
    pub user_input: BlockUserInput,
    pub description: String,
    pub color: usize,
}

impl BlockType {
    fn touch_contains(&self, block: &mut Block) {
        block.contains = match self.area_count {
            0 => (None, None),
            1 => (Some(1), None),
            2 => (Some(1), Some(1)),
            _ => (None, None),
        };
    }

    pub fn instanciate_block(
        &self,
        user_input: Option<String>,
        id_gen: BlockIDGenerator,
    ) -> Box<Block> {
        let mut block = Box::new(Block {
            id: id_gen.next(),
            rows: self.rows,
            contains: (None, None),
            expanded: true,
            typ: self.name.clone(),
            lbl: if let Some(inp) = user_input { inp } else { self.name.clone() },
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
            color: self.color,
        });
        self.touch_contains(&mut *block);
        block
    }
}

#[derive(Debug, Clone)]
pub struct BlockLanguage {
    types: HashMap<String, BlockType>,
    identifiers: HashMap<String, String>,
}

impl BlockLanguage {
    pub fn new() -> Self {
        Self { types: HashMap::new(), identifiers: HashMap::new() }
    }

    pub fn define_identifier(&mut self, id: &str) {
        let v = id.to_string();
        self.identifiers.insert(id.to_string(), v);
    }

    pub fn define(&mut self, typ: BlockType) {
        self.types.insert(typ.name.clone(), typ);
    }

    pub fn is_identifier(&self, id: &str) -> bool {
        self.identifiers.get(id).is_some()
    }

    pub fn list_identifiers(&self) -> Vec<String> {
        let mut identifiers: Vec<String> = self.identifiers.keys().cloned().collect();
        identifiers.sort();
        identifiers
    }

    pub fn get_type_outputs(&self, typ: &str) -> Option<&[Option<String>]> {
        let typ = self.types.get(typ)?;
        Some(&typ.outputs)
    }

    pub fn get_type_inputs(&self, typ: &str) -> Option<&[Option<String>]> {
        let typ = self.types.get(typ)?;
        Some(&typ.inputs)
    }

    pub fn get_output_name_at_index(&self, typ: &str, idx: usize) -> Option<String> {
        if let Some(outs) = self.get_type_outputs(typ) {
            let mut i = 0;
            for o in outs.iter() {
                if let Some(outname) = o {
                    if i == idx {
                        return Some(outname.to_string());
                    }
                    i += 1;
                }
            }
        }

        None
    }

    pub fn type_output_count(&self, typ: &str) -> usize {
        let mut cnt = 0;

        if let Some(outs) = self.get_type_outputs(typ) {
            for o in outs.iter() {
                if o.is_some() {
                    cnt += 1;
                }
            }
        }

        cnt
    }

    pub fn get_type_list(&self) -> Vec<(String, String, BlockUserInput)> {
        let mut out = vec![];
        for (_, typ) in &self.types {
            out.push((typ.category.clone(), typ.name.clone(), typ.user_input));
        }
        out
    }
}

pub trait BlockASTNode: std::fmt::Debug + Clone {
    fn from(id: usize, typ: &str, lbl: &str) -> Self;
    fn add_node(&self, in_port: String, out_port: String, node: Self);
    fn add_structural_node(&self, node: Self) {
        self.add_node("".to_string(), "".to_string(), node);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockDSPError {
    UnknownArea(usize),
    UnknownLanguageType(String),
    NoBlockAt(usize, i64, i64),
    CircularAction(usize, usize),
    NoSpaceAvailable(usize, i64, i64, usize),
}

#[derive(Debug, Clone)]
pub struct BlockFunSnapshot {
    areas: Vec<Box<BlockArea>>,
    cur_id: usize,
}

impl BlockFunSnapshot {
    pub fn serialize(&self) -> Value {
        let mut v = json!({
            "VERSION": 1,
        });

        v["current_block_id_counter"] = self.cur_id.into();

        let mut areas = json!([]);
        if let Value::Array(areas) = &mut areas {
            for area in self.areas.iter() {
                areas.push(area.serialize());
            }
        }

        v["areas"] = areas;

        v
    }

    pub fn deserialize(v: &Value) -> Result<BlockFunSnapshot, serde_json::Error> {
        let mut a = vec![];

        let areas = &v["areas"];
        if let Value::Array(areas) = areas {
            for v in areas.iter() {
                a.push(BlockArea::deserialize(v)?);
            }
        }

        Ok(BlockFunSnapshot {
            areas: a,
            cur_id: v["current_block_id_counter"].as_i64().unwrap_or(0) as usize,
        })
    }
}

#[derive(Debug, Clone)]
pub struct BlockFun {
    language: Rc<RefCell<BlockLanguage>>,
    areas: Vec<Box<BlockArea>>,
    size_work_dq: VecDeque<usize>,
    area_work_dq: VecDeque<usize>,
    id_gen: BlockIDGenerator,
    generation: u64,
}

#[derive(Debug)]
enum GenTreeJob<N: BlockASTNode> {
    Node { node: N, out: N },
    Output { area_id: usize, x: i64, y: i64, in_port: String, out: N },
    Sink { area_id: usize, x: i64, y: i64, out: N },
    Area { area_id: usize, out: N },
}

impl BlockFun {
    pub fn new(lang: Rc<RefCell<BlockLanguage>>) -> Self {
        Self {
            language: lang,
            areas: vec![Box::new(BlockArea::new(16, 16))],
            size_work_dq: VecDeque::new(),
            area_work_dq: VecDeque::new(),
            id_gen: BlockIDGenerator::new(),
            generation: 0,
        }
    }

    pub fn is_unset(&self) -> bool {
        self.generation == 0
    }

    pub fn block_language(&self) -> Rc<RefCell<BlockLanguage>> {
        self.language.clone()
    }

    pub fn block_ref(&self, id: usize, x: i64, y: i64) -> Option<&Block> {
        let area = self.areas.get(id)?;
        area.ref_at(x, y)
    }

    pub fn block_ref_mut(&mut self, id: usize, x: i64, y: i64) -> Option<&mut Block> {
        let area = self.areas.get_mut(id)?;
        area.ref_mut_at(x, y)
    }

    pub fn shift_port(&mut self, id: usize, x: i64, y: i64, row: usize, output: bool) {
        if let Some(block) = self.block_ref_mut(id, x, y) {
            block.shift_port(row, output);
            self.generation += 1;
        }
    }

    pub fn save_snapshot(&self) -> BlockFunSnapshot {
        BlockFunSnapshot {
            areas: self.areas.iter().cloned().collect(),
            cur_id: self.id_gen.current(),
        }
    }

    pub fn load_snapshot(&mut self, repr: &BlockFunSnapshot) {
        self.areas = repr.areas.iter().cloned().collect();
        self.id_gen = BlockIDGenerator::new_with_id(repr.cur_id);
        self.recalculate_area_sizes();
        self.generation += 1;
    }

    pub fn generate_tree<Node: BlockASTNode>(&self, null_typ: &str) -> Result<Node, BlockDSPError> {
        // This is a type for filling in unfilled outputs:
        let lang = self.language.borrow();
        let null_typ = lang
            .types
            .get(null_typ)
            .ok_or(BlockDSPError::UnknownLanguageType(null_typ.to_string()))?
            .name
            .to_string();

        // Next we build the root AST node set:
        let mut tree_builder: Vec<GenTreeJob<Node>> = vec![];

        let main_node = Node::from(0, "<r>", "");

        tree_builder.push(GenTreeJob::<Node>::Area { area_id: 0, out: main_node.clone() });

        // A HashMap to store those blocks, that have multiple outputs.
        // Their AST nodes need to be shared to multiple parent nodes.
        let mut multi_outs: HashMap<(usize, i64, i64), Node> = HashMap::new();

        // We do a depth first search here:
        while let Some(job) = tree_builder.pop() {
            match job {
                GenTreeJob::<Node>::Area { area_id, out } => {
                    let area =
                        self.areas.get(area_id).ok_or(BlockDSPError::UnknownArea(area_id))?;

                    let sinks = area.collect_sinks();

                    let area_node = Node::from(0, "<a>", "");
                    out.add_structural_node(area_node.clone());

                    for (x, y, uncon_out_row) in sinks {
                        if let Some(_row) = uncon_out_row {
                            let result_node = Node::from(0, "<res>", "");

                            tree_builder.push(GenTreeJob::<Node>::Output {
                                area_id,
                                x,
                                y,
                                in_port: "".to_string(),
                                out: result_node.clone(),
                            });

                            tree_builder.push(GenTreeJob::<Node>::Node {
                                node: result_node,
                                out: area_node.clone(),
                            });
                        } else {
                            tree_builder.push(GenTreeJob::<Node>::Sink {
                                area_id,
                                x,
                                y,
                                out: area_node.clone(),
                            });
                        }
                    }
                }
                GenTreeJob::<Node>::Node { node, out } => {
                    out.add_structural_node(node);
                }
                GenTreeJob::<Node>::Sink { area_id, x, y, out } => {
                    let area =
                        self.areas.get(area_id).ok_or(BlockDSPError::UnknownArea(area_id))?;

                    if let Some((block, xo, yo)) = area.ref_at_origin(x, y) {
                        let (node, needs_init) =
                            if let Some(node) = multi_outs.get(&(area_id, xo, yo)) {
                                (node.clone(), false)
                            } else {
                                (Node::from(block.id, &block.typ, &block.lbl), true)
                            };

                        out.add_structural_node(node.clone());

                        if needs_init {
                            multi_outs.insert((area_id, xo, yo), node.clone());

                            if let Some(cont_area_id) = block.contains.1 {
                                tree_builder.push(GenTreeJob::<Node>::Area {
                                    area_id: cont_area_id,
                                    out: node.clone(),
                                });
                            }

                            if let Some(cont_area_id) = block.contains.0 {
                                tree_builder.push(GenTreeJob::<Node>::Area {
                                    area_id: cont_area_id,
                                    out: node.clone(),
                                });
                            }

                            block.for_input_ports_reverse(|row, port_name| {
                                tree_builder.push(GenTreeJob::<Node>::Output {
                                    area_id,
                                    x: xo - 1,
                                    y: yo + (row as i64),
                                    in_port: port_name.to_string(),
                                    out: node.clone(),
                                });
                            });
                        }
                    }
                }
                GenTreeJob::<Node>::Output { area_id, x, y, in_port, out } => {
                    let area =
                        self.areas.get(area_id).ok_or(BlockDSPError::UnknownArea(area_id))?;

                    if let Some((block, xo, yo)) = area.ref_at_origin(x, y) {
                        let row = y - yo;

                        let (node, needs_init) =
                            if let Some(node) = multi_outs.get(&(area_id, xo, yo)) {
                                (node.clone(), false)
                            } else {
                                (Node::from(block.id, &block.typ, &block.lbl), true)
                            };

                        if let Some(out_name) = block.outputs.get(row as usize).cloned().flatten() {
                            out.add_node(in_port, out_name, node.clone());
                        } else {
                            let node = Node::from(0, &null_typ, "");
                            out.add_node(in_port, "".to_string(), node.clone());
                        }

                        if needs_init {
                            multi_outs.insert((area_id, xo, yo), node.clone());

                            if let Some(cont_area_id) = block.contains.1 {
                                tree_builder.push(GenTreeJob::<Node>::Area {
                                    area_id: cont_area_id,
                                    out: node.clone(),
                                });
                            }

                            if let Some(cont_area_id) = block.contains.0 {
                                tree_builder.push(GenTreeJob::<Node>::Area {
                                    area_id: cont_area_id,
                                    out: node.clone(),
                                });
                            }

                            block.for_input_ports_reverse(|row, port_name| {
                                tree_builder.push(GenTreeJob::<Node>::Output {
                                    area_id,
                                    x: xo - 1,
                                    y: yo + (row as i64),
                                    in_port: port_name.to_string(),
                                    out: node.clone(),
                                });
                            });
                        }
                    } else {
                        let node = Node::from(0, &null_typ, "");
                        out.add_node(in_port, "".to_string(), node.clone());
                    }
                }
            }
        }

        Ok(main_node)
    }

    pub fn recalculate_area_sizes(&mut self) {
        let mut parents = vec![0; self.areas.len()];
        let mut sizes = vec![(0, 0); self.areas.len()];

        // First we dive downwards, to record all the parents
        // and get the sizes of the (leafs).

        self.area_work_dq.clear();
        self.size_work_dq.clear();

        let parents_work_list = &mut self.area_work_dq;
        let size_work_list = &mut self.size_work_dq;

        // Push the root area:
        parents_work_list.push_back(0);

        let mut cur_sub = vec![];
        while let Some(area_idx) = parents_work_list.pop_back() {
            cur_sub.clear();

            self.areas[area_idx].get_direct_sub_areas(&mut cur_sub);

            // XXX: The resolver gets (0, 0), thats wrong for the
            //      areas with sub areas. But it resolves the leaf area
            //      sizes already correctly!
            let (w, h) = self.areas[area_idx].resolve_size(|_id| (0, 0));
            sizes[area_idx] = (w, h);

            if cur_sub.len() == 0 {
                size_work_list.push_front(area_idx);
            } else {
                for sub_idx in &cur_sub {
                    // XXX: Record the parent:
                    parents[*sub_idx] = area_idx;
                    parents_work_list.push_back(*sub_idx);
                }
            }
        }

        // XXX: Invariant now is:
        //      - `parents` contains all the parent area IDs.
        //      - `size_work_list` contains all the leaf area IDs.
        //      - `sizes`   contains correct sizes for the leafs
        //                  (but wrong for the non leafs).

        // Next we need to work through the size_work_list upwards.
        // That means, for each leaf in front of the Deque,
        // we push the parent to the back.
        while let Some(area_idx) = size_work_list.pop_front() {
            // XXX: The invariant as we walk upwards is, that once we
            //      encounter a parent area ID in the size_work_list,
            //      we know that all sub areas already have been computed.
            let (w, h) = self.areas[area_idx].resolve_size(|id| sizes[id]);
            sizes[area_idx] = (w, h);
            self.areas[area_idx].set_size(w, h);

            // XXX: area_idx == 0 is the root area, so skip that
            //      when pushing further parents!
            if area_idx > 0 {
                size_work_list.push_back(parents[area_idx]);
            }
        }
    }

    pub fn area_is_subarea_of(&mut self, area_id: usize, a_id: usize, x: i64, y: i64) -> bool {
        let mut areas = vec![];

        let block = if let Some(block) = self.block_ref(a_id, x, y) {
            block.clone()
        } else {
            return false;
        };

        self.all_sub_areas_of(&block, &mut areas);

        for a_id in &areas {
            if area_id == *a_id {
                return true;
            }
        }

        return false;
    }

    pub fn all_sub_areas_of(&mut self, block: &Block, areas: &mut Vec<usize>) {
        let contains = block.contains.clone();

        let area_work_list = &mut self.area_work_dq;
        area_work_list.clear();

        if let Some(area_id) = contains.0 {
            area_work_list.push_back(area_id);
        }
        if let Some(area_id) = contains.1 {
            area_work_list.push_back(area_id);
        }

        if area_work_list.len() <= 0 {
            return;
        }

        let mut cur_sub = vec![];
        while let Some(area_idx) = area_work_list.pop_front() {
            areas.push(area_idx);

            cur_sub.clear();
            self.areas[area_idx].get_direct_sub_areas(&mut cur_sub);

            for sub_idx in &cur_sub {
                area_work_list.push_back(*sub_idx);
            }
        }
    }

    pub fn retrieve_block_chain_at(
        &mut self,
        id: usize,
        x: i64,
        y: i64,
        remove_blocks: bool,
    ) -> Option<Box<BlockChain>> {
        let area = self.areas.get_mut(id)?;
        let mut chain = area.chain_at(x, y)?;

        if remove_blocks {
            chain.remove_load(area);
        } else {
            chain.clone_load(area, self.id_gen.clone());
        }

        Some(chain)
    }

    pub fn clone_block_from_to(
        &mut self,
        id: usize,
        x: i64,
        y: i64,
        id2: usize,
        x2: i64,
        mut y2: i64,
    ) -> Result<(), BlockDSPError> {
        let lang = self.language.clone();

        let (mut block, _xo, yo) = if let Some(area) = self.areas.get_mut(id) {
            let (block, xo, yo) =
                area.ref_mut_at_origin(x, y).ok_or(BlockDSPError::NoBlockAt(id, x, y))?;

            let mut new_block = Box::new(block.clone_with_new_id(self.id_gen.next()));
            if let Some(typ) = lang.borrow().types.get(&new_block.typ) {
                typ.touch_contains(new_block.as_mut());
            }

            (new_block, xo, yo)
        } else {
            return Err(BlockDSPError::UnknownArea(id));
        };

        self.create_areas_for_block(block.as_mut());

        // check if the user grabbed at a different row than the top row:
        if y > yo {
            // if so, adjust the destination:
            let offs = y - yo;
            y2 = (y2 - offs).max(0);
        }

        let area2 = self.areas.get_mut(id2).ok_or(BlockDSPError::UnknownArea(id2))?;
        let rows = block.rows;

        if area2.check_space_at(x2, y2, block.rows) {
            area2.set_block_at(x2, y2, block);
            self.generation += 1;
            Ok(())
        } else {
            Err(BlockDSPError::NoSpaceAvailable(id2, x2, y2, rows))
        }
    }

    pub fn split_block_chain_after(
        &mut self,
        id: usize,
        x: i64,
        y: i64,
        filler_type: Option<&str>,
    ) -> Result<(), BlockDSPError> {
        let mut area_clone = self.areas.get(id).ok_or(BlockDSPError::UnknownArea(id))?.clone();

        let mut chain = area_clone.chain_at(x, y).ok_or(BlockDSPError::NoBlockAt(id, x, y))?;

        chain.remove_load(area_clone.as_mut());

        let lang = self.language.borrow();
        let typ: Option<&BlockType> = if let Some(filler_type) = filler_type {
            Some(
                lang.types
                    .get(filler_type)
                    .ok_or(BlockDSPError::UnknownLanguageType(filler_type.to_string()))?,
            )
        } else {
            None
        };

        chain.split_load_after_x(x, y, typ, self.id_gen.clone());

        if !chain.area_has_space_for_load(&mut area_clone, 0, 0) {
            return Err(BlockDSPError::NoSpaceAvailable(id, x, y, 0));
        }

        chain.place_load(&mut area_clone);
        self.generation += 1;

        self.areas[id] = area_clone;

        Ok(())
    }

    pub fn move_block_chain_from_to(
        &mut self,
        id: usize,
        x: i64,
        y: i64,
        id2: usize,
        x2: i64,
        y2: i64,
    ) -> Result<(), BlockDSPError> {
        let mut area_clone = self.areas.get(id).ok_or(BlockDSPError::UnknownArea(id))?.clone();

        let mut chain = area_clone.chain_at(x, y).ok_or(BlockDSPError::NoBlockAt(id, x, y))?;

        chain.remove_load(area_clone.as_mut());
        self.generation += 1;

        if id2 == id {
            let move_x_offs = x2 - x;
            let move_y_offs = y2 - y;
            chain.move_by_offs(move_x_offs, move_y_offs);

            if !chain.try_fit_load_into_space(&mut area_clone) {
                return Err(BlockDSPError::NoSpaceAvailable(id, x2, y2, 0));
            }

            chain.place_load(&mut area_clone);
            self.areas[id] = area_clone;
        } else {
            // id2 != id
            if chain.area_is_subarea_of_loaded(id2, self) {
                return Err(BlockDSPError::CircularAction(id, id2));
            }

            let (xo, yo) = chain.normalize_load_pos();
            let (grab_x_offs, grab_y_offs) = (xo - x, yo - y);

            // println!("xo={}, yo={}, grab_x={}, grab_y={}, x2={}, y2={}",
            //     xo, yo, grab_x_offs, grab_y_offs, x2, y2);

            // XXX: .max(0) prevents us from moving the
            //      chain outside the subarea accendentally!
            chain.move_by_offs((grab_x_offs + x2).max(0), (grab_y_offs + y2).max(0));

            let mut area2_clone =
                self.areas.get(id2).ok_or(BlockDSPError::UnknownArea(id))?.clone();

            if !chain.try_fit_load_into_space(&mut area2_clone) {
                return Err(BlockDSPError::NoSpaceAvailable(id, x2, y2, 1));
            }

            chain.place_load(&mut area2_clone);

            self.areas[id] = area_clone;
            self.areas[id2] = area2_clone;
        }

        self.generation += 1;

        //        let mut chain =
        //            self.retrieve_block_chain_at(id, x, y, true)
        //                .ok_or(BlockDSPError::NoBlockAt(id, x, y))?;
        //
        //        chain.normalize_load_pos();

        Ok(())
    }

    pub fn move_block_from_to(
        &mut self,
        id: usize,
        x: i64,
        y: i64,
        id2: usize,
        x2: i64,
        mut y2: i64,
    ) -> Result<(), BlockDSPError> {
        if self.area_is_subarea_of(id2, id, x, y) {
            return Err(BlockDSPError::CircularAction(id, id2));
        }

        let (block, xo, yo) = if let Some(area) = self.areas.get_mut(id) {
            area.remove_at(x, y).ok_or(BlockDSPError::NoBlockAt(id, x, y))?
        } else {
            return Err(BlockDSPError::UnknownArea(id));
        };

        // check if the user grabbed at a different row than the top row:
        if y > yo {
            // if so, adjust the destination:
            let offs = y - yo;
            y2 = (y2 - offs).max(0);
        }

        let area2 = self.areas.get_mut(id2).ok_or(BlockDSPError::UnknownArea(id2))?;
        let rows = block.rows;

        self.generation += 1;

        if area2.check_space_at(x2, y2, block.rows) {
            area2.set_block_at(x2, y2, block);
            Ok(())
        } else {
            if let Some(area) = self.areas.get_mut(id) {
                area.set_block_at(xo, yo, block);
            }
            Err(BlockDSPError::NoSpaceAvailable(id2, x2, y2, rows))
        }
    }

    fn create_areas_for_block(&mut self, block: &mut Block) {
        if let Some(area_id) = &mut block.contains.0 {
            let mut area = Box::new(BlockArea::new(1, 1));
            area.set_auto_shrink(true);
            self.areas.push(area);
            *area_id = self.areas.len() - 1;
        }

        if let Some(area_id) = &mut block.contains.1 {
            let mut area = Box::new(BlockArea::new(1, 1));
            area.set_auto_shrink(true);
            self.areas.push(area);
            *area_id = self.areas.len() - 1;
        }
    }

    pub fn instanciate_at(
        &mut self,
        id: usize,
        x: i64,
        y: i64,
        typ: &str,
        user_input: Option<String>,
    ) -> Result<(), BlockDSPError> {
        let mut block = {
            let lang = self.language.borrow();

            if let Some(area) = self.areas.get_mut(id) {
                if let Some(typ) = lang.types.get(typ) {
                    if !area.check_space_at(x, y, typ.rows) {
                        return Err(BlockDSPError::NoSpaceAvailable(id, x, y, typ.rows));
                    }
                }
            } else {
                return Err(BlockDSPError::UnknownArea(id));
            }

            let typ =
                lang.types.get(typ).ok_or(BlockDSPError::UnknownLanguageType(typ.to_string()))?;

            typ.instanciate_block(user_input, self.id_gen.clone())
        };

        self.create_areas_for_block(block.as_mut());

        self.generation += 1;

        if let Some(area) = self.areas.get_mut(id) {
            area.set_block_at(x, y, block);
        }

        Ok(())
    }

    pub fn remove_at(&mut self, id: usize, x: i64, y: i64) -> Result<(), BlockDSPError> {
        let area = self.areas.get_mut(id).ok_or(BlockDSPError::UnknownArea(id))?;
        area.remove_at(x, y).ok_or(BlockDSPError::NoBlockAt(id, x, y))?;
        self.generation += 1;
        Ok(())
    }

    pub fn area_size(&self, id: usize) -> (usize, usize) {
        self.areas.get(id).map(|a| a.size).unwrap_or((0, 0))
    }

    pub fn block_at(&self, id: usize, x: i64, y: i64) -> Option<&dyn BlockView> {
        let area = self.areas.get(id)?;
        Some(area.blocks.get(&(x, y))?.as_ref())
    }

    pub fn origin_at(&self, id: usize, x: i64, y: i64) -> Option<(i64, i64)> {
        self.areas.get(id).map(|a| a.origin_map.get(&(x, y)).copied()).flatten()
    }
}

impl BlockCodeView for BlockFun {
    fn area_header(&self, id: usize) -> Option<&str> {
        self.areas.get(id).map(|a| &a.header[..])
    }

    fn area_size(&self, id: usize) -> (usize, usize) {
        self.area_size(id)
    }

    fn block_at(&self, id: usize, x: i64, y: i64) -> Option<&dyn BlockView> {
        self.block_at(id, x, y)
    }

    fn origin_at(&self, id: usize, x: i64, y: i64) -> Option<(i64, i64)> {
        self.origin_at(id, x, y)
    }

    fn generation(&self) -> u64 {
        self.generation
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_blockfun_serialize_empty() {
        let dsp_lib = synfx_dsp_jit::get_standard_library();
        let lang = crate::blocklang_def::setup_hxdsp_block_language(dsp_lib);
        let mut bf = BlockFun::new(lang.clone());

        let sn = bf.save_snapshot();
        let serialized = sn.serialize().to_string();
        assert_eq!(serialized, "{\"VERSION\":1,\"areas\":[{\"auto_shrink\":false,\"blocks\":[],\"header\":\"\",\"size\":[16,16]}],\"current_block_id_counter\":0}");

        let v: Value = serde_json::from_str(&serialized).unwrap();
        let sn = BlockFunSnapshot::deserialize(&v).expect("No deserialization error");
        let mut bf2 = BlockFun::new(lang);
        let bf2 = bf2.load_snapshot(&sn);
    }

    #[test]
    fn check_blockfun_serialize_1() {
        let dsp_lib = synfx_dsp_jit::get_standard_library();
        let lang = crate::blocklang_def::setup_hxdsp_block_language(dsp_lib);
        let mut bf = BlockFun::new(lang.clone());

        bf.instanciate_at(0, 0, 0, "+", None);

        let sn = bf.save_snapshot();
        let serialized = sn.serialize().to_string();
        assert_eq!(serialized,
        "{\"VERSION\":1,\"areas\":[{\"auto_shrink\":false,\"blocks\":[{\"block\":{\"color\":4,\"contains\":[null,null],\"expanded\":true,\"id\":1,\"inputs\":[\"\",\"\"],\"lbl\":\"+\",\"outputs\":[\"\"],\"rows\":2,\"typ\":\"+\"},\"x\":0,\"y\":0}],\"header\":\"\",\"size\":[16,16]}],\"current_block_id_counter\":1}");

        let v: Value = serde_json::from_str(&serialized).unwrap();
        let sn = BlockFunSnapshot::deserialize(&v).expect("No deserialization error");
        let mut bf2 = BlockFun::new(lang);
        bf2.load_snapshot(&sn);

        let bv = bf2.block_at(0, 0, 0).unwrap();
        assert!(bv.has_input(0));
    }
}
