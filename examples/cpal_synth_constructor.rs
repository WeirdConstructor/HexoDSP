use hexodsp::*;
use hexodsp::synth_constructor::SynthConstructor;

use anyhow;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let mut sc = SynthConstructor::new();

    start_backend(sc.executor().unwrap(), move || {
        use hexodsp::dsp::build::*;

        // Setup a sawtooth oscillator with 440Hz:
        let saw = bosc(0).set().wtype(2).set().freq(440.0);
        // Setup an amplifier node with a low gain:
        let amp = amp(0).set().gain(0.1).input().inp(&saw.output().sig());

        let out = out(0).input().ch1(&amp.output().sig());
        let out = out.input().ch2(&amp.output().sig());

        // Setup a triangle LFO with a cycletime of 4 seconds.
        let lfo = tslfo(0).set().time(4000.0);
        // Setup the "att"enuator input to 0.3 with a modulation amount of 0.0 to 0.7.
        // Redirect the output of the LFO (which oscillated between 0.0 and 1.0) to the
        // "att" input of the Amp node here:
        let amp = amp.set_mod().att(0.3, 0.7).input().att(&lfo.output().sig());

        sc.upload(&out);

        let mut pitch_counter = 0;
        loop {
            let new_pitch = match pitch_counter {
                0 => 440.0, // A4
                1 => 220.0, // A3
                2 => 880.0, // A5
                3 => 220.0, // A3
                4 => 110.0, // A2
                _ => {
                    pitch_counter = 0;
                    550.0
                }
            };
            pitch_counter += 1;

            println!("Update freq={}", new_pitch);
            sc.update_params(&bosc(0).set().freq(new_pitch));
//            node_conf.set_param(sin_freq_param, new_pitch.into());

            std::thread::sleep(std::time::Duration::from_millis(300));
        }
    });
}

pub fn run<T, F: FnMut()>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut node_exec: NodeExecutor,
    mut frontend_loop: F,
) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    node_exec.set_sample_rate(sample_rate);

    let input_bufs = [[0.0; hexodsp::dsp::MAX_BLOCK_SIZE]; 2];
    let mut outputbufs = [[0.0; hexodsp::dsp::MAX_BLOCK_SIZE]; 2];

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let mut frames_left = data.len() / channels;

            let mut out_iter = data.chunks_mut(channels);

            node_exec.process_graph_updates();

            while frames_left > 0 {
                let cur_nframes = if frames_left >= hexodsp::dsp::MAX_BLOCK_SIZE {
                    hexodsp::dsp::MAX_BLOCK_SIZE
                } else {
                    frames_left
                };

                let input = &[&input_bufs[0][0..cur_nframes], &input_bufs[1][0..cur_nframes]];

                let split = outputbufs.split_at_mut(1);

                let mut output =
                    [&mut ((split.0[0])[0..cur_nframes]), &mut ((split.1[0])[0..cur_nframes])];

                let mut context = Context { nframes: cur_nframes, output: &mut output[..], input };

                context.output[0].fill(0.0);
                context.output[1].fill(0.0);

                node_exec.process(&mut context);

                // This copy loop is a bit inefficient, it's likely you can
                // pass the right array slices directly into node_exec.process()
                // via the Context structure. But I was too lazy at this point
                // to figure this out. Check also the Jack example for a more
                // efficient solution.
                for i in 0..cur_nframes {
                    if let Some(frame) = out_iter.next() {
                        let mut ctx_chan = 0;
                        for sample in frame.iter_mut() {
                            let value: T = cpal::Sample::from::<f32>(&context.output[ctx_chan][i]);
                            *sample = value;

                            ctx_chan += 1;
                            if ctx_chan > context.output.len() {
                                ctx_chan = context.output.len() - 1;
                            }
                        }
                    }
                }

                frames_left -= cur_nframes;
            }
        },
        err_fn,
    )?;
    stream.play()?;

    frontend_loop();

    Ok(())
}

// This function starts the CPAL backend and
// runs the audio loop with the NodeExecutor.
fn start_backend<F: FnMut()>(node_exec: NodeExecutor, frontend_loop: F) {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("Finding useable audio device");
    let config = device.default_output_config().expect("A workable output config");

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32, F>(&device, &config.into(), node_exec, frontend_loop),
        cpal::SampleFormat::I16 => run::<i16, F>(&device, &config.into(), node_exec, frontend_loop),
        cpal::SampleFormat::U16 => run::<u16, F>(&device, &config.into(), node_exec, frontend_loop),
    }
    .expect("cpal works fine");
}
