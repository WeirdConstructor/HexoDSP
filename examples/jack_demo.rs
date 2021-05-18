use hexodsp::*;

use std::sync::Arc;
use std::sync::Mutex;
use std::rc::Rc;
use std::cell::RefCell;

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
            // implemented yet (2021-05-18) and is not implemented
            // in this jack interface).

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

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });
}

struct Notifications {
    node_exec: Arc<Mutex<NodeExecutor>>,
}

impl jack::NotificationHandler for Notifications {
    fn thread_init(&self, _: &jack::Client) {
        println!("JACK: thread init");
    }

    fn shutdown(&mut self, status: jack::ClientStatus, reason: &str) {
        println!(
            "JACK: shutdown with status {:?} because \"{}\"",
            status, reason
        );
    }

    fn freewheel(&mut self, _: &jack::Client, is_enabled: bool) {
        println!(
            "JACK: freewheel mode is {}",
            if is_enabled { "on" } else { "off" }
        );
    }

    fn buffer_size(&mut self, _: &jack::Client, sz: jack::Frames) -> jack::Control {
        println!("JACK: buffer size changed to {}", sz);
        jack::Control::Continue
    }

    fn sample_rate(&mut self, _: &jack::Client, srate: jack::Frames) -> jack::Control {
        println!("JACK: sample rate changed to {}", srate);
        let mut ne = self.node_exec.lock().unwrap();
        ne.set_sample_rate(srate as f32);
        jack::Control::Continue
    }

    fn client_registration(&mut self, _: &jack::Client, name: &str, is_reg: bool) {
        println!(
            "JACK: {} client with name \"{}\"",
            if is_reg { "registered" } else { "unregistered" },
            name
        );
    }

    fn port_registration(&mut self, client: &jack::Client, port_id: jack::PortId, is_reg: bool) {
        if let Some(p) = client.port_by_id(port_id) {
            if let Ok(name) = p.name() {
                println!("JACK: port registered: {}", name);
            }
        }
        println!(
            "JACK: {} port with id {}",
            if is_reg { "registered" } else { "unregistered" },
            port_id
        );
    }

    fn port_rename(
        &mut self,
        _: &jack::Client,
        port_id: jack::PortId,
        old_name: &str,
        new_name: &str,
    ) -> jack::Control {
        println!(
            "JACK: port with id {} renamed from {} to {}",
            port_id, old_name, new_name
        );
        jack::Control::Continue
    }

    fn ports_connected(
        &mut self,
        _: &jack::Client,
        port_id_a: jack::PortId,
        port_id_b: jack::PortId,
        are_connected: bool,
    ) {
        println!(
            "JACK: ports with id {} and {} are {}",
            port_id_a,
            port_id_b,
            if are_connected {
                "connected"
            } else {
                "disconnected"
            }
        );
    }

    fn graph_reorder(&mut self, _: &jack::Client) -> jack::Control {
        println!("JACK: graph reordered");
        jack::Control::Continue
    }

    fn xrun(&mut self, _: &jack::Client) -> jack::Control {
        println!("JACK: xrun occurred");
        jack::Control::Continue
    }

    fn latency(&mut self, _: &jack::Client, mode: jack::LatencyType) {
        println!(
            "JACK: {} latency has changed",
            match mode {
                jack::LatencyType::Capture => "capture",
                jack::LatencyType::Playback => "playback",
            }
        );
    }
}

// This function starts the Jack backend and
// runs the audio loop with the NodeExecutor.
fn start_backend<F: FnMut()>(node_exec: NodeExecutor, mut frontend_loop: F) {
    let (client, _status) =
        jack::Client::new("HexoDSPJackDemo", jack::ClientOptions::NO_START_SERVER)
        .unwrap();

    let in_a =
        client.register_port("hexodsp_in1", jack::AudioIn::default())
            .unwrap();
    let in_b =
        client.register_port("hexodsp_in2", jack::AudioIn::default())
            .unwrap();
    let mut out_a =
        client.register_port("hexodsp_out1", jack::AudioOut::default())
            .unwrap();
    let mut out_b =
        client.register_port("hexodsp_out2", jack::AudioOut::default())
            .unwrap();

    let ne        = Arc::new(Mutex::new(node_exec));
    let ne2       = ne.clone();

    let oversample_simulation =
        if let Some(arg) = std::env::args().skip(1).next() {
            arg == "4x"
        } else {
            false
        };

    let mut first = true;
    let process_callback = move |client: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        let out_a_p = out_a.as_mut_slice(ps);
        let out_b_p = out_b.as_mut_slice(ps);
        let in_a_p = in_a.as_slice(ps);
        let in_b_p = in_b.as_slice(ps);

        if first {
            client.connect_ports_by_name("HexoDSPJackDemo:hexodsp_out1", "system:playback_1")
                .expect("jack connect ports works");
            client.connect_ports_by_name("HexoDSPJackDemo:hexodsp_out2", "system:playback_2")
                .expect("jack connect ports works");
            first = false;
        }

        let nframes = out_a_p.len();

        // Please note, locking the NodeExecutor is wrong and broken
        // and should not be done on a real time thread.
        //
        // We do it for educational purposes here.
        // In reality the jack::NotificationHandler should send a message through
        // a ring buffer to the audio thread and set the new sample rate or
        // other parameters there!
        let mut node_exec = ne.lock().unwrap();

        // First task in the audio callback is processing any graph or parameter
        // updates that were sent by the frontend thread:
        node_exec.process_graph_updates();

        let mut frames_left = nframes;
        let mut offs        = 0;

        while frames_left > 0 {
            let cur_nframes =
                if frames_left >= hexodsp::dsp::MAX_BLOCK_SIZE {
                    hexodsp::dsp::MAX_BLOCK_SIZE
                } else {
                    frames_left
                };

            frames_left -= cur_nframes;

            let output = &mut [&mut out_a_p[offs..(offs + cur_nframes)],
                               &mut out_b_p[offs..(offs + cur_nframes)]];
            let input =
                &[&in_a_p[offs..(offs + cur_nframes)],
                  &in_b_p[offs..(offs + cur_nframes)]];

            let mut context =
                Context {
                    nframes: cur_nframes,
                    output,
                    input,
                };

            for i in 0..context.nframes {
                context.output[0][i] = 0.0;
                context.output[1][i] = 0.0;
            }

            node_exec.process(&mut context);

            if oversample_simulation {
                node_exec.process(&mut context);
                node_exec.process(&mut context);
                node_exec.process(&mut context);
            }

            offs += cur_nframes;
        }

        jack::Control::Continue
    };

    let process =
        jack::ClosureProcessHandler::new(process_callback);

    // Activate the client, which starts the processing.
    let active_client =
        client.activate_async(Notifications {
            node_exec: ne2,
        }, process).unwrap();

    frontend_loop();

    active_client.deactivate().unwrap();
}

