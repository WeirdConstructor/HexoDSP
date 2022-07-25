// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::util::AtomicFloat;
use std::sync::Arc;

/// A float buffer that can be written to and read from in an unsynchronized manner.
///
/// One use case is writing samples to this buffer in the audio thread while
/// a GUI thread reads from this buffer. Mostly useful for an oscilloscope.
///
///```
/// use hexodsp::UnsyncFloatBuf;
///
/// let handle1 = UnsyncFloatBuf::new_with_len(10);
/// let handle2 = handle1.clone();
///
/// std::thread::spawn(move || {
///     handle1.write(9, 2032.0);
/// }).join().unwrap();
///
/// std::thread::spawn(move || {
///     assert_eq!(handle2.read(9), 2032.0);
///     assert_eq!(handle2.read(20), 0.0); // out of range!
/// }).join().unwrap();
///```
#[derive(Debug, Clone)]
pub struct UnsyncFloatBuf(Arc<UnsyncFloatBufImpl>);

impl UnsyncFloatBuf {
    /// Creates a new unsynchronized float buffer with the given length.
    pub fn new_with_len(len: usize) -> Self {
        Self(UnsyncFloatBufImpl::new_shared(len))
    }

    /// Write float to the given index.
    ///
    /// If index is out of range, nothing will be written.
    pub fn write(&self, idx: usize, v: f32) {
        self.0.write(idx, v)
    }

    /// Reads a float from the given index.
    ///
    /// If index is out of range, 0.0 will be returned.
    pub fn read(&self, idx: usize) -> f32 {
        self.0.read(idx)
    }
}

#[derive(Debug)]
struct UnsyncFloatBufImpl {
    data_store: Vec<AtomicFloat>,
    len: usize,
    ptr: *mut AtomicFloat,
}

unsafe impl Sync for UnsyncFloatBuf {}
unsafe impl Send for UnsyncFloatBuf {}

impl UnsyncFloatBufImpl {
    fn new_shared(len: usize) -> Arc<Self> {
        let mut rc = Arc::new(Self { data_store: Vec::new(), len, ptr: std::ptr::null_mut() });

        let mut unsync_buf = Arc::get_mut(&mut rc).expect("No other reference to this Arc");
        unsync_buf.data_store.resize_with(len, || AtomicFloat::new(0.0));
        // Taking the pointer to the Vec data buffer is fine,
        // because it will not be moved when inside the Arc.
        unsync_buf.ptr = unsync_buf.data_store.as_mut_ptr();

        rc
    }

    fn write(&self, idx: usize, v: f32) {
        if idx < self.len {
            unsafe {
                (*self.ptr.add(idx)).set(v);
            }
        }
    }

    fn read(&self, idx: usize) -> f32 {
        if idx < self.len {
            unsafe { (*self.ptr.add(idx)).get() }
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_unsync_float_buf_working() {
        let handle1 = UnsyncFloatBuf::new_with_len(512);
        for i in 0..512 {
            handle1.write(i, i as f32);
        }
        let handle2 = handle1.clone();
        for i in 0..512 {
            assert_eq!(handle2.read(i), i as f32);
        }
    }

    #[test]
    fn check_unsync_float_buf_thread() {
        let handle1 = UnsyncFloatBuf::new_with_len(512);
        let handle2 = handle1.clone();

        std::thread::spawn(move || {
            for i in 0..512 {
                handle1.write(i, i as f32);
            }
        })
        .join()
        .unwrap();

        std::thread::spawn(move || {
            for i in 0..512 {
                assert_eq!(handle2.read(i), i as f32);
            }
        })
        .join()
        .unwrap();
    }
}
