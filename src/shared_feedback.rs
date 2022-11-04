// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

//! Provides an implementation for a shared feedback buffer for the DSP node graph.
//! It is used for instance by the `FbWr` and `FbRd` nodes to implement their functionality.
//!
//! See also [crate::NodeGlobalData] which provides the [SharedFeedback] to the nodes.

use crate::dsp::MAX_BLOCK_SIZE;
use std::sync::Arc;
use synfx_dsp::AtomicFloat;

pub const FB_DELAY_LENGTH_MS: f32 = 3.14;

/// The SharedFeedback is a feedback delay buffer for the `FbWr` and `FbRd` nodes.
///
/// They have a fixed delay of 3.14ms, which should be equal for all sample rates above 42kHz.
/// Below that the delay might be longer to accomodate the [crate::dsp::MAX_BLOCK_SIZE].
///
/// See also [crate::NodeGlobalData] which provides the [SharedFeedback] to the DSP nodes.
#[derive(Debug, Clone)]
pub struct SharedFeedback {
    buffer: Arc<Vec<AtomicFloat>>,
    delay_sample_count: usize,
}

impl SharedFeedback {
    pub fn new(sample_rate: f32) -> Self {
        let mut buf = vec![];
        let delay_sample_count = ((sample_rate * FB_DELAY_LENGTH_MS) / 1000.0) as usize;

        // Ensure we got at least MAX_BLOCK_SIZE though!
        let delay_sample_count = delay_sample_count.max(MAX_BLOCK_SIZE);

        // Multiply by 3, to make ample space for the FB_DELAY_LENGTH_MS,
        // probably 2*delay_sample_count would be fine too,
        // but I'm anxious about off by one bugs :-)
        buf.resize_with(3 * delay_sample_count, || AtomicFloat::new(0.0));

        Self { buffer: Arc::new(buf), delay_sample_count }
    }
}

/// This instance writes into the [SharedFeedback] buffer.
///
/// Even though it's safe to have multiple writers of this will not work
/// or produce any meaningful results. The goal is really, that one `FbWr` DSP node
/// in the audio thread writes the buffer, and one (or multiple) `FbRd` DSP nodes
/// read from that [SharedFeedback] buffer via a [SharedFeedbackReader].
#[derive(Debug, Clone)]
pub struct SharedFeedbackWriter {
    buffer: Arc<Vec<AtomicFloat>>,
    write_ptr: usize,
    delay_sample_count: usize,
}

impl SharedFeedbackWriter {
    pub fn new(sfb: &SharedFeedback) -> Self {
        let buffer = sfb.buffer.clone();
        Self {
            buffer,
            delay_sample_count: sfb.delay_sample_count,
            write_ptr: sfb.delay_sample_count,
        }
    }

    /// Write the next sample in to the feedback buffer.
    ///
    /// Even though it's safe to have multiple writers of this will not work
    /// or produce any meaningful results. The goal is really, that one `FbWr` DSP node
    /// on the audio thread writing the buffer per buffer iteration. And then one or more
    /// `FbRd` DSP node reading from that buffer.
    pub fn write(&mut self, s: f32) {
        self.buffer[self.write_ptr].set(s);
        self.write_ptr = (self.write_ptr + 1) % self.delay_sample_count;
    }
}

/// A reader for the [SharedFeedback] buffer, used to implement the `FbRd` DSP node.
///
/// Multiple readers are okay, and you can even read from the buffer across the threads.
/// It is sound to read from another thread. But keep in mind, that this is not a ring buffer
/// and you will get partially written buffer contents. There is also only a per sample reading
/// API, that means without the current sample rate you will not know how many samples the 3.14ms
/// buffer is big.
#[derive(Debug, Clone)]
pub struct SharedFeedbackReader {
    buffer: Arc<Vec<AtomicFloat>>,
    read_ptr: usize,
    delay_sample_count: usize,
}

impl SharedFeedbackReader {
    pub fn new(sfb: &SharedFeedback) -> Self {
        Self { buffer: sfb.buffer.clone(), delay_sample_count: sfb.delay_sample_count, read_ptr: 0 }
    }

    /// Read the next sample from the buffer. Wraps around after some internal buffer
    /// size (that is consistent with the [SharedFeedback] buffer size). Used by `FbRd` DSP node
    /// to do it's functionality.
    pub fn read(&mut self) -> f32 {
        let ret = self.buffer[self.read_ptr].get();
        self.read_ptr = (self.read_ptr + 1) % self.delay_sample_count;
        ret
    }
}
