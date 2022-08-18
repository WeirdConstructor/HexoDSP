// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{NodeId, ParamId, SAtom};
use crate::wblockdsp::BlockFunSnapshot;
use serde_json::{json, Value};

#[derive(Debug, Clone, Copy)]
pub struct CellRepr {
    pub node_id: NodeId,
    pub x: usize,
    pub y: usize,
    pub inp: [i16; 3],
    pub out: [i16; 3],
}

fn deserialize_node_id(v: &Value, i1: usize, i2: usize) -> Result<NodeId, MatrixDeserError> {
    let nid = NodeId::from_str(v[i1].as_str().unwrap_or("???"));
    if nid == NodeId::Nop {
        return Err(MatrixDeserError::UnknownNode(v[i1].as_str().unwrap_or("???").to_string()));
    }

    Ok(nid.to_instance(v[i2].as_i64().unwrap_or(0) as usize))
}

fn inp2idx(node_id: NodeId, v: &Value) -> i16 {
    let inp = v.as_i64().unwrap_or(-2) as i16;

    if inp > -2 {
        inp
    } else {
        v.as_str().map(|s| node_id.inp(s).map(|i| i as i16).unwrap_or(-1)).unwrap_or(-1)
    }
}

fn out2idx(node_id: NodeId, v: &Value) -> i16 {
    let out = v.as_i64().unwrap_or(-2) as i16;

    if out > -2 {
        out
    } else {
        v.as_str().map(|s| node_id.out(s).map(|i| i as i16).unwrap_or(-1)).unwrap_or(-1)
    }
}

fn inp_idx2value(node_id: NodeId, idx: i16) -> Value {
    if idx == -1 {
        json!(-1)
    } else {
        node_id.inp_name_by_idx(idx as u8).map(|n| json!(n)).unwrap_or_else(|| json!(-1))
    }
}

fn out_idx2value(node_id: NodeId, idx: i16) -> Value {
    if idx == -1 {
        json!(-1)
    } else {
        node_id.out_name_by_idx(idx as u8).map(|n| json!(n)).unwrap_or_else(|| json!(-1))
    }
}

impl CellRepr {
    pub fn serialize(&self) -> Value {
        json!([
            self.node_id.name(),
            self.node_id.instance(),
            self.x,
            self.y,
            [
                inp_idx2value(self.node_id, self.inp[0]),
                inp_idx2value(self.node_id, self.inp[1]),
                inp_idx2value(self.node_id, self.inp[2])
            ],
            [
                out_idx2value(self.node_id, self.out[0]),
                out_idx2value(self.node_id, self.out[1]),
                out_idx2value(self.node_id, self.out[2])
            ],
        ])
    }

    pub fn deserialize(v: &Value) -> Result<Self, MatrixDeserError> {
        let node_id = deserialize_node_id(v, 0, 1)?;

        Ok(Self {
            node_id,
            x: v[2].as_i64().unwrap_or(0) as usize,
            y: v[3].as_i64().unwrap_or(0) as usize,
            inp: [
                inp2idx(node_id, &v[4][0]),
                inp2idx(node_id, &v[4][1]),
                inp2idx(node_id, &v[4][2]),
            ],
            out: [
                out2idx(node_id, &v[5][0]),
                out2idx(node_id, &v[5][1]),
                out2idx(node_id, &v[5][2]),
            ],
        })
    }
}

use crate::dsp::tracker::{MAX_COLS, MAX_PATTERN_LEN};

#[derive(Debug, Clone)]
pub struct PatternRepr {
    pub col_types: [u8; MAX_COLS],
    pub data: Vec<Vec<i32>>,
    pub rows: usize,
    pub edit_step: usize,
    pub cursor: (usize, usize),
}

impl PatternRepr {
    fn serialize(&self) -> Value {
        let mut ret = json!({
            "rows":       self.rows,
            "edit_step":  self.edit_step,
            "cursor_row": self.cursor.0,
            "cursor_col": self.cursor.1,
        });

        let mut cts = json!([]);
        if let Value::Array(cts) = &mut cts {
            for ct in self.col_types.iter() {
                cts.push(json!(*ct as i64));
            }
        }
        ret["col_types"] = cts;

        let mut data = json!([]);
        if let Value::Array(data) = &mut data {
            for row in self.data.iter() {
                let mut out_col = json!([]);
                if let Value::Array(out_col) = &mut out_col {
                    for col in row.iter() {
                        out_col.push(json!(*col as i64));
                    }
                }
                data.push(out_col);
            }
        }
        ret["data"] = data;

        ret
    }

    fn deserialize(v: &Value) -> Result<Self, MatrixDeserError> {
        let mut col_types = [0; MAX_COLS];

        let cts = &v["col_types"];
        if let Value::Array(cts) = cts {
            for (i, ct) in cts.iter().enumerate() {
                col_types[i] = ct.as_i64().unwrap_or(0) as u8;
            }
        }

        let mut data = vec![vec![-1; MAX_COLS]; MAX_PATTERN_LEN];
        let dt = &v["data"];
        if let Value::Array(dt) = dt {
            for (row_idx, row) in dt.iter().enumerate() {
                if let Value::Array(row) = row {
                    for (col_idx, c) in row.iter().enumerate() {
                        data[row_idx][col_idx] = c.as_i64().unwrap_or(-1) as i32;
                    }
                }
            }
        }

        Ok(Self {
            col_types,
            data,
            rows: v["rows"].as_i64().unwrap_or(0) as usize,
            edit_step: v["edit_step"].as_i64().unwrap_or(0) as usize,
            cursor: (
                v["cursor_row"].as_i64().unwrap_or(0) as usize,
                v["cursor_col"].as_i64().unwrap_or(0) as usize,
            ),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MatrixRepr {
    pub cells: Vec<CellRepr>,
    pub params: Vec<(ParamId, f32, Option<f32>)>,
    pub atoms: Vec<(ParamId, SAtom)>,
    pub patterns: Vec<Option<PatternRepr>>,
    pub properties: Vec<(String, SAtom)>,
    pub block_funs: Vec<Option<BlockFunSnapshot>>,
    pub version: i64,
}

#[derive(Debug, Clone)]
pub enum MatrixDeserError {
    BadVersion,
    UnknownNode(String),
    UnknownParamId(String),
    Deserialization(String),
    IO(String),
    InvalidAtom(String),
    MatrixError(crate::matrix::MatrixError),
}

impl From<crate::matrix::MatrixError> for MatrixDeserError {
    fn from(err: crate::matrix::MatrixError) -> Self {
        MatrixDeserError::MatrixError(err)
    }
}

impl From<serde_json::Error> for MatrixDeserError {
    fn from(err: serde_json::Error) -> MatrixDeserError {
        MatrixDeserError::Deserialization(format!("{}", err))
    }
}

impl From<std::str::Utf8Error> for MatrixDeserError {
    fn from(err: std::str::Utf8Error) -> MatrixDeserError {
        MatrixDeserError::Deserialization(format!("{}", err))
    }
}

impl From<std::io::Error> for MatrixDeserError {
    fn from(err: std::io::Error) -> MatrixDeserError {
        MatrixDeserError::IO(format!("{}", err))
    }
}

fn deserialize_atom(v: &Value) -> Result<SAtom, MatrixDeserError> {
    match v[0].as_str().unwrap_or("?") {
        "i" => {
            if let Some(v) = v[1].as_i64() {
                Ok(SAtom::setting(v))
            } else {
                Err(MatrixDeserError::InvalidAtom(v.to_string()))
            }
        }
        "p" => {
            if let Some(v) = v[1].as_f64() {
                Ok(SAtom::param(v as f32))
            } else {
                Err(MatrixDeserError::InvalidAtom(v.to_string()))
            }
        }
        "s" => {
            if let Some(v) = v[1].as_str() {
                Ok(SAtom::str(v))
            } else {
                Err(MatrixDeserError::InvalidAtom(v.to_string()))
            }
        }
        "as" => {
            if let Some(v) = v[1].as_str() {
                Ok(SAtom::audio_unloaded(v))
            } else {
                Err(MatrixDeserError::InvalidAtom(v.to_string()))
            }
        }
        "ms" => {
            let mut buf: [f32; 8] = [0.0; 8];

            for i in 0..8 {
                if let Some(v) = v[i + 1].as_f64() {
                    buf[i] = v as f32;
                } else {
                    return Err(MatrixDeserError::InvalidAtom(v.to_string()));
                }
            }

            Ok(SAtom::micro(&buf))
        }
        _ => Err(MatrixDeserError::InvalidAtom(v.to_string())),
    }
}

fn serialize_atom(atom: &SAtom) -> Value {
    match atom {
        SAtom::MicroSample(s) => json!(["ms", s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7],]),
        SAtom::Str(s) => json!(["s", s]),
        SAtom::AudioSample((s, _)) => json!(["as", s]),
        SAtom::Setting(i) => json!(["i", i]),
        SAtom::Param(p) => json!(["p", p]),
    }
}

impl MatrixRepr {
    pub fn empty() -> Self {
        let cells = vec![];
        let params = vec![];
        let atoms = vec![];
        let patterns = vec![];
        let properties = vec![];
        let block_funs = vec![];

        Self { cells, params, atoms, patterns, block_funs, properties, version: 2 }
    }

    pub fn write_to_file(&mut self, filepath: &str) -> std::io::Result<()> {
        use std::fs::OpenOptions;
        use std::io::prelude::*;

        let tmp_filepath = format!("{}~", filepath);

        let mut ser = self.serialize();
        ser.push('\n');

        let mut file =
            OpenOptions::new().create(true).write(true).truncate(true).open(&tmp_filepath)?;
        file.write_all(ser.as_bytes())?;
        std::fs::rename(&tmp_filepath, &filepath)?;

        Ok(())
    }

    pub fn read_from_file(filepath: &str) -> Result<MatrixRepr, MatrixDeserError> {
        use std::fs::OpenOptions;
        use std::io::prelude::*;

        let mut file = OpenOptions::new().write(false).create(false).read(true).open(&filepath)?;

        let mut contents: Vec<u8> = Vec::new();
        file.read_to_end(&mut contents)?;

        let s = std::str::from_utf8(&contents)?;

        MatrixRepr::deserialize(s)
    }

    pub fn deserialize(s: &str) -> Result<MatrixRepr, MatrixDeserError> {
        let v: Value = serde_json::from_str(s)?;

        let mut m = MatrixRepr::empty();

        if let Some(version) = v.get("VERSION") {
            let version: i64 = version.as_i64().unwrap_or(0);

            if version > 2 {
                return Err(MatrixDeserError::BadVersion);
            }

            m.version = version;
        }

        let cells = &v["cells"];
        if let Value::Array(cells) = cells {
            for c in cells.iter() {
                m.cells.push(CellRepr::deserialize(c)?);
            }
        }

        let params = &v["params"];
        if let Value::Array(params) = params {
            for v in params.iter() {
                let node_id = deserialize_node_id(&v, 0, 1)?;
                let param_id = node_id.inp_param(v[2].as_str().unwrap_or(""));

                if let Some(param_id) = param_id {
                    m.params.push((
                        param_id,
                        v[3].as_f64().unwrap_or(0.0) as f32,
                        v[4].as_f64().map(|v| v as f32),
                    ));
                } else {
                    return Err(MatrixDeserError::UnknownParamId(v.to_string()));
                }
            }
        }

        let atoms = &v["atoms"];
        if let Value::Array(atoms) = atoms {
            for v in atoms.iter() {
                let node_id = deserialize_node_id(&v, 0, 1)?;
                let param_id = node_id.inp_param(v[2].as_str().unwrap_or(""));

                if let Some(param_id) = param_id {
                    m.atoms.push((param_id, deserialize_atom(&v[3])?))
                    //d// } else {
                    //d//     return Err(
                    //d//         MatrixDeserError::UnknownParamId(v.to_string()));
                }
            }
        }

        let props = &v["props"];
        if let Value::Array(props) = props {
            for v in props.iter() {
                let key = v[0].as_str().unwrap_or("");
                m.properties.push((key.to_string(), deserialize_atom(&v[1])?));
            }
        }

        let patterns = &v["patterns"];
        if let Value::Array(patterns) = patterns {
            for p in patterns.iter() {
                m.patterns.push(if p.is_object() {
                    Some(PatternRepr::deserialize(&p)?)
                } else {
                    None
                });
            }
        }

        let block_funs = &v["block_funs"];
        if let Value::Array(block_funs) = block_funs {
            for p in block_funs.iter() {
                m.block_funs.push(if p.is_object() {
                    Some(BlockFunSnapshot::deserialize(&p)?)
                } else {
                    None
                });
            }
        }

        Ok(m)
    }

    pub fn serialize(&mut self) -> String {
        let mut v = json!({
            "VERSION": self.version,
        });

        self.properties.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        self.params.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        self.atoms.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut params = json!([]);
        if let Value::Array(params) = &mut params {
            for (p, v, ma) in self.params.iter() {
                let mut param_v = json!([p.node_id().name(), p.node_id().instance(), p.name(), v,]);

                if let Value::Array(param_v) = &mut param_v {
                    if let Some(ma) = ma {
                        param_v.push(json!(ma));
                    }
                }

                params.push(param_v);
            }
        }

        v["params"] = params;

        let mut atoms = json!([]);
        if let Value::Array(atoms) = &mut atoms {
            for (p, v) in self.atoms.iter() {
                atoms.push(json!([
                    p.node_id().name(),
                    p.node_id().instance(),
                    p.name(),
                    serialize_atom(v),
                ]));
            }
        }

        v["atoms"] = atoms;

        let mut props = json!([]);
        if let Value::Array(props) = &mut props {
            for (k, v) in self.properties.iter() {
                props.push(json!([k, serialize_atom(v)]));
            }
        }

        v["props"] = props;

        let mut cells = json!([]);
        if let Value::Array(cells) = &mut cells {
            for cell in self.cells.iter() {
                cells.push(cell.serialize());
            }
        }

        v["cells"] = cells;

        let mut patterns = json!([]);
        if let Value::Array(patterns) = &mut patterns {
            for p in self.patterns.iter() {
                patterns.push(if let Some(p) = p { p.serialize() } else { Value::Null });
            }
        }

        v["patterns"] = patterns;

        let mut block_funs = json!([]);
        if let Value::Array(block_funs) = &mut block_funs {
            for p in self.block_funs.iter() {
                block_funs.push(if let Some(p) = p { p.serialize() } else { Value::Null });
            }
        }

        v["block_funs"] = block_funs;

        v.to_string()
    }
}

pub fn load_patch_from_file(
    matrix: &mut crate::matrix::Matrix,
    filepath: &str,
) -> Result<(), MatrixDeserError> {
    let mr = MatrixRepr::read_from_file(filepath)?;
    matrix.from_repr(&mr)?;
    Ok(())
}

pub fn save_patch_to_file(
    matrix: &mut crate::matrix::Matrix,
    filepath: &str,
) -> std::io::Result<()> {
    let mut mr = matrix.to_repr();
    mr.write_to_file(filepath)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::matrix::{Cell, Matrix};

    #[test]
    fn check_empty_repr_serialization() {
        let mut matrix_repr = MatrixRepr::empty();

        let s = matrix_repr.serialize();

        assert_eq!(
            s,
            "{\"VERSION\":2,\"atoms\":[],\"block_funs\":[],\"cells\":[],\"params\":[],\"patterns\":[],\"props\":[]}"
        );
        assert!(MatrixRepr::deserialize(&s).is_ok());
    }

    #[test]
    fn check_repr_serialization() {
        use crate::nodes::new_node_engine;

        let (node_conf, mut _node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        let sin = NodeId::Sin(2);

        matrix.place(0, 0, Cell::empty(sin).out(None, Some(0), None));
        matrix.place(
            1,
            0,
            Cell::empty(NodeId::Out(0)).input(None, Some(0), None).out(None, None, Some(0)),
        );
        matrix.sync().unwrap();

        let freq_param = sin.inp_param("freq").unwrap();
        matrix.set_param(freq_param, SAtom::param(-0.1));

        let mut mr = matrix.to_repr();

        let s = mr.serialize();

        assert_eq!(s,
            "{\"VERSION\":2,\"atoms\":[[\"out\",0,\"mono\",[\"i\",0]]],\"block_funs\":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null],\"cells\":[[\"sin\",2,0,0,[-1,-1,-1],[-1,\"sig\",-1]],[\"out\",0,1,0,[-1,\"ch1\",-1],[-1,-1,-1]]],\"params\":[[\"out\",0,\"ch1\",0.0],[\"out\",0,\"ch2\",0.0],[\"sin\",0,\"det\",0.0],[\"sin\",1,\"det\",0.0],[\"sin\",2,\"det\",0.0],[\"sin\",0,\"freq\",440.0],[\"sin\",1,\"freq\",440.0],[\"sin\",2,\"freq\",220.0],[\"out\",0,\"gain\",1.0]],\"patterns\":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null],\"props\":[]}");
        let mut mr2 = MatrixRepr::deserialize(&s).unwrap();

        let s2 = mr2.serialize();

        assert_eq!(s, s2);
    }

    #[test]
    fn check_atom_repr() {
        let v = serialize_atom(&SAtom::str("foo"));
        assert_eq!(v.to_string(), "[\"s\",\"foo\"]");
        let s = serialize_atom(&deserialize_atom(&v).unwrap()).to_string();
        assert_eq!(s, v.to_string());

        let v = serialize_atom(&SAtom::setting(1337));
        assert_eq!(v.to_string(), "[\"i\",1337]");
        let s = serialize_atom(&deserialize_atom(&v).unwrap()).to_string();
        assert_eq!(s, v.to_string());

        let v = serialize_atom(&SAtom::param(1.0));
        assert_eq!(v.to_string(), "[\"p\",1.0]");
        let s = serialize_atom(&deserialize_atom(&v).unwrap()).to_string();
        assert_eq!(s, v.to_string());

        let v = serialize_atom(&SAtom::micro(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]));
        assert_eq!(v.to_string(), "[\"ms\",1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0]");
        let s = serialize_atom(&deserialize_atom(&v).unwrap()).to_string();
        assert_eq!(s, v.to_string());

        let v = serialize_atom(&SAtom::audio(
            "lol.wav",
            std::sync::Arc::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
        ));
        assert_eq!(v.to_string(), "[\"as\",\"lol.wav\"]");
        let s = serialize_atom(&deserialize_atom(&v).unwrap()).to_string();
        assert_eq!(s, v.to_string());
    }

    #[test]
    fn check_cell_repr() {
        let cell = Cell::empty(NodeId::Out(2)).input(Some(2), Some(0), Some(1)).out(
            Some(11),
            Some(4),
            Some(1),
        );
        let cr = cell.to_repr();

        let s = cr.serialize().to_string();

        let v: Value = serde_json::from_str(&s).unwrap();
        let cr2 = CellRepr::deserialize(&v).unwrap();

        let s2 = cr2.serialize().to_string();
        assert_eq!(s, s2);
    }

    #[test]
    fn check_file_repr() {
        let orig_serial = {
            use crate::nodes::new_node_engine;

            let (node_conf, mut _node_exec) = new_node_engine();
            let mut matrix = Matrix::new(node_conf, 3, 3);

            let sin = NodeId::Sin(2);

            matrix.place(0, 0, Cell::empty(sin).out(None, Some(0), None));
            matrix.place(
                1,
                0,
                Cell::empty(NodeId::Out(0)).input(None, Some(0), None).out(None, None, Some(0)),
            );
            matrix.sync().unwrap();

            let freq_param = sin.inp_param("freq").unwrap();
            matrix.set_param(freq_param, SAtom::param(-0.1));

            let mut mr = matrix.to_repr();
            let s2 = mr.serialize().to_string();

            save_patch_to_file(&mut matrix, "hexosynth_test_patch.hxy").unwrap();

            s2
        };

        {
            use crate::nodes::new_node_engine;

            let (node_conf, mut _node_exec) = new_node_engine();
            let mut matrix = Matrix::new(node_conf, 3, 3);

            load_patch_from_file(&mut matrix, "hexosynth_test_patch.hxy").unwrap();

            let mut mr = matrix.to_repr();
            let s = mr.serialize().to_string();

            assert_eq!(s, orig_serial);
        }
    }

    #[test]
    fn check_matrix_track_repr() {
        use crate::dsp::tracker::UIPatternModel;

        let orig_serial = {
            use crate::nodes::new_node_engine;

            let (node_conf, mut _node_exec) = new_node_engine();
            let mut matrix = Matrix::new(node_conf, 3, 3);

            let ts = NodeId::TSeq(0);

            matrix.place(0, 0, Cell::empty(ts).out(None, Some(0), None));
            matrix.sync().unwrap();

            {
                let pat_ref = matrix.get_pattern_data(0).unwrap();
                let mut pat = pat_ref.lock().unwrap();

                for col in 0..MAX_COLS {
                    pat.set_col_note_type(col);
                    for v in 1..(MAX_PATTERN_LEN + 1) {
                        pat.set_cell_value(v - 1, col, v as u16);
                    }

                    pat.set_cursor(16, 3);
                    pat.set_edit_step(5);
                    pat.set_rows(133);
                }
            }

            let mut mr = matrix.to_repr();
            let s2 = mr.serialize().to_string();

            save_patch_to_file(&mut matrix, "hexosynth_test_patch_2.hxy").unwrap();

            s2
        };

        {
            use crate::nodes::new_node_engine;

            let (node_conf, mut _node_exec) = new_node_engine();
            let mut matrix = Matrix::new(node_conf, 3, 3);

            load_patch_from_file(&mut matrix, "hexosynth_test_patch_2.hxy").unwrap();

            let mut mr = matrix.to_repr();
            let s = mr.serialize().to_string();

            assert_eq!(s, orig_serial);

            let pat_ref = matrix.get_pattern_data(0).unwrap();
            let mut pat = pat_ref.lock().unwrap();

            for col in 0..MAX_COLS {
                assert!(pat.is_col_note(col));
                for v in 1..(MAX_PATTERN_LEN + 1) {
                    assert_eq!(pat.get_cell_value(v - 1, col), v as u16);
                }

                assert_eq!(pat.get_cursor(), (16, 3));
                assert_eq!(pat.get_edit_step(), 5);
                assert_eq!(pat.rows(), 133);
            }
        }
    }

    #[test]
    fn check_matrix_repr_version_1_load() {
        use crate::nodes::new_node_engine;

        let s =
            "{\"VERSION\":1,\"atoms\":[[\"out\",0,\"mono\",[\"i\",0]]],\"cells\":[[\"sin\",2,0,0,[-1,-1,-1],[-1,\"sig\",-1]],[\"out\",0,1,0,[-1,\"ch1\",-1],[-1,-1,-1]]],\"params\":[[\"out\",0,\"ch1\",0.0],[\"out\",0,\"ch2\",0.0],[\"sin\",0,\"det\",0.0],[\"sin\",1,\"det\",0.0],[\"sin\",2,\"det\",0.0],[\"sin\",0,\"freq\",0.0],[\"sin\",1,\"freq\",0.0],[\"sin\",2,\"freq\",-0.10000000149011612],[\"out\",0,\"gain\",0.5]],\"patterns\":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null],\"props\":[]}";

        {
            let (node_conf, mut _node_exec) = new_node_engine();
            let mut matrix = Matrix::new(node_conf, 3, 3);

            let mr = MatrixRepr::deserialize(&s).unwrap();
            matrix.from_repr(&mr).unwrap();

            let mut v = std::collections::HashMap::new();
            matrix.for_each_atom(|unique_idx, param_id, atom, _modamt| {
                v.insert(
                    format!("{}_{}", unique_idx, param_id.name().to_string()),
                    param_id.denorm(atom.f()),
                );
            });

            assert_eq!(*v.get("0_freq").unwrap(), 440.0);
            assert_eq!(*v.get("1_freq").unwrap(), 440.0);
            assert_eq!(*v.get("2_freq").unwrap(), 220.0);
            assert_eq!(*v.get("0_det").unwrap(), 0.0);
        }
    }

    #[test]
    fn check_matrix_repr_version_2_load() {
        use crate::nodes::new_node_engine;

        let s =
            "{\"VERSION\":2,\"atoms\":[[\"out\",0,\"mono\",[\"i\",0]]],\"cells\":[[\"sin\",2,0,0,[-1,-1,-1],[-1,\"sig\",-1]],[\"out\",0,1,0,[-1,\"ch1\",-1],[-1,-1,-1]]],\"params\":[[\"out\",0,\"ch1\",0.0],[\"out\",0,\"ch2\",0.0],[\"sin\",0,\"det\",0.0],[\"sin\",1,\"det\",0.0],[\"sin\",2,\"det\",0.0],[\"sin\",0,\"freq\",440.0],[\"sin\",1,\"freq\",441.0],[\"sin\",2,\"freq\",220.0],[\"out\",0,\"gain\",0.5]],\"patterns\":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null],\"props\":[]}";

        {
            let (node_conf, mut _node_exec) = new_node_engine();
            let mut matrix = Matrix::new(node_conf, 3, 3);

            let mr = MatrixRepr::deserialize(&s).unwrap();
            matrix.from_repr(&mr).unwrap();

            let mut v = std::collections::HashMap::new();
            matrix.for_each_atom(|unique_idx, param_id, atom, _modamt| {
                v.insert(
                    format!("{}_{}", unique_idx, param_id.name().to_string()),
                    param_id.denorm(atom.f()),
                );
            });

            assert_eq!(*v.get("0_freq").unwrap(), 440.0);
            assert_eq!(*v.get("1_freq").unwrap(), 441.0);
            assert_eq!(*v.get("2_freq").unwrap(), 220.0);
            assert_eq!(*v.get("0_det").unwrap(), 0.0);
        }
    }

    #[test]
    fn check_matrix_repr_mod_amt_1() {
        use crate::nodes::new_node_engine;

        let s = {
            let (node_conf, mut _node_exec) = new_node_engine();
            let mut matrix = Matrix::new(node_conf, 3, 3);

            let sin = NodeId::Sin(0);

            matrix.place(0, 0, Cell::empty(sin).out(None, Some(0), None));
            matrix.place(
                1,
                0,
                Cell::empty(NodeId::Out(0)).input(None, Some(0), None).out(None, None, Some(0)),
            );
            matrix.set_param_modamt(sin.inp_param("det").unwrap(), Some(-0.6)).unwrap();
            matrix.sync().unwrap();

            matrix.set_param_modamt(sin.inp_param("freq").unwrap(), Some(0.6)).unwrap();

            let mut mr = matrix.to_repr();
            mr.serialize().to_string()
        };

        {
            let (node_conf, mut _node_exec) = new_node_engine();
            let mut matrix = Matrix::new(node_conf, 3, 3);

            let mr = MatrixRepr::deserialize(&s).unwrap();
            matrix.from_repr(&mr).unwrap();

            let mut v = std::collections::HashMap::new();
            matrix.for_each_atom(|_unique_idx, param_id, _atom, modamt| {
                v.insert(param_id.name().to_string(), modamt);
            });

            assert_eq!(*v.get("freq").unwrap(), Some(0.6));
            assert_eq!(*v.get("det").unwrap(), Some(-0.6));
        }
    }

    #[test]
    fn check_matrix_repr_properties() {
        use crate::nodes::new_node_engine;

        let s = {
            let (node_conf, mut _node_exec) = new_node_engine();
            let mut matrix = Matrix::new(node_conf, 3, 3);

            matrix.set_prop("test", SAtom::setting(31337));

            let mut mr = matrix.to_repr();
            mr.serialize().to_string()
        };

        {
            let (node_conf, mut _node_exec) = new_node_engine();
            let mut matrix = Matrix::new(node_conf, 3, 3);

            let mr = MatrixRepr::deserialize(&s).unwrap();
            matrix.from_repr(&mr).unwrap();

            assert_eq!(matrix.get_prop("test").unwrap().i(), 31337);
        }
    }

    #[test]
    fn check_matrix_repr_old_format2new() {
        let old_format = "{\"VERSION\":1,\"atoms\":[[\"out\",0,\"mono\",[\"i\",0]]],\
             \"cells\":[[\"sin\",2,0,0,[-1,-1,-1],[-1,0,-1]],[\"out\",0,1,0,\
             [-1,0,-1],[-1,-1,0]]],\"params\":[[\"out\",0,\"ch1\",0.0],\
             [\"out\",0,\"ch2\",0.0],[\"sin\",0,\"det\",0.0],[\"sin\",1,\
             \"det\",0.0],[\"sin\",2,\"det\",0.0],[\"sin\",0,\"freq\",0.0],\
             [\"sin\",1,\"freq\",0.0],[\"sin\",2,\"freq\",\
             -0.10000000149011612],[\"out\",0,\"gain\",0.5]],\"patterns\":\
             [null,null,null,null,null,null,null,null,null,null,null,null,\
             null,null,null,null,null,null,null,null,null,null,null,null,\
             null,null,null,null,null,null,null,null,null,null,null,null,\
             null,null,null,null,null,null,null,null,null,null,null,null,\
             null,null,null,null,null,null,null,null,null,null,null,null,\
             null,null,null,null,null,null,null,null,null,null,null,null,\
             null,null,null,null,null,null,null,null,null,null,null,null,\
             null,null,null,null,null,null,null,null,null,null,null,null,\
             null,null,null,null,null,null,null,null,null,null,null,null,\
             null,null,null,null,null,null,null,null,null,null,null,null,\
             null,null,null,null,null,null,null,null],\"props\":[]}";

        let mut mr = MatrixRepr::deserialize(old_format).unwrap();
        let new_format = mr.serialize().to_string();

        assert_eq!(new_format,
            "{\"VERSION\":1,\"atoms\":[[\"out\",0,\"mono\",[\"i\",0]]],\"block_funs\":[],\"cells\":[[\"sin\",2,0,0,[-1,-1,-1],[-1,\"sig\",-1]],[\"out\",0,1,0,[-1,\"ch1\",-1],[-1,-1,-1]]],\"params\":[[\"out\",0,\"ch1\",0.0],[\"out\",0,\"ch2\",0.0],[\"sin\",0,\"det\",0.0],[\"sin\",1,\"det\",0.0],[\"sin\",2,\"det\",0.0],[\"sin\",0,\"freq\",0.0],[\"sin\",1,\"freq\",0.0],[\"sin\",2,\"freq\",-0.10000000149011612],[\"out\",0,\"gain\",0.5]],\"patterns\":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null],\"props\":[]}");
    }
}
