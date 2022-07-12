//WRTIEME: convert to node
// Copyright (c) 2022 theloni-monk <theo.acooper@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext};
use crate::dsp::goertzel::*;

#[macro_export]
macro_rules! fa_goertzel_type { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "normal",
            _  => "?",
        };
    write!($formatter, "{}", s)
} } }


//WRITME: all of this stuff
/// A simple amplifier
#[derive(Debug, Clone)]
pub struct GzFilt {
    computer: Goertzel,
    srate:   f32,
    ofreq:   f32,
    ogain:   f32,
    otype:   u8,
}

impl GzFilt {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            computer: Goertzel::new(),
            srate: 1.0 / 44100.0,
            otype: 99,   // value that can't be set by the user
            ofreq: -2.0, // value that can't be set by the user
            ogain: -2.0, // value that can't be set by the user
        }
    }
    pub const inp : &'static str =
        "GzFilt inp\nSignal input\nRange: (-1..1)\n";
    pub const freq : &'static str =
        "GzFilt freq\nFrequency to extract.\nRange: (20..20000)\n";
    pub const gain : &'static str =
        "GzFilt gain\nFilter gain.\nRange: (0..1)\n";
    pub const sig : &'static str =
        "GzFilt sig\nFiltered signal output.\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"Goertzel Algorithm

This is the implementation of a goertzel algorithm for extraction of a particular frequency. It is basically a fine bandpass around a specific frequency.
"#;
    pub const HELP : &'static str =
r#"GzFilt - Goertzel Filter (Fine Bandpass)

This is the implementation of a goertzel algorithm for extraction of a particular frequency. It is basically a fine bandpass around a specific frequency.
"#;
}

impl DspNode for GzFilt {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) {
        self.srate = srate;
        self.computer.srate = srate;
        self.otype = 99; // cause recalculation of the filter

        self.reset();
    }

    fn reset(&mut self) {
        self.computer.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, inp, denorm, at};

        let inp   = inp::GzFilt::inp(inputs);
        let freq  = inp::GzFilt::freq(inputs);
        let gain  = inp::GzFilt::gain(inputs);
        let out   = out::GzFilt::sig(outputs);

        let cfreq = denorm::GzFilt::freq(freq, 0);
        let cfreq = cfreq.clamp(0.0, 22000.0);
        let cgain = denorm::GzFilt::gain(gain, 0);

        if  (cfreq - self.ofreq).abs() > 0.0001
           || (cgain - self.ogain).abs() > 0.0001
        {
            // recalculate coeffs of all in the cascade
            self.computer.target_freq = cfreq;
            self.computer.reset();

            self.ofreq = cfreq;
            self.ogain = cgain; 
        }

        for frame in 0..ctx.nframes() {
            let gain  = denorm::GzFilt::gain(gain, frame);

            let mut s = inp.read(frame);
            s = self.computer.tick(s);

            out.write(frame, s * gain);
        }

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
    }
}
