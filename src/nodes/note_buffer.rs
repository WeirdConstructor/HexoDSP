// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::MAX_BLOCK_SIZE;

#[derive(Debug, Clone, Copy)]
pub struct NoteChannelState {
    pub vel: f32,
    pub note: u8,
    pub gate: u8,
}

impl std::fmt::Display for NoteChannelState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "C<N={},G={},V={:5.3}>", self.note, self.gate, self.vel)
    }
}

impl NoteChannelState {
    pub fn new() -> Self {
        Self { vel: 0.0, note: 0, gate: 0 }
    }
}

pub struct NoteBuffer {
    interleaved_chans: Vec<NoteChannelState>,
    buf_idx: usize,
}

impl NoteBuffer {
    pub fn new() -> Self {
        Self { interleaved_chans: vec![NoteChannelState::new(); 16 * MAX_BLOCK_SIZE], buf_idx: 15 }
    }

    #[inline]
    pub fn reset(&mut self) {
        let cur = self.buf_idx;
        if cur != 0 {
            self.interleaved_chans.copy_within((cur * 16)..((cur + 1) * 16), 0);
            self.buf_idx = 0;
        }
    }

    #[inline]
    pub fn step(&mut self) {
        let cur = self.buf_idx;
        let next = (self.buf_idx + 1) % MAX_BLOCK_SIZE;
        println!("COPY {}..{} => {}", (cur * 16), ((cur + 1) * 16), next * 16);
        self.interleaved_chans.copy_within((cur * 16)..((cur + 1) * 16), next * 16);
        self.buf_idx = next;
    }

    #[inline]
    pub fn note_on(&mut self, channel: u8, note: u8) {
        let mut chan = &mut self.interleaved_chans[(self.buf_idx * 16) + (channel as usize % 16)];
        if chan.gate == 0 {
            chan.gate = 1;
            chan.note = note;
        }
    }

    #[inline]
    pub fn note_off(&mut self, channel: u8, note: u8) {
        let mut chan = &mut self.interleaved_chans[(self.buf_idx * 16) + (channel as usize % 16)];
        if chan.gate == 1 && chan.note == note {
            chan.gate = 0;
            chan.note = 0;
        }
    }

    #[inline]
    pub fn set_velocity(&mut self, channel: u8, vel: f32) {
        self.interleaved_chans[(self.buf_idx * 16) + (channel as usize % 16)].vel = vel;
    }

    #[inline]
    pub fn get_chan_at(&self, channel: u8, frame: u8) -> &NoteChannelState {
        &self.interleaved_chans[frame as usize * 16 + (channel as usize % 16)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_note_buffer() {
        let mut buf = NoteBuffer::new();

        buf.reset();
        buf.note_on(0, 10);
        buf.note_on(2, 12);
        buf.set_velocity(0, 0.6);
        buf.set_velocity(2, 0.8);
        //d// println!("> {:?}", buf.get_chan_at(0, 0));
        buf.step();
        buf.note_on(0, 11);
        for _ in 0..10 {
            buf.step();
        }
        buf.note_off(0, 11);
        buf.step();
        buf.note_off(0, 10);
        buf.step();
        buf.set_velocity(2, 0.4);
        //d// println!("> {:?}", buf.get_chan_at(0, 0));
        for i in 0..(MAX_BLOCK_SIZE - 14) {
            buf.step();
        }

        //d// for i in 0..MAX_BLOCK_SIZE {
        //d//     println!(">{} {}", i, buf.get_chan_at(2, i as u8));
        //d// }

        assert_eq!(buf.get_chan_at(0, 0).to_string(), "C<N=10,G=1,V=0.600>");
        assert_eq!(buf.get_chan_at(0, 12).to_string(), "C<N=0,G=0,V=0.600>");
        assert_eq!(buf.get_chan_at(2, 0).to_string(), "C<N=12,G=1,V=0.800>");
        assert_eq!(buf.get_chan_at(2, 127).to_string(), "C<N=12,G=1,V=0.400>");
    }
}
