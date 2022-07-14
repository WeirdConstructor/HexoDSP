// Copyright (c) 2022 theloni-monk <theo.acooper@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext};
use crate::dsp::goertzel::*;

#[derive(Debug, Clone)]
pub struct Gz3Filt {
    g1: Goertzel,
    g2: Goertzel,
    g3: Goertzel,


    ofreq1:   f32,
    ofreq2:   f32,
    ofreq3:   f32,

    srate:   f32,
    olatency: usize, // how many samples before recomputing goertzel on new window
    frames_processed: usize,


    ogain:   f32,
}

impl Gz3Filt {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            g1: Goertzel::new(),
            g2: Goertzel::new(),
            g3: Goertzel::new(), 

            ofreq1: 220.0,
            ofreq2: 330.0,
            ofreq3: 440.0,

            olatency: 2048,
            frames_processed: 0,

            srate: 44100.0,
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
r#"Gz3Filt - Goertzel Filter (Fine Bandpass)
This is the implementation of a goertzel algorithm for extraction of a particular frequency. It is basically a fine bandpass around a specific frequency.

It can be used as a frequency follower to extract the amplitudes of 3 different frequencies from a signal.
"#;
}
const DEFAULT_BUFFSIZE:usize = 1000; //will get overwritten on first frame anyways
impl DspNode for Gz3Filt {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) {
        self.srate = srate;
        self.g1.setCoeff(self.ofreq1, DEFAULT_BUFFSIZE, srate);
        self.g2.setCoeff(self.ofreq2, DEFAULT_BUFFSIZE, srate);
        self.g3.setCoeff(self.ofreq3, DEFAULT_BUFFSIZE, srate);
        
        self.reset();
    }

    fn reset(&mut self) {
        self.g1.reset();
        self.g2.reset();
        self.g3.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, inp, denorm, at};

        let inp   = inp::Gz3Filt::inp(inputs);

        let freq1  = inp::Gz3Filt::freq1(inputs);
        let freq2  = inp::Gz3Filt::freq2(inputs);
        let freq3  = inp::Gz3Filt::freq3(inputs);

        let gain  = inp::Gz3Filt::gain(inputs);
        let out   = out::Gz3Filt::sig(outputs);

        //clamping parameters
        let cfreq1 = denorm::Gz3Filt::freq1(freq1, 0);
        let cfreq1 = cfreq1.clamp(0.0, 22000.0);
        let cfreq2 = denorm::Gz3Filt::freq2(freq2, 0);
        let cfreq2 = cfreq2.clamp(0.0, 22000.0);
        let cfreq3 = denorm::Gz3Filt::freq3(freq3, 0);
        let cfreq3 = cfreq3.clamp(0.0, 22000.0);
        
        let cgain = denorm::Gz3Filt::gain(gain, 0);

        let paramschanged = 
        (cfreq1 - self.ofreq1).abs() > 0.0001 
        || (cfreq2 - self.ofreq2).abs() > 0.0001 
        || (cfreq3 - self.ofreq3).abs() > 0.0001 
        ||(cgain - self.ogain).abs() > 0.0001;

        if paramschanged
        {
            // recalculate coeffs of all in the cascade
            self.g1.target_freq = cfreq1;
            self.ofreq1 = cfreq1;
            self.g2.target_freq = cfreq2;
            self.ofreq2 = cfreq2;
            self.g3.target_freq = cfreq3;
            self.ofreq3 = cfreq3;

            self.g1.reset();
            self.g2.reset();
            self.g3.reset();

            self.ogain = cgain; 
        }

        self.g1.setCoeff(cfreq1, ctx.nframes(), self.srate);
        self.g2.setCoeff(cfreq2, ctx.nframes(), self.srate);
        self.g3.setCoeff(cfreq3, ctx.nframes(), self.srate);
        
        if self.frames_processed > self.olatency{
            self.g1.reset();
            self.g2.reset();
            self.g3.reset();
        }

        for frame in 0..ctx.nframes() { //TODO: update coeff based on nframes
            let gain  = denorm::Gz3Filt::gain(gain, frame);

            let mut s = inp.read(frame);
            s = self.g1.tick(s) + self.g2.tick(s) + self.g3.tick(s);

            out.write(frame, s * gain);
        }

        self.frames_processed+=ctx.nframes();

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
    }
}
