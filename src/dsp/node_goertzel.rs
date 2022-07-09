//WRTIEME: convert to node
// Copyright (c) 2022 theloni-monk <theo.acooper@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext};
use crate::dsp::biquad::*;

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
pub struct BiqFilt {
    cascade: Vec<Biquad>,
    srate:   f32,
    ofreq:   f32,
    oq:      f32,
    ogain:   f32,
    otype:   u8,
}

impl BiqFilt {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            cascade: vec![Biquad::new(); 4],
            srate: 1.0 / 44100.0,
            otype: 99,   // value that can't be set by the user
            ofreq: -2.0, // value that can't be set by the user
            oq:    -2.0, // value that can't be set by the user
            ogain: -2.0, // value that can't be set by the user
        }
    }
    pub const inp : &'static str =
        "BiqFilt inp\nSignal input\nRange: (-1..1)\n";
    pub const freq : &'static str =
        "BiqFilt freq\nFilter cutoff frequency.\nRange: (-1..1)\n";
    pub const q : &'static str =
        "BiqFilt q\nFilter Q factor.\nRange: (0..1)\n";
    pub const gain : &'static str =
        "BiqFilt gain\nFilter gain.\nRange: (0..1)\n";
    pub const ftype : &'static str =
        "BiqFilt ftype\n'BtW LP' Butterworth Low-Pass, 'Res' Resonator";
    pub const order : &'static str =
        "BiqFilt order\n";
    pub const sig : &'static str =
        "BiqFilt sig\nFiltered signal output.\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"Biquad Filter

This is the implementation of a biquad filter cascade.
It is not meant for fast automation. Please use other nodes
like eg. SFilter for that.
"#;
    pub const HELP : &'static str =
r#"BiqFilt - Biquad Filter (Cascade)

This is the implementation of a biquad filter cascade.
It is not meant for fast automation and might blow up if you
treat it too rough. Please use other nodes like eg. SFilter for that.
"#;
}

impl DspNode for BiqFilt {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) {
        self.srate = srate;
        self.otype = 99; // cause recalculation of the filter

        for b in &mut self.cascade {
            b.reset();
        }
    }

    fn reset(&mut self) {
        for b in &mut self.cascade {
            b.reset();
        }
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, inp, denorm, at};

        let inp   = inp::BiqFilt::inp(inputs);
        let freq  = inp::BiqFilt::freq(inputs);
        let q     = inp::BiqFilt::q(inputs);
        let gain  = inp::BiqFilt::gain(inputs);
        let ftype = at::BiqFilt::ftype(atoms);
        let order = at::BiqFilt::order(atoms);
        let out   = out::BiqFilt::sig(outputs);

        let ftype = ftype.i() as u8;
        let cfreq = denorm::BiqFilt::freq(freq, 0);
        let cfreq = cfreq.clamp(0.0, 22000.0);
        let cq    = denorm::BiqFilt::q(q, 0);
        let cgain = denorm::BiqFilt::gain(gain, 0);

        if    ftype != self.otype
           || (cfreq - self.ofreq).abs() > 0.0001
           || (cq - self.oq).abs()       > 0.0001
           || (cgain - self.ogain).abs() > 0.0001
        {
            // recalculate coeffs of all in the cascade
            let coefs =
                match ftype {
                    1 => BiquadCoefs::resonator(self.srate, cfreq, cq),
                    _ => BiquadCoefs::butter_lowpass(self.srate, cfreq),
                };

            for o in &mut self.cascade {
                o.set_coefs(coefs);
                o.reset();
            }

            self.otype = ftype;
            self.ofreq = cfreq;
            self.oq    = cq;
            self.ogain = cgain;
        }

        let mut order = order.i() as u8;
        if ftype == 1 { // The resonator just blows up with higher orders.
            order = 0;
        }

        for frame in 0..ctx.nframes() {
//            let freq  = denorm::BiqFilt::freq(freq, frame);
//            let freq  = freq.clamp($minfreq, $maxfreq);
//            let q     = denorm::BiqFilt::q(q, frame);
//            let gain  = denorm::BiqFilt::gain(gain, frame);

            let mut s = inp.read(frame);
            for i in 0..=order {
                s = self.cascade[i as usize].tick(s);
            }

            out.write(frame, s);
        }

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
    }
}
