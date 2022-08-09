// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::MAX_BLOCK_SIZE;
use crate::util::Smoother;

#[derive(Debug, Clone, Copy)]
pub struct ChannelState {
    vel: f32,
    pitch: u8,
    gate: u8,
}

pub struct NoteBuffer {
    interleaved_chans: Vec<ChannelState>,
    buf_idx: usize,
}

impl NoteBuffer {
    pub fn new() -> Self {
        Self {
            interleaved_chans: vec![ChannelState::new(); 16 * MAX_BLOCK_SIZE],
            buf_idx: 15,
        }
    }

    #[inline]
    pub fn step(&mut self) {
        let cur = self.buf_idx;
        let next = (self.buf_idx + 1) % 16;
        self.interleaved_chans.copy_within((cur * 16)..((cur + 1) * 16), next * 16);
    }

//    pub fn play_velocity(&mut self, channel: u8, vel: f32) {
//        let mut vel = &mut self.velocity[channel % 16];
//        vel.set(vel.current(), vel);
//    }
}

