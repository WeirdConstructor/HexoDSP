// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

pub mod nodes;
#[allow(unused_macros)]
pub mod dsp;
pub mod matrix;
pub mod cell_dir;
pub mod monitor;
pub mod matrix_repr;
mod util;

pub use nodes::{new_node_engine, NodeConfigurator, NodeExecutor};
pub use cell_dir::CellDir;
pub use matrix::{Matrix, Cell};
pub use dsp::{NodeId, SAtom};
pub use matrix_repr::load_patch_from_file;
pub use matrix_repr::save_patch_to_file;

pub struct Context<'a, 'b, 'c, 'd> {
    pub nframes:    usize,
    pub output:     &'a mut [&'b mut [f32]],
    pub input:      &'c [&'d [f32]],
}

impl<'a, 'b, 'c, 'd> nodes::NodeAudioContext for Context<'a, 'b, 'c, 'd> {
    #[inline]
    fn nframes(&self) -> usize { self.nframes }

    #[inline]
    fn output(&mut self, channel: usize, frame: usize, v: f32) {
        self.output[channel][frame] = v;
    }

    #[inline]
    fn input(&mut self, channel: usize, frame: usize) -> f32 {
        self.input[channel][frame]
    }
}


pub fn test() -> bool {
    true
}
