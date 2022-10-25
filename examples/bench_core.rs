// This example demonstrates the SynthConstructor API with the CPAL backend
// for standalone audio output.
//
// Execute with:
//      $ cargo +nightly run --release --example cpal_synth_constructor

use hexodsp::synth_constructor::SynthConstructor;
use hexodsp::*;

use anyhow;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    // The SynthConstructor encapsulates the whole audio graph engine in HexoDSP:
    let mut sc = SynthConstructor::new();

    use hexodsp::build::*;

    // Setup a sawtooth oscillator with 440Hz:
    let saw = bosc(0).set().wtype(2).set().freq(440.0);
    // Setup an amplifier node with a low gain:
    let amp = amp(0).set().gain(0.2).input().inp(&saw.output().sig());

//    // Insert your own custom Rust function via a NodeId::Rust1x1 node
//    // into the DSP graph:
//    use hexodsp::dsp::{DynamicNode1x1, DynNode1x1Context};
    let r1x1 = rust1x1(0).input().inp(&amp.output().sig());
    let r1x1 = r1x1.set().alpha(0.75);
//    // You may replace this function anytime at runtime:
//    sc.set_dynamic_node1x1(0, Box::new(|inp: &[f32], out: &mut [f32], ctx: &DynNode1x1Context| {
//        let alpha = ctx.alpha_slice();
//        for (i, in_sample) in inp.iter().enumerate() {
//            out[i] = in_sample * alpha[i];
//        }
//
//        // This sets an atomic float that can be read out using SynthConstructor::led_value()!
//        ctx.led_value().set(out[0]);
//    }));
//
    // Assign amplifier node output to the two input channels
    // of the audio device output node:
//    let out = out(0).input().ch1(&r1x1.output().sig());
    let out = out(0).input().ch1(&amp.output().sig());
    let out = out.input().ch2(&r1x1.output().sig());

    // Setup a triangle LFO with a cycletime of 8 seconds.
    let lfo = tslfo(0).set().time(8000.0);
//    let out = out.input().ch2(&lfo.output().sig());
    // Setup the "att"enuator input to 0.3 with a modulation amount of 0.0 to 0.7.
    // Redirect the output of the LFO (which oscillated between 0.0 and 1.0) to the
    // "att" input of the Amp node here:
    amp.set_mod().att(0.3, 0.7).input().att(&lfo.output().sig());

    // Upload the program:
    sc.upload(&out).unwrap();

    use std::time::Instant;
    let mut exec = sc.executor().unwrap();
    let mut avg = 0;
    let mut cnt = 0;
    exec.test_run(10.0, false, &[]);
    for i in 0..10 {
        let now = Instant::now();
//        exec.dummy_run(100.0);
        exec.test_run(100.0, false, &[]);
        let millis = now.elapsed().as_millis();
        cnt += 1;
        avg += millis;
    }
    println!("avg: {}", avg / cnt);
}
