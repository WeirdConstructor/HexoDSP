// Copyright (c) 2021-2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

/*!

# HexoDSP DSP nodes and DSP code

## How to Add a New DSP Node

When adding a new node to HexoDSP, I recommend working through the following checklist:

- [ ] Implement boilerplate in node_yourname.rs
- [ ] Add input parameter and output signal definition to dsp/mod.rs
- [ ] Document boilerplate in node_yourname.rs
- [ ] DSP implementation
- [ ] Parameter fine tuning
- [ ] DSP tests for all (relevant) params
- [ ] Ensure Documentation is properly formatted for the GUI
- [ ] Format the source using `cargo fmt`
- [ ] Add CHANGELOG.md entry in HexoSynth
- [ ] Add CHANGELOG.md entry in HexoDSP
- [ ] Add table entry in README.md in HexoSynth
- [ ] Add table entry in README.md in HexoDSP

The boilerplate can be a bit daunting. But it pays off, because HexoSynth will give
you a GUI for your DSP code for free at the end.

Generally I recommend starting out small. Define your new node with minimal parameters
until you get the hang of all the things involved to make it compile in the first place.

**Be aware that new DSP nodes need to meet these quality guidelines to be included:**

- Clean Rust code that I can understand and maintain. You can use `cargo fmt` (rustfmt) to
format the code.
- Does not drag in huge dependency trees. One rationale here is,
that I don't want the sound of a HexoSynth patch to change (significantly) because
some upstream crate decided to change their DSP code. To have optimal
control over this, I would love to have all the DSP code
contained in HexoDSP. Make sure to link the repository the code comes
from though. If you add dependencies for your DSP node, make sure that it's
characteristics are properly covered by the automated tests. So that problems become
visible in case upstream breaks or changes it's DSP code. If DSP code changes just slightly,
the test cases of course need to be changed accordingly.
- Come with automated smoke tests like all the other nodes, most test
signal min/max/rms over time, as well as the frequency spectrum
where applicable.
- It's parameters have proper denormalized mappings, like `0.5 => 4000 Hz` or `0.3 => 200ms`.
- Provide short descriptions for the node and it's parameters.
- Provide a comprehensive longer help text with (more details further down in this guide):
  - What this node is about
  - How to use it
  - How the parameters work in combination
  - Suggestions which combinations with other nodes might be interesting
- If applicable: provide a graph function for visualizing what it does.

### Boilerplate

- I recommend copying an existing node code, like `node_ad.rs` for instance.
- In this file `mod.rs` copy it's entry in the `node_list` macro definition.
- Copy the `tests/node_ad.rs` file to have a starting point for the automated testing.
Also keep in mind looking in other tests, about how they test things. Commonly used
macros are found in the ´tests/common/´ module.

A `node_list` macro entry looks like this:

```ignore
    // node_id => node_label UIType UICategory
    //  |          |          /        |
    //  /   /------/         /         |
    // /    |               /          |
    xxx => Xxx UIType::Generic UICategory::Signal
     // node_param_idx
     //   name             denorm round format steps norm norm denorm
     //         norm_fun   fun    fun   fun    def   min  max  default
       (0 inp   n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
       (1 gain  n_gain     d_gain r_gain f_db  stp_v  0.0, 1.0, 1.0)
       (2 att   n_id       d_id   r_id  f_def  stp_d  0.0, 1.0, 1.0)
     // node_param_idx      UI widget type (mode, knob, sample)
     // | atom_idx          |     format fun
     // | | name constructor|     |     min max
     // | | |    |       def|ult_v|lue  |  /
     // | | |    |       |  |     |     |  |
       {3 0 mono setting(0) mode fa_out_mono 0  1},
       [0 sig],
```

The first entries, encapsulated in ´( )´ are the input ports or also called input parameters.
Input parameters can be connected to outputs of other DSP nodes. In contrast to the ´{ }´ encapsulated
so called _Atom parameters_. The data type for these is the [SAtom] datatype. And these parameters
can not be automated.

You can freely choose parameter names like eg. `inp` or `gain` and
pick names that suit the parameter semantics best. But I would like you to keep the naming
consistent with the rest of HexoDSP nodes if that is suitable to the DSP node.

There are some implicit conventions in HexoDSP for naming though:

- `inp` for single channel signal input
- `ch1`, `ch2`, ... for multiple channels
- `sig` for signal output
- `trig` for receiving a single trigger signal
- `t_*` if multiple trigger signals are expected
- If you have `freq` inputs, consider also adding `det` for detuning that frequency input.
But only if you think this makes sense in the context of the DSP node.

The macros in the node list definition like `n_gain`, `d_pit`, `r_fq` and so on
are all macros that are defined in the HexoDSP crate. You can create your own
normalization/denormalization, rounding, step and formatting function macros if
the existing ones don't suit the DSP node's needs.

### Signal Ranges in HexoDSP

The HexoDSP graph, or rather the nodes, operate with the raw normalized (audio)
signal range [-1, 1]. There is a second range that is also common in HexoDSP,
which is the control signal range [0, 1]. Following this convention will help combinding
HexoDSP nodes with each other. The existing normalization/denormalization functions for the
node list declaration already encode most of the conventions in HexoDSP, but here is a short
incomplete overview of common value mappings to the normalized signal ranges:

- Frequencies are usually using the `n_pit` and `d_pit` mappings. Where 0.0 is 440Hz
and the next octave is at 0.1 with 880Hz and the octave before that is at -0.1 with 220Hz.
This means one octave per 0.1 signal value.
- Triggers have to rise above the "high" threshold of 0.5 to be recognized, and the signal has to
fall below 0.25 to be detected as "low" again. Same works for gates.

### Node Documentation

**Attention: Defining the documentation for your DSP node is not optional. It's required to make
it compile in the first place!**

Every DSP node must come with online documentation. The online documentation is what makes the
node usable in the first place. It's the only hint for the user to learn how to use this node.
Keep in mind that the user is not an engineer, but usually a musician. They want to make music
and know how to use a parameter.

Every input parameter and atom parameter requires you to define a documenation entry in the
corresponding ´node_*.rs´ module/file. And also a _DESC_ and _HELP_ entry.

Here an example from ´node_ad.rs´:

```ignore
    pub const inp: &'static str =
        "Signal input. If you don't connect this, and set this to **1.0** \
        this will act as envelope signal generator. But you can also just \
        route a signal directly through this of course.";
```

Keep the description of the parameter short and concise. Look at the space available
in HexoSynth where this is displayed. If you want to write more elaborate documentation
for a paramter, write it in the `HELP` entry.

Next you need to document the node itself, how it works what it does and so on...
For this there are two entries:

```ignore
        pub const DESC: &'static str = r#"### `Ad` Attack-Decay Envelope

    This is a simple envelope offering an attack time and decay time with a shape parameter.
    You can use it as envelope generator to modulate other inputs or process a signal with it directly.
    "#;

        pub const HELP: &'static str = r#"## `Ad` Attack-Decay Envelope

    This simple two stage envelope with attack and decay offers shape parameters
    ...
    "#;
```

_DESC_ should only contain a short description of the node. It's space is as limited as the
space for the parameter description. It will be autowrapped. It should start by convention
with the written out (but short) purpose title of the node.

_HELP_ can be a multiple pages long detailed description of the node. Keep the
width of the lines below a certain limit (below 80 usually). Or else it will be
truncated in the help text window in HexoSynth. The _HELP_ should by convention
with a written out purpose title of the node.

As inspiration what should be in the help documentation:

- What the node does (even if it repeats mostly what _DESC_ already says)
- How the input parameters relate to each other.
- What the different atom settings (if any) mean.
- Which other DSP nodes this node is commonly combined with.
- Interesting or even uncommon uses of this DSP node.
- Try to inspire the user to experiment.

More conventions for the documentation:

- \**emphasis*\* - use this to emphasize some text.
- \`Node\` - use back ticks to highlight node names.
- \*\*Value\*\* - use '\*\*' to highlight values.
- \~\~Port\~\~ - use '\~\~' to highlight input ports/parameters.

### Node Code Structure

For non trivial DSP nodes, the DSP code itself should be separate from it's `dsp/node_*.rs`
file. That file should only care about interfacing the DSP code with HexoDSP, but not implement
all the complicated DSP code. It's good practice to factor out the DSP code into
a separate module or file. It is preferable to add your custom DSP code to the `synfx-dsp`
crate [synfx-dsp](https://github.com/WeirdConstructor/synfx-dsp).

Look at `node_tslfo.rs` for instance. It wires up the `TriSawLFO` from `synfx-dsp`
to the HexoDSP node interface.

```ignore
    // node_tslfo.rs
    use synfx_dsp::{TriSawLFO, Trigger};

    #[derive(Debug, Clone)]
    pub struct TsLFO {
        lfo: Box<TriSawLFO<f64>>,
        trig: Trigger,
    }

    // ...
    impl DspNode for TsLFO {
        // ...
        #[inline]
        fn process<T: NodeAudioContext>(&mut self, /* ... */) {
            // ...
            let lfo = &mut *self.lfo;

            for frame in 0..ctx.nframes() {
                // ...
                out.write(frame, lfo.next_unipolar() as f32);
            }

            // ...
        }
    }
```

The code for `TriSawLFO` in `synfx-dsp` is then independent and reusable else where.

### Node Parameter/Inputs

When implementing your node, you want to access the parameters or inputs of your DSP node.
This is done using the buffer access modules in `dsp/mod.rs` that are defined using the
`node_list` macro. Let me give you a short overview using `node_sin.rs` as an example:

```ignore
    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self,
        ctx: &mut T, // DSP execution context holding the DSP graph input and output buffers.
        _ectx: &mut NodeExecContext, // For providing access to external MIDI events and parameters from the DAW
        _nctx: &NodeContext, // Holds context info about the node, for instance which ports
                             // are connected.
        _atoms: &[SAtom],    // An array holding the Atom parameters
        inputs: &[ProcBuf],  // An array holding the input parameter buffers, containing
                             // either outputs from other DSP nodes or smoothed parameter
                             // settings from the GUI/frontend.
        outputs: &mut [ProcBuf], // An array holding the output buffers.
        ctx_vals: LedPhaseVals,  // Values for visual aids in the GUI (the hextile LED)
    ) {
        use crate::dsp::{denorm_offs, inp, out};

        let o = out::Sin::sig(outputs);
        let freq = inp::Sin::freq(inputs);
        let det = inp::Sin::det(inputs);
        let isr = 1.0 / self.srate;

        let mut last_val = 0.0;
        for frame in 0..ctx.nframes() {
            let freq = denorm_offs::Sampl::freq(freq, det.read(frame), frame);
            // ...
        }
        // ...
    }
```

There are three buffer/parameter function access modules loaded in this example:

```ignore
    use crate::dsp::{denorm_offs, inp, out};
```

`inp` holds a sub module for each of the available nodes. That means: `inp::Sin`, `inp::Ad`, ...
Those submodules each have a function that returns the corresponding buffer from the `inputs`
vector of buffers. That means `inp::Sin::det(inputs)` gives you a reference to a [ProcBuf]
you can read the normalized signal inputs (range -1 to 1) from.

It works similarly with `out::Sin::sig`, which provides you with a [ProcBuf] reference to
write your output to.

`denorm_offs` is a special module, that offers you functions to access the denormalized
value of a specific input parameter with a modulation offset.

Commonly you want to use the `denorm` module to access the denormalized values. That means
values in human understandable form and that can be used in your DSP arithmetics more easily.
For instance `denorm::TsLFO::time` from `node_tslfo.rs`:

```ignore
        use crate::dsp::{denorm, inp, out};

        let time = inp::TsLFO::time(inputs);
        for frame in 0..ctx.nframes() {
            let time_ms = denorm::TsLFO::time(time, frame).clamp(0.1, 300000.0);
            // ...
        }
```

`denorm::TsLFO::time` is a function that takes the [ProcBuf] with the raw normalized
input signal samples and returns the denormalized values in milliseconds for a specific
frame.

To get a hang of all the possibilities I suggest diving a bit into the other node source code
a bit.

### Node Beautification

To make nodes responsive in HexoSynth the `DspNode::process` function receives the [LedPhaseVals].
These should be written after the inner loop that calculates the samples. The first context value
is the _LED value_, it should be in the range between -1 and 1. The most easy way to set it is
by using the last written sample from your loop:

```ignore
        ctx_vals[0].set(out.read(ctx.nframes() - 1));
```

But consider giving it a more meaningful value if possible. The `node_ad.rs` sets the LED value
to the internal phase value of the envelope instead of it's output.

The second value in [LedPhaseVals] is the _Phase value_. It usually has special meaning for the
node specific visualizations. Such as TSeq emits the position of the playhead for instance.
The CQnt quantizer emits the most recently activated key to the GUI using the Phase value.

Consider also providing a visualization graph if possible. You can look eg at `node_ad.rs`
or `node_tslfo.rs` or many others how to provide a visualization graph function:

```ignore
    impl TsLFO {
        pub fn graph_fun() -> Option<GraphFun> {
            Some(Box::new(|gd: &dyn GraphAtomData, _init: bool, x: f32, xn: f32| -> f32 {
                // ...
            }))
        }
    }
```

Let me explain the callback function parameters quickly:

- `gd: &dyn GraphAtomData` this trait object gives you access to the input paramters of
this node. And also the LED and Phase values.
- `init: bool` allows you to detect when the first sample of the graph is being drawn/requested.
You can use this to reset any state that is carried with the callback.
- `x: f32` is the current X position of the graph. Use this to derive the Y value which
must be returned from the callback.
- `xn: f32` is the next value for X. This is useful for determining if your function might
reaches a min or max between _x_ and _xn_, so you could for instance return the min or max
value now instead of the next sample.

### Automated Testing of Your Node

First lets discuss shortly why automated tests are necessary. HexoDSP has an automated test
suite to check if any changes on the internal DSP helpers break something. Or if some
changes on some DSP node accidentally broke something. Or if a platform behaves weirdly.
Or even if upstream crates that are included broke or changed something essential.

A few things you can test your DSP code for:

- Is non 0.0 signal emitted?
- Is the signal inside the -1..1 or 0..1 range?
- Does the signal level change in expected ways if the input parameters are changed?
- Does the frequency spectrum peak at expected points in the FFT output?
- Does the frequency spectrum change to expected points in the FFT output when an input parameter
changed?

Try to nail down the characteristics of your DSP node with a few tests as well as possible.

For the FFT and other tests there are helper functions in `tests/common/mod.rs`

The start of your `tests/node_*.rs` file usually should look like this:

```ignore
    mod common;
    use common::*;

    #[test]
    fn check_node_ad_1() {
        let (node_conf, mut node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        let mut chain = MatrixCellChain::new(CellDir::B);
        chain.node_out("ad", "sig")
            .node_inp("out", "ch1")
            .place(&mut matrix, 0, 0).unwrap();
        matrix.sync().unwrap();

        let ad = NodeId::Ad(0);
        // ...
    }
```

Lets dissect this a bit. The beginning of each test case should setup an instance of the DSP engine
of HexoDSP using [crate::new_node_engine]. It returns a [crate::NodeConfigurator] and a [crate::NodeExecutor].
The first is responsible for setting up the DSP graph and modifying it at runtime.
The latter ([crate::NodeExecutor]) is responsible for executing the DSP graph and generate output samples.

```ignore
        let (node_conf, mut node_exec) = new_node_engine();
```

The [crate::Matrix] abstraction encapsulates the configurator and provides you an interface
to layout the nodes in a hexagonal grid. It is currently the easiest API for using HexoDSP.
The two parameters to _new_ are the width and height of the hex grid.

```ignore
        let mut matrix = Matrix::new(node_conf, 3, 3);
```

Next step is to create a DSP chain of nodes and place that onto the hexagonal matrix.
Luckily a simpler API has been created with the [crate::MatrixCellChain], that lets
you build DSP chains on the fly using only names of the nodes and the corresponding
input/output ports:

```ignore
        // Create a new cell chain that points in to the given direction (CellDir::B => to bottom).
        let mut chain = MatrixCellChain::new(CellDir::B);
        chain.node_out("ad", "sig") // Add a Node::Ad(0) cell, with the "sig" output set
            .node_inp("out", "ch1") // Add a Node::Out(0) cell, with the "ch1" input set
            .place(&mut matrix, 0, 0).unwrap();
```

After placing the new cells, we need to synchronize it with the audio backend:

```ignore
        matrix.sync().unwrap();
```

The `sync` is necessary to update the DSP graph.

Next you usually want to define short variable names for the [NodeId] that refer to the DSP
node instances:

```ignore
        let ad = NodeId::Ad(0);
```

The [NodeId] interface offers you functions to get the input parameter index from
a name like `out.inp("ch1")` or the output port index from a name: `ad.out("sig")`.
You can have multiple instances for a node. The number in the parenthesis are
the instance index of that node.

After you have setup everything for the test, you usually want to modify a paramter
and look at the values the graph returns.

```ignore
    #[test]
    fn check_node_ad_1() {
        // ...
        // matrix setup code above
        // ...

        let ad = NodeId::Ad(0);

        // Fetch parameter id:
        let trig_p = ad.inp_param("trig").unwrap();

        // Set parameter:
        matrix.set_param(trig_p, SAtom::param(1.0));

        /// Run the DSP graph for 25 milliseconds of audio.
        let res = run_for_ms(&mut node_exec, 25.0);

        // `res` now contains two vectors. one for first channel "ch1"
        // and one for the second channel "ch2".
        assert_decimated_slope_feq!(
            res.0,
            // ....
        )
    }
```

***Attention: This is important to keep in mind:*** After using `matrix.set_param(...)` to
set a paramter, keep in mind that the parameter values will be smoothed. That means it will
take a few milliseconds until `trig_p` reaches the 1.0. In case of the Ad node that means
the trigger threshold won't be triggered at the first sample, but a few milliseconds
later!

*/

#[allow(non_upper_case_globals)]
mod node_ad;
#[allow(non_upper_case_globals)]
mod node_adsr;
#[allow(non_upper_case_globals)]
mod node_allp;
#[allow(non_upper_case_globals)]
mod node_amp;
#[allow(non_upper_case_globals)]
mod node_biqfilt;
#[allow(non_upper_case_globals)]
mod node_bosc;
#[allow(non_upper_case_globals)]
mod node_bowstri;
#[allow(non_upper_case_globals)]
mod node_code;
#[allow(non_upper_case_globals)]
mod node_comb;
#[allow(non_upper_case_globals)]
mod node_cqnt;
#[allow(non_upper_case_globals)]
mod node_delay;
#[allow(non_upper_case_globals)]
mod node_ext;
#[allow(non_upper_case_globals)]
mod node_fbwr_fbrd;
#[allow(non_upper_case_globals)]
mod node_formfm;
#[allow(non_upper_case_globals)]
mod node_fvafilt;
#[allow(non_upper_case_globals)]
mod node_inp;
#[allow(non_upper_case_globals)]
mod node_map;
#[allow(non_upper_case_globals)]
mod node_midicc;
#[allow(non_upper_case_globals)]
mod node_midip;
#[allow(non_upper_case_globals)]
mod node_mix3;
#[allow(non_upper_case_globals)]
mod node_mux9;
#[allow(non_upper_case_globals)]
mod node_noise;
#[allow(non_upper_case_globals)]
mod node_out;
#[allow(non_upper_case_globals)]
mod node_pverb;
#[allow(non_upper_case_globals)]
mod node_quant;
#[allow(non_upper_case_globals)]
mod node_rndwk;
#[allow(non_upper_case_globals)]
mod node_rust;
#[allow(non_upper_case_globals)]
mod node_sampl;
#[allow(non_upper_case_globals)]
mod node_scope;
#[allow(non_upper_case_globals)]
mod node_sfilter;
#[allow(non_upper_case_globals)]
mod node_sin;
#[allow(non_upper_case_globals)]
mod node_smap;
#[allow(non_upper_case_globals)]
mod node_test;
#[allow(non_upper_case_globals)]
mod node_tseq;
#[allow(non_upper_case_globals)]
mod node_tslfo;
#[allow(non_upper_case_globals)]
mod node_vosc;

mod satom;
pub mod tracker;

use crate::nodes::NodeAudioContext;
use crate::nodes::NodeExecContext;
use crate::NodeGlobalRef;

use std::cell::UnsafeCell;
use std::sync::Arc;
use synfx_dsp::AtomicFloat;

#[derive(Debug)]
pub struct SyncUnsafeCell<T: ?Sized> {
    value: UnsafeCell<T>,
}

impl<T> SyncUnsafeCell<T> {
    pub fn new(t: T) -> Self {
        Self { value: UnsafeCell::new(t) }
    }
}

impl<T: ?Sized> SyncUnsafeCell<T> {
    /// Gets a pointer to a mutable memory cell that is marked being Sync.
    ///
    /// # Safety
    /// This method is used together with [std::sync::Arc] in an usually
    /// unsafe way to directly get write access to something that is potentially
    /// unsafely shared across multiple threads.
    ///
    /// In HexoDSP this is used for accessing [DspNode] state in the audio thread.
    /// The architecture of HexoDSP makes sure, that the
    /// `Arc<SyncUnsafeCell<dyn DspNode>>` is only accessed from the audio thread
    /// at a given time. Any UI or frontend thread is not going to access these
    /// in [crate::nodes::NodeConfigurator].
    pub unsafe fn get(&self) -> *mut T {
        self.value.get()
    }
}

unsafe impl<T: ?Sized> Sync for SyncUnsafeCell<T> {}

pub type LedPhaseVals<'a> = &'a [Arc<AtomicFloat>];

pub use satom::*;

pub use node_rust::new_dummy_dynamic_node1x1;
pub use node_rust::{DynNode1x1Context, DynamicNode1x1, DynNodeHandle, DynNodeBuffer};

use crate::fa_ad_mult;
use crate::fa_adsr_mult;
use crate::fa_amp_neg_att;
use crate::fa_biqfilt_ord;
use crate::fa_biqfilt_type;
use crate::fa_bosc_wtype;
use crate::fa_comb_mode;
use crate::fa_cqnt;
use crate::fa_cqnt_omax;
use crate::fa_cqnt_omin;
use crate::fa_delay_mode;
use crate::fa_fvafilt_lslope;
use crate::fa_fvafilt_svf_mode;
use crate::fa_fvafilt_type;
use crate::fa_map_clip;
use crate::fa_midicc_cc;
use crate::fa_midip_chan;
use crate::fa_midip_gmode;
use crate::fa_mux9_in_cnt;
use crate::fa_noise_mode;
use crate::fa_out_mono;
use crate::fa_quant;
use crate::fa_sampl_dclick;
use crate::fa_sampl_dir;
use crate::fa_sampl_pmode;
use crate::fa_scope_tsrc;
use crate::fa_sfilter_type;
use crate::fa_smap_clip;
use crate::fa_smap_mode;
use crate::fa_test_s;
use crate::fa_tseq_cmode;
use crate::fa_vosc_ovrsmpl;
use synfx_dsp::fa_distort;

use node_ad::Ad;
use node_adsr::Adsr;
use node_allp::AllP;
use node_amp::Amp;
use node_biqfilt::BiqFilt;
use node_bosc::BOsc;
use node_bowstri::BowStri;
use node_code::Code;
use node_comb::Comb;
use node_cqnt::CQnt;
use node_delay::Delay;
use node_ext::ExtA;
use node_ext::ExtB;
use node_ext::ExtC;
use node_ext::ExtD;
use node_ext::ExtE;
use node_ext::ExtF;
use node_fbwr_fbrd::FbRd;
use node_fbwr_fbrd::FbWr;
use node_formfm::FormFM;
use node_fvafilt::FVaFilt;
use node_inp::Inp;
use node_map::Map;
use node_midicc::MidiCC;
use node_midip::MidiP;
use node_mix3::Mix3;
use node_mux9::Mux9;
use node_noise::Noise;
use node_out::Out;
use node_pverb::PVerb;
use node_quant::Quant;
use node_rndwk::RndWk;
use node_rust::Rust1x1;
use node_sampl::Sampl;
use node_scope::Scope;
use node_sfilter::SFilter;
use node_sin::Sin;
use node_smap::SMap;
use node_test::Test;
use node_tseq::TSeq;
use node_tslfo::TsLFO;
use node_vosc::VOsc;

pub const MIDI_MAX_FREQ: f32 = 13289.75;

pub const MAX_BLOCK_SIZE: usize = 128;

/// A context structure that holds temporary information about the
/// currently executed node.
/// This structure is created by the [crate::nodes::NodeExecutor] on the fly.
pub struct NodeContext<'a> {
    /// The bitmask that indicates which input ports are used/connected
    /// to some output.
    pub in_connected: u64,
    /// The bitmask that indicates which output ports are used/connected
    /// to some input.
    pub out_connected: u64,
    /// The node parameters, which are usually not accessed directly.
    pub params: &'a [ProcBuf],
}

/// This trait is an interface between the graph functions
/// and the AtomDataModel of the UI.
pub trait GraphAtomData {
    fn get(&self, param_idx: u32) -> Option<SAtom>;
    fn get_denorm(&self, param_idx: u32) -> f32;
    fn get_norm(&self, param_idx: u32) -> f32;
    fn get_phase(&self) -> f32;
    fn get_led(&self) -> f32;
}

pub type GraphFun = Box<dyn FnMut(&dyn GraphAtomData, bool, f32, f32) -> f32>;

/// This trait represents a DspNode for the [crate::matrix::Matrix]
pub trait DspNode: std::fmt::Debug + Send {
    /// Updates the sample rate for the node.
    fn set_sample_rate(&mut self, _srate: f32);

    /// Reset any internal state of the node.
    fn reset(&mut self);

    /// The code DSP function.
    ///
    /// * `ctx` is the audio context, which informs the node about
    /// the number of samples to process. It also provides input/output
    /// ports for the in/out nodes.
    /// * `ectx` is the execution context, which provides global stuff
    /// for all nodes to potentially use. For instance it's used
    /// by the `FbWr` and `FbRd` nodes to share an audio buffer.
    /// * `atoms` are un-smoothed parameters. they can hold integer settings,
    /// samples or even strings.
    /// * `params` are smoother paramters, those who usually have a knob
    /// associated with them.
    /// * `inputs` contain all the possible inputs. In contrast to `params`
    /// these inputs might be overwritten by outputs of other nodes.
    /// * `outputs` are the output buffers of this node.
    fn process(
        &mut self,
        ctx: &mut dyn NodeAudioContext,
        ectx: &mut NodeExecContext,
        nctx: &NodeContext,
        atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        led: LedPhaseVals,
    );
}

/// A processing buffer with the exact right maximum size.
/// This is an unsafe abstraction, and should be used with a lot of care.
/// You will have to manually free the buffer, and take care if you
/// make copies of these.
///
/// This is an abstraction for the inner most DSP processing, where I
/// don't want to spend a nanosecond too much on accessing buffers.
///
/// The main user is [crate::nodes::NodeProg], which takes extra care
/// of allocating and managing the [ProcBuf] instances.
///
///```
/// let mut buf = hexodsp::dsp::ProcBuf::new();
///
/// buf.write(0, 0.42);
/// buf.write(1, 0.13);
/// buf.write(2, 0.37);
///
/// assert_eq!(((buf.read(0) * 100.0).floor()), 42.0);
/// assert_eq!(((buf.read(1) * 100.0).floor()), 13.0);
/// assert_eq!(((buf.read(2) * 100.0).floor()), 37.0);
///
/// buf.free(); // YOU MUST DO THIS!
///```
#[derive(Clone, Copy)]
pub struct ProcBuf(*mut [f32; MAX_BLOCK_SIZE]);

impl ProcBuf {
    /// Creates a new ProcBuf with the size of [MAX_BLOCK_SIZE].
    pub fn new() -> Self {
        ProcBuf(Box::into_raw(Box::new([0.0; MAX_BLOCK_SIZE])))
    }

    /// Create a new null ProcBuf, that can't be used.
    pub fn null() -> Self {
        ProcBuf(std::ptr::null_mut())
    }
}

impl crate::monitor::MonitorSource for &ProcBuf {
    /// Copies the contents of this [ProcBuf] to the given `slice`.
    ///
    /// * `len` - the number of samples to copy from this [ProcBuf].
    /// * `slice` - the slice to copy to.
    fn copy_to(&self, len: usize, slice: &mut [f32]) {
        unsafe { slice.copy_from_slice(&(*self.0)[0..len]) }
    }
}

unsafe impl Send for ProcBuf {}
unsafe impl Sync for ProcBuf {}
//unsafe impl Sync for HexoSynthShared {}

impl ProcBuf {
    /// Writes the sample `v` at `idx`.
    #[inline]
    pub fn write(&mut self, idx: usize, v: f32) {
        unsafe {
            (*self.0)[idx] = v;
        }
    }

    /// Writes the samples from `slice` to this [ProcBuf].
    /// Be careful, the `slice` must not exceed [MAX_BLOCK_SIZE], or else
    /// you will get UB.
    #[inline]
    pub fn write_from(&mut self, slice: &[f32]) {
        unsafe {
            (*self.0)[0..slice.len()].copy_from_slice(slice);
        }
    }

    /// Reads a sample at `idx`. Be careful to not let the `idx`
    /// land outside of [MAX_BLOCK_SIZE].
    #[inline]
    pub fn read(&self, idx: usize) -> f32 {
        unsafe { (*self.0)[idx] }
    }

    /// Returns a mutable slice to the inner buffer.
    /// `len` **must not** exceed [MAX_BLOCK_SIZE].
    #[inline]
    pub fn slice(&self, len: usize) -> &[f32] {
        unsafe { &(*self.0)[0..len] }
    }

    /// Returns a mutable slice to the inner buffer.
    /// `len` **must not** exceed [MAX_BLOCK_SIZE].
    #[inline]
    pub fn slice_mut(&mut self, len: usize) -> &mut [f32] {
        unsafe { &mut (*self.0)[0..len] }
    }

    /// Fills the [ProcBuf] with the sample `v`.
    #[inline]
    pub fn fill(&mut self, v: f32) {
        unsafe {
            (*self.0).fill(v);
        }
    }

    /// Checks if this is a [ProcBuf::null].
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Deallocates the [ProcBuf]. If you still keep around
    /// other copies of this [ProcBuf], you will most likely land in
    /// UB land.
    pub fn free(&self) {
        if !self.0.is_null() {
            drop(unsafe { Box::from_raw(self.0) });
        }
    }
}

impl std::fmt::Debug for ProcBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            write!(f, "ProcBuf(")?;
            if self.0.is_null() {
                write!(f, "NULL ")?;
            } else {
                for i in 0..MAX_BLOCK_SIZE {
                    write!(f, "{:5.4} ", (*self.0)[i])?;
                }
            }
            write!(f, ")")
        }
    }
}

impl std::fmt::Display for ProcBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { write!(f, "ProcBuf(0: {})", (*self.0)[0]) }
    }
}

//#[derive(Debug, Clone, Copy)]
//enum UIParamDesc {
//    Knob    { width: usize, prec: usize, unit: &'static str },
//    Setting { labels: &'static [&'static str], unit: &'static str },
//}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum UIType {
    Generic,
    LfoA,
    EnvA,
    OscA,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum UICategory {
    None,
    Osc,
    Mod,
    NtoM,
    Signal,
    Ctrl,
    IOUtil,
}

impl UICategory {
    pub fn default_color_idx(&self) -> u8 {
        match self {
            UICategory::None => 17,
            UICategory::Osc => 0,
            UICategory::Mod => 7,
            UICategory::NtoM => 4,
            UICategory::Signal => 2,
            UICategory::Ctrl => 12,
            UICategory::IOUtil => 10,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            UICategory::None => "None",
            UICategory::Osc => "Osc",
            UICategory::Mod => "Mod",
            UICategory::NtoM => "NtoM",
            UICategory::Signal => "Signal",
            UICategory::Ctrl => "Ctrl",
            UICategory::IOUtil => "IOUtil",
        }
    }
}

// The following macros define normalize/denormalize functions:
macro_rules! n_id {
    ($x: expr) => {
        $x
    };
}
macro_rules! d_id {
    ($x: expr) => {
        $x
    };
}

macro_rules! define_lin {
    ($n_id: ident $d_id: ident $min: expr, $max: expr) => {
        macro_rules! $n_id {
            ($x: expr) => {
                (($x - $min) / ($max - $min) as f32).abs()
            };
        }

        macro_rules! $d_id {
            ($x: expr) => {
                $min * (1.0 - $x) + $max * $x
            };
        }
    };
}

macro_rules! define_exp {
    ($n_id: ident $d_id: ident $min: expr, $max: expr) => {
        macro_rules! $n_id {
            ($x: expr) => {
                (($x - $min) / ($max - $min) as f32).abs().sqrt()
            };
        }
        macro_rules! $d_id {
            ($x: expr) => {{
                let x: f32 = $x * $x;
                $min * (1.0 - x) + $max * x
            }};
        }
    };
}

macro_rules! define_exp4 {
    ($n_id: ident $d_id: ident $min: expr, $max: expr) => {
        macro_rules! $n_id {
            ($x: expr) => {
                (($x - $min) / ($max - $min) as f32).abs().sqrt().sqrt()
            };
        }
        macro_rules! $d_id {
            ($x: expr) => {{
                let x: f32 = $x * $x * $x * $x;
                $min * (1.0 - x) + $max * x
            }};
        }
    };
}

macro_rules! define_exp6 {
    ($n_id: ident $d_id: ident $min: expr, $max: expr) => {
        macro_rules! $n_id {
            ($x: expr) => {
                (($x - $min) / ($max - $min) as f32).abs().powf(1.0 / 6.0)
            };
        }
        macro_rules! $d_id {
            ($x: expr) => {{
                let x: f32 = ($x).powf(6.0);
                $min * (1.0 - x) + $max * x
            }};
        }
    };
}

macro_rules! define_db {
    ($n_id: ident $d_id: ident $r_id: ident $min: expr, $max: expr) => {
        macro_rules! $n_id {
            ($x: expr) => {
                ((synfx_dsp::coef2gain_db($x) - ($min)) / ($max - ($min)) as f32).abs()
            };
        }
        macro_rules! $d_id {
            ($x: expr) => {{
                synfx_dsp::gain_db2coef(($min) * (1.0 - $x) + $x * $max)
            }};
        }
        macro_rules! $r_id {
            ($x: expr, $coarse: expr) => {
                if $coarse {
                    let db = synfx_dsp::coef2gain_db($d_id!($x));
                    $n_id!(synfx_dsp::gain_db2coef((db * 2.0).round() * 0.5))
                } else {
                    let db = synfx_dsp::coef2gain_db($d_id!($x));
                    $n_id!(synfx_dsp::gain_db2coef((db * 10.0).round() * 0.1))
                }
            };
        }
    };
}

#[macro_export]
/// This macro calculates the normalized pitch value from a pitch in Hz.
macro_rules! n_pit {
    ($x: expr) => {
        0.1 * (($x as f32).max(0.01) / 440.0).log2()
    };
}

#[macro_export]
/// This macro denormalizes a pitch value and returns the pitch in Hz.
macro_rules! d_pit {
    ($x: expr) => {{
        let note: f32 = ($x as f32) * 10.0;
        440.0 * (2.0_f32).powf(note.clamp(-10.0, 10.0))
    }};
}

// The following macros define detune parameter behaviour:
// 0.2         => 24.0
// 0.1         => 12.0
// 0.008333333 => 1.0
// 0.000083333 => 0.001
macro_rules! n_det {
    ($x: expr) => {
        $x / 120.0
    };
}
macro_rules! d_det {
    ($x: expr) => {
        $x * 120.0
    };
}
/// The rounding function for detune UI knobs
macro_rules! r_det {
    ($x: expr, $coarse: expr) => {
        if $coarse {
            n_det!((d_det!($x)).round())
        } else {
            n_det!((d_det!($x) * 100.0).round() / 100.0)
        }
    };
}

/// The rounding function for -1 to 1 signal knobs
macro_rules! r_s {
    ($x: expr, $coarse: expr) => {
        if $coarse {
            ($x * 10.0).round() / 10.0
        } else {
            ($x * 100.0).round() / 100.0
        }
    };
}

/// The rounding function for milliseconds knobs
macro_rules! r_dc_ms {
    ($x: expr, $coarse: expr) => {
        if $coarse {
            n_declick!((d_declick!($x)).round())
        } else {
            n_declick!((d_declick!($x) * 10.0).round() / 10.0)
        }
    };
}

/// The rounding function for milliseconds knobs
macro_rules! r_ems {
    ($x: expr, $coarse: expr) => {
        if $coarse {
            n_env!((d_env!($x)).round())
        } else {
            n_env!((d_env!($x) * 10.0).round() / 10.0)
        }
    };
}

/// The rounding function for milliseconds knobs
macro_rules! r_tms {
    ($x: expr, $coarse: expr) => {
        if $coarse {
            if d_time!($x) > 1000.0 {
                n_time!((d_time!($x) / 100.0).round() * 100.0)
            } else if d_time!($x) > 100.0 {
                n_time!((d_time!($x) / 10.0).round() * 10.0)
            } else {
                n_time!((d_time!($x)).round())
            }
        } else {
            n_time!((d_time!($x) * 10.0).round() / 10.0)
        }
    };
}

/// The rounding function for milliseconds knobs
macro_rules! r_fms {
    ($x: expr, $coarse: expr) => {
        if $coarse {
            if d_ftme!($x) > 1000.0 {
                n_ftme!((d_ftme!($x) / 100.0).round() * 100.0)
            } else if d_ftme!($x) > 100.0 {
                n_ftme!((d_ftme!($x) / 10.0).round() * 10.0)
            } else {
                n_ftme!((d_ftme!($x)).round())
            }
        } else {
            n_ftme!((d_ftme!($x) * 10.0).round() / 10.0)
        }
    };
}

/// The rounding function for milliseconds knobs that also have a 0.0 setting
macro_rules! r_tmz {
    ($x: expr, $coarse: expr) => {
        if $coarse {
            if d_timz!($x) > 1000.0 {
                n_timz!((d_timz!($x) / 100.0).round() * 100.0)
            } else if d_timz!($x) > 100.0 {
                n_timz!((d_timz!($x) / 10.0).round() * 10.0)
            } else {
                n_timz!((d_timz!($x)).round())
            }
        } else {
            n_timz!((d_timz!($x) * 10.0).round() / 10.0)
        }
    };
}

/// The rounding function for freq knobs (n_pit / d_pit)
macro_rules! r_fq {
    ($x: expr, $coarse: expr) => {
        if $coarse {
            ($x * 10.0).round() / 10.0
        } else {
            let p = d_pit!($x);
            if p < 10.0 {
                n_pit!((p * 10.0).round() / 10.0)
            } else if p < 100.0 {
                n_pit!(p.round())
            } else if p < 1000.0 {
                n_pit!((p / 10.0).round() * 10.0)
            } else if p < 10000.0 {
                n_pit!((p / 100.0).round() * 100.0)
            } else {
                n_pit!((p / 1000.0).round() * 1000.0)
            }
        }
    };
}

/// The rounding function for vs (v scale) UI knobs
macro_rules! r_vps {
    ($x: expr, $coarse: expr) => {
        if $coarse {
            n_vps!((d_vps!($x)).round())
        } else {
            n_vps!((d_vps!($x) * 10.0).round() / 10.0)
        }
    };
}

/// The rounding function for LFO time knobs
macro_rules! r_lfot {
    ($x: expr, $coarse: expr) => {
        if $coarse {
            let denv = d_lfot!($x);

            if denv < 10.0 {
                let hz = 1000.0 / denv;
                let hz = (hz / 10.0).round() * 10.0;
                n_lfot!(1000.0 / hz)
            } else if denv < 250.0 {
                n_lfot!((denv / 5.0).round() * 5.0)
            } else if denv < 1500.0 {
                n_lfot!((denv / 50.0).round() * 50.0)
            } else if denv < 2500.0 {
                n_lfot!((denv / 100.0).round() * 100.0)
            } else if denv < 5000.0 {
                n_lfot!((denv / 500.0).round() * 500.0)
            } else if denv < 60000.0 {
                n_lfot!((denv / 1000.0).round() * 1000.0)
            } else {
                n_lfot!((denv / 5000.0).round() * 5000.0)
            }
        } else {
            let denv = d_lfot!($x);

            let o = if denv < 10.0 {
                let hz = 1000.0 / denv;
                let hz = hz.round();
                n_lfot!(1000.0 / hz)
            } else if denv < 100.0 {
                n_lfot!(denv.round())
            } else if denv < 1000.0 {
                n_lfot!((denv / 5.0).round() * 5.0)
            } else if denv < 2500.0 {
                n_lfot!((denv / 10.0).round() * 10.0)
            } else if denv < 25000.0 {
                n_lfot!((denv / 100.0).round() * 100.0)
            } else {
                n_lfot!((denv / 500.0).round() * 500.0)
            };

            o
        }
    };
}

/// The default steps function:
macro_rules! stp_d {
    () => {
        (20.0, 100.0)
    };
}
/// The UI steps to control parameters with a finer fine control:
macro_rules! stp_m {
    () => {
        (20.0, 200.0)
    };
}
/// The UI steps to control parameters with a very fine fine control:
macro_rules! stp_f {
    () => {
        (20.0, 1000.0)
    };
}
/// The UI steps to control parameters with a very fine fine control (including default):
macro_rules! stp_v {
    () => {
        (40.0, 1000.0)
    };
}

// Rounding function that does nothing
macro_rules! r_id {
    ($x: expr, $coarse: expr) => {
        $x
    };
}

// Default formatting function
macro_rules! f_def {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {
        write!($formatter, "{:6.3}", $denorm_v)
    };
}

// Default formatting function with low precision
macro_rules! f_deflp {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {
        write!($formatter, "{:5.2}", $denorm_v)
    };
}

// Default formatting function with very low precision
macro_rules! f_defvlp {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {
        write!($formatter, "{:4.1}", $denorm_v)
    };
}

macro_rules! f_freq {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {
        if ($denorm_v >= 1000.0) {
            write!($formatter, "{:6.0}Hz", $denorm_v)
        } else if ($denorm_v >= 100.0) {
            write!($formatter, "{:6.1}Hz", $denorm_v)
        } else {
            write!($formatter, "{:6.2}Hz", $denorm_v)
        }
    };
}

macro_rules! f_ms {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {
        if $denorm_v >= 1000.0 {
            write!($formatter, "{:6.0}ms", $denorm_v)
        } else if $denorm_v >= 100.0 {
            write!($formatter, "{:5.1}ms", $denorm_v)
        } else {
            write!($formatter, "{:5.2}ms", $denorm_v)
        }
    };
}

macro_rules! f_lfot {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {
        if $denorm_v < 10.0 {
            write!($formatter, "{:5.1}Hz", 1000.0 / $denorm_v)
        } else if $denorm_v < 250.0 {
            write!($formatter, "{:4.1}ms", $denorm_v)
        } else if $denorm_v < 1500.0 {
            write!($formatter, "{:4.0}ms", $denorm_v)
        } else if $denorm_v < 10000.0 {
            write!($formatter, "{:5.2}s", $denorm_v / 1000.0)
        } else {
            write!($formatter, "{:5.1}s", $denorm_v / 1000.0)
        }
    };
}

macro_rules! f_lfoms {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {
        if $denorm_v < 10.0 {
            write!($formatter, "{:5.3}ms", $denorm_v)
        } else if $denorm_v < 250.0 {
            write!($formatter, "{:4.1}ms", $denorm_v)
        } else if $denorm_v < 1500.0 {
            write!($formatter, "{:4.0}ms", $denorm_v)
        } else if $denorm_v < 10000.0 {
            write!($formatter, "{:5.2}s", $denorm_v / 1000.0)
        } else {
            write!($formatter, "{:5.1}s", $denorm_v / 1000.0)
        }
    };
}

macro_rules! f_det {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let sign = if $denorm_v < 0.0 { -1.0 } else { 1.0 };
        let semitones = $denorm_v.trunc().abs();
        let cents = ($denorm_v.fract() * 100.0).round().abs();

        if (cents > 0.1) {
            write!($formatter, "{:2.0}s {:3.0}c", sign * semitones, cents)
        } else {
            write!($formatter, "{:2.0}s", sign * semitones)
        }
    }};
}

// Default formatting function
macro_rules! f_db {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let db = synfx_dsp::coef2gain_db($denorm_v);
        if db < -89.90 {
            write!($formatter, "-inf dB")
        } else {
            // snap to 0.00dB (not -0.00dB)
            let db = if db.abs() < 0.0001 { 0.0 } else { db };
            write!($formatter, "{:+4.1}dB", db)
        }
    }};
}

//          norm   denorm round  min     max
define_db! {n_vol  d_vol  r_vol  -90.0,  18.0}
define_db! {n_gain d_gain r_gain -24.0,  24.0}
define_db! {n_driv d_driv r_driv   0.0,  24.0}

//          norm-fun      denorm-min
//                 denorm-fun  denorm-max
define_exp! {n_declick d_declick 0.0, 50.0}

define_exp! {n_env d_env 0.0, 1000.0}

define_exp6! {n_lfot d_lfot 0.1,300000.0}
define_exp! {n_time d_time 0.5,  5000.0}
define_exp! {n_ftme d_ftme 0.1,  1000.0}
define_exp! {n_timz d_timz 0.0,  5000.0}

// Special linear gain factor for the Out node, to be able
// to reach more exact "1.0".
define_lin! {n_vps d_vps 0.0, 20.0}

// A note about the input-indicies:
//
// Atoms and Input parameters share the same global ID space
// because thats how the client of the Matrix API needs to refer to
// them. Beyond the Matrix API the atom data is actually split apart
// from the parameters, because they are not smoothed.
//
// The index there only matters for addressing the atoms in the global atom vector.
//
// But the actually second index here is for referring to the atom index
// relative to the absolute count of atom data a Node has.
// It is used by the [Matrix] to get the global ParamId for the atom data
// when iterating through the atoms of a Node and initializes the default data
// for new nodes.
#[macro_export]
macro_rules! node_list {
    ($inmacro: ident) => {
        $inmacro! {
            nop => Nop,
            amp => Amp UIType::Generic UICategory::Signal
             // node_param_idx
             //   name             denorm round format steps norm norm denorm
             //         norm_fun   fun    fun   fun    def   min  max  default
               (0 inp   n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (1 gain  n_gain     d_gain r_gain f_db  stp_v  0.0, 1.0, 1.0)
               (2 att   n_id       d_id   r_id  f_def  stp_d  0.0, 1.0, 1.0)
               {3 0 neg_att setting(1) mode fa_amp_neg_att 0  1}
               [0 sig],
            mix3 => Mix3 UIType::Generic UICategory::NtoM
               (0 ch1   n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (1 ch2   n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (2 ch3   n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (3 vol1  n_vol      d_vol  r_vol f_db   stp_v  0.0, 1.0, 1.0)
               (4 vol2  n_vol      d_vol  r_vol f_db   stp_v  0.0, 1.0, 1.0)
               (5 vol3  n_vol      d_vol  r_vol f_db   stp_v  0.0, 1.0, 1.0)
               (6 ovol  n_vol      d_vol  r_vol f_db   stp_v  0.0, 1.0, 1.0)
               [0 sig],
            mux9 => Mux9 UIType::Generic UICategory::NtoM
               ( 0 slct    n_id       d_id   r_id  f_def  stp_d  0.0, 1.0, 0.0)
               ( 1 t_rst   n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               ( 2 t_up    n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               ( 3 t_down  n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               ( 4 in_1    n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               ( 5 in_2    n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               ( 6 in_3    n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               ( 7 in_4    n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               ( 8 in_5    n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               ( 9 in_6    n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (10 in_7    n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (11 in_8    n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (12 in_9    n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               {13 0 in_cnt setting(3) mode fa_mux9_in_cnt 0 8}
               [0 sig],
            smap => SMap UIType::Generic UICategory::Ctrl
               (0 inp   n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (1 min   n_id       d_id   r_s   f_def  stp_d -1.0, 1.0, -1.0)
               (2 max   n_id       d_id   r_s   f_def  stp_d -1.0, 1.0, 1.0)
               {3 1 mode setting(0) mode fa_smap_mode 0  3}
               {4 0 clip setting(0) mode fa_smap_clip 0  1}
               [0 sig],
            map => Map UIType::Generic UICategory::Ctrl
               (0 inp   n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (1 atv   n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 1.0)
               (2 offs  n_id       d_id   r_s   f_def  stp_d -1.0, 1.0, 0.0)
               (3 imin  n_id       d_id   r_s   f_def  stp_d -1.0, 1.0, -1.0)
               (4 imax  n_id       d_id   r_s   f_def  stp_d -1.0, 1.0, 1.0)
               (5 min   n_id       d_id   r_s   f_def  stp_d -1.0, 1.0, -1.0)
               (6 max   n_id       d_id   r_s   f_def  stp_d -1.0, 1.0, 1.0)
               {7 0 clip setting(0) mode fa_map_clip 0  1}
               [0 sig],
            quant => Quant UIType::Generic UICategory::Ctrl
               (0 freq  n_pit      d_pit  r_id  f_freq stp_d -1.0, 0.5647131, 440.0)
               (1 oct   n_id       d_id   r_s   f_def  stp_d -1.0, 1.0, 0.0)
               {2 0 keys setting(0) keys fa_quant     0 0}
               [0 sig]
               [1 t],
            cqnt => CQnt UIType::Generic UICategory::Ctrl
               (0 inp   n_id       d_id   r_id  f_def  stp_d  0.0, 1.0, 0.0)
               (1 oct   n_id       d_id   r_s   f_def  stp_d -1.0, 1.0, 0.0)
               {2 0 keys setting(0) keys fa_cqnt      0 0}
               {3 1 omin setting(0) mode fa_cqnt_omin 0 4}
               {4 2 omax setting(0) mode fa_cqnt_omax 0 4}
               [0 sig]
               [1 t],
            tseq => TSeq UIType::Generic UICategory::Mod
               (0 clock n_id       d_id   r_id  f_def  stp_d  0.0, 1.0, 0.0)
               (1 trig  n_id       n_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               {2 0 cmode setting(1) mode fa_tseq_cmode 0  2}
               [0 trk1]
               [1 trk2]
               [2 trk3]
               [3 trk4]
               [4 trk5]
               [5 trk6]
               [6  gat1]
               [7  gat2]
               [8  gat3]
               [9  gat4]
               [10 gat5]
               [11 gat6],
            code => Code UIType::Generic UICategory::Signal
               (0 in1   n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (1 in2   n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (2 alpha n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (3 beta  n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (4 delta n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (5 gamma n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               [0 sig]
               [1 sig1]
               [2 sig2],
            rust1x1 => Rust1x1 UIType::Generic UICategory::Signal
               (0 inp   n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (1 alpha n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (2 beta  n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (3 delta n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               (4 gamma n_id       d_id   r_id  f_def  stp_d -1.0, 1.0, 0.0)
               [0 sig],
            sampl => Sampl UIType::Generic UICategory::Osc
               (0 freq  n_pit      d_pit  r_fq  f_freq   stp_d -1.0, 0.564713133, 440.0)
               (1 trig  n_id       d_id   r_id  f_def    stp_d -1.0, 1.0, 0.0)
               (2 offs  n_id       d_id   r_id  f_def    stp_d  0.0, 1.0, 0.0)
               (3 len   n_id       d_id   r_id  f_def    stp_d  0.0, 1.0, 1.0)
               (4 dcms  n_declick  d_declick r_dc_ms f_ms   stp_m  0.0, 1.0, 3.0)
               (5 det   n_det      d_det  r_det f_det    stp_f -0.2, 0.2, 0.0)
               {6 0 sample  audio_unloaded("")   sample f_def 0 0}
               {7 1 pmode   setting(0)           mode   fa_sampl_pmode   0 1}
               {8 2 dclick  setting(0)           mode   fa_sampl_dclick  0 1}
               {9 3 dir     setting(0)           mode   fa_sampl_dir     0 1}
               [0 sig],
             // node_param_idx
             //   name             denorm round format steps norm norm denorm
             //         norm_fun   fun    fun   fun    def   min  max  default
            sin => Sin UIType::Generic UICategory::Osc
               (0 freq  n_pit      d_pit r_fq  f_freq  stp_d -1.0, 0.5647131, 440.0)
               (1 det   n_det      d_det r_det f_det   stp_f -0.2, 0.2,   0.0)
               (2 pm    n_id       d_id  r_id  f_def   stp_d -1.0, 1.0,   0.0)
               [0 sig],
            bosc => BOsc UIType::Generic UICategory::Osc
               (0 freq  n_pit      d_pit r_fq  f_freq  stp_d -1.0, 0.5647131, 440.0)
               (1 det   n_det      d_det r_det f_det   stp_f -0.2, 0.2,   0.0)
               (2 pw    n_id       n_id  r_id  f_def   stp_d  0.0, 1.0,   0.5)
               {3 0 wtype setting(0) mode fa_bosc_wtype 0 4}
               [0 sig],
            vosc => VOsc UIType::Generic UICategory::Osc
               (0 freq  n_pit      d_pit r_fq  f_freq  stp_d -1.0, 0.5647131, 440.0)
               (1 det   n_det      d_det r_det f_det   stp_f -0.2, 0.2,   0.0)
               (2 d     n_id       n_id  r_id  f_def   stp_d  0.0, 1.0,   0.5)
               (3 v     n_id       n_id  r_id  f_def   stp_d  0.0, 1.0,   0.5)
               (4 vs    n_vps     d_vps r_vps f_defvlp stp_d  0.0, 1.0,   0.0)
               (5 damt  n_id       n_id  r_id  f_def   stp_d  0.0, 1.0,   0.0)
               {6 0 dist     setting(0) mode fa_distort 0 3}
               {7 1 ovrsmpl  setting(1) mode fa_vosc_ovrsmpl 0 1}
               [0 sig],
            bowstri => BowStri UIType::Generic UICategory::Osc
               (0 freq  n_pit      d_pit r_fq  f_freq  stp_d -1.0, 0.5647131, 440.0)
               (1 det   n_det      d_det r_det f_det   stp_f -0.2, 0.2, 0.0)
               (2 vel   n_id       n_id  r_id  f_def   stp_d  0.0, 1.0, 0.5)
               (3 force n_id       n_id  r_id  f_def   stp_d  0.0, 1.0, 0.5)
               (4 pos   n_id       n_id  r_id  f_def   stp_d  0.0, 1.0, 0.5)
               [0 sig],
            midip => MidiP UIType::Generic UICategory::IOUtil
               (0 det   n_det      d_det r_det f_det   stp_f -0.2, 0.2, 0.0)
               (1 glen  n_lfot   d_lfot r_lfot f_lfoms stp_f 0.0, 1.0, 250.0)
               {2 0 chan setting(0) mode fa_midip_chan 0 16}
               {3 1 gmode setting(0) mode fa_midip_gmode 0 2}
               [0 freq]
               [1 gate]
               [2 vel],
            midicc => MidiCC UIType::Generic UICategory::IOUtil
               (0 slew  n_timz   d_timz r_tmz  f_ms  stp_m  0.0, 1.0, 0.0)
               {1 0 chan setting(0) mode fa_midip_chan 0 16}
               {2 1 cc1  setting(0) mode fa_midicc_cc 0 127}
               {3 2 cc2  setting(0) mode fa_midicc_cc 0 127}
               {4 3 cc3  setting(0) mode fa_midicc_cc 0 127}
               [0 sig1]
               [1 sig2]
               [2 sig3],
            exta => ExtA UIType::Generic UICategory::IOUtil
               (0 slew  n_timz   d_timz r_tmz  f_ms  stp_m  0.0, 1.0, 0.0)
               (1 atv1  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               (2 atv2  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               (3 atv3  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               [0 sig1]
               [1 sig2]
               [2 sig3],
            extb => ExtB UIType::Generic UICategory::IOUtil
               (0 slew  n_timz   d_timz r_tmz  f_ms  stp_m  0.0, 1.0, 0.0)
               (1 atv1  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               (2 atv2  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               (3 atv3  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               [0 sig1]
               [1 sig2]
               [2 sig3],
            extc => ExtC UIType::Generic UICategory::IOUtil
               (0 slew  n_timz   d_timz r_tmz  f_ms  stp_m  0.0, 1.0, 0.0)
               (1 atv1  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               (2 atv2  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               (3 atv3  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               [0 sig1]
               [1 sig2]
               [2 sig3],
            extd => ExtD UIType::Generic UICategory::IOUtil
               (0 slew  n_timz   d_timz r_tmz  f_ms  stp_m  0.0, 1.0, 0.0)
               (1 atv1  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               (2 atv2  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               (3 atv3  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               [0 sig1]
               [1 sig2]
               [2 sig3],
            exte => ExtE UIType::Generic UICategory::IOUtil
               (0 slew  n_timz   d_timz r_tmz  f_ms  stp_m  0.0, 1.0, 0.0)
               (1 atv1  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               (2 atv2  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               (3 atv3  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               [0 sig1]
               [1 sig2]
               [2 sig3],
            extf => ExtF UIType::Generic UICategory::IOUtil
               (0 slew  n_timz   d_timz r_tmz  f_ms  stp_m  0.0, 1.0, 0.0)
               (1 atv1  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               (2 atv2  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               (3 atv3  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               [0 sig1]
               [1 sig2]
               [2 sig3],
            inp => Inp UIType::Generic UICategory::IOUtil
               (0  vol  n_vol     d_vol  r_vol f_db   stp_v  0.0, 1.0, 1.0)
               [0 sig1]
               [1 sig2],
            out => Out UIType::Generic UICategory::IOUtil
               (0  ch1   n_id      d_id  r_id   f_def  stp_d -1.0, 1.0, 0.0)
               (1  ch2   n_id      d_id  r_id   f_def  stp_d -1.0, 1.0, 0.0)
               (2  vol   n_vol     d_vol r_vol  f_db   stp_v  0.0, 1.0, 1.0)
             // node_param_idx      UI widget type (mode, knob, sample)
             // | atom_idx          |     format fun
             // | | name constructor|     |     min max
             // | | |    |       def|ult_v|lue  |  /
             // | | |    |       |  |     |     |  |
               {3 0 mono setting(0) mode fa_out_mono 0  1},
            fbwr => FbWr UIType::Generic UICategory::IOUtil
               (0  inp   n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0),
            fbrd => FbRd UIType::Generic UICategory::IOUtil
               (0  vol   n_vol     d_vol r_vol  f_db stp_v  0.0, 1.0, 1.0)
               [0 sig],
            scope => Scope UIType::Generic UICategory::IOUtil
               (0  in1   n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (1  in2   n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (2  in3   n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (3  time  n_lfot  d_lfot r_lfot f_lfoms stp_f 0.0, 1.0, 1000.0)
               (4  trig  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (5  thrsh n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (6  off1  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (7  off2  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (8  off3  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (9  gain1 n_gain   d_gain r_gain f_db  stp_v 0.0, 1.0, 1.0)
               (10 gain2 n_gain   d_gain r_gain f_db  stp_v 0.0, 1.0, 1.0)
               (11 gain3 n_gain   d_gain r_gain f_db  stp_v 0.0, 1.0, 1.0)
               {12 0 tsrc  setting(0) mode fa_scope_tsrc 0 2},
            ad   => Ad   UIType::Generic UICategory::Mod
               (0  inp   n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               (1  trig  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (2  atk   n_env     d_env r_ems  f_ms  stp_m  0.0, 1.0, 3.0)
               (3  dcy   n_env     d_env r_ems  f_ms  stp_m  0.0, 1.0, 10.0)
               (4  ashp  n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.5)
               (5  dshp  n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.5)
               {6 0 mult setting(0) mode fa_ad_mult  0 2}
               [0 sig]
               [1 eoet],
            adsr => Adsr UIType::Generic UICategory::Mod
               (0  inp   n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 1.0)
               (1  gate  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (2  atk   n_env     d_env r_ems  f_ms  stp_m  0.0, 1.0, 3.0)
               (3  dcy   n_env     d_env r_ems  f_ms  stp_m  0.0, 1.0, 10.0)
               (4  sus   n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.5)
               (5  rel   n_env     d_env r_ems  f_ms  stp_m  0.0, 1.0, 40.0)
               (6  ashp  n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.5)
               (7  dshp  n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.5)
               (8  rshp  n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.5)
               {9 0 mult setting(0) mode fa_adsr_mult  0 2}
               [0 sig]
               [1 eoet],
            tslfo => TsLFO UIType::Generic UICategory::Mod
                (0 time  n_lfot   d_lfot r_lfot f_lfot stp_f 0.0, 1.0, 1000.0)
                (1 trig  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
                (2 rev   n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.5)
                [0 sig],
            rndwk => RndWk UIType::Generic UICategory::Mod
                (0 trig  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
                (1 step  n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.2)
                (2 offs  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
                (3 min   n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.0)
                (4 max   n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 1.0)
                (5 slew  n_timz   d_timz r_tmz  f_ms  stp_m  0.0, 1.0, 75.0)
                [0 sig],
            delay => Delay UIType::Generic UICategory::Signal
               (0  inp   n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (1  trig  n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (2  time  n_time   d_time r_tms  f_ms  stp_m  0.0, 1.0, 250.0)
               (3  fb    n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (4  mix   n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.5)
               {5 0 mode setting(0) mode fa_delay_mode 0 1}
               [0 sig],
            allp  => AllP UIType::Generic UICategory::Signal
               (0  inp   n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (1  time  n_ftme   d_ftme r_fms  f_ms  stp_m  0.0, 1.0, 25.0)
               (2  g     n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.7)
               [0 sig],
            comb  => Comb UIType::Generic UICategory::Signal
               (0  inp   n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (1  time  n_ftme   d_ftme r_fms  f_ms  stp_m  0.0, 1.0, 25.0)
               (2  g     n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.7)
               {3 0 mode setting(0) mode fa_comb_mode 0 1}
               [0 sig],
            noise => Noise UIType::Generic UICategory::Osc
               (0  atv   n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.5)
               (1  offs  n_id      d_id  r_s    f_def stp_d -1.0, 1.0, 0.0)
               {2 0 mode setting(0) mode fa_noise_mode 0 1}
               [0 sig],
            formfm => FormFM UIType::Generic UICategory::Osc
               (0 freq n_pit d_pit r_fq f_freq stp_d -1.0, 0.5647131, 440.0)
               (1 det  n_det d_det r_det f_det stp_f -0.2, 0.2, 0.0)
               (2 form n_pit d_pit r_fq f_freq stp_d -1.0, 0.5647131, 440.0)
               (3 side  n_id d_id r_id f_def stp_d 0.0, 1.0, 0.2)
               (4 peak  n_id d_id r_id f_def stp_d 0.0, 1.0, 0.4)
               [0 sig],
            sfilter => SFilter UIType::Generic UICategory::Signal
               (0  inp   n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (1 freq  n_pit      d_pit r_fq  f_freq  stp_d -1.0, 0.5647131, 1000.0)
               (2  res   n_id      d_id  r_id   f_def stp_d 0.0, 1.0, 0.5)
               {3 0 ftype setting(8) mode fa_sfilter_type 0 13}
               [0 sig],
            fvafilt => FVaFilt UIType::Generic UICategory::Signal
               (0  inp   n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (1 freq  n_pit      d_pit r_fq  f_freq  stp_d -1.0, 0.55064, 1000.0)
               (2  res   n_id      d_id  r_id   f_def stp_d 0.0, 1.0, 0.5)
               (3 drive n_driv     d_driv r_driv f_db  stp_v  0.0, 1.0, 1.0)
               {4 0 ftype setting(0) mode fa_fvafilt_type 0 2}
               {5 1 smode setting(0) mode fa_fvafilt_svf_mode 0 4}
               {6 2 lslope setting(0) mode fa_fvafilt_lslope 0 3}
               [0 sig],
            biqfilt => BiqFilt UIType::Generic UICategory::Signal
               (0 inp    n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               (1 freq   n_pit     d_pit r_fq  f_freq stp_d -1.0, 0.5647131, 1000.0)
               (2 q      n_id      d_id  r_id   f_def stp_d 0.0, 1.0, 0.5)
               (3 gain   n_gain   d_gain r_gain f_db  stp_v 0.0, 1.0, 1.0)
               {4 0 ftype setting(0) mode fa_biqfilt_type 0 1}
               {5 1 order setting(0) mode fa_biqfilt_ord  0 3}
               [0 sig],
            pverb => PVerb UIType::Generic UICategory::Signal
               ( 0 in_l   n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               ( 1 in_r   n_id      d_id  r_id   f_def stp_d -1.0, 1.0, 0.0)
               ( 2 predly n_timz   d_timz r_tmz  f_ms  stp_m  0.0, 1.0, 0.0)
               ( 3 size   n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.5)
               ( 4 dcy    n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.25)
               ( 5 ilpf  n_pit      d_pit r_fq  f_freq stp_d -1.0, 0.5647131, 22050.0)
               ( 6 ihpf  n_pit      d_pit r_fq  f_freq stp_d -1.0, 0.5647131, 0.0)
               ( 7 dif    n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 1.0)
               ( 8 dmix   n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 1.0)
               ( 9 mspeed n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.0)
               (10 mshp   n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.5)
               (11 mdepth n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.2)
               (12 rlpf  n_pit      d_pit r_fq  f_freq stp_d -1.0, 0.5647131, 22050.0)
               (13 rhpf  n_pit      d_pit r_fq  f_freq stp_d -1.0, 0.5647131, 0.0)
               (14 mix   n_id      d_id  r_id   f_def stp_d  0.0, 1.0, 0.5)
               [0 sig_l]
               [1 sig_r],
           test => Test UIType::Generic UICategory::IOUtil
               (0 f     n_id      d_id   r_id   f_def stp_d 0.0, 1.0, 0.5)
               {1 0 p     param(0.0) knob fa_test_s 0  10}
               {2 1 trig  param(0.0) knob fa_test_s 0  0}
               [0 sig]
               [1 tsig]
               [2 out2]
               [3 out3]
               [4 out4]
               [5 outc],
        }
    };
}

impl UICategory {
    #[allow(unused_assignments)]
    pub fn get_node_ids<F: FnMut(NodeId)>(&self, mut skip: usize, mut fun: F) {
        macro_rules! make_cat_lister {
            ($s1: ident => $v1: ident,
                $($str: ident => $variant: ident
                    UIType:: $gui_type: ident
                    UICategory:: $ui_cat: ident
                    $(($in_idx: literal $para: ident
                       $n_fun: ident $d_fun: ident $r_fun: ident $f_fun: ident
                       $steps: ident $min: expr, $max: expr, $def: expr))*
                    $({$in_at_idx: literal $at_idx: literal $atom: ident
                       $at_fun: ident ($at_init: expr) $at_ui: ident $fa_fun: ident
                       $amin: literal $amax: literal})*
                    $([$out_idx: literal $out: ident])*
                    ,)+
            ) => {
                $(if UICategory::$ui_cat == *self {
                    if skip == 0 {
                        fun(NodeId::$variant(0));
                    } else {
                        skip -= 1
                    }
                })+
            }
        }

        node_list! {make_cat_lister};
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RandNodeSelector {
    Any,
    OnlyUseful,
}

fn rand_node_satisfies_spec(nid: NodeId, sel: RandNodeSelector) -> bool {
    if let NodeId::Nop = nid {
        return false;
    }

    match sel {
        RandNodeSelector::Any => true,
        RandNodeSelector::OnlyUseful => match nid {
            NodeId::Nop => false,
            NodeId::Out(_) => false,
            NodeId::FbRd(_) => false,
            NodeId::Test(_) => false,
            NodeId::Rust1x1(_) => false,
            _ => true,
        },
    }
}

pub fn get_rand_node_id(count: usize, sel: RandNodeSelector) -> Vec<NodeId> {
    let mut sm = synfx_dsp::SplitMix64::new_time_seed();
    let mut out = vec![];

    let mut cnt = 0;
    while cnt < 100 && out.len() < count {
        let cur = ALL_NODE_IDS[sm.next_u64() as usize % ALL_NODE_IDS.len()];

        if rand_node_satisfies_spec(cur, sel) {
            out.push(cur);
        }

        cnt += 1;
    }

    while out.len() < count {
        out.push(NodeId::Nop);
    }

    out
}

/// Holds information about the node type that was allocated.
/// It stores the names of inputs, output and atoms for uniform
/// access.
///
/// The [crate::NodeConfigurator] allocates and holds instances
/// of this type for access by [NodeId].
/// See also [crate::NodeConfigurator::node_by_id] and
/// [crate::Matrix::info_for].
#[derive(Clone)]
pub struct NodeInfo {
    node_id: NodeId,
    inputs: Vec<&'static str>,
    atoms: Vec<&'static str>,
    outputs: Vec<&'static str>,
    input_help: Vec<&'static str>,
    output_help: Vec<&'static str>,
    node_help: &'static str,
    node_desc: &'static str,
    node_name: &'static str,
    norm_v: std::rc::Rc<dyn Fn(usize, f32) -> f32>,
    denorm_v: std::rc::Rc<dyn Fn(usize, f32) -> f32>,
}

impl std::fmt::Debug for NodeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("NodeInfo").field("node_id", &self.node_id).finish()
    }
}

macro_rules! make_node_info_enum {
    ($s1: ident => $v1: ident,
        $($str: ident => $variant: ident
            UIType:: $gui_type: ident
            UICategory:: $ui_cat: ident
            $(($in_idx: literal $para: ident
               $n_fun: ident $d_fun: ident $r_fun: ident $f_fun: ident
               $steps: ident $min: expr, $max: expr, $def: expr))*
            $({$in_at_idx: literal $at_idx: literal $atom: ident
               $at_fun: ident ($at_init: expr) $at_ui: ident $fa_fun: ident
               $amin: literal $amax: literal})*
            $([$out_idx: literal $out: ident])*
            ,)+
    ) => {
        impl NodeInfo {
            /// Allocates a new [NodeInfo] from a [NodeId].
            /// Usually you access [NodeInfo] in the UI thread via
            /// [crate::NodeConfigurator::node_by_id]
            /// or [crate::Matrix::info_for].
            pub fn from_node_id(nid: NodeId) -> Self {
                match nid {
                    NodeId::$v1 => NodeInfo {
                        node_id:     crate::dsp::NodeId::Nop,
                        inputs:      vec![],
                        atoms:       vec![],
                        outputs:     vec![],
                        input_help:  vec![],
                        output_help: vec![],
                        node_help: "Nop Help",
                        node_desc: "Nop Desc",
                        node_name: "Nop",

                        norm_v:   std::rc::Rc::new(|_i, x| x),
                        denorm_v: std::rc::Rc::new(|_i, x| x),
                    },
                    $(NodeId::$variant(_) => crate::dsp::ni::$variant(nid)),+
                }
            }
        }

        /// Refers to an input paramter or atom of a specific
        /// [Node] referred to by a [NodeId].
        ///
        /// To obtain a [ParamId] you use one of these:
        /// * [NodeId::atom_param_by_idx]
        /// * [NodeId::inp_param_by_idx]
        /// * [NodeId::param_by_idx]
        /// * [NodeId::inp_param]
        ///
        /// To obtain an input and output index for a port use:
        /// * [NodeId::inp]
        /// * [NodeId::out]
        ///
        ///```
        /// use hexodsp::*;
        /// let freq_param = NodeId::Sin(2).inp_param("freq").unwrap();
        ///
        /// assert!(!freq_param.is_atom());
        ///
        /// // Access the UI min/max and fine/coarse step values of this paramter:
        /// assert_eq!(freq_param.param_min_max().unwrap(), ((-1.0, 0.5647131), (20.0, 100.0)));
        ///
        /// // Access the default value:
        /// assert_eq!(freq_param.as_atom_def().f(), 0.0);
        ///
        /// // Normalize a value (convert frequency to the 0.0 to 1.0 range)
        /// assert_eq!(freq_param.norm(220.0), -0.1);
        ///
        /// // Denormalize a value (convert 0.0 to 1.0 range to frequency)
        /// assert_eq!(freq_param.denorm(-0.1), 220.0);
        ///```
        #[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
        pub struct ParamId {
            name: &'static str,
            node: NodeId,
            idx:  u8,
        }

        impl ParamId {
            pub fn none() -> Self {
                Self {
                    name: "NOP",
                    node: NodeId::Nop,
                    idx: 0,
                }
            }

            pub fn node_id(&self) -> NodeId       { self.node }
            pub fn inp(&self)     -> u8           { self.idx }
            pub fn name(&self)    -> &'static str { self.name }

            /// Returns true if the [ParamId] has been associated with
            /// the atoms of a Node, and not the paramters. Even if the
            /// Atom is a `param()`.
            pub fn is_atom(&self) -> bool {
                match self.node {
                    NodeId::$v1           => false,
                    $(NodeId::$variant(_) => {
                        match self.idx {
                            $($in_idx    => false,)*
                            $($in_at_idx => true,)*
                            _            => false,
                        }
                    }),+
                }
            }

            pub fn atom_ui(&self) -> Option<&'static str> {
                match self.node {
                    NodeId::$v1           => None,
                    $(NodeId::$variant(_) => {
                        match self.idx {
                            $($in_at_idx => Some(stringify!($at_ui)),)*
                            _            => None,
                        }
                    }),+
                }
            }

            pub fn param_steps(&self) -> Option<(f32, f32)> {
                match self.node {
                    NodeId::$v1           => None,
                    $(NodeId::$variant(_) => {
                        match self.idx {
                            $($in_idx => Some(($min, $max)),)*
                            _         => None,
                        }
                    }),+
                }
            }

            pub fn param_min_max(&self) -> Option<((f32, f32), (f32, f32))> {
                match self.node {
                    NodeId::$v1           => None,
                    $(NodeId::$variant(_) => {
                        match self.idx {
                            $($in_idx => Some((($min, $max), $steps!())),)*
                            _         => None,
                        }
                    }),+
                }
            }

            pub fn format(&self, f: &mut dyn std::io::Write, v: f32) -> Option<std::io::Result<()>> {
                match self.node {
                    NodeId::$v1           => None,
                    $(NodeId::$variant(_) => {
                        match self.idx {
                            $($in_idx    => Some($f_fun!(f, v, $d_fun!(v))),)*
                            $($in_at_idx => Some($fa_fun!(f, v, v)),)*
                            _            => None,
                        }
                    }),+
                }
            }

            pub fn setting_min_max(&self) -> Option<(i64, i64)> {
                match self.node {
                    NodeId::$v1           => None,
                    $(NodeId::$variant(_) => {
                        match self.idx {
                            $($in_at_idx => Some(($amin, $amax)),)*
                            _            => None,
                        }
                    }),+
                }
            }

            pub fn as_atom_def(&self) -> SAtom {
                match self.node {
                    NodeId::$v1           => SAtom::param(0.0),
                    $(NodeId::$variant(_) => {
                        match self.idx {
                            $($in_idx    => SAtom::param(crate::dsp::norm_def::$variant::$para()),)*
                            $($in_at_idx => SAtom::$at_fun($at_init),)*
                            _            => SAtom::param(0.0),
                        }
                    }),+
                }
            }

            pub fn norm_def(&self) -> f32 {
                match self.node {
                    NodeId::$v1           => 0.0,
                    $(NodeId::$variant(_) => {
                        match self.idx {
                            $($in_idx => crate::dsp::norm_def::$variant::$para(),)*
                            _ => 0.0,
                        }
                    }),+
                }
            }

            pub fn round(&self, v: f32, coarse: bool) -> f32 {
                match self.node {
                    NodeId::$v1           => 0.0,
                    $(NodeId::$variant(_) => {
                        match self.idx {
                            $($in_idx => crate::dsp::round::$variant::$para(v, coarse),)*
                            _ => 0.0,
                        }
                    }),+
                }
            }

            pub fn norm(&self, v: f32) -> f32 {
                match self.node {
                    NodeId::$v1           => 0.0,
                    $(NodeId::$variant(_) => {
                        match self.idx {
                            $($in_idx => crate::dsp::norm_v::$variant::$para(v),)*
                            _ => 0.0,
                        }
                    }),+
                }
            }

            pub fn denorm(&self, v: f32) -> f32 {
                match self.node {
                    NodeId::$v1           => 0.0,
                    $(NodeId::$variant(_) => {
                        match self.idx {
                            $($in_idx => crate::dsp::denorm_v::$variant::$para(v),)*
                            _ => 0.0,
                        }
                    }),+
                }
            }
        }

        /// This enum is a collection of all implemented modules (aka nodes)
        /// that are implemented. The associated `u8` index is the so called
        /// _instance_ of the corresponding [Node] type.
        ///
        /// This is the primary way in this library to refer to a specific node
        /// in the node graph that is managed by [crate::NodeConfigurator]
        /// and executed by [crate::NodeExecutor].
        ///
        /// To see how to actually use this, refer to the documentation
        /// of [crate::Cell], where you will find an example.
        #[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
        pub enum NodeId {
            $v1,
            $($variant(u8)),+
        }

        impl std::fmt::Display for NodeId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    NodeId::$v1           => write!(f, "{}", stringify!($v1)),
                    $(NodeId::$variant(i) => write!(f, "{} {}", stringify!($variant), i)),+
                }
            }
        }

        impl NodeId {
            pub fn to_instance(&self, instance: usize) -> NodeId {
                match self {
                    NodeId::$v1           => NodeId::$v1,
                    $(NodeId::$variant(_) => NodeId::$variant(instance as u8)),+
                }
            }

            pub fn graph_fun(&self) -> Option<GraphFun> {
                match self {
                    NodeId::$v1           => None,
                    $(NodeId::$variant(_) => crate::dsp::$variant::graph_fun()),+
                }
            }

            pub fn eq_variant(&self, other: &NodeId) -> bool {
                match self {
                    NodeId::$v1           => *other == NodeId::$v1,
                    $(NodeId::$variant(_) =>
                        if let NodeId::$variant(_) = other { true }
                        else { false }),+
                }
            }

            pub fn from_node_info(ni: &NodeInfo) -> NodeId { ni.to_id() }

            pub fn label(&self) -> &'static str {
                match self {
                    NodeId::$v1           => stringify!($v1),
                    $(NodeId::$variant(_) => stringify!($variant)),+
                }
            }

            pub fn name(&self) -> &'static str {
                match self {
                    NodeId::$v1           => stringify!($s1),
                    $(NodeId::$variant(_) => stringify!($str)),+
                }
            }

            pub fn from_str(name: &str) -> Self {
                match name {
                    stringify!($s1)    => NodeId::$v1,
                    $(stringify!($str) => NodeId::$variant(0)),+,
                    _                  => NodeId::Nop,
                }
            }

            pub fn ui_type(&self) -> UIType {
                match self {
                    NodeId::$v1           => UIType::Generic,
                    $(NodeId::$variant(_) => UIType::$gui_type),+
                }
            }

            pub fn ui_category(&self) -> UICategory {
                match self {
                    NodeId::$v1           => UICategory::None,
                    $(NodeId::$variant(_) => UICategory::$ui_cat),+
                }
            }

            /// Consistently initialize the phase for oscillators.
            /// This does some fixed phase offset for the first 3
            /// instances, which is usually relied on by the automated
            /// tests.
            #[inline]
            pub fn init_phase(&self) -> f32 {
                // The first 3 instances get a fixed predefined phase to
                // not mess up the automated tests so easily.
                match self.instance() {
                    0 => 0.0,
                    1 => 0.05,
                    2 => 0.1,
                    // 0.25 just to protect against sine cancellation
                    _ => synfx_dsp::rand_01() * 0.25
                }
            }

            /// This maps the atom index of the node to the absolute
            /// ParamId in the GUI (and in the [crate::matrix::Matrix]).
            /// The Atom/Param duality is a bit weird because they share
            /// the same ID namespace for the UI. But in the actual
            /// backend, they are split. So the actual splitting happens
            /// in the [crate::matrix::Matrix].
            pub fn atom_param_by_idx(&self, idx: usize) -> Option<ParamId> {
                match self {
                    NodeId::$v1           => None,
                    $(NodeId::$variant(_) => {
                        match idx {
                            $($at_idx => Some(ParamId {
                                node: *self,
                                name: stringify!($atom),
                                idx:  $in_at_idx,
                            }),)*
                            _ => None,
                        }
                    }),+
                }
            }

            pub fn inp_param_by_idx(&self, idx: usize) -> Option<ParamId> {
                match self {
                    NodeId::$v1           => None,
                    $(NodeId::$variant(_) => {
                        match idx {
                            $($in_idx => Some(ParamId {
                                node: *self,
                                name: stringify!($para),
                                idx:  $in_idx,
                            }),)*
                            _ => None,
                        }
                    }),+
                }
            }

            pub fn param_by_idx(&self, idx: usize) -> Option<ParamId> {
                match self {
                    NodeId::$v1           => None,
                    $(NodeId::$variant(_) => {
                        match idx {
                            $($in_idx => Some(ParamId {
                                node: *self,
                                name: stringify!($para),
                                idx:  $in_idx,
                            }),)*
                            $($in_at_idx => Some(ParamId {
                                node: *self,
                                name: stringify!($atom),
                                idx:  $in_at_idx,
                            }),)*
                            _ => None,
                        }
                    }),+
                }
            }

            pub fn inp_param(&self, name: &str) -> Option<ParamId> {
                match self {
                    NodeId::$v1           => None,
                    $(NodeId::$variant(_) => {
                        match name {
                            $(stringify!($para) => Some(ParamId {
                                node: *self,
                                name: stringify!($para),
                                idx:  $in_idx,
                            }),)*
                            $(stringify!($atom) => Some(ParamId {
                                node: *self,
                                name: stringify!($atom),
                                idx:  $in_at_idx,
                            }),)*
                            _ => None,
                        }
                    }),+
                }
            }

            pub fn inp(&self, name: &str) -> Option<u8> {
                match self {
                    NodeId::$v1           => None,
                    $(NodeId::$variant(_) => {
                        match name {
                            $(stringify!($para) => Some($in_idx),)*
                            _ => None,
                        }
                    }),+
                }
            }

            pub fn inp_name_by_idx(&self, idx: u8) -> Option<&'static str> {
                match self {
                    NodeId::$v1           => None,
                    $(NodeId::$variant(_) => {
                        match idx {
                            $($in_idx    => Some(stringify!($para)),)*
                            $($in_at_idx => Some(stringify!($atom)),)*
                            _ => None,
                        }
                    }),+
                }
            }

            pub fn out_name_by_idx(&self, idx: u8) -> Option<&'static str> {
                match self {
                    NodeId::$v1           => None,
                    $(NodeId::$variant(_) => {
                        match idx {
                            $($out_idx => Some(stringify!($out)),)*
                            _ => None,
                        }
                    }),+
                }
            }

            pub fn out(&self, name: &str) -> Option<u8> {
                match self {
                    NodeId::$v1           => None,
                    $(NodeId::$variant(_) => {
                        match name {
                            $(stringify!($out) => Some($out_idx),)*
                            _ => None,
                        }
                    }),+
                }
            }

            pub fn instance(&self) -> usize {
                match self {
                    NodeId::$v1           => 0,
                    $(NodeId::$variant(i) => *i as usize),+
                }
            }
        }

        pub const ALL_NODE_IDS : &'static [NodeId] = &[$(NodeId::$variant(0)),+];

        #[allow(non_snake_case, unused_variables)]
        pub mod round {
            $(pub mod $variant {
                $(#[inline] pub fn $para(x: f32, coarse: bool) -> f32 {
                    $r_fun!(x, coarse).clamp($min, $max)
                })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod denorm_v {
            $(pub mod $variant {
                $(#[inline] pub fn $para(x: f32) -> f32 { $d_fun!(x) })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod norm_def {
            $(pub mod $variant {
                $(#[inline] pub fn $para() -> f32 { $n_fun!($def) })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod norm_v {
            $(pub mod $variant {
                $(#[inline] pub fn $para(v: f32) -> f32 { $n_fun!(v) })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod denorm {
            $(pub mod $variant {
                $(#[inline] pub fn $para(buf: &crate::dsp::ProcBuf, frame: usize) -> f32 {
                    $d_fun!(buf.read(frame))
                })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod denorm_offs {
            $(pub mod $variant {
                $(#[inline] pub fn $para(buf: &crate::dsp::ProcBuf, offs_val: f32, frame: usize) -> f32 {
                    $d_fun!(buf.read(frame) + offs_val)
                })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod inp_dir {
            $(pub mod $variant {
                $(#[inline] pub fn $para(buf: &crate::dsp::ProcBuf, frame: usize) -> f32 {
                    buf.read(frame)
                })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod inp {
            $(pub mod $variant {
                $(#[inline] pub fn $para(inputs: &[crate::dsp::ProcBuf]) -> &crate::dsp::ProcBuf {
                    &inputs[$in_idx]
                })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod inp_buf {
            $(pub mod $variant {
                $(#[inline] pub fn $para(inputs: &[crate::dsp::ProcBuf]) -> crate::dsp::ProcBuf {
                    inputs[$in_idx]
                })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod at {
            $(pub mod $variant {
                $(#[inline] pub fn $atom(atoms: &[crate::dsp::SAtom]) -> &crate::dsp::SAtom {
                    &atoms[$at_idx]
                })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod out_dir {
            $(pub mod $variant {
                $(#[inline] pub fn $out(outputs: &mut [crate::dsp::ProcBuf], frame: usize, v: f32) {
                    outputs[$out_idx].write(frame, v);
                })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod out {
            $(pub mod $variant {
                $(#[inline] pub fn $out(outputs: &mut [crate::dsp::ProcBuf]) -> &mut crate::dsp::ProcBuf {
                    &mut outputs[$out_idx]
                })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod out_buf {
            $(pub mod $variant {
                $(#[inline] pub fn $out(outputs: &mut [crate::dsp::ProcBuf]) -> crate::dsp::ProcBuf {
                    outputs[$out_idx]
                })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod out_idx {
            $(pub mod $variant {
                $(#[inline] pub fn $out() -> usize { $out_idx })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod is_out_con {
            $(pub mod $variant {
                $(#[inline] pub fn $out(nctx: &crate::dsp::NodeContext) -> bool {
                    nctx.out_connected & (1 << $out_idx) != 0x0
                })*
            })+
        }

        #[allow(non_snake_case)]
        pub mod is_in_con {
            $(pub mod $variant {
                $(#[inline] pub fn $para(nctx: &crate::dsp::NodeContext) -> bool {
                    nctx.in_connected & (1 << $in_idx) != 0x0
                })*
            })+
        }

        #[allow(unused_mut)]
        mod ni {
            $(
                pub fn $variant(node_id: crate::dsp::NodeId) -> crate::dsp::NodeInfo {
                    let mut input_help = vec![$(crate::dsp::$variant::$para,)*];
                    $(input_help.push(crate::dsp::$variant::$atom);)*

                    crate::dsp::NodeInfo {
                        node_id,
                        inputs:  vec![$(stringify!($para),)*],
                        atoms:   vec![$(stringify!($atom),)*],
                        outputs: vec![$(stringify!($out),)*],

                        input_help,
                        output_help: vec![$(crate::dsp::$variant::$out,)*],
                        node_help:   crate::dsp::$variant::HELP,
                        node_desc:   crate::dsp::$variant::DESC,
                        node_name:   stringify!($variant),

                        norm_v:
                            std::rc::Rc::new(|i, x|
                                match i {
                                    $($in_idx => crate::dsp::norm_v::$variant::$para(x),)+
                                    _ => x,
                                }),
                        denorm_v:
                            std::rc::Rc::new(|i, x|
                                match i {
                                    $($in_idx => crate::dsp::denorm_v::$variant::$para(x),)+
                                    _ => x,
                                }),
                    }
                }
            )+

        }

        impl NodeInfo {
            pub fn from(s: &str) -> Self {
                match s {
                    $(stringify!($str) => crate::dsp::ni::$variant(NodeId::$variant(0)),)+
                    _                  => NodeInfo::from_node_id(NodeId::Nop),
                }
            }

            pub fn name(&self) -> &'static str { self.node_name }

            pub fn in_name(&self, in_idx: usize) -> Option<&'static str> {
                if let Some(s) = self.inputs.get(in_idx) {
                    Some(*s)
                } else {
                    Some(*(self.atoms.get(in_idx)?))
                }
            }

            pub fn at_name(&self, in_idx: usize) -> Option<&'static str> {
                Some(*(self.atoms.get(in_idx)?))
            }

            pub fn out_name(&self, out_idx: usize) -> Option<&'static str> {
                Some(*(self.outputs.get(out_idx)?))
            }

            pub fn in_help(&self, in_idx: usize) -> Option<&'static str> {
                Some(*self.input_help.get(in_idx)?)
            }

            pub fn out_help(&self, out_idx: usize) -> Option<&'static str> {
                Some(*(self.output_help.get(out_idx)?))
            }

            pub fn norm(&self, in_idx: usize, x: f32) -> f32 {
                (*self.norm_v)(in_idx, x)
            }

            pub fn denorm(&self, in_idx: usize, x: f32) -> f32 {
                (*self.denorm_v)(in_idx, x)
            }

            pub fn desc(&self) -> &'static str { self.node_desc }
            pub fn help(&self) -> &'static str { self.node_help }

            pub fn out_count(&self) -> usize { self.outputs.len() }
            pub fn in_count(&self)  -> usize { self.inputs.len() }
            pub fn at_count(&self)  -> usize { self.atoms.len() }

            pub fn to_id(&self) -> NodeId { self.node_id }

            pub fn default_output(&self) -> Option<u8> {
                if self.out_count() > 0 {
                    Some(0)
                } else {
                    None
                }
            }

            pub fn default_input(&self) -> Option<u8> {
                if self.in_count() > 0 {
                    Some(0)
                } else {
                    None
                }
            }
        }
    }
}

//macro_rules! make_node_enum {
//    ($s1: ident => $v1: ident,
//        $($str: ident => $variant: ident
//            UIType:: $gui_type: ident
//            UICategory:: $ui_cat: ident
//            $(($in_idx: literal $para: ident
//               $n_fun: ident $d_fun: ident $r_fun: ident $f_fun: ident
//               $steps: ident $min: expr, $max: expr, $def: expr))*
//            $({$in_at_idx: literal $at_idx: literal $atom: ident
//               $at_fun: ident ($at_init: expr) $at_ui: ident $fa_fun: ident
//               $amin: literal $amax: literal})*
//            $([$out_idx: literal $out: ident])*
//            ,)+
//    ) => {
//        /// Represents the actually by the DSP thread ([crate::NodeExecutor])
//        /// executed [Node]. You don't construct this directly, but let the
//        /// [crate::NodeConfigurator] or more abstract types like
//        /// [crate::Matrix] do this for you. See also [NodeId] for a way to
//        /// refer to these.
//        ///
//        /// The method [Node::process] is called by [crate::NodeExecutor]
//        /// and comes with the overhead of a big `match` statement.
//        ///
//        /// This is the only point of primitive polymorphism inside
//        /// the DSP graph. Dynamic polymorphism via the trait object
//        /// is not done, as I hope the `match` dispatch is a slight bit faster
//        /// because it's more static.
//        ///
//        /// The size of a [Node] is also limited and protected by a test
//        /// in the test suite. The size should not be needlessly increased
//        /// by implementations, in the hope to achieve better
//        /// cache locality. All allocated [Node]s are held in a big
//        /// continuous vector inside the [crate::NodeExecutor].
//        ///
//        /// The function [node_factory] is responsible for actually creating
//        /// the [Node].
//        #[derive(Debug, Clone)]
//        pub enum Node {
//            /// An empty node that does nothing. It's a placeholder
//            /// for non allocated nodes.
//            $v1,
//            $($variant { node: $variant },)+
//        }
//
//        impl Node {
//            /// Returns the [NodeId] that can be used to refer to this node.
//            /// The node does not store it's instance index, so you have to
//            /// provide it. If the instance is of no meaning for the
//            /// use case pass 0 to `instance`.
//            pub fn to_id(&self, instance: usize) -> NodeId {
//                match self {
//                    Node::$v1               => NodeId::$v1,
//                    $(Node::$variant { .. } => NodeId::$variant(instance as u8)),+
//                }
//            }
//
//            /// Resets any state of this [Node], such as
//            /// any internal state variables or counters or whatever.
//            /// The [Node] should just behave as if it was freshly returned
//            /// from [node_factory].
//            pub fn reset(&mut self) {
//                match self {
//                    Node::$v1           => {},
//                    $(Node::$variant { node } => {
//                        node.reset();
//                    }),+
//                }
//            }
//
//            /// Sets the current sample rate this [Node] should operate at.
//            pub fn set_sample_rate(&mut self, sample_rate: f32) {
//                match self {
//                    Node::$v1           => {},
//                    $(Node::$variant { node } => {
//                        node.set_sample_rate(sample_rate);
//                    }),+
//                }
//            }
//
//        }
//    }
//}
//
node_list! {make_node_info_enum}
//node_list! {make_node_enum}

/// A warpper type for [DspNode] trait objects.
///
/// Make sure to call methods on this **only from one thread** at a time.
/// The common lifetime cycle of this is:
/// 1. Instanciation in [crate::NodeConfigurator::create_node] on eg. the frontend thread
/// 2. Sent wrapped in a [crate::nodes::NodeProg] to the audio/DSP thread.
/// 3. DSP thread calls methods on this.
/// 4. Upon update of the [crate::nodes::NodeProg] on the DSP thread it is disposed
/// by being sent to the drop thread.
#[derive(Clone)]
pub struct Node(pub Arc<SyncUnsafeCell<dyn DspNode>>, NodeId);

impl Node {
    pub fn id(&self) -> NodeId {
        self.1
    }

    #[inline]
    pub fn set_sample_rate(&self, srate: f32) {
        unsafe {
            (*self.0.get()).set_sample_rate(srate);
        }
    }

    #[inline]
    pub fn reset(&self) {
        unsafe {
            (*self.0.get()).reset();
        }
    }

    #[inline]
    pub fn process(
        &self,
        ctx: &mut dyn NodeAudioContext,
        ectx: &mut NodeExecContext,
        nctx: &NodeContext,
        atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        led: LedPhaseVals,
    ) {
        unsafe { (*self.0.get()).process(ctx, ectx, nctx, atoms, inputs, outputs, led) };
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node[{:?}]", unsafe { self.0.get() })
    }
}

#[derive(Debug)]
pub struct NopNode();

impl NopNode {
    pub fn new_node() -> Node {
        Node(Arc::new(SyncUnsafeCell::new(Self {})), NodeId::Nop)
    }
}

impl DspNode for NopNode {
    fn set_sample_rate(&mut self, _srate: f32) {}
    fn reset(&mut self) {}
    fn process(
        &mut self,
        _ctx: &mut dyn NodeAudioContext,
        _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        _atoms: &[SAtom],
        _inputs: &[ProcBuf],
        _outputs: &mut [ProcBuf],
        _led: LedPhaseVals,
    ) {
    }
}

pub fn node_factory(node_id: NodeId, node_global: &NodeGlobalRef) -> Option<(Node, NodeInfo)> {
    macro_rules! make_node_factory_match {
        ($s1: expr => $v1: ident,
            $($str: ident => $variant: ident
                UIType:: $gui_type: ident
                UICategory:: $ui_cat: ident
                $(($in_idx: literal $para: ident
                   $n_fun: ident $d_fun: ident $r_fun: ident $f_fun: ident
                   $steps: ident $min: expr, $max: expr, $def: expr))*
                $({$in_at_idx: literal $at_idx: literal $atom: ident
                   $at_fun: ident ($at_init: expr) $at_ui: ident $fa_fun: ident
                   $amin: literal $amax: literal})*
                $([$out_idx: literal $out: ident])*
            ,)+
        ) => {
            match node_id {
                $(NodeId::$variant(_) => Some((
                    Node(Arc::new(SyncUnsafeCell::new($variant::new(&node_id, node_global))), node_id),
                    NodeInfo::from_node_id(node_id),
                )),)+
                _ => None,
            }
        }
    }

    node_list! {make_node_factory_match}
}

//impl Node {
//    /// This function is the heart of any DSP.
//    /// It dispatches this call to the corresponding [Node] implementation.
//    ///
//    /// You don't want to call this directly, but let [crate::NodeConfigurator] and
//    /// [crate::NodeExecutor] do their magic for you.
//    ///
//    /// The slices get passed a [ProcBuf] which is a super _unsafe_
//    /// buffer, that requires special care and invariants to work safely.
//    ///
//    /// Arguments:
//    /// * `ctx`: The [NodeAudioContext] usually provides global context information
//    /// such as access to the actual buffers of the audio driver or access to
//    /// MIDI events.
//    /// * `atoms`: The [SAtom] settings the user can set in the UI or via
//    /// other means. These are usually non interpolated/smoothed settings.
//    /// * `params`: The smoothed input parameters as set by the user (eg. in the UI).
//    /// There is usually no reason to use these, because any parameter can be
//    /// overridden by assigning an output port to the corresponding input.
//    /// This is provided for the rare case that you still want to use the
//    /// value the user set in the interface, and not the input Ctrl signal.
//    /// * `inputs`: For each `params` parameter there is a input port.
//    /// This slice will contain either a buffer from `params` or some output
//    /// buffer from some other (previously executed) [Node]s output.
//    /// * `outputs`: The output buffers this node will write it's signal/Ctrl
//    /// results to.
//    /// * `led`: Contains the feedback [LedPhaseVals], which are used
//    /// to communicate the current value (set once per `process()` call, usually at the end)
//    /// of the most important internal signal. Usually stuff like the output
//    /// value of an oscillator, envelope or the current sequencer output
//    /// value. It also provides a second value, a so called _phase_
//    /// which is usually used by graphical frontends to determine
//    /// the phase of the oscillator, envelope or the sequencer to
//    /// display some kind of position indicator.
//    #[inline]
//    pub fn process<T: NodeAudioContext>(
//        &mut self,
//        ctx: &mut T,
//        ectx: &mut NodeExecContext,
//        nctx: &NodeContext,
//        atoms: &[SAtom],
//        inputs: &[ProcBuf],
//        outputs: &mut [ProcBuf],
//        led: LedPhaseVals,
//    ) {
//        macro_rules! make_node_process {
//            ($s1: ident => $v1: ident,
//                $($str: ident => $variant: ident
//                    UIType:: $gui_type: ident
//                    UICategory:: $ui_cat: ident
//                    $(($in_idx: literal $para: ident
//                       $n_fun: ident $d_fun: ident $r_fun: ident $f_fun: ident
//                       $steps: ident $min: expr, $max: expr, $def: expr))*
//                    $({$in_at_idx: literal $at_idx: literal $atom: ident
//                       $at_fun: ident ($at_init: expr) $at_ui: ident $fa_fun: ident
//                       $amin: literal $amax: literal})*
//                    $([$out_idx: literal $out: ident])*
//                ,)+
//            ) => {
//                match self {
//                    Node::$v1 => {},
//                    $(Node::$variant { node } =>
//                        node.process(ctx, ectx, nctx, atoms,
//                                     inputs, outputs, led),)+
//                }
//            }
//        }
//
//        node_list! {make_node_process}
//    }
//}
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_id_sizes_staying_small() {
        assert_eq!(std::mem::size_of::<NodeId>(), 2);
        assert_eq!(std::mem::size_of::<ParamId>(), 24);
    }

    #[test]
    fn check_pitch() {
        assert_eq!(d_pit!(-0.2).round() as i32, 110_i32);
        assert_eq!((n_pit!(110.0) * 100.0).round() as i32, -20_i32);
        assert_eq!(d_pit!(0.0).round() as i32, 440_i32);
        assert_eq!((n_pit!(440.0) * 100.0).round() as i32, 0_i32);
        assert_eq!(d_pit!(0.3).round() as i32, 3520_i32);
        assert_eq!((n_pit!(3520.0) * 100.0).round() as i32, 30_i32);

        for i in 1..999 {
            let x = (((i as f32) / 1000.0) - 0.5) * 2.0;
            let r = d_pit!(x);
            //d// println!("x={:8.5} => {:8.5}", x, r);
            assert_eq!((n_pit!(r) * 10000.0).round() as i32, (x * 10000.0).round() as i32);
        }
    }
}
