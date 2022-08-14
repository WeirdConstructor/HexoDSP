// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

#[derive(Debug, Clone, Copy)]
pub struct HxTimedEvent {
    /// The frame number in the current block by the audio driver or plugin API/DAW
    timing: usize,
    kind: HxMidiEvent,
}

impl HxTimedEvent {
    pub fn new_timed(timing: usize, kind: HxMidiEvent) -> Self {
        Self { timing, kind }
    }

    pub fn is_cc(&self) -> bool {
        if let HxMidiEvent::CC { .. } = self.kind {
            true
        } else {
            false
        }
    }

    pub fn kind(&self) -> HxMidiEvent {
        self.kind
    }

    pub fn cc(timing: usize, channel: u8, cc: u8, value: f32) -> Self {
        Self { timing, kind: HxMidiEvent::CC { channel, cc, value } }
    }

    pub fn note_on(timing: usize, channel: u8, note: u8, vel: f32) -> Self {
        Self { timing, kind: HxMidiEvent::NoteOn { channel, note, vel } }
    }

    pub fn note_off(timing: usize, channel: u8, note: u8) -> Self {
        Self { timing, kind: HxMidiEvent::NoteOff { channel, note } }
    }
}

pub struct MidiEventPointer<'a> {
    buf: &'a [HxTimedEvent],
    idx: usize,
}

impl<'a> MidiEventPointer<'a> {
    pub fn new(buf: &'a [HxTimedEvent]) -> Self {
        Self { buf, idx: 0 }
    }

    pub fn next_at(&mut self, time: usize) -> Option<HxMidiEvent> {
        if self.idx < self.buf.len() && self.buf[self.idx].timing <= time {
            self.idx += 1;
            Some(self.buf[self.idx - 1].kind)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HxMidiEvent {
    NoteOn { channel: u8, note: u8, vel: f32 },
    NoteOff { channel: u8, note: u8 },
    CC { channel: u8, cc: u8, value: f32 },
}

pub struct EventWindowing {
    pub event: Option<HxTimedEvent>,
}

impl EventWindowing {
    pub fn new() -> Self {
        Self { event: None }
    }

    #[inline]
    pub fn feed_me(&self) -> bool {
        self.event.is_none()
    }

    #[inline]
    pub fn feed(&mut self, event: HxTimedEvent) {
        self.event = Some(event);
    }

    #[inline]
    pub fn next_event_in_range(
        &mut self,
        to_time: usize,
        block_size: usize,
    ) -> Option<HxTimedEvent> {
        if let Some(event) = self.event.take() {
            if event.timing < (to_time + block_size) {
                return Some(HxTimedEvent { timing: event.timing - to_time, kind: event.kind });
            } else {
                self.event = Some(event);
            }
        }

        None
    }
}
