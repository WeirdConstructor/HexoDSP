// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals};

#[macro_export]
macro_rules! fa_ad_mult { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "x1",
            1  => "x10",
            2  => "x100",
            _  => "?",
        };
    write!($formatter, "{}", s)
} } }

const AD_STAGES : i8 = 2;

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Ad {
    srate:      f64,
    phase:      f64,
    last_value: f32,
    stage:      i8,
}

impl Ad {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            srate:      44100.0,
            phase:      0.0,
            last_value: 0.0,
            stage:      -1,
        }
    }
    pub const inp : &'static str =
        "Ad inp\nSignal input. If you don't connect this, and set this to 1.0 \
        this will act as envelope signal generator. But you can also just \
        route a signal directly through this of course.\nRange: (-1..1)\n";
    pub const atk : &'static str =
        "Ad atk\nAttack time of the envelope. You can extend the maximum \
        range of this with the 'mult' setting.\nRange: (0..1)\n";
    pub const dcy : &'static str =
        "Ad atk\nDecay time of the envelope. You can extend the maximum \
        range of this with the 'mult' setting.\nRange: (0..1)\n";
    pub const ashp : &'static str =
        "Ad ashp\nAttack shape. This allows you to change the shape \
        of the attack stage from a logarithmic, to a linear and to an \
        exponential shape.\nRange: (0..1)\n";
    pub const dshp : &'static str =
        "Ad dshp\nDecay shape. This allows you to change the shape \
        of the decay stage from a logarithmic, to a linear and to an \
        exponential shape.\nRange: (0..1)\n";
    pub const mult : &'static str =
        "Ad mult\nAttack and Decay time range multiplier. \
        This will extend the maximum range of the 'atk' and 'dcy' parameters.";
    pub const sig : &'static str =
        "Ad sig\nEnvelope signal output. If a signal is sent to the 'inp' port, \
        you will receive an attenuated signal here. If you set 'inp' to a \
        fixed value (for instance 1.0), this will output an envelope signal \
        in the range 0.0 to 'inp' (1.0).\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"Attack-Decay Envelope

This is a simple envelope offering an attack time and decay time with shape parameter.
"#;
    pub const HELP : &'static str =
r#"Ad - Attack-Decay Envelope

"#;

}

/*
    struct {
        srate_per_ms: f64,
        value     : f64 = 0.0;
        inc       : f64 = 0.0;
        stage     = 0;
        last_time = 0.0;
        target    : f64 = 1.0;
        shape     = 0.5;
    }
    set_sample_rate(srate) { self.srate_per_ms = srate / 1000.0 }

    // block start:
    let mut shape_src =
        match stage {
            2 => dcy_shape,
            _ => atk_shape,
        };
    let mut inc_time_src =
        match stage {
            2 => dcy,
            _ => atk,
        };
    let mut mult : f64 =
        if mult == 1 { 10.0 } else if mult == 2 {100.0 } else { 1.0};

    // each frame:
    if stage == 0 {
        if trigger(trig_in) {
            value = 0.0;

            // transition to stage 1 (attack):
            stage        = 1;
            target       = 1.0;
            shape_src    = atk_shape;
            inc_time_src = atk;
            last_time    = -1.0;
        }
    }

    let cur_time = denorm(inc_time_src);
    if last_time != cur_time {
        inc =
            (target - value)
            / ((cur_time as f64) * mult * srate_per_ms);
    }

    value += inc;
    shape  = read(frame, shape_src).clamp(0.0, 1.0);

    match stage {
         1 => {
            if value >= target {
                // transition to stage 2 (decay):
                stage           = 2;
                target          = 0.0;
                shape_src       = dcy_shape;
                inc_time_src    = dcy;
                last_time       = -1.0;
            }
         },
         2 => {
            if value <= target {
                stage = 0;
                eov_trigger.trigger();
            }
         },
         _ => {},
    }

    let in_val = inp.read(frame);
    out.write(
        frame,
        in_val
        * sqrt4_to_pow4(
            value.clamp(0.0, 1.0) as f32, shape));
    trig.write(frame, eov_trigger.next());
*/

impl DspNode for Ad {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, _srate: f32) { }
    fn reset(&mut self) { }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        atoms: &[SAtom], _params: &[ProcBuf], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, inp, denorm, denorm_v, inp_dir, at};

        let out = out::Ad::sig(outputs);

        let last_frame = ctx.nframes() - 1;

        for frame in 0..ctx.nframes() {
            out.write(frame, 0.0);
        }

        ctx_vals[0].set(0.0);
//        ctx_vals[1].set(self.phase / self. + self.stage * );
    }
}
