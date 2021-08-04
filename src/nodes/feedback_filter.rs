// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::NodeId;
use super::VisualSamplingFilter;

use std::collections::HashMap;

pub struct FeedbackFilter {
    led_filters:    HashMap<NodeId, VisualSamplingFilter>,
    out_filters:    HashMap<(NodeId, u8), VisualSamplingFilter>,
    recalc_state:   bool,
}

impl FeedbackFilter {
    pub fn new() -> Self {
        Self {
            led_filters:    HashMap::new(),
            out_filters:    HashMap::new(),
            recalc_state:   true,
        }
    }

    fn get_out_filter_for_node(&mut self, node_id: &NodeId, out: u8)
        -> &mut VisualSamplingFilter
    {
        self.out_filters
            .entry((*node_id, out))
            .or_insert_with(|| VisualSamplingFilter::new())
    }

    fn get_led_filter_for_node(&mut self, node_id: &NodeId) -> &mut VisualSamplingFilter {
        self.led_filters
            .entry(*node_id)
            .or_insert_with(|| VisualSamplingFilter::new())
    }

    pub fn trigger_recalc(&mut self) {
        self.recalc_state = !self.recalc_state;
    }

    pub fn get_led(&mut self, node_id: &NodeId, sample: f32) -> (f32, f32) {
        let recalc_state = self.recalc_state;
        let filter = self.get_led_filter_for_node(node_id);
        filter.get(recalc_state, sample)
    }

    pub fn get_out(&mut self, node_id: &NodeId, out: u8, sample: f32) -> (f32, f32) {
        let recalc_state = self.recalc_state;
        let filter = self.get_out_filter_for_node(node_id, out);
        filter.get(recalc_state, sample)
    }
}
