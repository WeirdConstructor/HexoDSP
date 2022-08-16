// Copyright (c) 2021-2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

/*!# HexoDSP - Comprehensive DSP graph and synthesis library for developing a modular synthesizer in Rust, such as HexoSynth.

This project contains the complete DSP backend of the modular
synthesizer [HexoSynth](https://github.com/WeirdConstructor/HexoSynth).

It's aimed to provide a toolkit for everyone who wants to develop
a synthesizer in Rust. You can use it to quickly define a DSP graph
that you can change at runtime. It comes with a (growing) collection
of already developed DSP modules/nodes, such as oscillators, filters,
amplifiers, envelopes and sequencers.

The DSP graph API also provides multiple kinds of feedback to track what the
signals in the DSP threads look like. From monitoring the inputs and outputs of
single nodes to get the current output value of all nodes.

There is also an (optional) JIT compiler for defining custom pieces of DSP code
that runs at native speed in a DSP graph module/node.

Here a short list of features:

* Runtime changeable DSP graph.
* Serialization and loading of the DSP graph and the parameters.
* Full monitoring and feedback introspection into the running DSP graph.
* Provides a wide variety of modules.
* (Optional) JIT (Just In Time) compiled custom DSP code for integrating your own
DSP algorithms at runtime. One possible frontend language is the visual
"BlockCode" programming language in HexoSynth.
* Extensible framework for quickly adding new nodes to HexoDSP.
* A comprehensive automated test suite covering all modules in HexoDSP.

And following DSP nodes:

| Category | Name | Function |
|-|-|-|
| IO Util | Out         | Audio output (to DAW or Jack) |
| Osc     | Sampl       | Sample player |
| Osc     | Sin         | Sine oscillator |
| Osc     | BOsc        | Basic bandlimited waveform oscillator (waveforms: Sin, Tri, Saw, Pulse/Square) |
| Osc     | VOsc        | Vector phase shaping oscillator |
| Osc     | Noise       | Noise oscillator |
| Osc     | FormFM      | Formant oscillator based on FM synthesis |
| Signal  | Amp         | Amplifier/Attenuator |
| Signal  | SFilter     | Simple collection of filters, useable for synthesis |
| Signal  | Delay       | Single tap signal delay |
| Signal  | PVerb       | Reverb node, based on Dattorros plate reverb algorithm |
| Signal  | AllP        | All-Pass filter based on internal delay line feedback |
| Signal  | Comb        | Comb filter |
| Signal  | Code        | JIT (Just In Time) compiled piece of custom DSP code. |
| N-\>M   | Mix3        | 3 channel mixer |
| N-\>M   | Mux9        | 9 channel to 1 output multiplexer/switch |
| Ctrl    | SMap        | Simple control signal mapper |
| Ctrl    | Map         | Control signal mapper |
| Ctrl    | CQnt        | Control signal pitch quantizer |
| Ctrl    | Quant       | Pitch signal quantizer |
| Mod     | TSeq        | Tracker/pattern sequencer |
| Mod     | Ad          | Attack-Decay envelope |
| Mod     | TsLFO       | Tri/Saw waveform low frequency oscillator (LFO) |
| Mod     | RndWk       | Random walker, a Sample & Hold noise generator |
| IO Util | FbWr / FbRd | Utility modules for feedback in patches |
| IO Util | Scope       | Oscilloscope for up to 3 channels |
| IO Util | MidiP       | MIDI Pitch/Note input from plugin host, DAW or hardware |

## API Examples

### Documentation

The development documentation with all private fields and functions can be
found separately hosted:
[HexoDSP API Developer Documentation](http://m8geil.de/hexodsp_doc/hexodsp/).

### Raw `NodeConfigurator` API

This API is the most low level API provided by HexoDSP.
It shows how to create nodes and connect them.
The execution of the nodes in the audio thread is
controlled by a `NodeProg`, which defines the order
the nodes are executed in.

This only showcases the non-realtime generation of audio
samples. For a real time application of this library please
refer to the examples that come with this library.

```rust
use hexodsp::*;

let (mut node_conf, mut node_exec) = new_node_engine();

node_conf.create_node(NodeId::Sin(0));
node_conf.create_node(NodeId::Amp(0));

let mut prog = node_conf.rebuild_node_ports();

node_conf.add_prog_node(&mut prog, &NodeId::Sin(0));
node_conf.add_prog_node(&mut prog, &NodeId::Amp(0));

node_conf.set_prog_node_exec_connection(
    &mut prog,
    (NodeId::Amp(0), NodeId::Amp(0).inp("inp").unwrap()),
    (NodeId::Sin(0), NodeId::Sin(0).out("sig").unwrap()));

node_conf.upload_prog(prog, true);

let (out_l, out_r) = node_exec.test_run(0.1, false);
```

### Hexagonal Matrix API

This is a short overview of the API provided by the
hexagonal Matrix API, which is the primary API used
inside [HexoSynth](https://github.com/WeirdConstructor/HexoSynth).

This only showcases the direct generation of audio samples, without any audio
device playing it. For a real time application of this library please refer to
the examples that come with this library.

```rust
use hexodsp::*;

let (node_conf, mut node_exec) = new_node_engine();
let mut matrix = Matrix::new(node_conf, 3, 3);

let sin = NodeId::Sin(0);
let amp = NodeId::Amp(0);
let out = NodeId::Out(0);
matrix.place(0, 0, Cell::empty(sin)
                   .out(None, None, sin.out("sig")));
matrix.place(0, 1, Cell::empty(amp)
                   .input(amp.inp("inp"), None, None)
                   .out(None, None, amp.out("sig")));
matrix.place(0, 2, Cell::empty(out)
                   .input(out.inp("inp"), None, None));
matrix.sync().unwrap();

let gain_p = amp.inp_param("gain").unwrap();
matrix.set_param(gain_p, SAtom::param(0.25));

let (out_l, out_r) = node_exec.test_run(0.11, true);
// out_l and out_r contain two channels of audio
// samples now.
```

### Simplified Hexagonal Matrix API

There is also a simplified version for easier setup of DSP chains
on the hexagonal grid, using the [crate::MatrixCellChain] abstraction:

```rust
use hexodsp::*;

let (node_conf, mut node_exec) = new_node_engine();
let mut matrix = Matrix::new(node_conf, 3, 3);
let mut chain = MatrixCellChain::new(CellDir::B);

chain.node_out("sin", "sig")
    .node_io("amp", "inp", "sig")
    .set_atom("gain", SAtom::param(0.25))
    .node_inp("out", "ch1")
    .place(&mut matrix, 0, 0);
matrix.sync().unwrap();

let (out_l, out_r) = node_exec.test_run(0.11, true);
// out_l and out_r contain two channels of audio
// samples now.
```

## State of Development

As of 2022-07-30: The architecture and it's functionality have been mostly
feature complete by now. The only part that is still lacking is the collection
of modules/nodes, this is the area of current development. Adding lots of
nodes.

Make sure to follow [Weird Constructors Mastodon
account](https://mastodon.online/@weirdconstructor) or the releases of this
project to be notified of updates.

## Running the Jack Example:


To run the example:

```text
    cargo run --release --example jack_demo_node_api
```

You might need following dependencies (Ubuntu Linux):

```text
    sudo apt install libjack0 libjack-jackd2-dev qjackctl
```

These might work on Debian too:

```text
    sudo apt install libjack0 libjack-dev
```

## Running the Automated Testsuite:

There exists an automate test suite for the DSP and backend code:

```text
    cargo test
```

## Known Bugs

* The ones you encounter and create as issues on GitHub.

## Credits

- Dimas Leenman (aka Skythedragon) contributed the `FormFM` node.

## Contributions

I currently have a quite precise vision of what I want to achieve and my goal
is to make music with this project eventually.

The projects is still young, and I currently don't have that much time to
devote for project coordination. So please don't be offended if your issue rots
in the GitHub issue tracker, or your pull requests is left dangling around
for ages.

If you want to contribute new DSP nodes/modules to HexoDSP/HexoSynth,
please look into the guide at the start of the [crate::dsp] module.

I might merge pull requests if I find the time and think that the contributions
are in line with my vision.

Please bear in mind, that I can only accept contributions under the License
of this project (GPLv3 or later).

## Contact the Author

You can reach me via Discord or Mastodon. I'm joined most public Rust Discord
servers, especially the "Rust Audio" Discord server. I am also sometimes on
freenode.net, for instance in the `#lad` channel (nick `weirdctr`).

## Support Development

You can support me (and the development of this project) via Liberapay:

<a href="https://liberapay.com/WeirdConstructor/donate"><img alt="Donate using Liberapay" src="https://liberapay.com/assets/widgets/donate.svg"></a>

## License

This project is licensed under the GNU Affero General Public License Version 3 or
later.

### Why GPL?

The obivious reason is that this project copied and translated code from many
other free software / open source synthesis projects. The sources
will show the origin and license of the individual parts.

#### My Reasons

Picking a license for my code bothered me for a long time. I read many
discussions about this topic. Read the license explanations. And discussed
this matter with other developers.

First about _why I write code for free_ at all, the reasons are:

- It's my passion to write computer programs. In my free time I can
write the code I want, when I want and the way I want. I can freely
allocate my time and freely choose the projects I want to work on.
- To help a friend or member of my family.
- To solve a problem I have.
- To learn something new.

Those are the reasons why I write code for free. Now the reasons
_why I publish the code_, when I could as well keep it to myself:

- So that it may bring value to users and the free software community.
- Show my work as an artist.
- To get into contact with other developers.
- To exchange knowledge and help other developers.
- And it's a nice change to put some more polish on my private projects.

Most of those reasons don't yet justify GPL. The main point of the GPL, as far
as I understand: The GPL makes sure the software stays free software until
eternity. That the _end user_ of the software always stays in control. That the users
have the means to adapt the software to new platforms or use cases.
Even if the original authors don't maintain the software anymore.
It ultimately prevents _"vendor lock in"_. I really dislike vendor lock in,
especially as developer. Especially as developer I want and need to stay
in control of the computers and software I use.

Another point is, that my work (and the work of any other developer) has a
value. If I give away my work without _any_ strings attached, I effectively
work for free. This compromises the price I (and potentially other developers)
can demand for the skill, workforce and time.

This makes two reasons for me to choose the GPL:

1. I do not want to support vendor lock in scenarios for free.
   I want to prevent those when I have a choice, when I invest my private
   time to bring value to the end users.
2. I don't want to low ball my own (and other developer's) wage and prices
   by giving away the work I spent my scarce private time on with no strings
   attached. I do not want companies to be able to use it in closed source
   projects to drive a vendor lock in scenario.

We can discuss relicensing of my code or project if you are interested in using
it in a closed source project. Bear in mind, that I can only relicense the
parts of the project I wrote. If the project contains GPL code from other
projects and authors, I can't relicense it.

*/

pub mod cell_dir;
pub mod chain_builder;
#[allow(unused_macros, non_snake_case)]
pub mod dsp;
pub mod log;
pub mod matrix;
pub mod matrix_repr;
pub mod monitor;
pub mod nodes;
pub mod sample_lib;
pub mod scope_handle;
mod util;
pub mod wblockdsp;

pub use cell_dir::CellDir;
pub use chain_builder::MatrixCellChain;
pub use dsp::{NodeId, NodeInfo, ParamId, SAtom};
pub use log::log;
pub use matrix::{Cell, Matrix};
pub use matrix_repr::load_patch_from_file;
pub use matrix_repr::save_patch_to_file;
pub use nodes::{new_node_engine, HxMidiEvent, NodeConfigurator, NodeExecutor};
pub use sample_lib::{SampleLibrary, SampleLoadError};
pub use scope_handle::ScopeHandle;

pub struct Context<'a, 'b, 'c, 'd> {
    pub nframes: usize,
    pub output: &'a mut [&'b mut [f32]],
    pub input: &'c [&'d [f32]],
}

impl<'a, 'b, 'c, 'd> nodes::NodeAudioContext for Context<'a, 'b, 'c, 'd> {
    #[inline]
    fn nframes(&self) -> usize {
        self.nframes
    }

    #[inline]
    fn output(&mut self, channel: usize, frame: usize, v: f32) {
        self.output[channel][frame] = v;
    }

    #[inline]
    fn input(&mut self, channel: usize, frame: usize) -> f32 {
        self.input[channel][frame]
    }
}

pub fn test() -> bool {
    true
}
