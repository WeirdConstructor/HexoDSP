// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

const VALUE_SAMPLING_FILTER_SIZE : usize = 10;

/// Accumulates the values for a single visible feedback value,
/// like an LED ([crate::Matrix::led_value_for]) or the
/// output feedbacks [crate::Matrix::out_fb_for] from the [crate::Matrix].
#[derive(Debug, Clone, Copy)]
pub struct VisualSamplingFilter {
    /// Holds a state bit, that is used to check if this
    /// filter needs to recalculate or not.
    recalc_state:   bool,

    /// Current write head into the sample buffer.
    write_ptr:      usize,

    /// Holds a set of the most recent samples to calculate
    /// the output.
    sample_buffer:  [f32; VALUE_SAMPLING_FILTER_SIZE],

    /// Holds the last output, will only be recalculated
    /// when necessary.
    last_output:    (f32, f32),
}

impl VisualSamplingFilter {
    pub fn new() -> Self {
        Self {
            recalc_state:   false,
            write_ptr:      0,
            sample_buffer:  [0.0; VALUE_SAMPLING_FILTER_SIZE],
            last_output:    (0.0, 0.0),
        }
    }

    /// Used to check if we need to update this filter.
    #[inline]
    fn needs_recalc(&mut self, recalc_value: bool) -> bool {
        if self.recalc_state != recalc_value {
            self.recalc_state = recalc_value;
            true
        } else {
            false
        }
    }

    /// Retrieves the current output value of the filter.
    /// Negate the input for `recalc_value` one each frame,
    /// to reduce access to the `retrieve_fn` to be done only
    /// once per frame and per [VisualSamplingFilter].
    ///
    ///```
    /// use hexodsp::nodes::visual_sampling_filter::*;
    ///
    /// let mut vsf = VisualSamplingFilter::new();
    ///
    /// let inputs = [-0.87, -0.8, 0.2, 0.75, 0.5, 0.0, 0.22];
    /// let mut recalc = true;
    ///
    /// let mut last_output = (0.0, 0.0);
    /// for ip in inputs {
    ///     last_output = vsf.get(recalc, ip);
    ///     recalc = !recalc;
    /// }
    ///
    /// assert_eq!(last_output, (0.87, 0.75));
    ///```
    pub fn get(&mut self, recalc_value: bool, sample: f32) -> (f32, f32) {
        if self.needs_recalc(recalc_value) {
            let write_ptr =
                (self.write_ptr + 1) % self.sample_buffer.len();
            self.write_ptr = write_ptr;

            self.sample_buffer[write_ptr] = sample;

            let mut neg_max : f32 = 0.0;
            let mut pos_max : f32 = 0.0;

            for v in self.sample_buffer.iter() {
                if *v >= 0.0 {
                    pos_max = pos_max.max((*v).abs());
                } else {
                    neg_max = neg_max.max((*v).abs());
                }
            }

            self.last_output = (neg_max, pos_max);
        }

        self.last_output
    }
}

