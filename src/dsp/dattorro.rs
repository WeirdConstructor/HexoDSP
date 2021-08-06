// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

// This file contains a reverb implementation that is based
// on Jon Dattorro's 1997 reverb algorithm. It's also largely
// based on the C++ implementation from ValleyAudio / ValleyRackFree
//
// ValleyRackFree Copyright (C) 2020, Valley Audio Soft, Dale Johnson
// Adapted under the GPL-3.0-or-later License.

use crate::dsp::helpers::{
    AllPass,
    TriSawLFO,
    OnePoleLPF,
    OnePoleHPF,
    DelayBuffer,
    DCBlockFilter
};

pub struct DattorroReverb {
    inp_dc_block:   [DCBlockFilter; 2],
    out_dc_block:   [DCBlockFilter; 2],

    lfos: [TriSawLFO; 4],

    input_hpf: OnePoleHPF,
    input_lpf: OnePoleLPF,

    pre_delay:  DelayBuffer,
    input_apfs: [AllPass; 4],

    apf1:   [AllPass; 2],
    hpf:    [OnePoleHPF; 2],
    lpf:    [OnePoleLPF; 2],
    apf2:   [AllPass; 2],
    delay1: [DelayBuffer; 2],
    delay2: [DelayBuffer; 2],
}
