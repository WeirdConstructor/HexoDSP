use hexodsp::*;

use anyhow;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let (mut node_conf, node_exec) = new_node_engine();

    start_backend(node_exec, move || {
        // To get an overview of the existing nodes you can
        // take a look in the file src/dsp/mod.rs
        // where the `macro_rules! node_list` definition
        // is.
        //
        // This defines all supported nodes and their
        // parameters/inputs ports and their outputs.
        let sin = NodeId::Sin(0);
        let amp = NodeId::Amp(0);
        let out = NodeId::Out(0);

        let amp_gain_param = amp.inp_param("gain").unwrap();
        let sin_freq_param = sin.inp_param("freq").unwrap();

        // Create the nodes in the frontend and in the
        // audio backend. You only have to do this once
        // and it's up to you to track which nodes you
        // already created.
        //
        // Keep in mind, that the only way to deallocate
        // notes is to call `node_conf.delete_nodes()`,
        // which deletes all nodes.
        //
        // You can't delete only one specific node.
        node_conf.create_node(sin);
        node_conf.create_node(amp);
        node_conf.create_node(out);

        // Silence the Amp for the start
        node_conf.set_param(amp_gain_param, (0.0).into());

        // Create a NodeProg from the currently created nodes:
        let mut prog = node_conf.rebuild_node_ports();

        // The order you add the nodes to the NodeProg determines
        // the order they will be executed by the audio thread.
        // You will have to take care that all nodes get their
        // data in the right order here.
        node_conf.add_prog_node(&mut prog, &NodeId::Sin(0));
        node_conf.add_prog_node(&mut prog, &NodeId::Amp(0));
        node_conf.add_prog_node(&mut prog, &NodeId::Out(0));

        // Define the connections between the nodes in the NodeProg:
        node_conf.set_prog_node_exec_connection(
            &mut prog,
            // first the input:
            (amp, amp.inp("inp").unwrap()),
            // then the output that is assigned to it:
            (sin, sin.out("sig").unwrap()));

        node_conf.set_prog_node_exec_connection(
            &mut prog,
            (out, out.inp("ch1").unwrap()),
            (amp, amp.out("sig").unwrap()));

        node_conf.set_prog_node_exec_connection(
            &mut prog,
            (out, out.inp("ch2").unwrap()),
            (amp, amp.out("sig").unwrap()));

        // Finally upload the NodeProg to the audio thread.
        node_conf.upload_prog(prog, true);

        // You can repeatedly create new NodeProgs with `rebuild_node_ports`
        // and change the graph all the way you like at runtime.

        let mut amp_counter = 0;
        let mut pitch_counter = 0;
        loop {
            // In this loop we simulate someone adjusting the paramter
            // knobs of the amplifier gain and sine oscillator pitch.
            //
            // Please note, that for sample accurate modulation you should
            // use the built in tracker or receive MIDI data from
            // different application (MIDI processing has not been
            // implemented yet though).

            let new_gain =
                match amp_counter {
                    0 => 0.2,
                    1 => 0.3,
                    2 => 0.35,
                    3 => 0.3,
                    4 => 0.1,
                    _ => {
                        amp_counter = 0;
                        // Pitch is defined in 0.1 per octave.
                        //  0.0 is A4,
                        //  0.1 is A5
                        // -0.1 is A3
                        let new_pitch =
                            match pitch_counter {
                                0 => 0.0,  // A4
                                1 => -0.1, // A3
                                2 => 0.1,  // A5
                                3 => -0.1, // A3
                                4 => -0.2, // A2
                                _ => {
                                    pitch_counter = 0;
                                    // -0.15 is 6 semitones above A3 => D#3
                                    -0.15
                                },
                            };
                        pitch_counter += 1;

                        println!("set pitch={:4.2}", new_pitch);
                        node_conf.set_param(sin_freq_param, new_pitch.into());

                        0.1
                    },
                };
            amp_counter += 1;

            println!("set gain={:4.2}", new_gain);
            node_conf.set_param(amp_gain_param, new_gain.into());

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
    let channels    = config.channels as usize;

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
                let cur_nframes =
                    if frames_left >= hexodsp::dsp::MAX_BLOCK_SIZE {
                        hexodsp::dsp::MAX_BLOCK_SIZE
                    } else {
                        frames_left
                    };

                let input = &[
                    &input_bufs[0][0..cur_nframes],
                    &input_bufs[1][0..cur_nframes],
                ];

                let split = outputbufs.split_at_mut(1);

                let mut output = [
                    &mut ((split.0[0])[0..cur_nframes]),
                    &mut ((split.1[0])[0..cur_nframes]),
                ];

                let mut context =
                    Context {
                        nframes: cur_nframes,
                        output: &mut output[..],
                        input,
                    };

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
                            let value: T =
                                cpal::Sample::from::<f32>(&context.output[ctx_chan][i]);
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

// This function starts the Jack backend and
// runs the audio loop with the NodeExecutor.
fn start_backend<F: FnMut()>(node_exec: NodeExecutor, frontend_loop: F) {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("Finding useable audio device");
    let config = device.default_output_config().expect("A workable output config");

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32, F>(&device, &config.into(), node_exec, frontend_loop),
        cpal::SampleFormat::I16 => run::<i16, F>(&device, &config.into(), node_exec, frontend_loop),
        cpal::SampleFormat::U16 => run::<u16, F>(&device, &config.into(), node_exec, frontend_loop),
    }.expect("cpal works fine");
}

