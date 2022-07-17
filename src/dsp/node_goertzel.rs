// Copyright (c) 2022 theloni-monk <theo.acooper@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::goertzel::*;
use crate::dsp::{DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::{NodeAudioContext, NodeExecContext};

#[derive(Debug, Clone)]
pub struct Gz3Filt {
    g1: Goertzel,
    g2: Goertzel,
    g3: Goertzel,

    ofreq1: f32,
    ofreq2: f32,
    ofreq3: f32,

    srate: f32,
    olatency: f32, // how many samples before recomputing goertzel on new window
    frames_processed: usize,

    tempf1: f32,
    tempf2: f32,
    tempf3: f32,

    ogain: f32,
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

            olatency: 2048.0,
            frames_processed: 0,

            tempf1: 0.0,
            tempf2: 0.0,
            tempf3: 0.0,

            srate: 44100.0,
            ogain: -2.0, // value that can't be set by the user
        }
    }
    pub const inp: &'static str = "GzFilt inp\nSignal input\nRange: (-1..1)\n";
    pub const freq1: &'static str = "GzFilt freq\nFrequency to extract.\nRange: (20..20000)\n";
    pub const freq2: &'static str = "GzFilt freq\nFrequency to extract.\nRange: (20..20000)\n";
    pub const freq3: &'static str = "GzFilt freq\nFrequency to extract.\nRange: (20..20000)\n";
    pub const latency : &'static str =
        "GzFilt latency\n How many samples to average the frequency strength over. Higher is more accurate but less time-specific\nRange: (256..65536)\n";
    pub const gain: &'static str = "GzFilt gain\nFilter gain.\nRange: (0..1)\n";
    pub const sigf1: &'static str =
        "GzFilt sig\nFiltered signal output with respect to the first frequency.\nRange: (-1..1)\n";
    pub const sigf2: &'static str = "GzFilt sig\nFiltered signal output with respect to the second frequency.\nRange: (-1..1)\n";
    pub const sigf3: &'static str =
        "GzFilt sig\nFiltered signal output with respect to the third frequency.\nRange: (-1..1)\n";

    pub const DESC: &'static str = r#"Goertzel Algorithm

This is the implementation of a goertzel algorithm for extraction of a particular frequency. It is basically a fine bandpass around a specific frequency.
"#;
    pub const HELP: &'static str = r#"Gz3Filt - Goertzel Filter (Fine Bandpass)
This is the implementation of a goertzel algorithm for extraction of a particular frequency. It is basically a fine bandpass around a specific frequency.

It can be used as a frequency follower to extract the amplitudes of 3 different frequencies from a signal.
"#;
}
const DEFAULT_BUFFSIZE: usize = 1000; //will get overwritten on first frame anyways
impl DspNode for Gz3Filt {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, srate: f32) {
        self.srate = srate;
        self.g1.setCoeff(self.ofreq1, DEFAULT_BUFFSIZE, srate);
        self.g2.setCoeff(self.ofreq2, DEFAULT_BUFFSIZE, srate);
        self.g3.setCoeff(self.ofreq3, DEFAULT_BUFFSIZE, srate);
        self.reset();
    }

    fn reset(&mut self) {
        self.tempf1 = 0.0;
        self.g1.reset();
        self.g2.reset();
        self.g3.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self,
        ctx: &mut T,
        _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{denorm, inp,out_idx};

        // aquiring params from context
        let inp = inp::Gz3Filt::inp(inputs);

        let latency = inp::Gz3Filt::latency(inputs);

        let gain = inp::Gz3Filt::gain(inputs);
        //let out_i = out_idx::Gz3Filt::sigf1(); //TODO: put this into out_idx crate
        let outf1;
        let mut outf2;
        let outf3;
        (outf1, outf2) = outputs.split_at_mut(1);
        (outf2, outf3) = outf2.split_at_mut(1); //assumes channels are same size

        let outf1 = &mut outf1[0];
        let outf2 = &mut outf2[0];
        let outf3 = &mut outf3[0];
        


        let freq1 = inp::Gz3Filt::freq1(inputs);
        let freq2 = inp::Gz3Filt::freq2(inputs);
        let freq3 = inp::Gz3Filt::freq3(inputs);

        // clamping parameters
        let cfreq1 = denorm::Gz3Filt::freq1(freq1, 0);
        let cfreq1 = cfreq1.clamp(0.0, 22000.0);
        let cfreq2 = denorm::Gz3Filt::freq2(freq2, 0);
        let cfreq2 = cfreq2.clamp(0.0, 22000.0);
        let cfreq3 = denorm::Gz3Filt::freq3(freq3, 0);
        let cfreq3 = cfreq3.clamp(0.0, 22000.0);

        let clatency = denorm::Gz3Filt::latency(latency, 0);
        let clatency = clatency.clamp(256.0, 65536.0);

        let cgain = denorm::Gz3Filt::gain(gain, 0);

        let paramschanged = (cfreq1 - self.ofreq1).abs() > 0.0001
            || (cfreq2 - self.ofreq2).abs() > 0.0001
            || (cfreq3 - self.ofreq3).abs() > 0.0001
            || (cgain - self.ogain).abs() > 0.0001
            || (clatency - self.olatency).abs() > 1.0;
        if paramschanged {
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

            self.olatency = clatency;

            self.ogain = cgain;
        }

        // recompute param based on buffer size
        self.g1.setCoeff(cfreq1, ctx.nframes(), self.srate);
        self.g2.setCoeff(cfreq2, ctx.nframes(), self.srate);
        self.g3.setCoeff(cfreq3, ctx.nframes(), self.srate);

        // latency winds up rounding to int multiple of buffer size because thats simpler

        let mut s1 = 0.0;
        let mut s2: f32;
        let mut s3: f32;
        for frame in 0..ctx.nframes() {
            let s = inp.read(frame);
            //WRITEME: split into 3 signals instead of summing
            s1 = self.g1.tick(s);
            s2 = self.g2.tick(s);
            s3 = self.g3.tick(s);
            self.frames_processed += 1;
            if self.frames_processed as f32 > self.olatency {
                self.tempf1 = s1; //only updates after a calculation on a new window
                self.tempf2 = s2; //only updates after a calculation on a new window
                self.tempf3 = s3; //only updates after a calculation on a new window

                self.frames_processed = 0;
                self.g1.reset();
                self.g2.reset();
                self.g3.reset();
            }
            let gain = denorm::Gz3Filt::gain(gain, frame);
            outf1.write(frame, self.tempf1 * gain);
            outf2.write(frame, self.tempf2 * gain);
            outf3.write(frame, self.tempf3 * gain);
        }

        ctx_vals[0].set(s1); //not sure what this does
    }
}
