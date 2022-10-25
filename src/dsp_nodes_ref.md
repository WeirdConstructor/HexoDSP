| Node | Category | Description |
|-|-|-|
| [**Amp**](#nodeidamp) | Signal | **Signal Amplifier**:   This is a simple amplifier to amplify or attenuate a signal.  |
| [**Mix3**](#nodeidmix3) | NtoM | **3 Ch. Signal Mixer**:   A very simple 3 channel signal mixer. You can mix anything, from audio signals to control signals.  |
| [**Mux9**](#nodeidmux9) | NtoM | **9 Ch. Multiplexer**:   An up to 9 channel multiplexer aka switch or junction. You can route one of the 9 (or fewer) inputs to the output. The opposite of this node is the `Demux9`, which demultiplexes or routes the one input signal to one of the 9 outputs.  |
| [**SMap**](#nodeidsmap) | Ctrl | **Simple Range Mapper**:   This node allows to map an unipolar (**0**..**1**) or bipolar signal (**-1**..**1**) to a defined `min`/`max` signal range.  See also the 'Map' node for a more sophisticated version of this.  |
| [**Map**](#nodeidmap) | Ctrl | **Range Mapper**:   This node allows to map an input signal range to a precise output signal range. It's mostly useful to map control signals to modulate inputs.  See also the `SMap` node, which is a simplified version of this node.  |
| [**Quant**](#nodeidquant) | Ctrl | **Pitch Quantizer**:   This is a simple quantizer, that snaps a pitch signal on `freq` to the closest selected notes within their octave.  |
| [**CQnt**](#nodeidcqnt) | Ctrl | **Ctrl Pitch Quantizer**:   This special quantizer maps the unipolar **0**..**1** control signal input range on `inp` evenly to the selected keys and octaves.  |
| [**TSeq**](#nodeidtseq) | Mod | **Tracker Sequencer**:   This node implements a sequencer that can be programmed using the tracker interface in HexoSynth on the right. It provides 6 control signals and 6 gate outputs. |
| [**Code**](#nodeidcode) | Signal | **WBlockDSP Code Execution**:   This node executes just in time compiled code as fast as machine code. Use this to implement real time DSP code yourself. The inputs are freely useable in your code. All the ports (input and output) can be used either for audio or for control signals. |
| [**Rust1x1**](#nodeidrust1x1) | Signal | **Rust Code Node**:   This node does provide the user of HexoDSP or the SynthConstructor with an API to code custom DSP node implementations in pure Rust at compile time. It does not have any relevance for HexoSynth. See also [crate::SynthConstructor] and [crate::DynamicNode1x1].  |
| [**Sampl**](#nodeidsampl) | Osc | **Sample Player**:  Provides a simple sample player that you can load a single audio sample from a WAV file into. |
| [**Sin**](#nodeidsin) | Osc | **Sine Oscillator**:   This is a very simple oscillator that generates a sine wave.  |
| [**BOsc**](#nodeidbosc) | Osc | **Basic Oscillator**:   A very basic band limited oscillator with a sine, triangle, pulse and sawtooth waveform.  |
| [**VOsc**](#nodeidvosc) | Osc | **V Oscillator**:   A vector phase shaping oscillator, to create interesting waveforms and ways to manipulate them. It has two parameters (`v` and `d`) to shape the phase of the sinusoid wave, and a `vs` parameter to add extra spice. Distortion can beef up the oscillator output and you can apply oversampling.  |
| [**BowStri**](#nodeidbowstri) | Osc | **Bowed String Oscillator**:   This is an oscillator that simulates a bowed string.  |
| [**MidiP**](#nodeidmidip) | IOUtil | **MIDI Pitch/Note Input**:   This node is an input of MIDI note events into the DSP graph. You get 3 outputs: frequency of the note, gate signal for the length of the note and the velocity. |
| [**MidiCC**](#nodeidmidicc) | IOUtil | **MIDI CC Input**:   This node is an input of MIDI CC events/values into the DSP graph. You get 3 CC value outputs: `sig1`, `sig2` and `sig3`. To set which CC gets which output you have to set the corresponding `cc1`, `cc2` and `cc3` parameters. |
| [**ExtA**](#nodeidexta) | IOUtil | **Ext. Param. Set A-F Input**:   This node gives access to the 24 input parameters of the HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick changes a bit if you need it. Attenuverters (attenuators that can also invert) allow to reduce the amplitude or invert the signal. |
| [**ExtB**](#nodeidextb) | IOUtil | **Ext. Param. Set A-F Input**:   This node gives access to the 24 input parameters of the HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick changes a bit if you need it. Attenuverters (attenuators that can also invert) allow to reduce the amplitude or invert the signal. |
| [**ExtC**](#nodeidextc) | IOUtil | **Ext. Param. Set A-F Input**:   This node gives access to the 24 input parameters of the HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick changes a bit if you need it. Attenuverters (attenuators that can also invert) allow to reduce the amplitude or invert the signal. |
| [**ExtD**](#nodeidextd) | IOUtil | **Ext. Param. Set A-F Input**:   This node gives access to the 24 input parameters of the HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick changes a bit if you need it. Attenuverters (attenuators that can also invert) allow to reduce the amplitude or invert the signal. |
| [**ExtE**](#nodeidexte) | IOUtil | **Ext. Param. Set A-F Input**:   This node gives access to the 24 input parameters of the HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick changes a bit if you need it. Attenuverters (attenuators that can also invert) allow to reduce the amplitude or invert the signal. |
| [**ExtF**](#nodeidextf) | IOUtil | **Ext. Param. Set A-F Input**:   This node gives access to the 24 input parameters of the HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick changes a bit if you need it. Attenuverters (attenuators that can also invert) allow to reduce the amplitude or invert the signal. |
| [**Inp**](#nodeidinp) | IOUtil | **Audio Input Port**:   This node gives you access to the two input ports of the HexoSynth plugin. Build effects or what ever you can imagine with this!          |
| [**Out**](#nodeidout) | IOUtil | **Audio Output Port**:   This output port node allows you to send audio signals to audio devices or tracks in your DAW. |
| [**FbWr**](#nodeidfbwr) | IOUtil | **Feedback Delay Writer**:   HexoSynth does not allow direct feedback cycles in it's graph. To make feedback possible anyways the `FbWr` and `FbRd` nodes are provided. This node allows you to write a signal into the corresponsing signal delay buffer. Use `FbRd` for using the signal. The delay is **3.14ms**. |
| [**FbRd**](#nodeidfbrd) | IOUtil | **Feedback Delay Reader**:   HexoSynth does not allow direct feedback cycles in it's graph. To make feedback possible anyways the `FbWr` and `FbRd` nodes are provided. This node allows you to tap into the corresponding `FbWr` signal delay for feedback. The delay is **3.14ms**. |
| [**Scope**](#nodeidscope) | IOUtil | **Signal Oscilloscope Probe**:   This is a signal oscilloscope probe node, you can capture up to 3 signals. You can enable internal or external triggering for capturing signals or pinning fast waveforms.  |
| [**Ad**](#nodeidad) | Mod | **Attack-Decay Envelope**:   This is a simple envelope offering an attack time and decay time with a shape parameter. You can use it as envelope generator to modulate other inputs or process a signal with it directly.  |
| [**Adsr**](#nodeidadsr) | Mod | **Attack-Decay Envelope**:   This is an ADSR envelope, offering an attack time, decay time, a sustain phase and a release time. Attack, decay and release each have their own shape parameter. You can use it as envelope generator to modulate other inputs or process a signal with it directly.  |
| [**TsLFO**](#nodeidtslfo) | Mod | **TriSaw LFO**:   This simple LFO has a configurable waveform. You can blend between triangular to sawtooth waveforms using the `rev` parameter.  |
| [**RndWk**](#nodeidrndwk) | Mod | **Random Walker**:   This modulator generates a random number by walking a pre defined maximum random `step` width. For smoother transitions a slew rate limiter is integrated.  |
| [**Delay**](#nodeiddelay) | Signal | **Simple Delay Line**:   This is a very simple single buffer delay node. It provides an internal feedback and dry/wet mix.  |
| [**AllP**](#nodeidallp) | Signal | **Single Allpass Filter**:  This is an allpass filter that can be used to build reverbs or anything you might find it useful for.  |
| [**Comb**](#nodeidcomb) | Signal | **Comb Filter**:   A very simple comb filter. It has interesting filtering effects and can also be used to build custom reverbs.  |
| [**Noise**](#nodeidnoise) | Osc | **Noise Oscillator**:   This is a very simple noise oscillator, which can be used for any kind of audio rate noise. And as a source for sample & hold like nodes to generate low frequency modulation. The white noise is uniformly distributed and not normal distributed (which could be a bit more natural in some contexts). See also the `XNoise` node for more noise alternatives.  |
| [**FormFM**](#nodeidformfm) | Osc | **Formant oscillator**:   Simple formant oscillator that generates a formant like sound. Loosely based on the ModFM synthesis method.  |
| [**SFilter**](#nodeidsfilter) | Signal | **Simple Filter**:   This is a collection of more or less simple filters. There are only two parameters: Filter cutoff `freq` and the `res` resonance.  |
| [**FVaFilt**](#nodeidfvafilt) | Signal | **F's Virtual Analog (Stereo) Filter**:   This is a collection of virtual analog filters that were implemented by Fredemus (aka Frederik Halkjær). They behave well when driven hard but that comes with the price that they are more expensive.  |
| [**BiqFilt**](#nodeidbiqfilt) | Signal | **Biquad Filter**:   This is the implementation of a biquad filter cascade. It is not meant for fast automation. Please use other nodes like eg. `SFilter` for that.  |
| [**PVerb**](#nodeidpverb) | Signal | **Plate Reverb**:   This is a simple but yet powerful small plate reverb based on the design by Jon Dattorro. It should suit your needs from small rooms up to large atmospheric sound scapes.  |
| [**Test**](#nodeidtest) | IOUtil | ****:  |
### NodeId::Amp
**Signal Amplifier**

This is a simple amplifier to amplify or attenuate a signal.

- [input **inp**](#nodeidamp-input-inp) - Signal input
- [input **gain**](#nodeidamp-input-gain) - Gain input. This control can actually amplify the signal.
- [input **att**](#nodeidamp-input-att) - Attenuate input. Does only attenuate the signal, not amplify it. Use this for envelope input.
- [setting **neg_att**](#nodeidamp-setting-neg_att) - If this is set to 'Clip', only the **0.0**-**1.0** input range of the `att` input port is used. Negative values are clipped to **0.0**.
- output **sig**
Amplified signal output
#### NodeId::Amp Help
**Signal Amplifier**

It serves the simple purpose of taking an input signal and attenuate (either
with the `att` or the `gain` parameter) or just amplifying it with
the `gain` parameter.

You can even use it as simple fixed control signal source if you leave the
`inp` port unconnected and just dial in the desired output value with the
parameter.

The main idea with the `gain` and `att` parameters is, that you can set
the desired amplification with the `gain` parameter and automate it using
the `att` parameter. The `neg` setting then defines what happens with
negative inputs on the `att` port.

#### NodeId::Amp input inp
Signal input

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `amp(0).set().inp(0)` | `NodeId::Amp(0).inp_param("inp")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `amp(0).set().inp(-1)` | `NodeId::Amp(0).inp_param("inp")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `amp(0).set().inp(0)` | `NodeId::Amp(0).inp_param("inp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `amp(0).set().inp(1)` | `NodeId::Amp(0).inp_param("inp")` |
#### NodeId::Amp input gain
Gain input. This control can actually amplify the signal.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      1.00 | +0.0dB | `amp(0).set().gain(1)` | `NodeId::Amp(0).inp_param("gain")` |
| **min** |  0.0000 |      0.06 | -24.0dB | `amp(0).set().gain(0.063095726)` | `NodeId::Amp(0).inp_param("gain")` |
| **mid** |  0.5000 |      1.00 | +0.0dB | `amp(0).set().gain(1)` | `NodeId::Amp(0).inp_param("gain")` |
| **max** |  1.0000 |     15.85 | +24.0dB | `amp(0).set().gain(15.848933)` | `NodeId::Amp(0).inp_param("gain")` |
#### NodeId::Amp input att
Attenuate input. Does only attenuate the signal, not amplify it.
Use this for envelope input.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `amp(0).set().att(1)` | `NodeId::Amp(0).inp_param("att")` |
| **min** |  0.0000 |      0.00 |  0.000 | `amp(0).set().att(0)` | `NodeId::Amp(0).inp_param("att")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `amp(0).set().att(0.5)` | `NodeId::Amp(0).inp_param("att")` |
| **max** |  1.0000 |      1.00 |  1.000 | `amp(0).set().att(1)` | `NodeId::Amp(0).inp_param("att")` |
#### NodeId::Amp setting neg_att
If this is set to 'Clip', only the **0.0**-**1.0** input range of the `att` input port is used. Negative values are clipped to **0.0**.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Allow | `amp(0).set().neg_att(0)` | `NodeId::Amp(0).inp_param("neg_att")` |
| 1 | Clip | `amp(0).set().neg_att(1)` | `NodeId::Amp(0).inp_param("neg_att")` |
### NodeId::Mix3
**3 Ch. Signal Mixer**

A very simple 3 channel signal mixer.
You can mix anything, from audio signals to control signals.

- [input **ch1**](#nodeidmix3-input-ch1) - Channel 1 Signal input
- [input **ch2**](#nodeidmix3-input-ch2) - Channel 2 Signal input
- [input **ch3**](#nodeidmix3-input-ch3) - Channel 3 Signal input
- [input **vol1**](#nodeidmix3-input-vol1) - Channel 1 volume
- [input **vol2**](#nodeidmix3-input-vol2) - Channel 2 volume
- [input **vol3**](#nodeidmix3-input-vol3) - Channel 3 volume
- [input **ovol**](#nodeidmix3-input-ovol) - Output volume of the sum
- output **sig**
Mixed signal output
#### NodeId::Mix3 Help
**3 Channel Signal Mixer**

Just a small 3 channel mixer to create a sum of multiple signals.
You can mix anything, from audio signals to control signals.

There is even a convenient output volume knob,
to turn down the output.

#### NodeId::Mix3 input ch1
Channel 1 Signal input

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mix3(0).set().ch1(0)` | `NodeId::Mix3(0).inp_param("ch1")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mix3(0).set().ch1(-1)` | `NodeId::Mix3(0).inp_param("ch1")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mix3(0).set().ch1(0)` | `NodeId::Mix3(0).inp_param("ch1")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mix3(0).set().ch1(1)` | `NodeId::Mix3(0).inp_param("ch1")` |
#### NodeId::Mix3 input ch2
Channel 2 Signal input

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mix3(0).set().ch2(0)` | `NodeId::Mix3(0).inp_param("ch2")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mix3(0).set().ch2(-1)` | `NodeId::Mix3(0).inp_param("ch2")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mix3(0).set().ch2(0)` | `NodeId::Mix3(0).inp_param("ch2")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mix3(0).set().ch2(1)` | `NodeId::Mix3(0).inp_param("ch2")` |
#### NodeId::Mix3 input ch3
Channel 3 Signal input

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mix3(0).set().ch3(0)` | `NodeId::Mix3(0).inp_param("ch3")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mix3(0).set().ch3(-1)` | `NodeId::Mix3(0).inp_param("ch3")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mix3(0).set().ch3(0)` | `NodeId::Mix3(0).inp_param("ch3")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mix3(0).set().ch3(1)` | `NodeId::Mix3(0).inp_param("ch3")` |
#### NodeId::Mix3 input vol1
Channel 1 volume

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.8333 |      1.00 | +0.0dB | `mix3(0).set().vol1(0.99999976)` | `NodeId::Mix3(0).inp_param("vol1")` |
| **min** |  0.0000 |      0.00 | -inf dB | `mix3(0).set().vol1(0)` | `NodeId::Mix3(0).inp_param("vol1")` |
| **mid** |  0.5000 |      0.02 | -36.0dB | `mix3(0).set().vol1(0.015848929)` | `NodeId::Mix3(0).inp_param("vol1")` |
| **max** |  1.0000 |      7.94 | +18.0dB | `mix3(0).set().vol1(7.943283)` | `NodeId::Mix3(0).inp_param("vol1")` |
#### NodeId::Mix3 input vol2
Channel 2 volume

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.8333 |      1.00 | +0.0dB | `mix3(0).set().vol2(0.99999976)` | `NodeId::Mix3(0).inp_param("vol2")` |
| **min** |  0.0000 |      0.00 | -inf dB | `mix3(0).set().vol2(0)` | `NodeId::Mix3(0).inp_param("vol2")` |
| **mid** |  0.5000 |      0.02 | -36.0dB | `mix3(0).set().vol2(0.015848929)` | `NodeId::Mix3(0).inp_param("vol2")` |
| **max** |  1.0000 |      7.94 | +18.0dB | `mix3(0).set().vol2(7.943283)` | `NodeId::Mix3(0).inp_param("vol2")` |
#### NodeId::Mix3 input vol3
Channel 3 volume

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.8333 |      1.00 | +0.0dB | `mix3(0).set().vol3(0.99999976)` | `NodeId::Mix3(0).inp_param("vol3")` |
| **min** |  0.0000 |      0.00 | -inf dB | `mix3(0).set().vol3(0)` | `NodeId::Mix3(0).inp_param("vol3")` |
| **mid** |  0.5000 |      0.02 | -36.0dB | `mix3(0).set().vol3(0.015848929)` | `NodeId::Mix3(0).inp_param("vol3")` |
| **max** |  1.0000 |      7.94 | +18.0dB | `mix3(0).set().vol3(7.943283)` | `NodeId::Mix3(0).inp_param("vol3")` |
#### NodeId::Mix3 input ovol
Output volume of the sum

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.8333 |      1.00 | +0.0dB | `mix3(0).set().ovol(0.99999976)` | `NodeId::Mix3(0).inp_param("ovol")` |
| **min** |  0.0000 |      0.00 | -inf dB | `mix3(0).set().ovol(0)` | `NodeId::Mix3(0).inp_param("ovol")` |
| **mid** |  0.5000 |      0.02 | -36.0dB | `mix3(0).set().ovol(0.015848929)` | `NodeId::Mix3(0).inp_param("ovol")` |
| **max** |  1.0000 |      7.94 | +18.0dB | `mix3(0).set().ovol(7.943283)` | `NodeId::Mix3(0).inp_param("ovol")` |
### NodeId::Mux9
**9 Ch. Multiplexer**

An up to 9 channel multiplexer aka switch or junction.
You can route one of the 9 (or fewer) inputs to the output.
The opposite of this node is the `Demux9`,
which demultiplexes or routes the one input signal to one of the 9 outputs.

- [input **slct**](#nodeidmux9-input-slct) - Selects the input that is routed to the output `sig`.But only if this input is actually connected. If there is no connection, the `t_rst`, `t_up` and `t_down` trigger inputs are used to control the current routing. The maximum routed input is determined by the `in_cnt` setting.
- [input **t_rst**](#nodeidmux9-input-t_rst) - Trigger resets the internal routing to the first input `in_1`.Keep in mind: This input is only used if `slct` is not connected.
- [input **t_up**](#nodeidmux9-input-t_up) - Trigger increases the internal routing to the next input port.If the last input (depending on the `in_cnt` setting) was selectedif will wrap around to `in_1`.Keep in mind: This input is only used if `slct` is not connected.
- [input **t_down**](#nodeidmux9-input-t_down) - Trigger decreases the internal routing to the previous input port (eg. `in_3` => `in_2`). If `in_1` as selected, then it will wrap around to the highest possible input port (depending on the `in_cnt` setting).Keep in mind: This input is only used if `slct` is not connected.
- [input **in_1**](#nodeidmux9-input-in_1) - Input port 1.
- [input **in_2**](#nodeidmux9-input-in_2) - Input port 2.
- [input **in_3**](#nodeidmux9-input-in_3) - Input port 3.
- [input **in_4**](#nodeidmux9-input-in_4) - Input port 4.
- [input **in_5**](#nodeidmux9-input-in_5) - Input port 5.
- [input **in_6**](#nodeidmux9-input-in_6) - Input port 6.
- [input **in_7**](#nodeidmux9-input-in_7) - Input port 7.
- [input **in_8**](#nodeidmux9-input-in_8) - Input port 8.
- [input **in_9**](#nodeidmux9-input-in_9) - Input port 9.
- [setting **in_cnt**](#nodeidmux9-setting-in_cnt) - The number of inputs that are routed to the output. This will limit the number of maximally used inputs. 
- output **sig**
The currently selected input port will be presented on this output port.
#### NodeId::Mux9 Help
**9 Channel Multiplexer/Switch**

This is an up to 9 channel multiplexer, also known as switch or junction.
You can route one of the 9 (or fewer) inputs to the one output.
Selection of the input is done either via a control signal to the
`slct` input (range **0**..**1**) (exclusive) or via the `t_rst`, `t_up` or
`t_down` triggers.

If the `slct` input is not connected, the trigger inputs are active.
If you still prefer a knob for manually selecting the input, consider using
some constant signal source like an `Amp` node with an unconnected input.

The `in_cnt` parameter allows selecting the number of routed input channels.

The opposite of this node is the `Demux9`, which demultiplexes or routes
the one input signal to one of the 9 outputs.

Tip:
    An interesting use case for this node is to use it as (up to) 9 step
    control signal sequencer. Leave the `in_1` to `in_9` ports unconnected
    and dial in the desired value via the parameter knobs. This can lead to
    interesting results. Even more interesting it can become if you stack
    multiple `Demux9` in series and connect just some of the input ports
    for slightly changing sequences. Attach a slew limiter node (eg. `LSlew`
    or `ESlew`) if less harsh transitions between the input routings is
    desired.

#### NodeId::Mux9 input slct
Selects the input that is routed to the output `sig`.But only if this input is actually connected. If there is no connection, the `t_rst`, `t_up` and `t_down` trigger inputs are used to control the current routing. The maximum routed input is determined by the `in_cnt` setting.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().slct(0)` | `NodeId::Mux9(0).inp_param("slct")` |
| **min** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().slct(0)` | `NodeId::Mux9(0).inp_param("slct")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `mux9(0).set().slct(0.5)` | `NodeId::Mux9(0).inp_param("slct")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mux9(0).set().slct(1)` | `NodeId::Mux9(0).inp_param("slct")` |
#### NodeId::Mux9 input t_rst
Trigger resets the internal routing to the first input `in_1`.Keep in mind: This input is only used if `slct` is not connected.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().t_rst(0)` | `NodeId::Mux9(0).inp_param("t_rst")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mux9(0).set().t_rst(-1)` | `NodeId::Mux9(0).inp_param("t_rst")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().t_rst(0)` | `NodeId::Mux9(0).inp_param("t_rst")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mux9(0).set().t_rst(1)` | `NodeId::Mux9(0).inp_param("t_rst")` |
#### NodeId::Mux9 input t_up
Trigger increases the internal routing to the next input port.If the last input (depending on the `in_cnt` setting) was selectedif will wrap around to `in_1`.Keep in mind: This input is only used if `slct` is not connected.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().t_up(0)` | `NodeId::Mux9(0).inp_param("t_up")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mux9(0).set().t_up(-1)` | `NodeId::Mux9(0).inp_param("t_up")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().t_up(0)` | `NodeId::Mux9(0).inp_param("t_up")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mux9(0).set().t_up(1)` | `NodeId::Mux9(0).inp_param("t_up")` |
#### NodeId::Mux9 input t_down
Trigger decreases the internal routing to the previous input port (eg. `in_3` => `in_2`). If `in_1` as selected, then it will wrap around to the highest possible input port (depending on the `in_cnt` setting).Keep in mind: This input is only used if `slct` is not connected.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().t_down(0)` | `NodeId::Mux9(0).inp_param("t_down")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mux9(0).set().t_down(-1)` | `NodeId::Mux9(0).inp_param("t_down")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().t_down(0)` | `NodeId::Mux9(0).inp_param("t_down")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mux9(0).set().t_down(1)` | `NodeId::Mux9(0).inp_param("t_down")` |
#### NodeId::Mux9 input in_1
Input port 1.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_1(0)` | `NodeId::Mux9(0).inp_param("in_1")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mux9(0).set().in_1(-1)` | `NodeId::Mux9(0).inp_param("in_1")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_1(0)` | `NodeId::Mux9(0).inp_param("in_1")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mux9(0).set().in_1(1)` | `NodeId::Mux9(0).inp_param("in_1")` |
#### NodeId::Mux9 input in_2
Input port 2.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_2(0)` | `NodeId::Mux9(0).inp_param("in_2")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mux9(0).set().in_2(-1)` | `NodeId::Mux9(0).inp_param("in_2")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_2(0)` | `NodeId::Mux9(0).inp_param("in_2")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mux9(0).set().in_2(1)` | `NodeId::Mux9(0).inp_param("in_2")` |
#### NodeId::Mux9 input in_3
Input port 3.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_3(0)` | `NodeId::Mux9(0).inp_param("in_3")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mux9(0).set().in_3(-1)` | `NodeId::Mux9(0).inp_param("in_3")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_3(0)` | `NodeId::Mux9(0).inp_param("in_3")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mux9(0).set().in_3(1)` | `NodeId::Mux9(0).inp_param("in_3")` |
#### NodeId::Mux9 input in_4
Input port 4.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_4(0)` | `NodeId::Mux9(0).inp_param("in_4")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mux9(0).set().in_4(-1)` | `NodeId::Mux9(0).inp_param("in_4")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_4(0)` | `NodeId::Mux9(0).inp_param("in_4")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mux9(0).set().in_4(1)` | `NodeId::Mux9(0).inp_param("in_4")` |
#### NodeId::Mux9 input in_5
Input port 5.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_5(0)` | `NodeId::Mux9(0).inp_param("in_5")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mux9(0).set().in_5(-1)` | `NodeId::Mux9(0).inp_param("in_5")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_5(0)` | `NodeId::Mux9(0).inp_param("in_5")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mux9(0).set().in_5(1)` | `NodeId::Mux9(0).inp_param("in_5")` |
#### NodeId::Mux9 input in_6
Input port 6.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_6(0)` | `NodeId::Mux9(0).inp_param("in_6")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mux9(0).set().in_6(-1)` | `NodeId::Mux9(0).inp_param("in_6")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_6(0)` | `NodeId::Mux9(0).inp_param("in_6")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mux9(0).set().in_6(1)` | `NodeId::Mux9(0).inp_param("in_6")` |
#### NodeId::Mux9 input in_7
Input port 7.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_7(0)` | `NodeId::Mux9(0).inp_param("in_7")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mux9(0).set().in_7(-1)` | `NodeId::Mux9(0).inp_param("in_7")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_7(0)` | `NodeId::Mux9(0).inp_param("in_7")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mux9(0).set().in_7(1)` | `NodeId::Mux9(0).inp_param("in_7")` |
#### NodeId::Mux9 input in_8
Input port 8.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_8(0)` | `NodeId::Mux9(0).inp_param("in_8")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mux9(0).set().in_8(-1)` | `NodeId::Mux9(0).inp_param("in_8")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_8(0)` | `NodeId::Mux9(0).inp_param("in_8")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mux9(0).set().in_8(1)` | `NodeId::Mux9(0).inp_param("in_8")` |
#### NodeId::Mux9 input in_9
Input port 9.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_9(0)` | `NodeId::Mux9(0).inp_param("in_9")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `mux9(0).set().in_9(-1)` | `NodeId::Mux9(0).inp_param("in_9")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `mux9(0).set().in_9(0)` | `NodeId::Mux9(0).inp_param("in_9")` |
| **max** |  1.0000 |      1.00 |  1.000 | `mux9(0).set().in_9(1)` | `NodeId::Mux9(0).inp_param("in_9")` |
#### NodeId::Mux9 setting in_cnt
The number of inputs that are routed to the output. This will limit the number of maximally used inputs.


| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | 1 | `mux9(0).set().in_cnt(0)` | `NodeId::Mux9(0).inp_param("in_cnt")` |
| 1 | 2 | `mux9(0).set().in_cnt(1)` | `NodeId::Mux9(0).inp_param("in_cnt")` |
| 2 | 3 | `mux9(0).set().in_cnt(2)` | `NodeId::Mux9(0).inp_param("in_cnt")` |
| 3 | 4 | `mux9(0).set().in_cnt(3)` | `NodeId::Mux9(0).inp_param("in_cnt")` |
| 4 | 5 | `mux9(0).set().in_cnt(4)` | `NodeId::Mux9(0).inp_param("in_cnt")` |
| 5 | 6 | `mux9(0).set().in_cnt(5)` | `NodeId::Mux9(0).inp_param("in_cnt")` |
| 6 | 7 | `mux9(0).set().in_cnt(6)` | `NodeId::Mux9(0).inp_param("in_cnt")` |
| 7 | 8 | `mux9(0).set().in_cnt(7)` | `NodeId::Mux9(0).inp_param("in_cnt")` |
| 8 | 9 | `mux9(0).set().in_cnt(8)` | `NodeId::Mux9(0).inp_param("in_cnt")` |
### NodeId::SMap
**Simple Range Mapper**

This node allows to map an unipolar (**0**..**1**) or bipolar signal (**-1**..**1**) to a defined
`min`/`max` signal range.

See also the 'Map' node for a more sophisticated version of this.

- [input **inp**](#nodeidsmap-input-inp) - Signal input
- [input **min**](#nodeidsmap-input-min) - Minimum of the output signal range.
- [input **max**](#nodeidsmap-input-max) - Maximum of the output signal range.
- [setting **clip**](#nodeidsmap-setting-clip) - The **Clip** mode allows you to limit the output exactly to the `min`/`max` range. If this is **Off**, the output may be outside the output signal range.
- [setting **mode**](#nodeidsmap-setting-mode) - This mode defines what kind of input signal is expected and how it will be mapped to the output `min`/`max` range. These modes are available:  - **Unipolar** (**0**..**1**) - **Bipolar**  (**-1**..**1**) - **UniInv**   (**1**..**0**) - **BiInv**    (**1**..**-1**) 
- output **sig**
Mapped signal output
#### NodeId::SMap Help
**Simple Range Mapper**

This node allows to map an unipolar (**0**..**1**) or bipolar signal (**-1**..**1**)
to a defined `min`/`max` signal range.

The **Clip** mode allows you to limit the output exactly to the `min`/`max`
range. If this is **Off**, the output may be outside the output signal
range if the input signal is outside the input signal range.

The `input` mode allows you to choose between 4 options:

- **Unipolar** (**0**..**1**)
- **Bipolar**  (**-1**..**1**)
- **UniInv**   (**1**..**0**)
- **BiInv**    (**1**..**-1**)

The inverse settings will map **1** to `min` and **0** to `max` for **UniInv**.
And **1** to `min` and **-1** to `max` for **BiInv**.

For a more sophisticated version of this node see also `Map`.

#### NodeId::SMap input inp
Signal input

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `smap(0).set().inp(0)` | `NodeId::SMap(0).inp_param("inp")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `smap(0).set().inp(-1)` | `NodeId::SMap(0).inp_param("inp")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `smap(0).set().inp(0)` | `NodeId::SMap(0).inp_param("inp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `smap(0).set().inp(1)` | `NodeId::SMap(0).inp_param("inp")` |
#### NodeId::SMap input min
Minimum of the output signal range.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** | -1.0000 |     -1.00 | -1.000 | `smap(0).set().min(-1)` | `NodeId::SMap(0).inp_param("min")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `smap(0).set().min(-1)` | `NodeId::SMap(0).inp_param("min")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `smap(0).set().min(0)` | `NodeId::SMap(0).inp_param("min")` |
| **max** |  1.0000 |      1.00 |  1.000 | `smap(0).set().min(1)` | `NodeId::SMap(0).inp_param("min")` |
#### NodeId::SMap input max
Maximum of the output signal range.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `smap(0).set().max(1)` | `NodeId::SMap(0).inp_param("max")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `smap(0).set().max(-1)` | `NodeId::SMap(0).inp_param("max")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `smap(0).set().max(0)` | `NodeId::SMap(0).inp_param("max")` |
| **max** |  1.0000 |      1.00 |  1.000 | `smap(0).set().max(1)` | `NodeId::SMap(0).inp_param("max")` |
#### NodeId::SMap setting clip
The **Clip** mode allows you to limit the output exactly to the `min`/`max` range. If this is **Off**, the output may be outside the output signal range.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Off | `smap(0).set().clip(0)` | `NodeId::SMap(0).inp_param("clip")` |
| 1 | Clip | `smap(0).set().clip(1)` | `NodeId::SMap(0).inp_param("clip")` |
#### NodeId::SMap setting mode
This mode defines what kind of input signal is expected and how it will be mapped to the output `min`/`max` range. These modes are available:

- **Unipolar** (**0**..**1**)
- **Bipolar**  (**-1**..**1**)
- **UniInv**   (**1**..**0**)
- **BiInv**    (**1**..**-1**)


| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Unipolar | `smap(0).set().mode(0)` | `NodeId::SMap(0).inp_param("mode")` |
| 1 | Bipolar | `smap(0).set().mode(1)` | `NodeId::SMap(0).inp_param("mode")` |
| 2 | UniInv | `smap(0).set().mode(2)` | `NodeId::SMap(0).inp_param("mode")` |
| 3 | BiInv | `smap(0).set().mode(3)` | `NodeId::SMap(0).inp_param("mode")` |
### NodeId::Map
**Range Mapper**

This node allows to map an input signal range to a precise output signal range.
It's mostly useful to map control signals to modulate inputs.

See also the `SMap` node, which is a simplified version of this node.

- [input **inp**](#nodeidmap-input-inp) - Signal input
- [input **atv**](#nodeidmap-input-atv) - Input signal attenuverter, to attenuate or invert the input signal.
- [input **offs**](#nodeidmap-input-offs) - Input signal offset after `atv` has been applied.
- [input **imin**](#nodeidmap-input-imin) - Minimum of the input signal range, it's mapped to the `min` output signal range.
- [input **imax**](#nodeidmap-input-imax) - Maximum of the input signal range, it's mapped to the `max` output signal range.
- [input **min**](#nodeidmap-input-min) - Minimum of the output signal range.
- [input **max**](#nodeidmap-input-max) - Maximum of the output signal range.
- [setting **clip**](#nodeidmap-setting-clip) - The `clip` mode allows you to limit the output exactly to the `min`/`max` range. If this is off, the output may be outside the output signal range if the input signal is outside the input signal range.
- output **sig**
Mapped signal output
#### NodeId::Map Help
**Range Mapper**

This node allows to map an input signal range to a precise output signal
range. It's main use is for precise control of an input of another node.

It processes the input signal as follows. First the input is attenuverted
using the `atv` parameter and then the `offs` offset parameter is added:

```text
    inp * atv + offs
```

The resulting signal is then processed by the mapping, that maps
the input signal range `imin`/`imax` to the ouput signal range `min`/`max`.

The `clip` mode allows you to limit the output exactly to the `min`/`max`
range. If this is off, the output may be outside the output signal
range if the input signal is outside the input signal range.

This can also be used to invert the signal.

For a more simplified version of this node see also `SMap`.

#### NodeId::Map input inp
Signal input

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `map(0).set().inp(0)` | `NodeId::Map(0).inp_param("inp")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `map(0).set().inp(-1)` | `NodeId::Map(0).inp_param("inp")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `map(0).set().inp(0)` | `NodeId::Map(0).inp_param("inp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `map(0).set().inp(1)` | `NodeId::Map(0).inp_param("inp")` |
#### NodeId::Map input atv
Input signal attenuverter, to attenuate or invert the input signal.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `map(0).set().atv(1)` | `NodeId::Map(0).inp_param("atv")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `map(0).set().atv(-1)` | `NodeId::Map(0).inp_param("atv")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `map(0).set().atv(0)` | `NodeId::Map(0).inp_param("atv")` |
| **max** |  1.0000 |      1.00 |  1.000 | `map(0).set().atv(1)` | `NodeId::Map(0).inp_param("atv")` |
#### NodeId::Map input offs
Input signal offset after `atv` has been applied.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `map(0).set().offs(0)` | `NodeId::Map(0).inp_param("offs")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `map(0).set().offs(-1)` | `NodeId::Map(0).inp_param("offs")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `map(0).set().offs(0)` | `NodeId::Map(0).inp_param("offs")` |
| **max** |  1.0000 |      1.00 |  1.000 | `map(0).set().offs(1)` | `NodeId::Map(0).inp_param("offs")` |
#### NodeId::Map input imin
Minimum of the input signal range, it's mapped to the `min` output signal range.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** | -1.0000 |     -1.00 | -1.000 | `map(0).set().imin(-1)` | `NodeId::Map(0).inp_param("imin")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `map(0).set().imin(-1)` | `NodeId::Map(0).inp_param("imin")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `map(0).set().imin(0)` | `NodeId::Map(0).inp_param("imin")` |
| **max** |  1.0000 |      1.00 |  1.000 | `map(0).set().imin(1)` | `NodeId::Map(0).inp_param("imin")` |
#### NodeId::Map input imax
Maximum of the input signal range, it's mapped to the `max` output signal range.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `map(0).set().imax(1)` | `NodeId::Map(0).inp_param("imax")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `map(0).set().imax(-1)` | `NodeId::Map(0).inp_param("imax")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `map(0).set().imax(0)` | `NodeId::Map(0).inp_param("imax")` |
| **max** |  1.0000 |      1.00 |  1.000 | `map(0).set().imax(1)` | `NodeId::Map(0).inp_param("imax")` |
#### NodeId::Map input min
Minimum of the output signal range.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** | -1.0000 |     -1.00 | -1.000 | `map(0).set().min(-1)` | `NodeId::Map(0).inp_param("min")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `map(0).set().min(-1)` | `NodeId::Map(0).inp_param("min")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `map(0).set().min(0)` | `NodeId::Map(0).inp_param("min")` |
| **max** |  1.0000 |      1.00 |  1.000 | `map(0).set().min(1)` | `NodeId::Map(0).inp_param("min")` |
#### NodeId::Map input max
Maximum of the output signal range.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `map(0).set().max(1)` | `NodeId::Map(0).inp_param("max")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `map(0).set().max(-1)` | `NodeId::Map(0).inp_param("max")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `map(0).set().max(0)` | `NodeId::Map(0).inp_param("max")` |
| **max** |  1.0000 |      1.00 |  1.000 | `map(0).set().max(1)` | `NodeId::Map(0).inp_param("max")` |
#### NodeId::Map setting clip
The `clip` mode allows you to limit the output exactly to the `min`/`max` range. If this is off, the output may be outside the output signal range if the input signal is outside the input signal range.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Off | `map(0).set().clip(0)` | `NodeId::Map(0).inp_param("clip")` |
| 1 | Clip | `map(0).set().clip(1)` | `NodeId::Map(0).inp_param("clip")` |
### NodeId::Quant
**Pitch Quantizer**

This is a simple quantizer, that snaps a pitch signal on `freq` to the closest selected notes within their octave.

- [input **freq**](#nodeidquant-input-freq) - Any signal that is to be pitch quantized.
- [input **oct**](#nodeidquant-input-oct) - Pitch offset, the knob is snapping to octave offsets. Feed signal values snapped to **0.1** multiples for exact octave offsets.
- [setting **keys**](#nodeidquant-setting-keys) - Select the notes you want to snap to here. If no notes are selected, the quantizer will snap the incoming signal to any closest note.
- output **sig**
The quantized output signal that is rounded to the next selected note pitch within the octave of the original input to `freq`.
- output **t**
Everytime the quantizer snaps to a new pitch, it will emit a short trigger on this signal output. This is useful to trigger for example an envelope.
#### NodeId::Quant Help
**A pitch quantizer**

This is a simple quantizer, that snaps a pitch signal on `freq` to the
closest selected notes within their octave.

If you sweep along pitches you will notice that notes that are closer together
are travelled across faster. That means the notes are not evenly distributed
across the pitch input. If you want a more evenly distributed pitch selection
please see also the `CQnt` node.

#### NodeId::Quant input freq
Any signal that is to be pitch quantized.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |    440.00 |  440.0Hz | `quant(0).set().freq(440)` | `NodeId::Quant(0).inp_param("freq")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `quant(0).set().freq(0.4296875)` | `NodeId::Quant(0).inp_param("freq")` |
| **mid** | -0.2176 |     97.34 |  97.34Hz | `quant(0).set().freq(97.33759)` | `NodeId::Quant(0).inp_param("freq")` |
| **max** |  0.5647 |  22049.99 |  22050Hz | `quant(0).set().freq(22049.994)` | `NodeId::Quant(0).inp_param("freq")` |
#### NodeId::Quant input oct
Pitch offset, the knob is snapping to octave offsets. Feed signal values snapped to **0.1** multiples for exact octave offsets.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `quant(0).set().oct(0)` | `NodeId::Quant(0).inp_param("oct")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `quant(0).set().oct(-1)` | `NodeId::Quant(0).inp_param("oct")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `quant(0).set().oct(0)` | `NodeId::Quant(0).inp_param("oct")` |
| **max** |  1.0000 |      1.00 |  1.000 | `quant(0).set().oct(1)` | `NodeId::Quant(0).inp_param("oct")` |
#### NodeId::Quant setting keys
Select the notes you want to snap to here. If no notes are selected, the quantizer will snap the incoming signal to any closest note.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | ? | `quant(0).set().keys(0)` | `NodeId::Quant(0).inp_param("keys")` |
### NodeId::CQnt
**Ctrl Pitch Quantizer**

This special quantizer maps the unipolar **0**..**1** control signal
input range on `inp` evenly to the selected keys and octaves.

- [input **inp**](#nodeidcqnt-input-inp) - The unipolar input signal that is to be mapped to the selected pitch range.
- [input **oct**](#nodeidcqnt-input-oct) - The octave offset from A4.
- [setting **keys**](#nodeidcqnt-setting-keys) - Here you can select the individual notes of the range. If no note is selected, it's the same as if all notes were selected.
- [setting **omin**](#nodeidcqnt-setting-omin) - The minimum octave of the range. If **0** it will be `oct`.
- [setting **omax**](#nodeidcqnt-setting-omax) - The maximum octave of the range. If **0** it will be `oct`.
- output **sig**
The output pitch signal.
- output **t**
Everytime the quantizer snaps to a new pitch, it will emit a short trigger on this signal output. This is useful to trigger for example an envelope.
#### NodeId::CQnt Help
**A control signal to pitch quantizer**

This is a specialized control signal quantizer to generate a pitch/frequency
from a signal within the **0**..**1** range. It does not quantize a typical **-1**..**1**
frequency signal like the `Quant` node.

In contrast to `Quant`, this quantizer maps the incoming signal evenly
to the available note range. It will result in more evenly played notes
if you sweep across the input signal range.

#### NodeId::CQnt input inp
The unipolar input signal that is to be mapped to the selected pitch range.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `cqnt(0).set().inp(0)` | `NodeId::CQnt(0).inp_param("inp")` |
| **min** |  0.0000 |      0.00 |  0.000 | `cqnt(0).set().inp(0)` | `NodeId::CQnt(0).inp_param("inp")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `cqnt(0).set().inp(0.5)` | `NodeId::CQnt(0).inp_param("inp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `cqnt(0).set().inp(1)` | `NodeId::CQnt(0).inp_param("inp")` |
#### NodeId::CQnt input oct
The octave offset from A4.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `cqnt(0).set().oct(0)` | `NodeId::CQnt(0).inp_param("oct")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `cqnt(0).set().oct(-1)` | `NodeId::CQnt(0).inp_param("oct")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `cqnt(0).set().oct(0)` | `NodeId::CQnt(0).inp_param("oct")` |
| **max** |  1.0000 |      1.00 |  1.000 | `cqnt(0).set().oct(1)` | `NodeId::CQnt(0).inp_param("oct")` |
#### NodeId::CQnt setting keys
Here you can select the individual notes of the range. If no note is selected, it's the same as if all notes were selected.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | ? | `cqnt(0).set().keys(0)` | `NodeId::CQnt(0).inp_param("keys")` |
#### NodeId::CQnt setting omin
The minimum octave of the range. If **0** it will be `oct`.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | -0 | `cqnt(0).set().omin(0)` | `NodeId::CQnt(0).inp_param("omin")` |
| 1 | -1 | `cqnt(0).set().omin(1)` | `NodeId::CQnt(0).inp_param("omin")` |
| 2 | -2 | `cqnt(0).set().omin(2)` | `NodeId::CQnt(0).inp_param("omin")` |
| 3 | -3 | `cqnt(0).set().omin(3)` | `NodeId::CQnt(0).inp_param("omin")` |
| 4 | -4 | `cqnt(0).set().omin(4)` | `NodeId::CQnt(0).inp_param("omin")` |
#### NodeId::CQnt setting omax
The maximum octave of the range. If **0** it will be `oct`.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | +0 | `cqnt(0).set().omax(0)` | `NodeId::CQnt(0).inp_param("omax")` |
| 1 | +1 | `cqnt(0).set().omax(1)` | `NodeId::CQnt(0).inp_param("omax")` |
| 2 | +2 | `cqnt(0).set().omax(2)` | `NodeId::CQnt(0).inp_param("omax")` |
| 3 | +3 | `cqnt(0).set().omax(3)` | `NodeId::CQnt(0).inp_param("omax")` |
| 4 | +4 | `cqnt(0).set().omax(4)` | `NodeId::CQnt(0).inp_param("omax")` |
### NodeId::TSeq
**Tracker Sequencer**

This node implements a sequencer that can be programmed using the tracker interface in HexoSynth on the right.
It provides 6 control signals and 6 gate outputs.
- [input **clock**](#nodeidtseq-input-clock) - Clock input
- [input **trig**](#nodeidtseq-input-trig) - Synchronization trigger which restarts the sequence.
- [setting **cmode**](#nodeidtseq-setting-cmode) - `clock` input signal mode: - **RowT**: Trigger = advance row - **PatT**: Trigger = pattern rate - **Phase**: Phase to pattern index  
- output **trk1**
Track 1 signal output
- output **trk2**
Track 2 signal output
- output **trk3**
Track 3 signal output
- output **trk4**
Track 4 signal output
- output **trk5**
Track 5 signal output
- output **trk6**
Track 6 signal output
- output **gat1**
Track 1 gate output
- output **gat2**
Track 2 gate output
- output **gat3**
Track 3 gate output
- output **gat4**
Track 4 gate output
- output **gat5**
Track 5 gate output
- output **gat6**
Track 6 gate output
#### NodeId::TSeq Help
**Tracker (based) Sequencer**

This sequencer gets it's speed from the clock source. The `clock`
signal can be interpreted in different modes. But if you want to
run multiple sequencers in parallel, you want to synchronize them.
For this you can use the `trig` input, it resets the played row to
the beginning of the sequence every time a trigger is received.

Alternatively you can run the sequencer clock using the phase mode.
With that the phase (**0..1**) signal on the `clock` input determines the
exact play head position in the pattern. With this you just need to
synchronize the phase generators for different sequencers.

For an idea how to chain multiple tracker sequencers, see the next page.

This tracker provides 6 columns that each can have one of the following
types:

- *Note* column: for specifying pitches.
- *Step* column: for specifying non interpolated control signals.
- *Value* column: for specifying linearly interpolated control signals.
- *Gate* column: for specifying gates, with probability and ratcheting.

Step, value and gate cells can be set to **4096** (**0xFFF**) different values
or contain nothing at all. For step and value columns these values
are mapped to the **0.0-1.0** control signal range, with **0xFFF** being **1.0**
and **0x000** being **0.0**.

```text
    Value examples:     1.0   0.9  0.75   0.5  0.25   0.1
                      0xFFF 0xE70 0xC00 0x800 0x400 0x19A
    Gate examples:
        Probability  Ratcheting   Gate Length        full on gate: 0xFFF
          6%  0x000  16   0x000   1/16  0x000      2 short pulses: 0xFE0
         18%  0x200  14   0x020   3/16  0x002      4 short pulses: 0xFC0
         25%  0x300  13   0x030   4/16  0x003        2 50% pulses: 0xFE7
         50%  0x700   9   0x070   8/16  0x007        half on gate: 0xFF7
         62%  0x900   7   0x090  10/16  0x009         short pulse: 0xFF0
         75%  0xC00   4   0x0C0  12/16  0x00C    rare short pulse: 0xEF0
         87%  0xE00   2   0x0E0  15/16  0x00E   50/50 short pulse: 0x7F0
        100%  0xF00   1   0x0F0  16/16  0x00F   50/50 full gate:   0x7FF
```

## Gate Input and Output

The gate cells are differently coded:

- **0x00F**: The least significant nibble controls the gate length.
         With **0x00f** being the full row, and **0x000** being 1/16th of a row.
- **0x0F0**: The second nibble controls ratcheting, with **0x0F0** being one
         gate per row, and **0x000** being 16 gates per row.
         Length of these gates is controlled by the last significant nibble.
- **0xF00**: The most significant nibble controls probability of the
         whole gate cell. With **0xF00** meaning the gate will always be
         triggered, and **0x000** means that the gate is only triggered with
         **6%** probability. **50%** is **0x700.**

The behaviour of the 6 gate outputs of `TSeq` depend on the corresponding
column type:

- Step `gat1-gat6`: Like note columns, this will output a **1.0** for the whole
                   row if a step value is set. With two step values directly
                   following each other no **0.0** will be emitted in between
                   the rows. This means if you want to drive an envelope
                   with release phase with this signal, you need to make
                   space for the release phase.
- Note `gat1-gat6`: Behaves just like step columns.
- Gate `gat1-gat6`: Behaves just like step columns.
- Value `gat1-gat6`: Outputs a **1.0** value for the duration of the last row.
                   You can use this to trigger other things once the
                   sequence has been played.

*Tip*:

If you want to use the end of a tracker sequence as trigger for
something else, eg. switching to a different `TSeq` and restart
it using it's `trig` input, you will need to use the gate output
of a value column and invert it.

#### NodeId::TSeq input clock
Clock input

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `tseq(0).set().clock(0)` | `NodeId::TSeq(0).inp_param("clock")` |
| **min** |  0.0000 |      0.00 |  0.000 | `tseq(0).set().clock(0)` | `NodeId::TSeq(0).inp_param("clock")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `tseq(0).set().clock(0.5)` | `NodeId::TSeq(0).inp_param("clock")` |
| **max** |  1.0000 |      1.00 |  1.000 | `tseq(0).set().clock(1)` | `NodeId::TSeq(0).inp_param("clock")` |
#### NodeId::TSeq input trig
Synchronization trigger which restarts the sequence.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `tseq(0).set().trig(0)` | `NodeId::TSeq(0).inp_param("trig")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `tseq(0).set().trig(-1)` | `NodeId::TSeq(0).inp_param("trig")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `tseq(0).set().trig(0)` | `NodeId::TSeq(0).inp_param("trig")` |
| **max** |  1.0000 |      1.00 |  1.000 | `tseq(0).set().trig(1)` | `NodeId::TSeq(0).inp_param("trig")` |
#### NodeId::TSeq setting cmode
`clock` input signal mode:
- **RowT**: Trigger = advance row
- **PatT**: Trigger = pattern rate
- **Phase**: Phase to pattern index



| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | RowT | `tseq(0).set().cmode(0)` | `NodeId::TSeq(0).inp_param("cmode")` |
| 1 | PatT | `tseq(0).set().cmode(1)` | `NodeId::TSeq(0).inp_param("cmode")` |
| 2 | Phase | `tseq(0).set().cmode(2)` | `NodeId::TSeq(0).inp_param("cmode")` |
### NodeId::Code
**WBlockDSP Code Execution**

This node executes just in time compiled code as fast as machine code. Use this to implement real time DSP code yourself. The inputs are freely useable in your code. All the ports (input and output) can be used either for audio or for control signals.
- [input **in1**](#nodeidcode-input-in1) - Input Signal 1
- [input **in2**](#nodeidcode-input-in2) - Input Signal 2
- [input **alpha**](#nodeidcode-input-alpha) - Input Parameter Alpha
- [input **beta**](#nodeidcode-input-beta) - Input Parameter Beta
- [input **delta**](#nodeidcode-input-delta) - Input Parameter Delta
- [input **gamma**](#nodeidcode-input-gamma) - Input Parameter Gamma
- output **sig**
Return output
- output **sig1**
Signal channel 1 output
- output **sig2**
Signal channel 2 output
#### NodeId::Code Help
**WBlockDSP Code Execution**

This node executes just in time compiled code as fast as machine code.
Use this to implement real time DSP code yourself. The inputs are freely
useable in your code. All the ports (input and output) can be used either
for audio or for control signals.

The inputs `in1` and `in2` are thought to be a stereo signal input. But
you are free to repurpose them as you like.

The inputs `alpha`, `beta`, `delta` and `gamma` can be used as parameters
in your code. But are also not restricted, so you may use them as audio signal
inputs.

The outputs `sig`, `sig1` and `sig3` are also freely useable.

Some ideas how to use this, you can build your own:

- Waveshapers
- Signal Generators (Oscillators)
- Custom LFO
- Control Signal shapers or generators
- Sequencers
- ... and many more things!

#### NodeId::Code input in1
Input Signal 1

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `code(0).set().in1(0)` | `NodeId::Code(0).inp_param("in1")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `code(0).set().in1(-1)` | `NodeId::Code(0).inp_param("in1")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `code(0).set().in1(0)` | `NodeId::Code(0).inp_param("in1")` |
| **max** |  1.0000 |      1.00 |  1.000 | `code(0).set().in1(1)` | `NodeId::Code(0).inp_param("in1")` |
#### NodeId::Code input in2
Input Signal 2

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `code(0).set().in2(0)` | `NodeId::Code(0).inp_param("in2")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `code(0).set().in2(-1)` | `NodeId::Code(0).inp_param("in2")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `code(0).set().in2(0)` | `NodeId::Code(0).inp_param("in2")` |
| **max** |  1.0000 |      1.00 |  1.000 | `code(0).set().in2(1)` | `NodeId::Code(0).inp_param("in2")` |
#### NodeId::Code input alpha
Input Parameter Alpha

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `code(0).set().alpha(0)` | `NodeId::Code(0).inp_param("alpha")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `code(0).set().alpha(-1)` | `NodeId::Code(0).inp_param("alpha")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `code(0).set().alpha(0)` | `NodeId::Code(0).inp_param("alpha")` |
| **max** |  1.0000 |      1.00 |  1.000 | `code(0).set().alpha(1)` | `NodeId::Code(0).inp_param("alpha")` |
#### NodeId::Code input beta
Input Parameter Beta

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `code(0).set().beta(0)` | `NodeId::Code(0).inp_param("beta")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `code(0).set().beta(-1)` | `NodeId::Code(0).inp_param("beta")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `code(0).set().beta(0)` | `NodeId::Code(0).inp_param("beta")` |
| **max** |  1.0000 |      1.00 |  1.000 | `code(0).set().beta(1)` | `NodeId::Code(0).inp_param("beta")` |
#### NodeId::Code input delta
Input Parameter Delta

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `code(0).set().delta(0)` | `NodeId::Code(0).inp_param("delta")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `code(0).set().delta(-1)` | `NodeId::Code(0).inp_param("delta")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `code(0).set().delta(0)` | `NodeId::Code(0).inp_param("delta")` |
| **max** |  1.0000 |      1.00 |  1.000 | `code(0).set().delta(1)` | `NodeId::Code(0).inp_param("delta")` |
#### NodeId::Code input gamma
Input Parameter Gamma

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `code(0).set().gamma(0)` | `NodeId::Code(0).inp_param("gamma")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `code(0).set().gamma(-1)` | `NodeId::Code(0).inp_param("gamma")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `code(0).set().gamma(0)` | `NodeId::Code(0).inp_param("gamma")` |
| **max** |  1.0000 |      1.00 |  1.000 | `code(0).set().gamma(1)` | `NodeId::Code(0).inp_param("gamma")` |
### NodeId::Rust1x1
**Rust Code Node**

This node does provide the user of HexoDSP or the SynthConstructor with an API
to code custom DSP node implementations in pure Rust at compile time.
It does not have any relevance for HexoSynth.
See also [crate::SynthConstructor] and [crate::DynamicNode1x1].

- [input **inp**](#nodeidrust1x1-input-inp) - Signal input. Signal input to the dynamically dispatched Rust node.
- [input **alpha**](#nodeidrust1x1-input-alpha) - Alpha parameter for the dynamically dispatched Rust node.
- [input **beta**](#nodeidrust1x1-input-beta) - Beta parameter for the dynamically dispatched Rust node.
- [input **delta**](#nodeidrust1x1-input-delta) - Delta parameter for the dynamically dispatched Rust node.
- [input **gamma**](#nodeidrust1x1-input-gamma) - Gamma parameter for the dynamically dispatched Rust node.
- output **sig**
Signal output. Signal output of the dynamically dispatched Rust node.
#### NodeId::Rust1x1 Help
**Rust Code Node**

This node does provide the user of HexoDSP or the SynthConstructor with an API
to code custom DSP node implementations in pure Rust at compile time.

Treat this node as plugin API into the HexoDSP DSP graph.

This node does nothing in HexoSynth.

See also [crate::SynthConstructor] and [crate::DynamicNode1x1].

#### NodeId::Rust1x1 input inp
Signal input. Signal input to the dynamically dispatched Rust node.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `rust1x1(0).set().inp(0)` | `NodeId::Rust1x1(0).inp_param("inp")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `rust1x1(0).set().inp(-1)` | `NodeId::Rust1x1(0).inp_param("inp")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `rust1x1(0).set().inp(0)` | `NodeId::Rust1x1(0).inp_param("inp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `rust1x1(0).set().inp(1)` | `NodeId::Rust1x1(0).inp_param("inp")` |
#### NodeId::Rust1x1 input alpha
Alpha parameter for the dynamically dispatched Rust node.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `rust1x1(0).set().alpha(0)` | `NodeId::Rust1x1(0).inp_param("alpha")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `rust1x1(0).set().alpha(-1)` | `NodeId::Rust1x1(0).inp_param("alpha")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `rust1x1(0).set().alpha(0)` | `NodeId::Rust1x1(0).inp_param("alpha")` |
| **max** |  1.0000 |      1.00 |  1.000 | `rust1x1(0).set().alpha(1)` | `NodeId::Rust1x1(0).inp_param("alpha")` |
#### NodeId::Rust1x1 input beta
Beta parameter for the dynamically dispatched Rust node.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `rust1x1(0).set().beta(0)` | `NodeId::Rust1x1(0).inp_param("beta")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `rust1x1(0).set().beta(-1)` | `NodeId::Rust1x1(0).inp_param("beta")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `rust1x1(0).set().beta(0)` | `NodeId::Rust1x1(0).inp_param("beta")` |
| **max** |  1.0000 |      1.00 |  1.000 | `rust1x1(0).set().beta(1)` | `NodeId::Rust1x1(0).inp_param("beta")` |
#### NodeId::Rust1x1 input delta
Delta parameter for the dynamically dispatched Rust node.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `rust1x1(0).set().delta(0)` | `NodeId::Rust1x1(0).inp_param("delta")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `rust1x1(0).set().delta(-1)` | `NodeId::Rust1x1(0).inp_param("delta")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `rust1x1(0).set().delta(0)` | `NodeId::Rust1x1(0).inp_param("delta")` |
| **max** |  1.0000 |      1.00 |  1.000 | `rust1x1(0).set().delta(1)` | `NodeId::Rust1x1(0).inp_param("delta")` |
#### NodeId::Rust1x1 input gamma
Gamma parameter for the dynamically dispatched Rust node.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `rust1x1(0).set().gamma(0)` | `NodeId::Rust1x1(0).inp_param("gamma")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `rust1x1(0).set().gamma(-1)` | `NodeId::Rust1x1(0).inp_param("gamma")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `rust1x1(0).set().gamma(0)` | `NodeId::Rust1x1(0).inp_param("gamma")` |
| **max** |  1.0000 |      1.00 |  1.000 | `rust1x1(0).set().gamma(1)` | `NodeId::Rust1x1(0).inp_param("gamma")` |
### NodeId::Sampl
**Sample Player**
Provides a simple sample player that you can load a single audio sample from a WAV file into.
- [input **freq**](#nodeidsampl-input-freq) - Pitch input for the sampler, giving the playback speed of the sample.
- [input **trig**](#nodeidsampl-input-trig) - The trigger input causes a resync of the playback phase and triggers the playback if the `pmode` is **OneShot**
- [input **offs**](#nodeidsampl-input-offs) - Start position offset.
- [input **len**](#nodeidsampl-input-len) - Adjusts the playback length of the sample in relation to the original length of the sample.
- [input **dcms**](#nodeidsampl-input-dcms) - Declick fade time in milliseconds. Not audio rate!
- [input **det**](#nodeidsampl-input-det) - Detune the oscillator in semitones and cents. the input of this value is rounded to semitones on coarse input. Fine input lets you detune in cents (rounded). A signal sent to this port is not rounded. Note: The signal input allows detune +-10 octaves.
- [setting **sample**](#nodeidsampl-setting-sample) - The audio sample that is played back.
- [setting **pmode**](#nodeidsampl-setting-pmode) - The playback mode of the sampler. - **Loop** constantly plays back the sample. You can reset/sync the phase using the `trig` input in this case. - **OneShot** plays back the sample if a trigger is received on `trig` input. 
- [setting **dclick**](#nodeidsampl-setting-dclick) - If this is enabled it will enable short fade in and out ramps. This if useful if you don't want to add an envelope just for getting rid of the clicks if spos and epos are modulated.
- [setting **dir**](#nodeidsampl-setting-dir) - Sets the direction of the playhead, plays the sample forwards or backwards.
- output **sig**
Sampler audio output
#### NodeId::Sampl Help
**Sample Player**

Provides a simple sample player for playing back one loaded audio sample.
It can be used for purposes like:

* Adding ambient samples to your patches.
* Using drum samples (set `pmode`) to **OneShot**
* Having an oscillator with a custom waveform (set `pmode`) to **Loop**
* As custom control signal source for very long or very custom envelopes.

Only a single audio sample can be loaded into this player.

You can adjust the playback speed of the sample either by the `freq` parameter
or the `det` parameter. You can offset into the sample using the `offs`
parameter and modify the playback length relative to the original
sample length using the `len` parameter.

Even though you are advised to use an envelope for controlling the playback
volume of the sample to prevent clicks a simple in and out ramp is provided
using by the `dclick` setting. The length of these ramps can be controlled
using the `dcms` parameter.

When `pmode` is set to **Loop** the sample will restart playing immediately
after it has finished. This is useful when you just want to load a waveform
into the sample player to use it as oscillator.

To start samples when `pmode` is set to **OneShot** a trigger input needs to
be provided on the `trig` input port. The `trig` input also works in
**Loop** mode to retrigger the sample.

#### NodeId::Sampl input freq
Pitch input for the sampler, giving the playback speed of the sample.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |    440.00 |  440.0Hz | `sampl(0).set().freq(440)` | `NodeId::Sampl(0).inp_param("freq")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `sampl(0).set().freq(0.4296875)` | `NodeId::Sampl(0).inp_param("freq")` |
| **mid** | -0.2176 |     97.34 |  97.34Hz | `sampl(0).set().freq(97.33759)` | `NodeId::Sampl(0).inp_param("freq")` |
| **max** |  0.5647 |  22049.99 |  22050Hz | `sampl(0).set().freq(22049.994)` | `NodeId::Sampl(0).inp_param("freq")` |
#### NodeId::Sampl input trig
The trigger input causes a resync of the playback phase and triggers the playback if the `pmode` is **OneShot**

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `sampl(0).set().trig(0)` | `NodeId::Sampl(0).inp_param("trig")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `sampl(0).set().trig(-1)` | `NodeId::Sampl(0).inp_param("trig")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `sampl(0).set().trig(0)` | `NodeId::Sampl(0).inp_param("trig")` |
| **max** |  1.0000 |      1.00 |  1.000 | `sampl(0).set().trig(1)` | `NodeId::Sampl(0).inp_param("trig")` |
#### NodeId::Sampl input offs
Start position offset.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `sampl(0).set().offs(0)` | `NodeId::Sampl(0).inp_param("offs")` |
| **min** |  0.0000 |      0.00 |  0.000 | `sampl(0).set().offs(0)` | `NodeId::Sampl(0).inp_param("offs")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `sampl(0).set().offs(0.5)` | `NodeId::Sampl(0).inp_param("offs")` |
| **max** |  1.0000 |      1.00 |  1.000 | `sampl(0).set().offs(1)` | `NodeId::Sampl(0).inp_param("offs")` |
#### NodeId::Sampl input len
Adjusts the playback length of the sample in relation to the original length of the sample.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `sampl(0).set().len(1)` | `NodeId::Sampl(0).inp_param("len")` |
| **min** |  0.0000 |      0.00 |  0.000 | `sampl(0).set().len(0)` | `NodeId::Sampl(0).inp_param("len")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `sampl(0).set().len(0.5)` | `NodeId::Sampl(0).inp_param("len")` |
| **max** |  1.0000 |      1.00 |  1.000 | `sampl(0).set().len(1)` | `NodeId::Sampl(0).inp_param("len")` |
#### NodeId::Sampl input dcms
Declick fade time in milliseconds.
Not audio rate!

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.2449 |      3.00 |  3.00ms | `sampl(0).set().dcms(3)` | `NodeId::Sampl(0).inp_param("dcms")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `sampl(0).set().dcms(0)` | `NodeId::Sampl(0).inp_param("dcms")` |
| **mid** |  0.5000 |     12.50 | 12.50ms | `sampl(0).set().dcms(12.5)` | `NodeId::Sampl(0).inp_param("dcms")` |
| **max** |  1.0000 |     50.00 | 50.00ms | `sampl(0).set().dcms(50)` | `NodeId::Sampl(0).inp_param("dcms")` |
#### NodeId::Sampl input det
Detune the oscillator in semitones and cents. the input of this value is rounded to semitones on coarse input. Fine input lets you detune in cents (rounded). A signal sent to this port is not rounded.
Note: The signal input allows detune +-10 octaves.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0s | `sampl(0).set().det(0)` | `NodeId::Sampl(0).inp_param("det")` |
| **min** | -0.2000 |    -24.00 | -24s | `sampl(0).set().det(-24)` | `NodeId::Sampl(0).inp_param("det")` |
| **mid** |  0.0000 |      0.00 |  0s | `sampl(0).set().det(0)` | `NodeId::Sampl(0).inp_param("det")` |
| **max** |  0.2000 |     24.00 | 24s | `sampl(0).set().det(24)` | `NodeId::Sampl(0).inp_param("det")` |
#### NodeId::Sampl setting sample
The audio sample that is played back.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 |  0.000 | `sampl(0).set().sample(0)` | `NodeId::Sampl(0).inp_param("sample")` |
#### NodeId::Sampl setting pmode
The playback mode of the sampler.
- **Loop** constantly plays back the sample. You can reset/sync the phase using the `trig` input in this case.
- **OneShot** plays back the sample if a trigger is received on `trig` input.


| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Loop | `sampl(0).set().pmode(0)` | `NodeId::Sampl(0).inp_param("pmode")` |
| 1 | OneShot | `sampl(0).set().pmode(1)` | `NodeId::Sampl(0).inp_param("pmode")` |
#### NodeId::Sampl setting dclick
If this is enabled it will enable short fade in and out ramps.
This if useful if you don't want to add an envelope just for getting rid of the clicks if spos and epos are modulated.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Off | `sampl(0).set().dclick(0)` | `NodeId::Sampl(0).inp_param("dclick")` |
| 1 | On | `sampl(0).set().dclick(1)` | `NodeId::Sampl(0).inp_param("dclick")` |
#### NodeId::Sampl setting dir
Sets the direction of the playhead, plays the sample forwards or backwards.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Forward | `sampl(0).set().dir(0)` | `NodeId::Sampl(0).inp_param("dir")` |
| 1 | Reverse | `sampl(0).set().dir(1)` | `NodeId::Sampl(0).inp_param("dir")` |
### NodeId::Sin
**Sine Oscillator**

This is a very simple oscillator that generates a sine wave.

- [input **freq**](#nodeidsin-input-freq) - Frequency of the oscillator. 
- [input **det**](#nodeidsin-input-det) - Detune the oscillator in semitones and cents. the input of this value is rounded to semitones on coarse input. Fine input lets you detune in cents (rounded). A signal sent to this port is not rounded. Note: The signal input allows detune +-10 octaves.
- [input **pm**](#nodeidsin-input-pm) - Phase modulation input or phase offset. Use this for linear FM/PM modulation. 
- output **sig**
Oscillator signal output. 
#### NodeId::Sin Help
**A Sine Oscillator**

This is a very simple oscillator that generates a sine wave.
The `freq` parameter specifies the frequency, and the `det` parameter
allows you to detune the oscillator easily.

You can send any signal to these input ports. The `det` parameter takes
the same signal range as `freq`, which means, that a value of 0.1 detunes
by one octave. And a value 1.0 detunes by 10 octaves. This means that
for `det` to be usefully modulated you need to attenuate the modulation input.

For linear FM, you can use the `pm` input. It allows you to modulate the phase
of the oscillator linearly. It does so *through zero* which means that the pitch
should not detune by the amount of modulation in low frequencies.

You can do exponential FM with this node using the `det` or `freq` input,
but for easy exponential FM synthesis there might be other nodes available.

#### NodeId::Sin input freq
Frequency of the oscillator.


| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |    440.00 |  440.0Hz | `sin(0).set().freq(440)` | `NodeId::Sin(0).inp_param("freq")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `sin(0).set().freq(0.4296875)` | `NodeId::Sin(0).inp_param("freq")` |
| **mid** | -0.2176 |     97.34 |  97.34Hz | `sin(0).set().freq(97.33759)` | `NodeId::Sin(0).inp_param("freq")` |
| **max** |  0.5647 |  22049.99 |  22050Hz | `sin(0).set().freq(22049.994)` | `NodeId::Sin(0).inp_param("freq")` |
#### NodeId::Sin input det
Detune the oscillator in semitones and cents. the input of this value is rounded to semitones on coarse input. Fine input lets you detune in cents (rounded). A signal sent to this port is not rounded.
Note: The signal input allows detune +-10 octaves.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0s | `sin(0).set().det(0)` | `NodeId::Sin(0).inp_param("det")` |
| **min** | -0.2000 |    -24.00 | -24s | `sin(0).set().det(-24)` | `NodeId::Sin(0).inp_param("det")` |
| **mid** |  0.0000 |      0.00 |  0s | `sin(0).set().det(0)` | `NodeId::Sin(0).inp_param("det")` |
| **max** |  0.2000 |     24.00 | 24s | `sin(0).set().det(24)` | `NodeId::Sin(0).inp_param("det")` |
#### NodeId::Sin input pm
Phase modulation input or phase offset. Use this for linear FM/PM modulation.


| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `sin(0).set().pm(0)` | `NodeId::Sin(0).inp_param("pm")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `sin(0).set().pm(-1)` | `NodeId::Sin(0).inp_param("pm")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `sin(0).set().pm(0)` | `NodeId::Sin(0).inp_param("pm")` |
| **max** |  1.0000 |      1.00 |  1.000 | `sin(0).set().pm(1)` | `NodeId::Sin(0).inp_param("pm")` |
### NodeId::BOsc
**Basic Oscillator**

A very basic band limited oscillator with a sine, triangle, pulse and sawtooth waveform.

- [input **freq**](#nodeidbosc-input-freq) - Base frequency of the oscillator. 
- [input **det**](#nodeidbosc-input-det) - Detune the oscillator in semitones and cents. the input of this value is rounded to semitones on coarse input. Fine input lets you detune in cents (rounded). A signal sent to this port is not rounded. Note: The signal input allows detune +-10 octaves.
- [input **pw**](#nodeidbosc-input-pw) - 
- [setting **wtype**](#nodeidbosc-setting-wtype) - Waveform type. Available waveforms: - **Sin**   - Sine Waveform - **Tri**   - Triangle Waveform - **Saw**   - Sawtooth Waveform - **Pulse** - Pulse Waveform - **Pulse-DC** - Pulse Waveform (DC corrected)
- output **sig**
Oscillator output
#### NodeId::BOsc Help
**Basic Waveform Oscillator**

A very basic band limited oscillator with a sine, triangle, pulse and sawtooth
waveform.  The pulse width `pw` parameter only has an effect for the
**Pulse** waveform.

There are two pulse waveforms: **Pulse** and **Pulse-DC**. Depending on the pulse width
setting of the oscillator the output of the pulse might introduce DC (direct current) into
the signal. The **Pulse-DC** variant compensates that DC component by shifting the signal,
just like a high pass filter would do.

#### NodeId::BOsc input freq
Base frequency of the oscillator.


| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |    440.00 |  440.0Hz | `bosc(0).set().freq(440)` | `NodeId::BOsc(0).inp_param("freq")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `bosc(0).set().freq(0.4296875)` | `NodeId::BOsc(0).inp_param("freq")` |
| **mid** | -0.2176 |     97.34 |  97.34Hz | `bosc(0).set().freq(97.33759)` | `NodeId::BOsc(0).inp_param("freq")` |
| **max** |  0.5647 |  22049.99 |  22050Hz | `bosc(0).set().freq(22049.994)` | `NodeId::BOsc(0).inp_param("freq")` |
#### NodeId::BOsc input det
Detune the oscillator in semitones and cents. the input of this value is rounded to semitones on coarse input. Fine input lets you detune in cents (rounded). A signal sent to this port is not rounded.
Note: The signal input allows detune +-10 octaves.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0s | `bosc(0).set().det(0)` | `NodeId::BOsc(0).inp_param("det")` |
| **min** | -0.2000 |    -24.00 | -24s | `bosc(0).set().det(-24)` | `NodeId::BOsc(0).inp_param("det")` |
| **mid** |  0.0000 |      0.00 |  0s | `bosc(0).set().det(0)` | `NodeId::BOsc(0).inp_param("det")` |
| **max** |  0.2000 |     24.00 | 24s | `bosc(0).set().det(24)` | `NodeId::BOsc(0).inp_param("det")` |
#### NodeId::BOsc input pw


| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `bosc(0).set().pw(0.5)` | `NodeId::BOsc(0).inp_param("pw")` |
| **min** |  0.0000 |      0.00 |  0.000 | `bosc(0).set().pw(0)` | `NodeId::BOsc(0).inp_param("pw")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `bosc(0).set().pw(0.5)` | `NodeId::BOsc(0).inp_param("pw")` |
| **max** |  1.0000 |      1.00 |  1.000 | `bosc(0).set().pw(1)` | `NodeId::BOsc(0).inp_param("pw")` |
#### NodeId::BOsc setting wtype
Waveform type. Available waveforms:
- **Sin**   - Sine Waveform
- **Tri**   - Triangle Waveform
- **Saw**   - Sawtooth Waveform
- **Pulse** - Pulse Waveform
- **Pulse-DC** - Pulse Waveform (DC corrected)

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Sin | `bosc(0).set().wtype(0)` | `NodeId::BOsc(0).inp_param("wtype")` |
| 1 | Tri | `bosc(0).set().wtype(1)` | `NodeId::BOsc(0).inp_param("wtype")` |
| 2 | Saw | `bosc(0).set().wtype(2)` | `NodeId::BOsc(0).inp_param("wtype")` |
| 3 | Pulse | `bosc(0).set().wtype(3)` | `NodeId::BOsc(0).inp_param("wtype")` |
| 4 | Pulse-DC | `bosc(0).set().wtype(4)` | `NodeId::BOsc(0).inp_param("wtype")` |
### NodeId::VOsc
**V Oscillator**

A vector phase shaping oscillator, to create interesting waveforms and ways to manipulate them.
It has two parameters (`v` and `d`) to shape the phase of the sinusoid wave,
and a `vs` parameter to add extra spice.
Distortion can beef up the oscillator output and you can apply oversampling.

- [input **freq**](#nodeidvosc-input-freq) - Base frequency of the oscillator. 
- [input **det**](#nodeidvosc-input-det) - Detune the oscillator in semitones and cents. the input of this value is rounded to semitones on coarse input. Fine input lets you detune in cents (rounded). A signal sent to this port is not rounded. Note: The signal input allows detune +-10 octaves.
- [input **d**](#nodeidvosc-input-d) - This is the horzontal bending point of the waveform. It has a similar effect that pulse width settings have on other oscillators. Make sure to try modulating this parameter at audio rate!
- [input **v**](#nodeidvosc-input-v) - This is the vertical bending point of the waveform. You can adjust the effect that `d` has on the waveform with this parameter. Make sure to try to modulate this parameter at audio rate!
- [input **vs**](#nodeidvosc-input-vs) - Scaling factor for `v`. If you increase this beyond **1.0**, you will hear formant like sounds from the oscillator. Try adjusting `d` to move the formants around.
- [input **damt**](#nodeidvosc-input-damt) - Distortion amount.
- [setting **dist**](#nodeidvosc-setting-dist) - A collection of waveshaper/distortions to choose from.
- [setting **ovrsmpl**](#nodeidvosc-setting-ovrsmpl) - Enable/Disable oversampling.
- output **sig**
Oscillator output
#### NodeId::VOsc Help
**Vector Phase Shaping Oscillator**

A vector phase shaping oscillator, to create interesting waveforms and
ways to manipulate them. It has two parameters (`v` and `d`) to shape the
phase of the sinusoid wave, and a third parameter `vs` to add extra spice.
With distortion you can beef up the oscillator output even more and to
make it more harmonic you can apply oversampling.

#### NodeId::VOsc input freq
Base frequency of the oscillator.


| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |    440.00 |  440.0Hz | `vosc(0).set().freq(440)` | `NodeId::VOsc(0).inp_param("freq")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `vosc(0).set().freq(0.4296875)` | `NodeId::VOsc(0).inp_param("freq")` |
| **mid** | -0.2176 |     97.34 |  97.34Hz | `vosc(0).set().freq(97.33759)` | `NodeId::VOsc(0).inp_param("freq")` |
| **max** |  0.5647 |  22049.99 |  22050Hz | `vosc(0).set().freq(22049.994)` | `NodeId::VOsc(0).inp_param("freq")` |
#### NodeId::VOsc input det
Detune the oscillator in semitones and cents. the input of this value is rounded to semitones on coarse input. Fine input lets you detune in cents (rounded). A signal sent to this port is not rounded.
Note: The signal input allows detune +-10 octaves.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0s | `vosc(0).set().det(0)` | `NodeId::VOsc(0).inp_param("det")` |
| **min** | -0.2000 |    -24.00 | -24s | `vosc(0).set().det(-24)` | `NodeId::VOsc(0).inp_param("det")` |
| **mid** |  0.0000 |      0.00 |  0s | `vosc(0).set().det(0)` | `NodeId::VOsc(0).inp_param("det")` |
| **max** |  0.2000 |     24.00 | 24s | `vosc(0).set().det(24)` | `NodeId::VOsc(0).inp_param("det")` |
#### NodeId::VOsc input d
This is the horzontal bending point of the waveform. It has a similar effect that pulse width settings have on other oscillators. Make sure to try modulating this parameter at audio rate!

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `vosc(0).set().d(0.5)` | `NodeId::VOsc(0).inp_param("d")` |
| **min** |  0.0000 |      0.00 |  0.000 | `vosc(0).set().d(0)` | `NodeId::VOsc(0).inp_param("d")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `vosc(0).set().d(0.5)` | `NodeId::VOsc(0).inp_param("d")` |
| **max** |  1.0000 |      1.00 |  1.000 | `vosc(0).set().d(1)` | `NodeId::VOsc(0).inp_param("d")` |
#### NodeId::VOsc input v
This is the vertical bending point of the waveform. You can adjust the effect that `d` has on the waveform with this parameter. Make sure to try to modulate this parameter at audio rate!

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `vosc(0).set().v(0.5)` | `NodeId::VOsc(0).inp_param("v")` |
| **min** |  0.0000 |      0.00 |  0.000 | `vosc(0).set().v(0)` | `NodeId::VOsc(0).inp_param("v")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `vosc(0).set().v(0.5)` | `NodeId::VOsc(0).inp_param("v")` |
| **max** |  1.0000 |      1.00 |  1.000 | `vosc(0).set().v(1)` | `NodeId::VOsc(0).inp_param("v")` |
#### NodeId::VOsc input vs
Scaling factor for `v`. If you increase this beyond **1.0**, you will hear formant like sounds from the oscillator. Try adjusting `d` to move the formants around.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.0 | `vosc(0).set().vs(0)` | `NodeId::VOsc(0).inp_param("vs")` |
| **min** |  0.0000 |      0.00 |  0.0 | `vosc(0).set().vs(0)` | `NodeId::VOsc(0).inp_param("vs")` |
| **mid** |  0.5000 |     10.00 | 10.0 | `vosc(0).set().vs(10)` | `NodeId::VOsc(0).inp_param("vs")` |
| **max** |  1.0000 |     20.00 | 20.0 | `vosc(0).set().vs(20)` | `NodeId::VOsc(0).inp_param("vs")` |
#### NodeId::VOsc input damt
Distortion amount.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `vosc(0).set().damt(0)` | `NodeId::VOsc(0).inp_param("damt")` |
| **min** |  0.0000 |      0.00 |  0.000 | `vosc(0).set().damt(0)` | `NodeId::VOsc(0).inp_param("damt")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `vosc(0).set().damt(0.5)` | `NodeId::VOsc(0).inp_param("damt")` |
| **max** |  1.0000 |      1.00 |  1.000 | `vosc(0).set().damt(1)` | `NodeId::VOsc(0).inp_param("damt")` |
#### NodeId::VOsc setting dist
A collection of waveshaper/distortions to choose from.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Off | `vosc(0).set().dist(0)` | `NodeId::VOsc(0).inp_param("dist")` |
| 1 | TanH | `vosc(0).set().dist(1)` | `NodeId::VOsc(0).inp_param("dist")` |
| 2 | B.D.Jong | `vosc(0).set().dist(2)` | `NodeId::VOsc(0).inp_param("dist")` |
| 3 | Fold | `vosc(0).set().dist(3)` | `NodeId::VOsc(0).inp_param("dist")` |
#### NodeId::VOsc setting ovrsmpl
Enable/Disable oversampling.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Off | `vosc(0).set().ovrsmpl(0)` | `NodeId::VOsc(0).inp_param("ovrsmpl")` |
| 1 | On | `vosc(0).set().ovrsmpl(1)` | `NodeId::VOsc(0).inp_param("ovrsmpl")` |
### NodeId::BowStri
**Bowed String Oscillator**

This is an oscillator that simulates a bowed string.

- [input **freq**](#nodeidbowstri-input-freq) - Frequency of the bowed string oscillator. 
- [input **det**](#nodeidbowstri-input-det) - Detune the oscillator in semitones and cents. the input of this value is rounded to semitones on coarse input. Fine input lets you detune in cents (rounded). A signal sent to this port is not rounded. Note: The signal input allows detune +-10 octaves.
- [input **vel**](#nodeidbowstri-input-vel) - Velocity of the bow
- [input **force**](#nodeidbowstri-input-force) - Force of the bow
- [input **pos**](#nodeidbowstri-input-pos) - Position of the bow
- output **sig**
Oscillator signal output. 
#### NodeId::BowStri Help
**A Bowed String Simulation Oscillator**

This is an oscillator that simulates a bowed string.
It's a bit wonky, so play around with the parameters and see what
works and what doesn't. It plays find in the area from **~55Hz** up to
**~1760Hz**, beyond that it might not produce a sound.

I can recommend to apply an envelope to the `vel` parameter,
which is basically the bow's velocity.

#### NodeId::BowStri input freq
Frequency of the bowed string oscillator.


| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |    440.00 |  440.0Hz | `bowstri(0).set().freq(440)` | `NodeId::BowStri(0).inp_param("freq")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `bowstri(0).set().freq(0.4296875)` | `NodeId::BowStri(0).inp_param("freq")` |
| **mid** | -0.2176 |     97.34 |  97.34Hz | `bowstri(0).set().freq(97.33759)` | `NodeId::BowStri(0).inp_param("freq")` |
| **max** |  0.5647 |  22049.99 |  22050Hz | `bowstri(0).set().freq(22049.994)` | `NodeId::BowStri(0).inp_param("freq")` |
#### NodeId::BowStri input det
Detune the oscillator in semitones and cents. the input of this value is rounded to semitones on coarse input. Fine input lets you detune in cents (rounded). A signal sent to this port is not rounded.
Note: The signal input allows detune +-10 octaves.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0s | `bowstri(0).set().det(0)` | `NodeId::BowStri(0).inp_param("det")` |
| **min** | -0.2000 |    -24.00 | -24s | `bowstri(0).set().det(-24)` | `NodeId::BowStri(0).inp_param("det")` |
| **mid** |  0.0000 |      0.00 |  0s | `bowstri(0).set().det(0)` | `NodeId::BowStri(0).inp_param("det")` |
| **max** |  0.2000 |     24.00 | 24s | `bowstri(0).set().det(24)` | `NodeId::BowStri(0).inp_param("det")` |
#### NodeId::BowStri input vel
Velocity of the bow

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `bowstri(0).set().vel(0.5)` | `NodeId::BowStri(0).inp_param("vel")` |
| **min** |  0.0000 |      0.00 |  0.000 | `bowstri(0).set().vel(0)` | `NodeId::BowStri(0).inp_param("vel")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `bowstri(0).set().vel(0.5)` | `NodeId::BowStri(0).inp_param("vel")` |
| **max** |  1.0000 |      1.00 |  1.000 | `bowstri(0).set().vel(1)` | `NodeId::BowStri(0).inp_param("vel")` |
#### NodeId::BowStri input force
Force of the bow

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `bowstri(0).set().force(0.5)` | `NodeId::BowStri(0).inp_param("force")` |
| **min** |  0.0000 |      0.00 |  0.000 | `bowstri(0).set().force(0)` | `NodeId::BowStri(0).inp_param("force")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `bowstri(0).set().force(0.5)` | `NodeId::BowStri(0).inp_param("force")` |
| **max** |  1.0000 |      1.00 |  1.000 | `bowstri(0).set().force(1)` | `NodeId::BowStri(0).inp_param("force")` |
#### NodeId::BowStri input pos
Position of the bow

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `bowstri(0).set().pos(0.5)` | `NodeId::BowStri(0).inp_param("pos")` |
| **min** |  0.0000 |      0.00 |  0.000 | `bowstri(0).set().pos(0)` | `NodeId::BowStri(0).inp_param("pos")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `bowstri(0).set().pos(0.5)` | `NodeId::BowStri(0).inp_param("pos")` |
| **max** |  1.0000 |      1.00 |  1.000 | `bowstri(0).set().pos(1)` | `NodeId::BowStri(0).inp_param("pos")` |
### NodeId::MidiP
**MIDI Pitch/Note Input**

This node is an input of MIDI note events into the DSP graph. You get 3 outputs: frequency of the note, gate signal for the length of the note and the velocity.
- [input **det**](#nodeidmidip-input-det) - Detune input pitch a bit
- [input **glen**](#nodeidmidip-input-glen) - MIDI gate length If `gmode` is set to **Gate Len** this controls and overrides the gate length on a MIDI note event. **Trigger** will just send a short trigger when a note event is received. **MIDI** means the gate reflects the note on/off duration.
- [setting **chan**](#nodeidmidip-setting-chan) - MIDI Channel 0 to 15 
- [setting **gmode**](#nodeidmidip-setting-gmode) - MIDI gate mode. - **MIDI** gate same as MIDI input - **Trigger** output only triggers on `gate` output - **Gate Len** output gate with the length of the `glen` parameter 
- output **freq**
MIDI note frequency, detuned by `det`.
- output **gate**
MIDI note gate
- output **vel**
MIDI note velocity
#### NodeId::MidiP Help
**MIDI Pitch/Note Input**

This node is an input of MIDI note events into the DSP graph.
You get 3 outputs: frequency of the note, gate signal for the length of
the note and the velocity.

You can modify the gate length using the `gmode` and `glen` settings.
Setting `gmode` to **Trigger** allows you to get only a short trigger
signal, which might be helpful in some situations.
The **Gate Len** setting allows you to overwrite the gate length with a
custom and fixed gate length. However, if new note is played on this
MIDI channel, the gate will restart after a very short pause.

#### NodeId::MidiP input det
Detune input pitch a bit

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0s | `midip(0).set().det(0)` | `NodeId::MidiP(0).inp_param("det")` |
| **min** | -0.2000 |    -24.00 | -24s | `midip(0).set().det(-24)` | `NodeId::MidiP(0).inp_param("det")` |
| **mid** |  0.0000 |      0.00 |  0s | `midip(0).set().det(0)` | `NodeId::MidiP(0).inp_param("det")` |
| **max** |  0.2000 |     24.00 | 24s | `midip(0).set().det(24)` | `NodeId::MidiP(0).inp_param("det")` |
#### NodeId::MidiP input glen
MIDI gate length
If `gmode` is set to **Gate Len** this controls and overrides the gate length on a MIDI note event. **Trigger** will just send a short trigger when a note event is received. **MIDI** means the gate reflects the note on/off duration.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.3067 |    250.00 |  250ms | `midip(0).set().glen(250)` | `NodeId::MidiP(0).inp_param("glen")` |
| **min** |  0.0000 |      0.10 | 0.100ms | `midip(0).set().glen(0.1)` | `NodeId::MidiP(0).inp_param("glen")` |
| **mid** |  0.5000 |   4687.60 |  4.69s | `midip(0).set().glen(4687.5986)` | `NodeId::MidiP(0).inp_param("glen")` |
| **max** |  1.0000 | 300000.00 | 300.0s | `midip(0).set().glen(300000)` | `NodeId::MidiP(0).inp_param("glen")` |
#### NodeId::MidiP setting chan
MIDI Channel 0 to 15


| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | 0 | `midip(0).set().chan(0)` | `NodeId::MidiP(0).inp_param("chan")` |
| 1 | 1 | `midip(0).set().chan(1)` | `NodeId::MidiP(0).inp_param("chan")` |
| 2 | 2 | `midip(0).set().chan(2)` | `NodeId::MidiP(0).inp_param("chan")` |
| 3 | 3 | `midip(0).set().chan(3)` | `NodeId::MidiP(0).inp_param("chan")` |
| 4 | 4 | `midip(0).set().chan(4)` | `NodeId::MidiP(0).inp_param("chan")` |
| 5 | 5 | `midip(0).set().chan(5)` | `NodeId::MidiP(0).inp_param("chan")` |
| 6 | 6 | `midip(0).set().chan(6)` | `NodeId::MidiP(0).inp_param("chan")` |
| 7 | 7 | `midip(0).set().chan(7)` | `NodeId::MidiP(0).inp_param("chan")` |
| 8 | 8 | `midip(0).set().chan(8)` | `NodeId::MidiP(0).inp_param("chan")` |
| 9 | 9 | `midip(0).set().chan(9)` | `NodeId::MidiP(0).inp_param("chan")` |
| 10 | 10 | `midip(0).set().chan(10)` | `NodeId::MidiP(0).inp_param("chan")` |
| 11 | 11 | `midip(0).set().chan(11)` | `NodeId::MidiP(0).inp_param("chan")` |
| 12 | 12 | `midip(0).set().chan(12)` | `NodeId::MidiP(0).inp_param("chan")` |
| 13 | 13 | `midip(0).set().chan(13)` | `NodeId::MidiP(0).inp_param("chan")` |
| 14 | 14 | `midip(0).set().chan(14)` | `NodeId::MidiP(0).inp_param("chan")` |
| 15 | 15 | `midip(0).set().chan(15)` | `NodeId::MidiP(0).inp_param("chan")` |
| 16 | 16 | `midip(0).set().chan(16)` | `NodeId::MidiP(0).inp_param("chan")` |
#### NodeId::MidiP setting gmode
MIDI gate mode.
- **MIDI** gate same as MIDI input
- **Trigger** output only triggers on `gate` output
- **Gate Len** output gate with the length of the `glen` parameter


| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | MIDI | `midip(0).set().gmode(0)` | `NodeId::MidiP(0).inp_param("gmode")` |
| 1 | Trigger | `midip(0).set().gmode(1)` | `NodeId::MidiP(0).inp_param("gmode")` |
| 2 | Gate Len | `midip(0).set().gmode(2)` | `NodeId::MidiP(0).inp_param("gmode")` |
### NodeId::MidiCC
**MIDI CC Input**

This node is an input of MIDI CC events/values into the DSP graph. You get 3 CC value outputs: `sig1`, `sig2` and `sig3`. To set which CC gets which output you have to set the corresponding `cc1`, `cc2` and `cc3` parameters.
- [input **slew**](#nodeidmidicc-input-slew) - Slew limiter for the 3 CCs
- [setting **chan**](#nodeidmidicc-setting-chan) - MIDI Channel 0 to 15 
- [setting **cc1**](#nodeidmidicc-setting-cc1) - MIDI selected CC 1
- [setting **cc2**](#nodeidmidicc-setting-cc2) - MIDI selected CC 2
- [setting **cc3**](#nodeidmidicc-setting-cc3) - MIDI selected CC 3
- output **sig1**
CC output channel 1
- output **sig2**
CC output channel 2
- output **sig3**
CC output channel 3
#### NodeId::MidiCC Help
**MIDI CC Input**

This node is an input of MIDI CC events/values into the DSP graph.
You get 3 CC value outputs: `sig1`, `sig2` and `sig3`. To set which CC
gets which output you have to set the corresponding `cc1`, `cc2` and
`cc3` parameters.

If the CC values change to rapidly or you hear audible artifacts, you can
try to limit the speed of change with the `slew` limiter.

If you need different `slew` values for the CCs, I recommend creating other
`MidiCC` instances with different `slew` settings.

#### NodeId::MidiCC input slew
Slew limiter for the 3 CCs

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.00ms | `midicc(0).set().slew(0)` | `NodeId::MidiCC(0).inp_param("slew")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `midicc(0).set().slew(0)` | `NodeId::MidiCC(0).inp_param("slew")` |
| **mid** |  0.5000 |   1250.00 |   1250ms | `midicc(0).set().slew(1250)` | `NodeId::MidiCC(0).inp_param("slew")` |
| **max** |  1.0000 |   5000.00 |   5000ms | `midicc(0).set().slew(5000)` | `NodeId::MidiCC(0).inp_param("slew")` |
#### NodeId::MidiCC setting chan
MIDI Channel 0 to 15


| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | 0 | `midicc(0).set().chan(0)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 1 | 1 | `midicc(0).set().chan(1)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 2 | 2 | `midicc(0).set().chan(2)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 3 | 3 | `midicc(0).set().chan(3)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 4 | 4 | `midicc(0).set().chan(4)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 5 | 5 | `midicc(0).set().chan(5)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 6 | 6 | `midicc(0).set().chan(6)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 7 | 7 | `midicc(0).set().chan(7)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 8 | 8 | `midicc(0).set().chan(8)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 9 | 9 | `midicc(0).set().chan(9)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 10 | 10 | `midicc(0).set().chan(10)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 11 | 11 | `midicc(0).set().chan(11)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 12 | 12 | `midicc(0).set().chan(12)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 13 | 13 | `midicc(0).set().chan(13)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 14 | 14 | `midicc(0).set().chan(14)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 15 | 15 | `midicc(0).set().chan(15)` | `NodeId::MidiCC(0).inp_param("chan")` |
| 16 | 16 | `midicc(0).set().chan(16)` | `NodeId::MidiCC(0).inp_param("chan")` |
#### NodeId::MidiCC setting cc1
MIDI selected CC 1

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | 0 | `midicc(0).set().cc1(0)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 1 | 1 | `midicc(0).set().cc1(1)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 2 | 2 | `midicc(0).set().cc1(2)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 3 | 3 | `midicc(0).set().cc1(3)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 4 | 4 | `midicc(0).set().cc1(4)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 5 | 5 | `midicc(0).set().cc1(5)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 6 | 6 | `midicc(0).set().cc1(6)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 7 | 7 | `midicc(0).set().cc1(7)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 8 | 8 | `midicc(0).set().cc1(8)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 9 | 9 | `midicc(0).set().cc1(9)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 10 | 10 | `midicc(0).set().cc1(10)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 11 | 11 | `midicc(0).set().cc1(11)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 12 | 12 | `midicc(0).set().cc1(12)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 13 | 13 | `midicc(0).set().cc1(13)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 14 | 14 | `midicc(0).set().cc1(14)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 15 | 15 | `midicc(0).set().cc1(15)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 16 | 16 | `midicc(0).set().cc1(16)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 17 | 17 | `midicc(0).set().cc1(17)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 18 | 18 | `midicc(0).set().cc1(18)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 19 | 19 | `midicc(0).set().cc1(19)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 20 | 20 | `midicc(0).set().cc1(20)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 21 | 21 | `midicc(0).set().cc1(21)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 22 | 22 | `midicc(0).set().cc1(22)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 23 | 23 | `midicc(0).set().cc1(23)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 24 | 24 | `midicc(0).set().cc1(24)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 25 | 25 | `midicc(0).set().cc1(25)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 26 | 26 | `midicc(0).set().cc1(26)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 27 | 27 | `midicc(0).set().cc1(27)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 28 | 28 | `midicc(0).set().cc1(28)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 29 | 29 | `midicc(0).set().cc1(29)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 30 | 30 | `midicc(0).set().cc1(30)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 31 | 31 | `midicc(0).set().cc1(31)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 32 | 32 | `midicc(0).set().cc1(32)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 33 | 33 | `midicc(0).set().cc1(33)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 34 | 34 | `midicc(0).set().cc1(34)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 35 | 35 | `midicc(0).set().cc1(35)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 36 | 36 | `midicc(0).set().cc1(36)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 37 | 37 | `midicc(0).set().cc1(37)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 38 | 38 | `midicc(0).set().cc1(38)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 39 | 39 | `midicc(0).set().cc1(39)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 40 | 40 | `midicc(0).set().cc1(40)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 41 | 41 | `midicc(0).set().cc1(41)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 42 | 42 | `midicc(0).set().cc1(42)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 43 | 43 | `midicc(0).set().cc1(43)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 44 | 44 | `midicc(0).set().cc1(44)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 45 | 45 | `midicc(0).set().cc1(45)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 46 | 46 | `midicc(0).set().cc1(46)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 47 | 47 | `midicc(0).set().cc1(47)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 48 | 48 | `midicc(0).set().cc1(48)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 49 | 49 | `midicc(0).set().cc1(49)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 50 | 50 | `midicc(0).set().cc1(50)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 51 | 51 | `midicc(0).set().cc1(51)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 52 | 52 | `midicc(0).set().cc1(52)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 53 | 53 | `midicc(0).set().cc1(53)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 54 | 54 | `midicc(0).set().cc1(54)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 55 | 55 | `midicc(0).set().cc1(55)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 56 | 56 | `midicc(0).set().cc1(56)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 57 | 57 | `midicc(0).set().cc1(57)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 58 | 58 | `midicc(0).set().cc1(58)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 59 | 59 | `midicc(0).set().cc1(59)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 60 | 60 | `midicc(0).set().cc1(60)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 61 | 61 | `midicc(0).set().cc1(61)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 62 | 62 | `midicc(0).set().cc1(62)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 63 | 63 | `midicc(0).set().cc1(63)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 64 | 64 | `midicc(0).set().cc1(64)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 65 | 65 | `midicc(0).set().cc1(65)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 66 | 66 | `midicc(0).set().cc1(66)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 67 | 67 | `midicc(0).set().cc1(67)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 68 | 68 | `midicc(0).set().cc1(68)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 69 | 69 | `midicc(0).set().cc1(69)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 70 | 70 | `midicc(0).set().cc1(70)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 71 | 71 | `midicc(0).set().cc1(71)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 72 | 72 | `midicc(0).set().cc1(72)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 73 | 73 | `midicc(0).set().cc1(73)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 74 | 74 | `midicc(0).set().cc1(74)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 75 | 75 | `midicc(0).set().cc1(75)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 76 | 76 | `midicc(0).set().cc1(76)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 77 | 77 | `midicc(0).set().cc1(77)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 78 | 78 | `midicc(0).set().cc1(78)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 79 | 79 | `midicc(0).set().cc1(79)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 80 | 80 | `midicc(0).set().cc1(80)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 81 | 81 | `midicc(0).set().cc1(81)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 82 | 82 | `midicc(0).set().cc1(82)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 83 | 83 | `midicc(0).set().cc1(83)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 84 | 84 | `midicc(0).set().cc1(84)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 85 | 85 | `midicc(0).set().cc1(85)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 86 | 86 | `midicc(0).set().cc1(86)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 87 | 87 | `midicc(0).set().cc1(87)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 88 | 88 | `midicc(0).set().cc1(88)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 89 | 89 | `midicc(0).set().cc1(89)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 90 | 90 | `midicc(0).set().cc1(90)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 91 | 91 | `midicc(0).set().cc1(91)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 92 | 92 | `midicc(0).set().cc1(92)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 93 | 93 | `midicc(0).set().cc1(93)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 94 | 94 | `midicc(0).set().cc1(94)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 95 | 95 | `midicc(0).set().cc1(95)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 96 | 96 | `midicc(0).set().cc1(96)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 97 | 97 | `midicc(0).set().cc1(97)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 98 | 98 | `midicc(0).set().cc1(98)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 99 | 99 | `midicc(0).set().cc1(99)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 100 | 100 | `midicc(0).set().cc1(100)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 101 | 101 | `midicc(0).set().cc1(101)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 102 | 102 | `midicc(0).set().cc1(102)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 103 | 103 | `midicc(0).set().cc1(103)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 104 | 104 | `midicc(0).set().cc1(104)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 105 | 105 | `midicc(0).set().cc1(105)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 106 | 106 | `midicc(0).set().cc1(106)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 107 | 107 | `midicc(0).set().cc1(107)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 108 | 108 | `midicc(0).set().cc1(108)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 109 | 109 | `midicc(0).set().cc1(109)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 110 | 110 | `midicc(0).set().cc1(110)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 111 | 111 | `midicc(0).set().cc1(111)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 112 | 112 | `midicc(0).set().cc1(112)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 113 | 113 | `midicc(0).set().cc1(113)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 114 | 114 | `midicc(0).set().cc1(114)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 115 | 115 | `midicc(0).set().cc1(115)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 116 | 116 | `midicc(0).set().cc1(116)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 117 | 117 | `midicc(0).set().cc1(117)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 118 | 118 | `midicc(0).set().cc1(118)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 119 | 119 | `midicc(0).set().cc1(119)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 120 | 120 | `midicc(0).set().cc1(120)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 121 | 121 | `midicc(0).set().cc1(121)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 122 | 122 | `midicc(0).set().cc1(122)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 123 | 123 | `midicc(0).set().cc1(123)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 124 | 124 | `midicc(0).set().cc1(124)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 125 | 125 | `midicc(0).set().cc1(125)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 126 | 126 | `midicc(0).set().cc1(126)` | `NodeId::MidiCC(0).inp_param("cc1")` |
| 127 | 127 | `midicc(0).set().cc1(127)` | `NodeId::MidiCC(0).inp_param("cc1")` |
#### NodeId::MidiCC setting cc2
MIDI selected CC 2

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | 0 | `midicc(0).set().cc2(0)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 1 | 1 | `midicc(0).set().cc2(1)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 2 | 2 | `midicc(0).set().cc2(2)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 3 | 3 | `midicc(0).set().cc2(3)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 4 | 4 | `midicc(0).set().cc2(4)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 5 | 5 | `midicc(0).set().cc2(5)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 6 | 6 | `midicc(0).set().cc2(6)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 7 | 7 | `midicc(0).set().cc2(7)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 8 | 8 | `midicc(0).set().cc2(8)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 9 | 9 | `midicc(0).set().cc2(9)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 10 | 10 | `midicc(0).set().cc2(10)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 11 | 11 | `midicc(0).set().cc2(11)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 12 | 12 | `midicc(0).set().cc2(12)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 13 | 13 | `midicc(0).set().cc2(13)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 14 | 14 | `midicc(0).set().cc2(14)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 15 | 15 | `midicc(0).set().cc2(15)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 16 | 16 | `midicc(0).set().cc2(16)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 17 | 17 | `midicc(0).set().cc2(17)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 18 | 18 | `midicc(0).set().cc2(18)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 19 | 19 | `midicc(0).set().cc2(19)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 20 | 20 | `midicc(0).set().cc2(20)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 21 | 21 | `midicc(0).set().cc2(21)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 22 | 22 | `midicc(0).set().cc2(22)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 23 | 23 | `midicc(0).set().cc2(23)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 24 | 24 | `midicc(0).set().cc2(24)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 25 | 25 | `midicc(0).set().cc2(25)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 26 | 26 | `midicc(0).set().cc2(26)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 27 | 27 | `midicc(0).set().cc2(27)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 28 | 28 | `midicc(0).set().cc2(28)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 29 | 29 | `midicc(0).set().cc2(29)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 30 | 30 | `midicc(0).set().cc2(30)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 31 | 31 | `midicc(0).set().cc2(31)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 32 | 32 | `midicc(0).set().cc2(32)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 33 | 33 | `midicc(0).set().cc2(33)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 34 | 34 | `midicc(0).set().cc2(34)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 35 | 35 | `midicc(0).set().cc2(35)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 36 | 36 | `midicc(0).set().cc2(36)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 37 | 37 | `midicc(0).set().cc2(37)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 38 | 38 | `midicc(0).set().cc2(38)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 39 | 39 | `midicc(0).set().cc2(39)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 40 | 40 | `midicc(0).set().cc2(40)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 41 | 41 | `midicc(0).set().cc2(41)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 42 | 42 | `midicc(0).set().cc2(42)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 43 | 43 | `midicc(0).set().cc2(43)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 44 | 44 | `midicc(0).set().cc2(44)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 45 | 45 | `midicc(0).set().cc2(45)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 46 | 46 | `midicc(0).set().cc2(46)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 47 | 47 | `midicc(0).set().cc2(47)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 48 | 48 | `midicc(0).set().cc2(48)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 49 | 49 | `midicc(0).set().cc2(49)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 50 | 50 | `midicc(0).set().cc2(50)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 51 | 51 | `midicc(0).set().cc2(51)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 52 | 52 | `midicc(0).set().cc2(52)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 53 | 53 | `midicc(0).set().cc2(53)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 54 | 54 | `midicc(0).set().cc2(54)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 55 | 55 | `midicc(0).set().cc2(55)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 56 | 56 | `midicc(0).set().cc2(56)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 57 | 57 | `midicc(0).set().cc2(57)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 58 | 58 | `midicc(0).set().cc2(58)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 59 | 59 | `midicc(0).set().cc2(59)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 60 | 60 | `midicc(0).set().cc2(60)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 61 | 61 | `midicc(0).set().cc2(61)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 62 | 62 | `midicc(0).set().cc2(62)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 63 | 63 | `midicc(0).set().cc2(63)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 64 | 64 | `midicc(0).set().cc2(64)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 65 | 65 | `midicc(0).set().cc2(65)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 66 | 66 | `midicc(0).set().cc2(66)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 67 | 67 | `midicc(0).set().cc2(67)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 68 | 68 | `midicc(0).set().cc2(68)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 69 | 69 | `midicc(0).set().cc2(69)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 70 | 70 | `midicc(0).set().cc2(70)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 71 | 71 | `midicc(0).set().cc2(71)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 72 | 72 | `midicc(0).set().cc2(72)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 73 | 73 | `midicc(0).set().cc2(73)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 74 | 74 | `midicc(0).set().cc2(74)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 75 | 75 | `midicc(0).set().cc2(75)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 76 | 76 | `midicc(0).set().cc2(76)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 77 | 77 | `midicc(0).set().cc2(77)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 78 | 78 | `midicc(0).set().cc2(78)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 79 | 79 | `midicc(0).set().cc2(79)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 80 | 80 | `midicc(0).set().cc2(80)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 81 | 81 | `midicc(0).set().cc2(81)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 82 | 82 | `midicc(0).set().cc2(82)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 83 | 83 | `midicc(0).set().cc2(83)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 84 | 84 | `midicc(0).set().cc2(84)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 85 | 85 | `midicc(0).set().cc2(85)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 86 | 86 | `midicc(0).set().cc2(86)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 87 | 87 | `midicc(0).set().cc2(87)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 88 | 88 | `midicc(0).set().cc2(88)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 89 | 89 | `midicc(0).set().cc2(89)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 90 | 90 | `midicc(0).set().cc2(90)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 91 | 91 | `midicc(0).set().cc2(91)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 92 | 92 | `midicc(0).set().cc2(92)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 93 | 93 | `midicc(0).set().cc2(93)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 94 | 94 | `midicc(0).set().cc2(94)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 95 | 95 | `midicc(0).set().cc2(95)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 96 | 96 | `midicc(0).set().cc2(96)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 97 | 97 | `midicc(0).set().cc2(97)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 98 | 98 | `midicc(0).set().cc2(98)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 99 | 99 | `midicc(0).set().cc2(99)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 100 | 100 | `midicc(0).set().cc2(100)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 101 | 101 | `midicc(0).set().cc2(101)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 102 | 102 | `midicc(0).set().cc2(102)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 103 | 103 | `midicc(0).set().cc2(103)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 104 | 104 | `midicc(0).set().cc2(104)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 105 | 105 | `midicc(0).set().cc2(105)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 106 | 106 | `midicc(0).set().cc2(106)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 107 | 107 | `midicc(0).set().cc2(107)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 108 | 108 | `midicc(0).set().cc2(108)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 109 | 109 | `midicc(0).set().cc2(109)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 110 | 110 | `midicc(0).set().cc2(110)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 111 | 111 | `midicc(0).set().cc2(111)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 112 | 112 | `midicc(0).set().cc2(112)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 113 | 113 | `midicc(0).set().cc2(113)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 114 | 114 | `midicc(0).set().cc2(114)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 115 | 115 | `midicc(0).set().cc2(115)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 116 | 116 | `midicc(0).set().cc2(116)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 117 | 117 | `midicc(0).set().cc2(117)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 118 | 118 | `midicc(0).set().cc2(118)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 119 | 119 | `midicc(0).set().cc2(119)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 120 | 120 | `midicc(0).set().cc2(120)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 121 | 121 | `midicc(0).set().cc2(121)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 122 | 122 | `midicc(0).set().cc2(122)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 123 | 123 | `midicc(0).set().cc2(123)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 124 | 124 | `midicc(0).set().cc2(124)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 125 | 125 | `midicc(0).set().cc2(125)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 126 | 126 | `midicc(0).set().cc2(126)` | `NodeId::MidiCC(0).inp_param("cc2")` |
| 127 | 127 | `midicc(0).set().cc2(127)` | `NodeId::MidiCC(0).inp_param("cc2")` |
#### NodeId::MidiCC setting cc3
MIDI selected CC 3

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | 0 | `midicc(0).set().cc3(0)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 1 | 1 | `midicc(0).set().cc3(1)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 2 | 2 | `midicc(0).set().cc3(2)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 3 | 3 | `midicc(0).set().cc3(3)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 4 | 4 | `midicc(0).set().cc3(4)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 5 | 5 | `midicc(0).set().cc3(5)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 6 | 6 | `midicc(0).set().cc3(6)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 7 | 7 | `midicc(0).set().cc3(7)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 8 | 8 | `midicc(0).set().cc3(8)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 9 | 9 | `midicc(0).set().cc3(9)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 10 | 10 | `midicc(0).set().cc3(10)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 11 | 11 | `midicc(0).set().cc3(11)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 12 | 12 | `midicc(0).set().cc3(12)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 13 | 13 | `midicc(0).set().cc3(13)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 14 | 14 | `midicc(0).set().cc3(14)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 15 | 15 | `midicc(0).set().cc3(15)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 16 | 16 | `midicc(0).set().cc3(16)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 17 | 17 | `midicc(0).set().cc3(17)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 18 | 18 | `midicc(0).set().cc3(18)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 19 | 19 | `midicc(0).set().cc3(19)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 20 | 20 | `midicc(0).set().cc3(20)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 21 | 21 | `midicc(0).set().cc3(21)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 22 | 22 | `midicc(0).set().cc3(22)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 23 | 23 | `midicc(0).set().cc3(23)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 24 | 24 | `midicc(0).set().cc3(24)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 25 | 25 | `midicc(0).set().cc3(25)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 26 | 26 | `midicc(0).set().cc3(26)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 27 | 27 | `midicc(0).set().cc3(27)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 28 | 28 | `midicc(0).set().cc3(28)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 29 | 29 | `midicc(0).set().cc3(29)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 30 | 30 | `midicc(0).set().cc3(30)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 31 | 31 | `midicc(0).set().cc3(31)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 32 | 32 | `midicc(0).set().cc3(32)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 33 | 33 | `midicc(0).set().cc3(33)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 34 | 34 | `midicc(0).set().cc3(34)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 35 | 35 | `midicc(0).set().cc3(35)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 36 | 36 | `midicc(0).set().cc3(36)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 37 | 37 | `midicc(0).set().cc3(37)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 38 | 38 | `midicc(0).set().cc3(38)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 39 | 39 | `midicc(0).set().cc3(39)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 40 | 40 | `midicc(0).set().cc3(40)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 41 | 41 | `midicc(0).set().cc3(41)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 42 | 42 | `midicc(0).set().cc3(42)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 43 | 43 | `midicc(0).set().cc3(43)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 44 | 44 | `midicc(0).set().cc3(44)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 45 | 45 | `midicc(0).set().cc3(45)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 46 | 46 | `midicc(0).set().cc3(46)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 47 | 47 | `midicc(0).set().cc3(47)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 48 | 48 | `midicc(0).set().cc3(48)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 49 | 49 | `midicc(0).set().cc3(49)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 50 | 50 | `midicc(0).set().cc3(50)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 51 | 51 | `midicc(0).set().cc3(51)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 52 | 52 | `midicc(0).set().cc3(52)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 53 | 53 | `midicc(0).set().cc3(53)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 54 | 54 | `midicc(0).set().cc3(54)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 55 | 55 | `midicc(0).set().cc3(55)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 56 | 56 | `midicc(0).set().cc3(56)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 57 | 57 | `midicc(0).set().cc3(57)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 58 | 58 | `midicc(0).set().cc3(58)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 59 | 59 | `midicc(0).set().cc3(59)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 60 | 60 | `midicc(0).set().cc3(60)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 61 | 61 | `midicc(0).set().cc3(61)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 62 | 62 | `midicc(0).set().cc3(62)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 63 | 63 | `midicc(0).set().cc3(63)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 64 | 64 | `midicc(0).set().cc3(64)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 65 | 65 | `midicc(0).set().cc3(65)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 66 | 66 | `midicc(0).set().cc3(66)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 67 | 67 | `midicc(0).set().cc3(67)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 68 | 68 | `midicc(0).set().cc3(68)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 69 | 69 | `midicc(0).set().cc3(69)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 70 | 70 | `midicc(0).set().cc3(70)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 71 | 71 | `midicc(0).set().cc3(71)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 72 | 72 | `midicc(0).set().cc3(72)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 73 | 73 | `midicc(0).set().cc3(73)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 74 | 74 | `midicc(0).set().cc3(74)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 75 | 75 | `midicc(0).set().cc3(75)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 76 | 76 | `midicc(0).set().cc3(76)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 77 | 77 | `midicc(0).set().cc3(77)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 78 | 78 | `midicc(0).set().cc3(78)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 79 | 79 | `midicc(0).set().cc3(79)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 80 | 80 | `midicc(0).set().cc3(80)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 81 | 81 | `midicc(0).set().cc3(81)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 82 | 82 | `midicc(0).set().cc3(82)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 83 | 83 | `midicc(0).set().cc3(83)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 84 | 84 | `midicc(0).set().cc3(84)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 85 | 85 | `midicc(0).set().cc3(85)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 86 | 86 | `midicc(0).set().cc3(86)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 87 | 87 | `midicc(0).set().cc3(87)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 88 | 88 | `midicc(0).set().cc3(88)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 89 | 89 | `midicc(0).set().cc3(89)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 90 | 90 | `midicc(0).set().cc3(90)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 91 | 91 | `midicc(0).set().cc3(91)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 92 | 92 | `midicc(0).set().cc3(92)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 93 | 93 | `midicc(0).set().cc3(93)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 94 | 94 | `midicc(0).set().cc3(94)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 95 | 95 | `midicc(0).set().cc3(95)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 96 | 96 | `midicc(0).set().cc3(96)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 97 | 97 | `midicc(0).set().cc3(97)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 98 | 98 | `midicc(0).set().cc3(98)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 99 | 99 | `midicc(0).set().cc3(99)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 100 | 100 | `midicc(0).set().cc3(100)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 101 | 101 | `midicc(0).set().cc3(101)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 102 | 102 | `midicc(0).set().cc3(102)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 103 | 103 | `midicc(0).set().cc3(103)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 104 | 104 | `midicc(0).set().cc3(104)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 105 | 105 | `midicc(0).set().cc3(105)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 106 | 106 | `midicc(0).set().cc3(106)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 107 | 107 | `midicc(0).set().cc3(107)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 108 | 108 | `midicc(0).set().cc3(108)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 109 | 109 | `midicc(0).set().cc3(109)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 110 | 110 | `midicc(0).set().cc3(110)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 111 | 111 | `midicc(0).set().cc3(111)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 112 | 112 | `midicc(0).set().cc3(112)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 113 | 113 | `midicc(0).set().cc3(113)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 114 | 114 | `midicc(0).set().cc3(114)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 115 | 115 | `midicc(0).set().cc3(115)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 116 | 116 | `midicc(0).set().cc3(116)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 117 | 117 | `midicc(0).set().cc3(117)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 118 | 118 | `midicc(0).set().cc3(118)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 119 | 119 | `midicc(0).set().cc3(119)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 120 | 120 | `midicc(0).set().cc3(120)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 121 | 121 | `midicc(0).set().cc3(121)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 122 | 122 | `midicc(0).set().cc3(122)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 123 | 123 | `midicc(0).set().cc3(123)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 124 | 124 | `midicc(0).set().cc3(124)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 125 | 125 | `midicc(0).set().cc3(125)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 126 | 126 | `midicc(0).set().cc3(126)` | `NodeId::MidiCC(0).inp_param("cc3")` |
| 127 | 127 | `midicc(0).set().cc3(127)` | `NodeId::MidiCC(0).inp_param("cc3")` |
### NodeId::ExtA
**Ext. Param. Set A-F Input**

This node gives access to the 24 input parameters of the HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick changes a bit if you need it. Attenuverters (attenuators that can also invert) allow to reduce the amplitude or invert the signal.
- [input **slew**](#nodeidexta-input-slew) - Slew limiter for the 3 parameters
- [input **atv1**](#nodeidexta-input-atv1) - Attenuverter for the A1 parameter
- [input **atv2**](#nodeidexta-input-atv2) - Attenuverter for the A2 parameter
- [input **atv3**](#nodeidexta-input-atv3) - Attenuverter for the A3 parameter
- output **sig1**
A-F1 output channel
- output **sig2**
A-F2 output channel
- output **sig3**
A-F3 output channel
#### NodeId::ExtA Help
**External Parameter Set A-F Input**

This node gives access to the 24 input parameters of the
HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick
changes a bit if you need it. Attenuverters (attenuators that can also invert)
allow to reduce the amplitude or invert the signal.

All instances of the nodes `ExtA`, `ExtB`, ..., `ExtF` have access to the same
3 input parameters (`A1`-`A3`, `B1`-`B3`, ..., `F1`-`F3`). That means there is no
difference whether you use the same instance of different ones.
Except that you can of course set the `atv` and `slew` parameters to different
values.

If you absolutely need more parameters to control the HexoSynth patch:
Keep in mind, that there is also the `MidiCC` node, that allows HexoSynth to
react to MIDI CC messages.

#### NodeId::ExtA input slew
Slew limiter for the 3 parameters

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.00ms | `exta(0).set().slew(0)` | `NodeId::ExtA(0).inp_param("slew")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `exta(0).set().slew(0)` | `NodeId::ExtA(0).inp_param("slew")` |
| **mid** |  0.5000 |   1250.00 |   1250ms | `exta(0).set().slew(1250)` | `NodeId::ExtA(0).inp_param("slew")` |
| **max** |  1.0000 |   5000.00 |   5000ms | `exta(0).set().slew(5000)` | `NodeId::ExtA(0).inp_param("slew")` |
#### NodeId::ExtA input atv1
Attenuverter for the A1 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `exta(0).set().atv1(1)` | `NodeId::ExtA(0).inp_param("atv1")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `exta(0).set().atv1(-1)` | `NodeId::ExtA(0).inp_param("atv1")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `exta(0).set().atv1(0)` | `NodeId::ExtA(0).inp_param("atv1")` |
| **max** |  1.0000 |      1.00 |  1.000 | `exta(0).set().atv1(1)` | `NodeId::ExtA(0).inp_param("atv1")` |
#### NodeId::ExtA input atv2
Attenuverter for the A2 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `exta(0).set().atv2(1)` | `NodeId::ExtA(0).inp_param("atv2")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `exta(0).set().atv2(-1)` | `NodeId::ExtA(0).inp_param("atv2")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `exta(0).set().atv2(0)` | `NodeId::ExtA(0).inp_param("atv2")` |
| **max** |  1.0000 |      1.00 |  1.000 | `exta(0).set().atv2(1)` | `NodeId::ExtA(0).inp_param("atv2")` |
#### NodeId::ExtA input atv3
Attenuverter for the A3 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `exta(0).set().atv3(1)` | `NodeId::ExtA(0).inp_param("atv3")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `exta(0).set().atv3(-1)` | `NodeId::ExtA(0).inp_param("atv3")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `exta(0).set().atv3(0)` | `NodeId::ExtA(0).inp_param("atv3")` |
| **max** |  1.0000 |      1.00 |  1.000 | `exta(0).set().atv3(1)` | `NodeId::ExtA(0).inp_param("atv3")` |
### NodeId::ExtB
**Ext. Param. Set A-F Input**

This node gives access to the 24 input parameters of the HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick changes a bit if you need it. Attenuverters (attenuators that can also invert) allow to reduce the amplitude or invert the signal.
- [input **slew**](#nodeidextb-input-slew) - Slew limiter for the 3 parameters
- [input **atv1**](#nodeidextb-input-atv1) - Attenuverter for the A1 parameter
- [input **atv2**](#nodeidextb-input-atv2) - Attenuverter for the A2 parameter
- [input **atv3**](#nodeidextb-input-atv3) - Attenuverter for the A3 parameter
- output **sig1**
A-F1 output channel
- output **sig2**
A-F2 output channel
- output **sig3**
A-F3 output channel
#### NodeId::ExtB Help
**External Parameter Set A-F Input**

This node gives access to the 24 input parameters of the
HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick
changes a bit if you need it. Attenuverters (attenuators that can also invert)
allow to reduce the amplitude or invert the signal.

All instances of the nodes `ExtA`, `ExtB`, ..., `ExtF` have access to the same
3 input parameters (`A1`-`A3`, `B1`-`B3`, ..., `F1`-`F3`). That means there is no
difference whether you use the same instance of different ones.
Except that you can of course set the `atv` and `slew` parameters to different
values.

If you absolutely need more parameters to control the HexoSynth patch:
Keep in mind, that there is also the `MidiCC` node, that allows HexoSynth to
react to MIDI CC messages.

#### NodeId::ExtB input slew
Slew limiter for the 3 parameters

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.00ms | `extb(0).set().slew(0)` | `NodeId::ExtB(0).inp_param("slew")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `extb(0).set().slew(0)` | `NodeId::ExtB(0).inp_param("slew")` |
| **mid** |  0.5000 |   1250.00 |   1250ms | `extb(0).set().slew(1250)` | `NodeId::ExtB(0).inp_param("slew")` |
| **max** |  1.0000 |   5000.00 |   5000ms | `extb(0).set().slew(5000)` | `NodeId::ExtB(0).inp_param("slew")` |
#### NodeId::ExtB input atv1
Attenuverter for the A1 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `extb(0).set().atv1(1)` | `NodeId::ExtB(0).inp_param("atv1")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `extb(0).set().atv1(-1)` | `NodeId::ExtB(0).inp_param("atv1")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `extb(0).set().atv1(0)` | `NodeId::ExtB(0).inp_param("atv1")` |
| **max** |  1.0000 |      1.00 |  1.000 | `extb(0).set().atv1(1)` | `NodeId::ExtB(0).inp_param("atv1")` |
#### NodeId::ExtB input atv2
Attenuverter for the A2 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `extb(0).set().atv2(1)` | `NodeId::ExtB(0).inp_param("atv2")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `extb(0).set().atv2(-1)` | `NodeId::ExtB(0).inp_param("atv2")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `extb(0).set().atv2(0)` | `NodeId::ExtB(0).inp_param("atv2")` |
| **max** |  1.0000 |      1.00 |  1.000 | `extb(0).set().atv2(1)` | `NodeId::ExtB(0).inp_param("atv2")` |
#### NodeId::ExtB input atv3
Attenuverter for the A3 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `extb(0).set().atv3(1)` | `NodeId::ExtB(0).inp_param("atv3")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `extb(0).set().atv3(-1)` | `NodeId::ExtB(0).inp_param("atv3")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `extb(0).set().atv3(0)` | `NodeId::ExtB(0).inp_param("atv3")` |
| **max** |  1.0000 |      1.00 |  1.000 | `extb(0).set().atv3(1)` | `NodeId::ExtB(0).inp_param("atv3")` |
### NodeId::ExtC
**Ext. Param. Set A-F Input**

This node gives access to the 24 input parameters of the HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick changes a bit if you need it. Attenuverters (attenuators that can also invert) allow to reduce the amplitude or invert the signal.
- [input **slew**](#nodeidextc-input-slew) - Slew limiter for the 3 parameters
- [input **atv1**](#nodeidextc-input-atv1) - Attenuverter for the A1 parameter
- [input **atv2**](#nodeidextc-input-atv2) - Attenuverter for the A2 parameter
- [input **atv3**](#nodeidextc-input-atv3) - Attenuverter for the A3 parameter
- output **sig1**
A-F1 output channel
- output **sig2**
A-F2 output channel
- output **sig3**
A-F3 output channel
#### NodeId::ExtC Help
**External Parameter Set A-F Input**

This node gives access to the 24 input parameters of the
HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick
changes a bit if you need it. Attenuverters (attenuators that can also invert)
allow to reduce the amplitude or invert the signal.

All instances of the nodes `ExtA`, `ExtB`, ..., `ExtF` have access to the same
3 input parameters (`A1`-`A3`, `B1`-`B3`, ..., `F1`-`F3`). That means there is no
difference whether you use the same instance of different ones.
Except that you can of course set the `atv` and `slew` parameters to different
values.

If you absolutely need more parameters to control the HexoSynth patch:
Keep in mind, that there is also the `MidiCC` node, that allows HexoSynth to
react to MIDI CC messages.

#### NodeId::ExtC input slew
Slew limiter for the 3 parameters

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.00ms | `extc(0).set().slew(0)` | `NodeId::ExtC(0).inp_param("slew")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `extc(0).set().slew(0)` | `NodeId::ExtC(0).inp_param("slew")` |
| **mid** |  0.5000 |   1250.00 |   1250ms | `extc(0).set().slew(1250)` | `NodeId::ExtC(0).inp_param("slew")` |
| **max** |  1.0000 |   5000.00 |   5000ms | `extc(0).set().slew(5000)` | `NodeId::ExtC(0).inp_param("slew")` |
#### NodeId::ExtC input atv1
Attenuverter for the A1 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `extc(0).set().atv1(1)` | `NodeId::ExtC(0).inp_param("atv1")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `extc(0).set().atv1(-1)` | `NodeId::ExtC(0).inp_param("atv1")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `extc(0).set().atv1(0)` | `NodeId::ExtC(0).inp_param("atv1")` |
| **max** |  1.0000 |      1.00 |  1.000 | `extc(0).set().atv1(1)` | `NodeId::ExtC(0).inp_param("atv1")` |
#### NodeId::ExtC input atv2
Attenuverter for the A2 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `extc(0).set().atv2(1)` | `NodeId::ExtC(0).inp_param("atv2")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `extc(0).set().atv2(-1)` | `NodeId::ExtC(0).inp_param("atv2")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `extc(0).set().atv2(0)` | `NodeId::ExtC(0).inp_param("atv2")` |
| **max** |  1.0000 |      1.00 |  1.000 | `extc(0).set().atv2(1)` | `NodeId::ExtC(0).inp_param("atv2")` |
#### NodeId::ExtC input atv3
Attenuverter for the A3 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `extc(0).set().atv3(1)` | `NodeId::ExtC(0).inp_param("atv3")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `extc(0).set().atv3(-1)` | `NodeId::ExtC(0).inp_param("atv3")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `extc(0).set().atv3(0)` | `NodeId::ExtC(0).inp_param("atv3")` |
| **max** |  1.0000 |      1.00 |  1.000 | `extc(0).set().atv3(1)` | `NodeId::ExtC(0).inp_param("atv3")` |
### NodeId::ExtD
**Ext. Param. Set A-F Input**

This node gives access to the 24 input parameters of the HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick changes a bit if you need it. Attenuverters (attenuators that can also invert) allow to reduce the amplitude or invert the signal.
- [input **slew**](#nodeidextd-input-slew) - Slew limiter for the 3 parameters
- [input **atv1**](#nodeidextd-input-atv1) - Attenuverter for the A1 parameter
- [input **atv2**](#nodeidextd-input-atv2) - Attenuverter for the A2 parameter
- [input **atv3**](#nodeidextd-input-atv3) - Attenuverter for the A3 parameter
- output **sig1**
A-F1 output channel
- output **sig2**
A-F2 output channel
- output **sig3**
A-F3 output channel
#### NodeId::ExtD Help
**External Parameter Set A-F Input**

This node gives access to the 24 input parameters of the
HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick
changes a bit if you need it. Attenuverters (attenuators that can also invert)
allow to reduce the amplitude or invert the signal.

All instances of the nodes `ExtA`, `ExtB`, ..., `ExtF` have access to the same
3 input parameters (`A1`-`A3`, `B1`-`B3`, ..., `F1`-`F3`). That means there is no
difference whether you use the same instance of different ones.
Except that you can of course set the `atv` and `slew` parameters to different
values.

If you absolutely need more parameters to control the HexoSynth patch:
Keep in mind, that there is also the `MidiCC` node, that allows HexoSynth to
react to MIDI CC messages.

#### NodeId::ExtD input slew
Slew limiter for the 3 parameters

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.00ms | `extd(0).set().slew(0)` | `NodeId::ExtD(0).inp_param("slew")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `extd(0).set().slew(0)` | `NodeId::ExtD(0).inp_param("slew")` |
| **mid** |  0.5000 |   1250.00 |   1250ms | `extd(0).set().slew(1250)` | `NodeId::ExtD(0).inp_param("slew")` |
| **max** |  1.0000 |   5000.00 |   5000ms | `extd(0).set().slew(5000)` | `NodeId::ExtD(0).inp_param("slew")` |
#### NodeId::ExtD input atv1
Attenuverter for the A1 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `extd(0).set().atv1(1)` | `NodeId::ExtD(0).inp_param("atv1")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `extd(0).set().atv1(-1)` | `NodeId::ExtD(0).inp_param("atv1")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `extd(0).set().atv1(0)` | `NodeId::ExtD(0).inp_param("atv1")` |
| **max** |  1.0000 |      1.00 |  1.000 | `extd(0).set().atv1(1)` | `NodeId::ExtD(0).inp_param("atv1")` |
#### NodeId::ExtD input atv2
Attenuverter for the A2 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `extd(0).set().atv2(1)` | `NodeId::ExtD(0).inp_param("atv2")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `extd(0).set().atv2(-1)` | `NodeId::ExtD(0).inp_param("atv2")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `extd(0).set().atv2(0)` | `NodeId::ExtD(0).inp_param("atv2")` |
| **max** |  1.0000 |      1.00 |  1.000 | `extd(0).set().atv2(1)` | `NodeId::ExtD(0).inp_param("atv2")` |
#### NodeId::ExtD input atv3
Attenuverter for the A3 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `extd(0).set().atv3(1)` | `NodeId::ExtD(0).inp_param("atv3")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `extd(0).set().atv3(-1)` | `NodeId::ExtD(0).inp_param("atv3")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `extd(0).set().atv3(0)` | `NodeId::ExtD(0).inp_param("atv3")` |
| **max** |  1.0000 |      1.00 |  1.000 | `extd(0).set().atv3(1)` | `NodeId::ExtD(0).inp_param("atv3")` |
### NodeId::ExtE
**Ext. Param. Set A-F Input**

This node gives access to the 24 input parameters of the HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick changes a bit if you need it. Attenuverters (attenuators that can also invert) allow to reduce the amplitude or invert the signal.
- [input **slew**](#nodeidexte-input-slew) - Slew limiter for the 3 parameters
- [input **atv1**](#nodeidexte-input-atv1) - Attenuverter for the A1 parameter
- [input **atv2**](#nodeidexte-input-atv2) - Attenuverter for the A2 parameter
- [input **atv3**](#nodeidexte-input-atv3) - Attenuverter for the A3 parameter
- output **sig1**
A-F1 output channel
- output **sig2**
A-F2 output channel
- output **sig3**
A-F3 output channel
#### NodeId::ExtE Help
**External Parameter Set A-F Input**

This node gives access to the 24 input parameters of the
HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick
changes a bit if you need it. Attenuverters (attenuators that can also invert)
allow to reduce the amplitude or invert the signal.

All instances of the nodes `ExtA`, `ExtB`, ..., `ExtF` have access to the same
3 input parameters (`A1`-`A3`, `B1`-`B3`, ..., `F1`-`F3`). That means there is no
difference whether you use the same instance of different ones.
Except that you can of course set the `atv` and `slew` parameters to different
values.

If you absolutely need more parameters to control the HexoSynth patch:
Keep in mind, that there is also the `MidiCC` node, that allows HexoSynth to
react to MIDI CC messages.

#### NodeId::ExtE input slew
Slew limiter for the 3 parameters

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.00ms | `exte(0).set().slew(0)` | `NodeId::ExtE(0).inp_param("slew")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `exte(0).set().slew(0)` | `NodeId::ExtE(0).inp_param("slew")` |
| **mid** |  0.5000 |   1250.00 |   1250ms | `exte(0).set().slew(1250)` | `NodeId::ExtE(0).inp_param("slew")` |
| **max** |  1.0000 |   5000.00 |   5000ms | `exte(0).set().slew(5000)` | `NodeId::ExtE(0).inp_param("slew")` |
#### NodeId::ExtE input atv1
Attenuverter for the A1 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `exte(0).set().atv1(1)` | `NodeId::ExtE(0).inp_param("atv1")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `exte(0).set().atv1(-1)` | `NodeId::ExtE(0).inp_param("atv1")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `exte(0).set().atv1(0)` | `NodeId::ExtE(0).inp_param("atv1")` |
| **max** |  1.0000 |      1.00 |  1.000 | `exte(0).set().atv1(1)` | `NodeId::ExtE(0).inp_param("atv1")` |
#### NodeId::ExtE input atv2
Attenuverter for the A2 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `exte(0).set().atv2(1)` | `NodeId::ExtE(0).inp_param("atv2")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `exte(0).set().atv2(-1)` | `NodeId::ExtE(0).inp_param("atv2")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `exte(0).set().atv2(0)` | `NodeId::ExtE(0).inp_param("atv2")` |
| **max** |  1.0000 |      1.00 |  1.000 | `exte(0).set().atv2(1)` | `NodeId::ExtE(0).inp_param("atv2")` |
#### NodeId::ExtE input atv3
Attenuverter for the A3 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `exte(0).set().atv3(1)` | `NodeId::ExtE(0).inp_param("atv3")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `exte(0).set().atv3(-1)` | `NodeId::ExtE(0).inp_param("atv3")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `exte(0).set().atv3(0)` | `NodeId::ExtE(0).inp_param("atv3")` |
| **max** |  1.0000 |      1.00 |  1.000 | `exte(0).set().atv3(1)` | `NodeId::ExtE(0).inp_param("atv3")` |
### NodeId::ExtF
**Ext. Param. Set A-F Input**

This node gives access to the 24 input parameters of the HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick changes a bit if you need it. Attenuverters (attenuators that can also invert) allow to reduce the amplitude or invert the signal.
- [input **slew**](#nodeidextf-input-slew) - Slew limiter for the 3 parameters
- [input **atv1**](#nodeidextf-input-atv1) - Attenuverter for the A1 parameter
- [input **atv2**](#nodeidextf-input-atv2) - Attenuverter for the A2 parameter
- [input **atv3**](#nodeidextf-input-atv3) - Attenuverter for the A3 parameter
- output **sig1**
A-F1 output channel
- output **sig2**
A-F2 output channel
- output **sig3**
A-F3 output channel
#### NodeId::ExtF Help
**External Parameter Set A-F Input**

This node gives access to the 24 input parameters of the
HexoSynth VST3/CLAP plugin. A `slew` limiter allows you to smooth out quick
changes a bit if you need it. Attenuverters (attenuators that can also invert)
allow to reduce the amplitude or invert the signal.

All instances of the nodes `ExtA`, `ExtB`, ..., `ExtF` have access to the same
3 input parameters (`A1`-`A3`, `B1`-`B3`, ..., `F1`-`F3`). That means there is no
difference whether you use the same instance of different ones.
Except that you can of course set the `atv` and `slew` parameters to different
values.

If you absolutely need more parameters to control the HexoSynth patch:
Keep in mind, that there is also the `MidiCC` node, that allows HexoSynth to
react to MIDI CC messages.

#### NodeId::ExtF input slew
Slew limiter for the 3 parameters

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.00ms | `extf(0).set().slew(0)` | `NodeId::ExtF(0).inp_param("slew")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `extf(0).set().slew(0)` | `NodeId::ExtF(0).inp_param("slew")` |
| **mid** |  0.5000 |   1250.00 |   1250ms | `extf(0).set().slew(1250)` | `NodeId::ExtF(0).inp_param("slew")` |
| **max** |  1.0000 |   5000.00 |   5000ms | `extf(0).set().slew(5000)` | `NodeId::ExtF(0).inp_param("slew")` |
#### NodeId::ExtF input atv1
Attenuverter for the A1 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `extf(0).set().atv1(1)` | `NodeId::ExtF(0).inp_param("atv1")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `extf(0).set().atv1(-1)` | `NodeId::ExtF(0).inp_param("atv1")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `extf(0).set().atv1(0)` | `NodeId::ExtF(0).inp_param("atv1")` |
| **max** |  1.0000 |      1.00 |  1.000 | `extf(0).set().atv1(1)` | `NodeId::ExtF(0).inp_param("atv1")` |
#### NodeId::ExtF input atv2
Attenuverter for the A2 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `extf(0).set().atv2(1)` | `NodeId::ExtF(0).inp_param("atv2")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `extf(0).set().atv2(-1)` | `NodeId::ExtF(0).inp_param("atv2")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `extf(0).set().atv2(0)` | `NodeId::ExtF(0).inp_param("atv2")` |
| **max** |  1.0000 |      1.00 |  1.000 | `extf(0).set().atv2(1)` | `NodeId::ExtF(0).inp_param("atv2")` |
#### NodeId::ExtF input atv3
Attenuverter for the A3 parameter

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `extf(0).set().atv3(1)` | `NodeId::ExtF(0).inp_param("atv3")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `extf(0).set().atv3(-1)` | `NodeId::ExtF(0).inp_param("atv3")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `extf(0).set().atv3(0)` | `NodeId::ExtF(0).inp_param("atv3")` |
| **max** |  1.0000 |      1.00 |  1.000 | `extf(0).set().atv3(1)` | `NodeId::ExtF(0).inp_param("atv3")` |
### NodeId::Inp
**Audio Input Port**

This node gives you access to the two input ports of the HexoSynth plugin. Build effects or what ever you can imagine with this!
        
- [input **vol**](#nodeidinp-input-vol) - The volume of the two plugin input ports, applied to all channels. Please note that this is a linear control, to prevent inaccuracies for **1.0**. 
- output **sig1**
Audio input channel 1 (left)
- output **sig2**
Audio input channel 2 (right)
#### NodeId::Inp Help
**Audio Input Port**

This node gives you access to the two input ports of the HexoSynth plugin.
You can build an effects plugin with this node and the `Out` node.
Or a synthesizer that reacts to audio rate control signals on these two
input ports.

#### NodeId::Inp input vol
The volume of the two plugin input ports, applied to all channels. Please note that this is a linear control, to prevent inaccuracies for **1.0**. 

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.8333 |      1.00 | +0.0dB | `inp(0).set().vol(0.99999976)` | `NodeId::Inp(0).inp_param("vol")` |
| **min** |  0.0000 |      0.00 | -inf dB | `inp(0).set().vol(0)` | `NodeId::Inp(0).inp_param("vol")` |
| **mid** |  0.5000 |      0.02 | -36.0dB | `inp(0).set().vol(0.015848929)` | `NodeId::Inp(0).inp_param("vol")` |
| **max** |  1.0000 |      7.94 | +18.0dB | `inp(0).set().vol(7.943283)` | `NodeId::Inp(0).inp_param("vol")` |
### NodeId::Out
**Audio Output Port**

This output port node allows you to send audio signals to audio devices or tracks in your DAW.
- [input **ch1**](#nodeidout-input-ch1) - Audio channel 1 (left)
- [input **ch2**](#nodeidout-input-ch2) - Audio channel 2 (right)
- [input **vol**](#nodeidout-input-vol) - The main volume of the synthesizer output, applied to all channels. Please note that this is a linear control, to prevent inaccuracies for **1.0**. 
- [setting **mono**](#nodeidout-setting-mono) - If set to **Mono**, `ch1` will be sent to both output channels. (UI only)
#### NodeId::Out Help
**Audio Output Port**

This output port node allows you to send audio signals to audio devices
or tracks in your DAW. If you need a stereo output but only have a mono
signal you can use the `mono` setting to duplicate the signal on the `ch1`
input to the second channel `ch2`.

#### NodeId::Out input ch1
Audio channel 1 (left)

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `out(0).set().ch1(0)` | `NodeId::Out(0).inp_param("ch1")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `out(0).set().ch1(-1)` | `NodeId::Out(0).inp_param("ch1")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `out(0).set().ch1(0)` | `NodeId::Out(0).inp_param("ch1")` |
| **max** |  1.0000 |      1.00 |  1.000 | `out(0).set().ch1(1)` | `NodeId::Out(0).inp_param("ch1")` |
#### NodeId::Out input ch2
Audio channel 2 (right)

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `out(0).set().ch2(0)` | `NodeId::Out(0).inp_param("ch2")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `out(0).set().ch2(-1)` | `NodeId::Out(0).inp_param("ch2")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `out(0).set().ch2(0)` | `NodeId::Out(0).inp_param("ch2")` |
| **max** |  1.0000 |      1.00 |  1.000 | `out(0).set().ch2(1)` | `NodeId::Out(0).inp_param("ch2")` |
#### NodeId::Out input vol
The main volume of the synthesizer output, applied to all channels. Please note that this is a linear control, to prevent inaccuracies for **1.0**. 

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.8333 |      1.00 | +0.0dB | `out(0).set().vol(0.99999976)` | `NodeId::Out(0).inp_param("vol")` |
| **min** |  0.0000 |      0.00 | -inf dB | `out(0).set().vol(0)` | `NodeId::Out(0).inp_param("vol")` |
| **mid** |  0.5000 |      0.02 | -36.0dB | `out(0).set().vol(0.015848929)` | `NodeId::Out(0).inp_param("vol")` |
| **max** |  1.0000 |      7.94 | +18.0dB | `out(0).set().vol(7.943283)` | `NodeId::Out(0).inp_param("vol")` |
#### NodeId::Out setting mono
If set to **Mono**, `ch1` will be sent to both output channels.
(UI only)

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Stereo | `out(0).set().mono(0)` | `NodeId::Out(0).inp_param("mono")` |
| 1 | Mono | `out(0).set().mono(1)` | `NodeId::Out(0).inp_param("mono")` |
### NodeId::FbWr
**Feedback Delay Writer**

HexoSynth does not allow direct feedback cycles in it's graph.
To make feedback possible anyways the `FbWr` and `FbRd` nodes are provided.
This node allows you to write a signal into the corresponsing signal delay buffer.
Use `FbRd` for using the signal. The delay is **3.14ms**.
- [input **inp**](#nodeidfbwr-input-inp) - Signal input
#### NodeId::FbWr Help
**Feedback Delay Writer**

HexoSynth does not allow direct feedback cycles in it's graph.
To make feedback possible anyways the `FbWr` and `FbRd` nodes are provided.
This node allows you to send a signal into the corresponding `FbWr` signal
delay.

The instance id of the node defines which `FbWr` and `FbRd` are connected.
That means `FbRd 0` is connected to the corresponding `FbWr 0`. You can use
the signal multiple times by connecting the `FbRd 0` `sig` port to multiple
inputs.

The delay is always **3.14ms**, regardless of the sampling rate the synthesizer
is running at.

#### NodeId::FbWr input inp
Signal input

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `fbwr(0).set().inp(0)` | `NodeId::FbWr(0).inp_param("inp")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `fbwr(0).set().inp(-1)` | `NodeId::FbWr(0).inp_param("inp")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `fbwr(0).set().inp(0)` | `NodeId::FbWr(0).inp_param("inp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `fbwr(0).set().inp(1)` | `NodeId::FbWr(0).inp_param("inp")` |
### NodeId::FbRd
**Feedback Delay Reader**

HexoSynth does not allow direct feedback cycles in it's graph.
To make feedback possible anyways the `FbWr` and `FbRd` nodes are provided.
This node allows you to tap into the corresponding `FbWr` signal delay for feedback. The delay is **3.14ms**.
- [input **vol**](#nodeidfbrd-input-vol) - Volume of the input. Use this to adjust the feedback amount.
- output **sig**
Feedback signal output.
#### NodeId::FbRd Help
**Feedback Delay Reader**

HexoSynth does not allow direct feedback cycles in it's graph.
To make feedback possible anyways the `FbWr` and `FbRd` nodes are provided.
This node allows you to tap into the corresponding `FbWr` signal delay for
feedback.

The instance id of the node defines which `FbWr` and `FbRd` are connected.
That means `FbRd 0` is connected to the corresponding `FbWr 0`. You can use
the signal multiple times by connecting the `FbRd 0` `sig` port to multiple
inputs.

The delay is always **3.14ms**, regardless of the sampling rate the synthesizer
is running at.

The `vol` parameter is a convenience parameter to allow to control the
volume of the feedback.

#### NodeId::FbRd input vol
Volume of the input.
Use this to adjust the feedback amount.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.8333 |      1.00 | +0.0dB | `fbrd(0).set().vol(0.99999976)` | `NodeId::FbRd(0).inp_param("vol")` |
| **min** |  0.0000 |      0.00 | -inf dB | `fbrd(0).set().vol(0)` | `NodeId::FbRd(0).inp_param("vol")` |
| **mid** |  0.5000 |      0.02 | -36.0dB | `fbrd(0).set().vol(0.015848929)` | `NodeId::FbRd(0).inp_param("vol")` |
| **max** |  1.0000 |      7.94 | +18.0dB | `fbrd(0).set().vol(7.943283)` | `NodeId::FbRd(0).inp_param("vol")` |
### NodeId::Scope
**Signal Oscilloscope Probe**

This is a signal oscilloscope probe node, you can capture up to 3 signals.
You can enable internal or external triggering for capturing signals or pinning fast waveforms.

- [input **in1**](#nodeidscope-input-in1) - Signal input 1.
- [input **in2**](#nodeidscope-input-in2) - Signal input 2.
- [input **in3**](#nodeidscope-input-in3) - Signal input 3.
- [input **time**](#nodeidscope-input-time) - Displayed time range of the oscilloscope view.
- [input **trig**](#nodeidscope-input-trig) - External trigger input. Only active if `tsrc` is set to **Extern**. `thrsh` applies also for external triggers.
- [input **thrsh**](#nodeidscope-input-thrsh) - Trigger threshold. If the threshold is passed by the signal from low to high the signal recording will be reset. Either for internal or for external triggering. Trigger is only active if `tsrc` is not **Off**.
- [input **off1**](#nodeidscope-input-off1) - Visual offset of signal input 1.
- [input **off2**](#nodeidscope-input-off2) - Visual offset of signal input 2.
- [input **off3**](#nodeidscope-input-off3) - Visual offset of signal input 3.
- [input **gain1**](#nodeidscope-input-gain1) - Visual amplification/attenuation of the signal input 1.
- [input **gain2**](#nodeidscope-input-gain2) - Visual amplification/attenuation of the signal input 2.
- [input **gain3**](#nodeidscope-input-gain3) - Visual amplification/attenuation of the signal input 3.
- [setting **tsrc**](#nodeidscope-setting-tsrc) - Triggering allows you to capture fast signals or pinning fast waveforms into the scope view for better inspection. You can let the scope freeze and manually recapture waveforms by setting `tsrc` to **Extern** and hitting the `trig` button manually.
#### NodeId::Scope Help
**Signal Oscilloscope Probe**

You can have up to 8 different scopes in your patch. That means you can
in record up to 24 signals for displaying them in the scope view.
The received signal will be forwarded to the GUI and you can inspect
the waveform there.

You can enable an internal trigger with the `tsrc` setting set to **Intern**.
**Intern** here means that the signal input 1 `in1` is used as trigger signal.
The `thrsh` parameter is the trigger detection parameter. That means, if your
signal passes that threshold in negative to positive direction, the signal
recording will be reset to that point.

You can also route in an external trigger to capture signals with the `trig`
input and `tsrc` set to **Extern**. Of course you can also hit the `trig` button
manually to recapture a waveform.

The inputs `off1`, `off2` and `off3` define a vertical offset of the signal
waveform in the scope view. Use `gain1`, `gain2` and `gain3` for scaling
the input signals up/down.

#### NodeId::Scope input in1
Signal input 1.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `scope(0).set().in1(0)` | `NodeId::Scope(0).inp_param("in1")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `scope(0).set().in1(-1)` | `NodeId::Scope(0).inp_param("in1")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `scope(0).set().in1(0)` | `NodeId::Scope(0).inp_param("in1")` |
| **max** |  1.0000 |      1.00 |  1.000 | `scope(0).set().in1(1)` | `NodeId::Scope(0).inp_param("in1")` |
#### NodeId::Scope input in2
Signal input 2.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `scope(0).set().in2(0)` | `NodeId::Scope(0).inp_param("in2")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `scope(0).set().in2(-1)` | `NodeId::Scope(0).inp_param("in2")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `scope(0).set().in2(0)` | `NodeId::Scope(0).inp_param("in2")` |
| **max** |  1.0000 |      1.00 |  1.000 | `scope(0).set().in2(1)` | `NodeId::Scope(0).inp_param("in2")` |
#### NodeId::Scope input in3
Signal input 3.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `scope(0).set().in3(0)` | `NodeId::Scope(0).inp_param("in3")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `scope(0).set().in3(-1)` | `NodeId::Scope(0).inp_param("in3")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `scope(0).set().in3(0)` | `NodeId::Scope(0).inp_param("in3")` |
| **max** |  1.0000 |      1.00 |  1.000 | `scope(0).set().in3(1)` | `NodeId::Scope(0).inp_param("in3")` |
#### NodeId::Scope input time
Displayed time range of the oscilloscope view.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.3865 |   1000.00 | 1000ms | `scope(0).set().time(999.9997)` | `NodeId::Scope(0).inp_param("time")` |
| **min** |  0.0000 |      0.10 | 0.100ms | `scope(0).set().time(0.1)` | `NodeId::Scope(0).inp_param("time")` |
| **mid** |  0.5000 |   4687.60 |  4.69s | `scope(0).set().time(4687.5986)` | `NodeId::Scope(0).inp_param("time")` |
| **max** |  1.0000 | 300000.00 | 300.0s | `scope(0).set().time(300000)` | `NodeId::Scope(0).inp_param("time")` |
#### NodeId::Scope input trig
External trigger input. Only active if `tsrc` is set to **Extern**. `thrsh` applies also for external triggers.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `scope(0).set().trig(0)` | `NodeId::Scope(0).inp_param("trig")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `scope(0).set().trig(-1)` | `NodeId::Scope(0).inp_param("trig")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `scope(0).set().trig(0)` | `NodeId::Scope(0).inp_param("trig")` |
| **max** |  1.0000 |      1.00 |  1.000 | `scope(0).set().trig(1)` | `NodeId::Scope(0).inp_param("trig")` |
#### NodeId::Scope input thrsh
Trigger threshold. If the threshold is passed by the signal from low to high the signal recording will be reset. Either for internal or for external triggering. Trigger is only active if `tsrc` is not **Off**.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `scope(0).set().thrsh(0)` | `NodeId::Scope(0).inp_param("thrsh")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `scope(0).set().thrsh(-1)` | `NodeId::Scope(0).inp_param("thrsh")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `scope(0).set().thrsh(0)` | `NodeId::Scope(0).inp_param("thrsh")` |
| **max** |  1.0000 |      1.00 |  1.000 | `scope(0).set().thrsh(1)` | `NodeId::Scope(0).inp_param("thrsh")` |
#### NodeId::Scope input off1
Visual offset of signal input 1.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `scope(0).set().off1(0)` | `NodeId::Scope(0).inp_param("off1")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `scope(0).set().off1(-1)` | `NodeId::Scope(0).inp_param("off1")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `scope(0).set().off1(0)` | `NodeId::Scope(0).inp_param("off1")` |
| **max** |  1.0000 |      1.00 |  1.000 | `scope(0).set().off1(1)` | `NodeId::Scope(0).inp_param("off1")` |
#### NodeId::Scope input off2
Visual offset of signal input 2.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `scope(0).set().off2(0)` | `NodeId::Scope(0).inp_param("off2")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `scope(0).set().off2(-1)` | `NodeId::Scope(0).inp_param("off2")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `scope(0).set().off2(0)` | `NodeId::Scope(0).inp_param("off2")` |
| **max** |  1.0000 |      1.00 |  1.000 | `scope(0).set().off2(1)` | `NodeId::Scope(0).inp_param("off2")` |
#### NodeId::Scope input off3
Visual offset of signal input 3.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `scope(0).set().off3(0)` | `NodeId::Scope(0).inp_param("off3")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `scope(0).set().off3(-1)` | `NodeId::Scope(0).inp_param("off3")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `scope(0).set().off3(0)` | `NodeId::Scope(0).inp_param("off3")` |
| **max** |  1.0000 |      1.00 |  1.000 | `scope(0).set().off3(1)` | `NodeId::Scope(0).inp_param("off3")` |
#### NodeId::Scope input gain1
Visual amplification/attenuation of the signal input 1.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      1.00 | +0.0dB | `scope(0).set().gain1(1)` | `NodeId::Scope(0).inp_param("gain1")` |
| **min** |  0.0000 |      0.06 | -24.0dB | `scope(0).set().gain1(0.063095726)` | `NodeId::Scope(0).inp_param("gain1")` |
| **mid** |  0.5000 |      1.00 | +0.0dB | `scope(0).set().gain1(1)` | `NodeId::Scope(0).inp_param("gain1")` |
| **max** |  1.0000 |     15.85 | +24.0dB | `scope(0).set().gain1(15.848933)` | `NodeId::Scope(0).inp_param("gain1")` |
#### NodeId::Scope input gain2
Visual amplification/attenuation of the signal input 2.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      1.00 | +0.0dB | `scope(0).set().gain2(1)` | `NodeId::Scope(0).inp_param("gain2")` |
| **min** |  0.0000 |      0.06 | -24.0dB | `scope(0).set().gain2(0.063095726)` | `NodeId::Scope(0).inp_param("gain2")` |
| **mid** |  0.5000 |      1.00 | +0.0dB | `scope(0).set().gain2(1)` | `NodeId::Scope(0).inp_param("gain2")` |
| **max** |  1.0000 |     15.85 | +24.0dB | `scope(0).set().gain2(15.848933)` | `NodeId::Scope(0).inp_param("gain2")` |
#### NodeId::Scope input gain3
Visual amplification/attenuation of the signal input 3.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      1.00 | +0.0dB | `scope(0).set().gain3(1)` | `NodeId::Scope(0).inp_param("gain3")` |
| **min** |  0.0000 |      0.06 | -24.0dB | `scope(0).set().gain3(0.063095726)` | `NodeId::Scope(0).inp_param("gain3")` |
| **mid** |  0.5000 |      1.00 | +0.0dB | `scope(0).set().gain3(1)` | `NodeId::Scope(0).inp_param("gain3")` |
| **max** |  1.0000 |     15.85 | +24.0dB | `scope(0).set().gain3(15.848933)` | `NodeId::Scope(0).inp_param("gain3")` |
#### NodeId::Scope setting tsrc
Triggering allows you to capture fast signals or pinning fast waveforms into the scope view for better inspection. You can let the scope freeze and manually recapture waveforms by setting `tsrc` to **Extern** and hitting the `trig` button manually.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Off | `scope(0).set().tsrc(0)` | `NodeId::Scope(0).inp_param("tsrc")` |
| 1 | Intern | `scope(0).set().tsrc(1)` | `NodeId::Scope(0).inp_param("tsrc")` |
| 2 | Extern | `scope(0).set().tsrc(2)` | `NodeId::Scope(0).inp_param("tsrc")` |
### NodeId::Ad
**Attack-Decay Envelope**

This is a simple envelope offering an attack time and decay time with a shape parameter.
You can use it as envelope generator to modulate other inputs or process a signal with it directly.

- [input **inp**](#nodeidad-input-inp) - Signal input. If you don't connect this, and set this to **1.0** this will act as envelope signal generator. But you can also just route a signal directly through this of course.
- [input **trig**](#nodeidad-input-trig) - Trigger input that starts the attack phase.
- [input **atk**](#nodeidad-input-atk) - Attack time of the envelope. You can extend the maximum range of this with the `mult` setting.
- [input **dcy**](#nodeidad-input-dcy) - Decay time of the envelope. You can extend the maximum range of this with the `mult` setting.
- [input **ashp**](#nodeidad-input-ashp) - Attack shape. This allows you to change the shape of the attack stage from a logarithmic, to a linear and to an exponential shape.
- [input **dshp**](#nodeidad-input-dshp) - Decay shape. This allows you to change the shape of the decay stage from a logarithmic, to a linear and to an exponential shape.
- [setting **mult**](#nodeidad-setting-mult) - Attack and Decay time range multiplier. This will extend the maximum range of the `atk` and `dcy` parameters.
- output **sig**
Envelope signal output. If a signal is sent to the 'inp' port, you will receive an attenuated signal here. If you set 'inp' to a fixed value (**for instance 1.0**), this will output an envelope signal in the range 0.0 to 'inp' (**1.0**).
- output **eoet**
End of envelope trigger. This output sends a trigger once the end of the decay stage has been reached.
#### NodeId::Ad Help
**Attack-Decay Envelope**

This simple two stage envelope with attack and decay offers shape parameters
for each stage. The attack and decay times can be extended using the `mult`
setting.

The `inp` can either be used to process a signal, or set the target output
value of the envelope. In the latter case this node is just a simple
envelope generator, with which you can generate control signals to modulate
other inputs.

With the `eoet` output you can either trigger other envelopes or via
`FbWr`/`FbRd` retrigger the envelope.

#### NodeId::Ad input inp
Signal input. If you don't connect this, and set this to **1.0** this will act as envelope signal generator. But you can also just route a signal directly through this of course.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `ad(0).set().inp(1)` | `NodeId::Ad(0).inp_param("inp")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `ad(0).set().inp(-1)` | `NodeId::Ad(0).inp_param("inp")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `ad(0).set().inp(0)` | `NodeId::Ad(0).inp_param("inp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `ad(0).set().inp(1)` | `NodeId::Ad(0).inp_param("inp")` |
#### NodeId::Ad input trig
Trigger input that starts the attack phase.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `ad(0).set().trig(0)` | `NodeId::Ad(0).inp_param("trig")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `ad(0).set().trig(-1)` | `NodeId::Ad(0).inp_param("trig")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `ad(0).set().trig(0)` | `NodeId::Ad(0).inp_param("trig")` |
| **max** |  1.0000 |      1.00 |  1.000 | `ad(0).set().trig(1)` | `NodeId::Ad(0).inp_param("trig")` |
#### NodeId::Ad input atk
Attack time of the envelope. You can extend the maximum range of this with the `mult` setting.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0548 |      3.00 |  3.00ms | `ad(0).set().atk(3.0000002)` | `NodeId::Ad(0).inp_param("atk")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `ad(0).set().atk(0)` | `NodeId::Ad(0).inp_param("atk")` |
| **mid** |  0.5000 |    250.00 | 250.0ms | `ad(0).set().atk(250)` | `NodeId::Ad(0).inp_param("atk")` |
| **max** |  1.0000 |   1000.00 |   1000ms | `ad(0).set().atk(1000)` | `NodeId::Ad(0).inp_param("atk")` |
#### NodeId::Ad input dcy
Decay time of the envelope. You can extend the maximum range of this with the `mult` setting.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.1000 |     10.00 | 10.00ms | `ad(0).set().dcy(10.000001)` | `NodeId::Ad(0).inp_param("dcy")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `ad(0).set().dcy(0)` | `NodeId::Ad(0).inp_param("dcy")` |
| **mid** |  0.5000 |    250.00 | 250.0ms | `ad(0).set().dcy(250)` | `NodeId::Ad(0).inp_param("dcy")` |
| **max** |  1.0000 |   1000.00 |   1000ms | `ad(0).set().dcy(1000)` | `NodeId::Ad(0).inp_param("dcy")` |
#### NodeId::Ad input ashp
Attack shape. This allows you to change the shape of the attack stage from a logarithmic, to a linear and to an exponential shape.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `ad(0).set().ashp(0.5)` | `NodeId::Ad(0).inp_param("ashp")` |
| **min** |  0.0000 |      0.00 |  0.000 | `ad(0).set().ashp(0)` | `NodeId::Ad(0).inp_param("ashp")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `ad(0).set().ashp(0.5)` | `NodeId::Ad(0).inp_param("ashp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `ad(0).set().ashp(1)` | `NodeId::Ad(0).inp_param("ashp")` |
#### NodeId::Ad input dshp
Decay shape. This allows you to change the shape of the decay stage from a logarithmic, to a linear and to an exponential shape.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `ad(0).set().dshp(0.5)` | `NodeId::Ad(0).inp_param("dshp")` |
| **min** |  0.0000 |      0.00 |  0.000 | `ad(0).set().dshp(0)` | `NodeId::Ad(0).inp_param("dshp")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `ad(0).set().dshp(0.5)` | `NodeId::Ad(0).inp_param("dshp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `ad(0).set().dshp(1)` | `NodeId::Ad(0).inp_param("dshp")` |
#### NodeId::Ad setting mult
Attack and Decay time range multiplier. This will extend the maximum range of the `atk` and `dcy` parameters.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | x1 | `ad(0).set().mult(0)` | `NodeId::Ad(0).inp_param("mult")` |
| 1 | x10 | `ad(0).set().mult(1)` | `NodeId::Ad(0).inp_param("mult")` |
| 2 | x100 | `ad(0).set().mult(2)` | `NodeId::Ad(0).inp_param("mult")` |
### NodeId::Adsr
**Attack-Decay Envelope**

This is an ADSR envelope, offering an attack time, decay time, a sustain phase and a release time.
Attack, decay and release each have their own shape parameter.
You can use it as envelope generator to modulate other inputs or process a
signal with it directly.

- [input **inp**](#nodeidadsr-input-inp) - Signal input. If you don't connect this, and set this to **1.0** this will act as envelope signal generator. But you can also just route a signal directly through this of course.
- [input **gate**](#nodeidadsr-input-gate) - Gate input that starts the attack phase and ends the sustain phase if it goes low.
- [input **atk**](#nodeidadsr-input-atk) - Attack time of the envelope. You can extend the maximum range of this with the `mult` setting.
- [input **dcy**](#nodeidadsr-input-dcy) - Decay time of the envelope. You can extend the maximum range of this with the `mult` setting.
- [input **sus**](#nodeidadsr-input-sus) - Sustain value. This is the value the decay phase goes to. Setting this to eg. **0.0** will make an AD envelope from this.
- [input **rel**](#nodeidadsr-input-rel) - Release time of the envelope. You can extend the maximum range of this with the `mult` setting.
- [input **ashp**](#nodeidadsr-input-ashp) - Attack shape. This allows you to change the shape of the attack stage from a logarithmic, to a linear and to an exponential shape.
- [input **dshp**](#nodeidadsr-input-dshp) - Decay shape. This allows you to change the shape of the decay stage from a logarithmic, to a linear and to an exponential shape.
- [input **rshp**](#nodeidadsr-input-rshp) - Release shape. This allows you to change the shape of the release stage from a logarithmic, to a linear and to an exponential shape.
- [setting **mult**](#nodeidadsr-setting-mult) - Attack and Decay time range multiplier. This will extend the maximum range of the `atk`, `dcy` and `rel` parameters.
- output **sig**
Envelope signal output. If a signal is sent to the 'inp' port, you will receive an attenuated signal here. If you set 'inp' to a fixed value (**for instance 1.0**), this will output an envelope signal in the range 0.0 to 'inp' (**1.0**).
- output **eoet**
End of envelope trigger output. This output sends a trigger pulse once the end of the decay stage has been reached.
#### NodeId::Adsr Help
**Attack-Decay Envelope**

This is an ADSR envelope, offering an attack time, decay time, a sustain phase and a release time.
Attack, decay and release each have their own shape parameter.

The `mult` setting allows you to multiply the times for the parameters and thus making really
long envelopes possible.

The `inp` can either be used to process a signal, or set the target output
value of the envelope (**1.0** by default). In the latter case this node is just a simple
envelope generator, with which you can generate control signals to modulate
other inputs. You could for instance control a filter cutoff frequency and an `Amp` `att`
parameter at the same time with this.

With the `eoet` output you can either trigger other envelopes or via
`FbWr`/`FbRd` retrigger the same envelope. You could also make a chain of multiple
envelopes following each other.

#### NodeId::Adsr input inp
Signal input. If you don't connect this, and set this to **1.0** this will act as envelope signal generator. But you can also just route a signal directly through this of course.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `adsr(0).set().inp(1)` | `NodeId::Adsr(0).inp_param("inp")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `adsr(0).set().inp(-1)` | `NodeId::Adsr(0).inp_param("inp")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `adsr(0).set().inp(0)` | `NodeId::Adsr(0).inp_param("inp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `adsr(0).set().inp(1)` | `NodeId::Adsr(0).inp_param("inp")` |
#### NodeId::Adsr input gate
Gate input that starts the attack phase and ends the sustain phase if it goes low.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `adsr(0).set().gate(0)` | `NodeId::Adsr(0).inp_param("gate")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `adsr(0).set().gate(-1)` | `NodeId::Adsr(0).inp_param("gate")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `adsr(0).set().gate(0)` | `NodeId::Adsr(0).inp_param("gate")` |
| **max** |  1.0000 |      1.00 |  1.000 | `adsr(0).set().gate(1)` | `NodeId::Adsr(0).inp_param("gate")` |
#### NodeId::Adsr input atk
Attack time of the envelope. You can extend the maximum range of this with the `mult` setting.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0548 |      3.00 |  3.00ms | `adsr(0).set().atk(3.0000002)` | `NodeId::Adsr(0).inp_param("atk")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `adsr(0).set().atk(0)` | `NodeId::Adsr(0).inp_param("atk")` |
| **mid** |  0.5000 |    250.00 | 250.0ms | `adsr(0).set().atk(250)` | `NodeId::Adsr(0).inp_param("atk")` |
| **max** |  1.0000 |   1000.00 |   1000ms | `adsr(0).set().atk(1000)` | `NodeId::Adsr(0).inp_param("atk")` |
#### NodeId::Adsr input dcy
Decay time of the envelope. You can extend the maximum range of this with the `mult` setting.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.1000 |     10.00 | 10.00ms | `adsr(0).set().dcy(10.000001)` | `NodeId::Adsr(0).inp_param("dcy")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `adsr(0).set().dcy(0)` | `NodeId::Adsr(0).inp_param("dcy")` |
| **mid** |  0.5000 |    250.00 | 250.0ms | `adsr(0).set().dcy(250)` | `NodeId::Adsr(0).inp_param("dcy")` |
| **max** |  1.0000 |   1000.00 |   1000ms | `adsr(0).set().dcy(1000)` | `NodeId::Adsr(0).inp_param("dcy")` |
#### NodeId::Adsr input sus
Sustain value. This is the value the decay phase goes to. Setting this to eg. **0.0** will make an AD envelope from this.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `adsr(0).set().sus(0.5)` | `NodeId::Adsr(0).inp_param("sus")` |
| **min** |  0.0000 |      0.00 |  0.000 | `adsr(0).set().sus(0)` | `NodeId::Adsr(0).inp_param("sus")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `adsr(0).set().sus(0.5)` | `NodeId::Adsr(0).inp_param("sus")` |
| **max** |  1.0000 |      1.00 |  1.000 | `adsr(0).set().sus(1)` | `NodeId::Adsr(0).inp_param("sus")` |
#### NodeId::Adsr input rel
Release time of the envelope. You can extend the maximum range of this with the `mult` setting.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.2000 |     40.00 | 40.00ms | `adsr(0).set().rel(40.000004)` | `NodeId::Adsr(0).inp_param("rel")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `adsr(0).set().rel(0)` | `NodeId::Adsr(0).inp_param("rel")` |
| **mid** |  0.5000 |    250.00 | 250.0ms | `adsr(0).set().rel(250)` | `NodeId::Adsr(0).inp_param("rel")` |
| **max** |  1.0000 |   1000.00 |   1000ms | `adsr(0).set().rel(1000)` | `NodeId::Adsr(0).inp_param("rel")` |
#### NodeId::Adsr input ashp
Attack shape. This allows you to change the shape of the attack stage from a logarithmic, to a linear and to an exponential shape.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `adsr(0).set().ashp(0.5)` | `NodeId::Adsr(0).inp_param("ashp")` |
| **min** |  0.0000 |      0.00 |  0.000 | `adsr(0).set().ashp(0)` | `NodeId::Adsr(0).inp_param("ashp")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `adsr(0).set().ashp(0.5)` | `NodeId::Adsr(0).inp_param("ashp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `adsr(0).set().ashp(1)` | `NodeId::Adsr(0).inp_param("ashp")` |
#### NodeId::Adsr input dshp
Decay shape. This allows you to change the shape of the decay stage from a logarithmic, to a linear and to an exponential shape.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `adsr(0).set().dshp(0.5)` | `NodeId::Adsr(0).inp_param("dshp")` |
| **min** |  0.0000 |      0.00 |  0.000 | `adsr(0).set().dshp(0)` | `NodeId::Adsr(0).inp_param("dshp")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `adsr(0).set().dshp(0.5)` | `NodeId::Adsr(0).inp_param("dshp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `adsr(0).set().dshp(1)` | `NodeId::Adsr(0).inp_param("dshp")` |
#### NodeId::Adsr input rshp
Release shape. This allows you to change the shape of the release stage from a logarithmic, to a linear and to an exponential shape.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `adsr(0).set().rshp(0.5)` | `NodeId::Adsr(0).inp_param("rshp")` |
| **min** |  0.0000 |      0.00 |  0.000 | `adsr(0).set().rshp(0)` | `NodeId::Adsr(0).inp_param("rshp")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `adsr(0).set().rshp(0.5)` | `NodeId::Adsr(0).inp_param("rshp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `adsr(0).set().rshp(1)` | `NodeId::Adsr(0).inp_param("rshp")` |
#### NodeId::Adsr setting mult
Attack and Decay time range multiplier. This will extend the maximum range of the `atk`, `dcy` and `rel` parameters.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | x1 | `adsr(0).set().mult(0)` | `NodeId::Adsr(0).inp_param("mult")` |
| 1 | x10 | `adsr(0).set().mult(1)` | `NodeId::Adsr(0).inp_param("mult")` |
| 2 | x100 | `adsr(0).set().mult(2)` | `NodeId::Adsr(0).inp_param("mult")` |
### NodeId::TsLFO
**TriSaw LFO**

This simple LFO has a configurable waveform.
You can blend between triangular to sawtooth waveforms using the `rev` parameter.

- [input **time**](#nodeidtslfo-input-time) - The frequency or period time of the LFO, goes all the way from **0.1ms** up to **30s**. Please note, that the text entry is always in milliseconds.
- [input **trig**](#nodeidtslfo-input-trig) - Triggers a phase reset of the LFO.
- [input **rev**](#nodeidtslfo-input-rev) - The reverse point of the LFO waveform. At **0.5** the LFO will follow a triangle waveform. At **0.0** or **1.0** the LFO waveform will be (almost) a (reversed) saw tooth. Node: A perfect sawtooth can not be achieved with this oscillator, as there will always be a minimal rise/fall time.
- output **sig**
The LFO output.
#### NodeId::TsLFO Help
**TriSaw LFO**

This simple LFO has a configurable waveform. You can blend between
triangular to sawtooth waveforms using the `rev` parameter.

Using the `trig` input you can reset the LFO phase, which allows to use it
kind of like an envelope.

#### NodeId::TsLFO input time
The frequency or period time of the LFO, goes all the way from **0.1ms** up to **30s**. Please note, that the text entry is always in milliseconds.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.3865 |   1000.00 | 1000ms | `tslfo(0).set().time(999.9997)` | `NodeId::TsLFO(0).inp_param("time")` |
| **min** |  0.0000 |      0.10 | 10000.0Hz | `tslfo(0).set().time(0.1)` | `NodeId::TsLFO(0).inp_param("time")` |
| **mid** |  0.5000 |   4687.60 |  4.69s | `tslfo(0).set().time(4687.5986)` | `NodeId::TsLFO(0).inp_param("time")` |
| **max** |  1.0000 | 300000.00 | 300.0s | `tslfo(0).set().time(300000)` | `NodeId::TsLFO(0).inp_param("time")` |
#### NodeId::TsLFO input trig
Triggers a phase reset of the LFO.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `tslfo(0).set().trig(0)` | `NodeId::TsLFO(0).inp_param("trig")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `tslfo(0).set().trig(-1)` | `NodeId::TsLFO(0).inp_param("trig")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `tslfo(0).set().trig(0)` | `NodeId::TsLFO(0).inp_param("trig")` |
| **max** |  1.0000 |      1.00 |  1.000 | `tslfo(0).set().trig(1)` | `NodeId::TsLFO(0).inp_param("trig")` |
#### NodeId::TsLFO input rev
The reverse point of the LFO waveform. At **0.5** the LFO will follow a triangle waveform. At **0.0** or **1.0** the LFO waveform will be (almost) a (reversed) saw tooth. Node: A perfect sawtooth can not be achieved with this oscillator, as there will always be a minimal rise/fall time.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `tslfo(0).set().rev(0.5)` | `NodeId::TsLFO(0).inp_param("rev")` |
| **min** |  0.0000 |      0.00 |  0.000 | `tslfo(0).set().rev(0)` | `NodeId::TsLFO(0).inp_param("rev")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `tslfo(0).set().rev(0.5)` | `NodeId::TsLFO(0).inp_param("rev")` |
| **max** |  1.0000 |      1.00 |  1.000 | `tslfo(0).set().rev(1)` | `NodeId::TsLFO(0).inp_param("rev")` |
### NodeId::RndWk
**Random Walker**

This modulator generates a random number by walking a pre defined maximum random `step` width.
For smoother transitions a slew rate limiter is integrated.

- [input **trig**](#nodeidrndwk-input-trig) - This trigger generates a new random number within the current `min`/`max` range.
- [input **step**](#nodeidrndwk-input-step) - This is the maximum possible step size of the random number drawn upon `trig`. Setting this to **0.0** will disable the randomness. The minimum step size can be defined by the `offs` parameter.
- [input **offs**](#nodeidrndwk-input-offs) - The minimum step size and direction that is done on each `trig`.Depending on the size of the `offs` and the `min`/`max` range, this might result in the output value being close to the limits of that range.
- [input **min**](#nodeidrndwk-input-min) - The minimum of the new target value. If a value is drawn that is outside of this range, it will be reflected back into it.
- [input **max**](#nodeidrndwk-input-max) - The maximum of the new target value. If a value is drawn that is outside of this range, it will be reflected back into it.
- [input **slew**](#nodeidrndwk-input-slew) - The slew rate limiting time. Thats the time it takes to get to **1.0** from **0.0**. Useful for smoothing modulation of audio signals. The higher the time, the smoother/slower the transition to new target values will be.
- output **sig**
Oscillator output
#### NodeId::RndWk Help
**Random Walker**

This modulator generates a random number by walking a pre defined
maximum random `step` width. The newly generated target value will always
be folded within the defined `min`/`max` range. The `offs` parameter defines a
minimal step width each `trig` has to change the target value.

For smoother transitions, if you want to modulate an audio signal with this,
a slew rate limiter (`slew`) is integrated.

You can disable all randomness by setting `step` to **0.0**.

Tip: Interesting and smooth results can be achieved if you set `slew`
to a (way) longer time than the `trig` interval. It will smooth
off the step widths and the overall motion even more.

#### NodeId::RndWk input trig
This trigger generates a new random number within the current `min`/`max` range.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `rndwk(0).set().trig(0)` | `NodeId::RndWk(0).inp_param("trig")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `rndwk(0).set().trig(-1)` | `NodeId::RndWk(0).inp_param("trig")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `rndwk(0).set().trig(0)` | `NodeId::RndWk(0).inp_param("trig")` |
| **max** |  1.0000 |      1.00 |  1.000 | `rndwk(0).set().trig(1)` | `NodeId::RndWk(0).inp_param("trig")` |
#### NodeId::RndWk input step
This is the maximum possible step size of the random number drawn upon `trig`. Setting this to **0.0** will disable the randomness.
The minimum step size can be defined by the `offs` parameter.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.2000 |      0.20 |  0.200 | `rndwk(0).set().step(0.2)` | `NodeId::RndWk(0).inp_param("step")` |
| **min** |  0.0000 |      0.00 |  0.000 | `rndwk(0).set().step(0)` | `NodeId::RndWk(0).inp_param("step")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `rndwk(0).set().step(0.5)` | `NodeId::RndWk(0).inp_param("step")` |
| **max** |  1.0000 |      1.00 |  1.000 | `rndwk(0).set().step(1)` | `NodeId::RndWk(0).inp_param("step")` |
#### NodeId::RndWk input offs
The minimum step size and direction that is done on each `trig`.Depending on the size of the `offs` and the `min`/`max` range, this might result in the output value being close to the limits of that range.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `rndwk(0).set().offs(0)` | `NodeId::RndWk(0).inp_param("offs")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `rndwk(0).set().offs(-1)` | `NodeId::RndWk(0).inp_param("offs")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `rndwk(0).set().offs(0)` | `NodeId::RndWk(0).inp_param("offs")` |
| **max** |  1.0000 |      1.00 |  1.000 | `rndwk(0).set().offs(1)` | `NodeId::RndWk(0).inp_param("offs")` |
#### NodeId::RndWk input min
The minimum of the new target value. If a value is drawn that is outside of this range, it will be reflected back into it.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `rndwk(0).set().min(0)` | `NodeId::RndWk(0).inp_param("min")` |
| **min** |  0.0000 |      0.00 |  0.000 | `rndwk(0).set().min(0)` | `NodeId::RndWk(0).inp_param("min")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `rndwk(0).set().min(0.5)` | `NodeId::RndWk(0).inp_param("min")` |
| **max** |  1.0000 |      1.00 |  1.000 | `rndwk(0).set().min(1)` | `NodeId::RndWk(0).inp_param("min")` |
#### NodeId::RndWk input max
The maximum of the new target value. If a value is drawn that is outside of this range, it will be reflected back into it.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `rndwk(0).set().max(1)` | `NodeId::RndWk(0).inp_param("max")` |
| **min** |  0.0000 |      0.00 |  0.000 | `rndwk(0).set().max(0)` | `NodeId::RndWk(0).inp_param("max")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `rndwk(0).set().max(0.5)` | `NodeId::RndWk(0).inp_param("max")` |
| **max** |  1.0000 |      1.00 |  1.000 | `rndwk(0).set().max(1)` | `NodeId::RndWk(0).inp_param("max")` |
#### NodeId::RndWk input slew
The slew rate limiting time. Thats the time it takes to get to **1.0** from **0.0**. Useful for smoothing modulation of audio signals. The higher the time, the smoother/slower the transition to new target values will be.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.1225 |     75.00 | 75.00ms | `rndwk(0).set().slew(75)` | `NodeId::RndWk(0).inp_param("slew")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `rndwk(0).set().slew(0)` | `NodeId::RndWk(0).inp_param("slew")` |
| **mid** |  0.5000 |   1250.00 |   1250ms | `rndwk(0).set().slew(1250)` | `NodeId::RndWk(0).inp_param("slew")` |
| **max** |  1.0000 |   5000.00 |   5000ms | `rndwk(0).set().slew(5000)` | `NodeId::RndWk(0).inp_param("slew")` |
### NodeId::Delay
**Simple Delay Line**

This is a very simple single buffer delay node.
It provides an internal feedback and dry/wet mix.

- [input **inp**](#nodeiddelay-input-inp) - The signal input for the delay. You can mix in this input to the output with the `mix` parameter.
- [input **trig**](#nodeiddelay-input-trig) - If you set `mode` to **Sync** the delay time will be synchronized to the trigger signals received on this input.
- [input **time**](#nodeiddelay-input-time) - The delay time. It can be freely modulated to your likings.
- [input **fb**](#nodeiddelay-input-fb) - The feedback amount of the delay output to it's input. 
- [input **mix**](#nodeiddelay-input-mix) - The dry/wet mix of the delay.
- [setting **mode**](#nodeiddelay-setting-mode) - Allows different operating modes of the delay. **Time** is the default, and means that the `time` input specifies the delay time. **Sync** will synchronize the delay time with the trigger signals on the `trig` input.
- output **sig**
The output of the dry/wet mix.
#### NodeId::Delay Help
**A Simple Delay Line**

This node provides a very simple delay line with the bare minimum of
parameters. Most importantly a freely modulateable `time` parameter
and a feedback `fb` parameter.

Via the `mix` parameter you can mix in the input signal to the output.

You can use this node to delay any kind of signal, from a simple control
signal to an audio signal.

For other kinds of delay/feedback please see also the `FbWr`/`FbRd` nodes.

#### NodeId::Delay input inp
The signal input for the delay. You can mix in this input to the output with the `mix` parameter.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `delay(0).set().inp(0)` | `NodeId::Delay(0).inp_param("inp")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `delay(0).set().inp(-1)` | `NodeId::Delay(0).inp_param("inp")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `delay(0).set().inp(0)` | `NodeId::Delay(0).inp_param("inp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `delay(0).set().inp(1)` | `NodeId::Delay(0).inp_param("inp")` |
#### NodeId::Delay input trig
If you set `mode` to **Sync** the delay time will be synchronized to the trigger signals received on this input.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `delay(0).set().trig(0)` | `NodeId::Delay(0).inp_param("trig")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `delay(0).set().trig(-1)` | `NodeId::Delay(0).inp_param("trig")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `delay(0).set().trig(0)` | `NodeId::Delay(0).inp_param("trig")` |
| **max** |  1.0000 |      1.00 |  1.000 | `delay(0).set().trig(1)` | `NodeId::Delay(0).inp_param("trig")` |
#### NodeId::Delay input time
The delay time. It can be freely modulated to your likings.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.2234 |    250.00 | 250.0ms | `delay(0).set().time(249.99998)` | `NodeId::Delay(0).inp_param("time")` |
| **min** |  0.0000 |      0.50 |  0.50ms | `delay(0).set().time(0.5)` | `NodeId::Delay(0).inp_param("time")` |
| **mid** |  0.5000 |   1250.38 |   1250ms | `delay(0).set().time(1250.375)` | `NodeId::Delay(0).inp_param("time")` |
| **max** |  1.0000 |   5000.00 |   5000ms | `delay(0).set().time(5000)` | `NodeId::Delay(0).inp_param("time")` |
#### NodeId::Delay input fb
The feedback amount of the delay output to it's input. 

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `delay(0).set().fb(0)` | `NodeId::Delay(0).inp_param("fb")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `delay(0).set().fb(-1)` | `NodeId::Delay(0).inp_param("fb")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `delay(0).set().fb(0)` | `NodeId::Delay(0).inp_param("fb")` |
| **max** |  1.0000 |      1.00 |  1.000 | `delay(0).set().fb(1)` | `NodeId::Delay(0).inp_param("fb")` |
#### NodeId::Delay input mix
The dry/wet mix of the delay.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `delay(0).set().mix(0.5)` | `NodeId::Delay(0).inp_param("mix")` |
| **min** |  0.0000 |      0.00 |  0.000 | `delay(0).set().mix(0)` | `NodeId::Delay(0).inp_param("mix")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `delay(0).set().mix(0.5)` | `NodeId::Delay(0).inp_param("mix")` |
| **max** |  1.0000 |      1.00 |  1.000 | `delay(0).set().mix(1)` | `NodeId::Delay(0).inp_param("mix")` |
#### NodeId::Delay setting mode
Allows different operating modes of the delay. **Time** is the default, and means that the `time` input specifies the delay time. **Sync** will synchronize the delay time with the trigger signals on the `trig` input.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Time | `delay(0).set().mode(0)` | `NodeId::Delay(0).inp_param("mode")` |
| 1 | Sync | `delay(0).set().mode(1)` | `NodeId::Delay(0).inp_param("mode")` |
### NodeId::AllP
**Single Allpass Filter**
This is an allpass filter that can be used to build reverbs
or anything you might find it useful for.

- [input **inp**](#nodeidallp-input-inp) - The signal input for the allpass filter.
- [input **time**](#nodeidallp-input-time) - The allpass delay time.
- [input **g**](#nodeidallp-input-g) - The internal factor for the allpass filter.
- output **sig**
The output of allpass filter.
#### NodeId::AllP Help
**A Simple Single Allpass Filter**

This is an allpass filter that can be used to build reverbs
or anything you might find it useful for.

Typical arrangements are (Schroeder Reverb):

```text
                    t=4.5ms
                    g=0.7   -> Comb
    AllP -> AllP -> AllP -> -> Comb
    t=42ms  t=13.5ms        -> Comb
    g=0.7   g=0.7           -> Comb
```

Or:

```text
    Comb ->                 t=0.48ms
    Comb ->                 g=0.7
    Comb -> AllP -> AllP -> AllP
    Comb -> t=5ms   t=1.68ms
            g=0.7   g=0.7
```

Typical values for the comb filters are in the range `g`=**0.6** to **0.9**
and time in the range of **30ms** to **250ms**.

Feel free to deviate from this and experiment around.

Building your own reverbs is fun!

(And don't forget that you can create feedback using the `FbWr` and `FbRd` nodes!)

#### NodeId::AllP input inp
The signal input for the allpass filter.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `allp(0).set().inp(0)` | `NodeId::AllP(0).inp_param("inp")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `allp(0).set().inp(-1)` | `NodeId::AllP(0).inp_param("inp")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `allp(0).set().inp(0)` | `NodeId::AllP(0).inp_param("inp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `allp(0).set().inp(1)` | `NodeId::AllP(0).inp_param("inp")` |
#### NodeId::AllP input time
The allpass delay time.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.1578 |     25.00 | 25.00ms | `allp(0).set().time(24.999996)` | `NodeId::AllP(0).inp_param("time")` |
| **min** |  0.0000 |      0.10 |  0.10ms | `allp(0).set().time(0.1)` | `NodeId::AllP(0).inp_param("time")` |
| **mid** |  0.5000 |    250.07 | 250.1ms | `allp(0).set().time(250.075)` | `NodeId::AllP(0).inp_param("time")` |
| **max** |  1.0000 |   1000.00 |   1000ms | `allp(0).set().time(1000)` | `NodeId::AllP(0).inp_param("time")` |
#### NodeId::AllP input g
The internal factor for the allpass filter.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.7000 |      0.70 |  0.700 | `allp(0).set().g(0.7)` | `NodeId::AllP(0).inp_param("g")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `allp(0).set().g(-1)` | `NodeId::AllP(0).inp_param("g")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `allp(0).set().g(0)` | `NodeId::AllP(0).inp_param("g")` |
| **max** |  1.0000 |      1.00 |  1.000 | `allp(0).set().g(1)` | `NodeId::AllP(0).inp_param("g")` |
### NodeId::Comb
**Comb Filter**

A very simple comb filter. It has interesting filtering effects
and can also be used to build custom reverbs.

- [input **inp**](#nodeidcomb-input-inp) - The signal input for the comb filter.
- [input **time**](#nodeidcomb-input-time) - The comb delay time.
- [input **g**](#nodeidcomb-input-g) - The internal factor for the comb filter. Be careful with high `g` values (> **0.75**) in feedback mode, you will probably have to attenuate the output a bit yourself.
- [setting **mode**](#nodeidcomb-setting-mode) - The mode of the comb filter, whether it's a feedback or feedforward comb filter.
- output **sig**
The output of comb filter.
#### NodeId::Comb Help
**A Simple Comb Filter**

This is a comb filter that can be used for filtering
as well as for building reverbs or anything you might
find it useful for.

Attention: Be careful with high `g` values, you might need to
attenuate the output manually for the feedback combfilter mode,
because the feedback adds up quickly.

For typical arrangements in combination with allpass filters,
see the documentation of the `AllP` node!

#### NodeId::Comb input inp
The signal input for the comb filter.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `comb(0).set().inp(0)` | `NodeId::Comb(0).inp_param("inp")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `comb(0).set().inp(-1)` | `NodeId::Comb(0).inp_param("inp")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `comb(0).set().inp(0)` | `NodeId::Comb(0).inp_param("inp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `comb(0).set().inp(1)` | `NodeId::Comb(0).inp_param("inp")` |
#### NodeId::Comb input time
The comb delay time.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.1578 |     25.00 | 25.00ms | `comb(0).set().time(24.999996)` | `NodeId::Comb(0).inp_param("time")` |
| **min** |  0.0000 |      0.10 |  0.10ms | `comb(0).set().time(0.1)` | `NodeId::Comb(0).inp_param("time")` |
| **mid** |  0.5000 |    250.07 | 250.1ms | `comb(0).set().time(250.075)` | `NodeId::Comb(0).inp_param("time")` |
| **max** |  1.0000 |   1000.00 |   1000ms | `comb(0).set().time(1000)` | `NodeId::Comb(0).inp_param("time")` |
#### NodeId::Comb input g
The internal factor for the comb filter. Be careful with high `g` values (> **0.75**) in feedback mode, you will probably have to attenuate the output a bit yourself.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.7000 |      0.70 |  0.700 | `comb(0).set().g(0.7)` | `NodeId::Comb(0).inp_param("g")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `comb(0).set().g(-1)` | `NodeId::Comb(0).inp_param("g")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `comb(0).set().g(0)` | `NodeId::Comb(0).inp_param("g")` |
| **max** |  1.0000 |      1.00 |  1.000 | `comb(0).set().g(1)` | `NodeId::Comb(0).inp_param("g")` |
#### NodeId::Comb setting mode
The mode of the comb filter, whether it's a feedback or feedforward comb filter.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | FedBack | `comb(0).set().mode(0)` | `NodeId::Comb(0).inp_param("mode")` |
| 1 | FedForw | `comb(0).set().mode(1)` | `NodeId::Comb(0).inp_param("mode")` |
### NodeId::Noise
**Noise Oscillator**

This is a very simple noise oscillator, which can be used for any kind of audio rate noise.
And as a source for sample & hold like nodes to generate low frequency modulation. The white
noise is uniformly distributed and not normal distributed (which could be a bit more natural
in some contexts). See also the `XNoise` node for more noise alternatives.

- [input **atv**](#nodeidnoise-input-atv) - Attenuverter input, to attenuate or invert the noise
- [input **offs**](#nodeidnoise-input-offs) - Offset input, that is added to the output signal after attenuvertig it.
- [setting **mode**](#nodeidnoise-setting-mode) - You can switch between **Bipolar** noise, which uses the full range from **-1** to **1**, or **Unipolar** noise that only uses the range from **0** to **1**.
- output **sig**
The noise output.
#### NodeId::Noise Help
**A Simple Noise Oscillator**

This is a very simple noise oscillator, which can be used for
any kind of audio rate noise. And as a source for sample & hold
like nodes to generate low frequency modulation.

The noise follows a uniform distribution. That means all amplitudes are equally likely to occur.
While it might sound similar, white noise is usually following a normal distribution, which makes
some amplitudes more likely to occur than others.
See also the `XNoise` node for more noise alternatives.

The `atv` attenuverter and `offs` parameters control the value range
of the noise, and the `mode` allows to switch the oscillator between
unipolar and bipolar output.

#### NodeId::Noise input atv
Attenuverter input, to attenuate or invert the noise

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `noise(0).set().atv(0.5)` | `NodeId::Noise(0).inp_param("atv")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `noise(0).set().atv(-1)` | `NodeId::Noise(0).inp_param("atv")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `noise(0).set().atv(0)` | `NodeId::Noise(0).inp_param("atv")` |
| **max** |  1.0000 |      1.00 |  1.000 | `noise(0).set().atv(1)` | `NodeId::Noise(0).inp_param("atv")` |
#### NodeId::Noise input offs
Offset input, that is added to the output signal after attenuvertig it.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `noise(0).set().offs(0)` | `NodeId::Noise(0).inp_param("offs")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `noise(0).set().offs(-1)` | `NodeId::Noise(0).inp_param("offs")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `noise(0).set().offs(0)` | `NodeId::Noise(0).inp_param("offs")` |
| **max** |  1.0000 |      1.00 |  1.000 | `noise(0).set().offs(1)` | `NodeId::Noise(0).inp_param("offs")` |
#### NodeId::Noise setting mode
You can switch between **Bipolar** noise, which uses the full range from **-1** to **1**, or **Unipolar** noise that only uses the range from **0** to **1**.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Bipolar | `noise(0).set().mode(0)` | `NodeId::Noise(0).inp_param("mode")` |
| 1 | Unipolar | `noise(0).set().mode(1)` | `NodeId::Noise(0).inp_param("mode")` |
### NodeId::FormFM
**Formant oscillator**

Simple formant oscillator that generates a formant like sound.
Loosely based on the ModFM synthesis method.

- [input **freq**](#nodeidformfm-input-freq) - Base frequency to oscillate at 
- [input **det**](#nodeidformfm-input-det) - Detune the oscillator in semitones and cents. 
- [input **form**](#nodeidformfm-input-form) - Frequency of the formant This affects how much lower or higher tones the sound has.
- [input **side**](#nodeidformfm-input-side) - Which side the peak of the wave is. Values more towards **0.0** or **1.0** make the base frequency more pronounced
- [input **peak**](#nodeidformfm-input-peak) - How high the peak amplitude is. Lower values make the effect more pronounced
- output **sig**
Generated formant signal
#### NodeId::FormFM Help
**Direct formant synthesizer**

This is a formant synthesizer that directly generates 
the audio of a single formant.

This can be seen as passing a saw wave with frequency `freq` 
into a bandpass filter with the cutoff at `form`

`freq` controls the base frequency of the formant.
`form` controls the formant frequency. Lower values give more bass to the sound,
and higher values give the high frequencies more sound.

`side` controls where the peak of the carrier wave is, 
and in turn controls the bandwidth of the effect. The more towards **0.0** or **1.0**,
the more the formant is audible.

`peak` controls how high the peak of the carrier wave is.
This also controls the bandwidth of the effect, where lower means a higher 
bandwidth, and thus more audible formant.

#### NodeId::FormFM input freq
Base frequency to oscillate at


| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |    440.00 |  440.0Hz | `formfm(0).set().freq(440)` | `NodeId::FormFM(0).inp_param("freq")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `formfm(0).set().freq(0.4296875)` | `NodeId::FormFM(0).inp_param("freq")` |
| **mid** | -0.2176 |     97.34 |  97.34Hz | `formfm(0).set().freq(97.33759)` | `NodeId::FormFM(0).inp_param("freq")` |
| **max** |  0.5647 |  22049.99 |  22050Hz | `formfm(0).set().freq(22049.994)` | `NodeId::FormFM(0).inp_param("freq")` |
#### NodeId::FormFM input det
Detune the oscillator in semitones and cents.


| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0s | `formfm(0).set().det(0)` | `NodeId::FormFM(0).inp_param("det")` |
| **min** | -0.2000 |    -24.00 | -24s | `formfm(0).set().det(-24)` | `NodeId::FormFM(0).inp_param("det")` |
| **mid** |  0.0000 |      0.00 |  0s | `formfm(0).set().det(0)` | `NodeId::FormFM(0).inp_param("det")` |
| **max** |  0.2000 |     24.00 | 24s | `formfm(0).set().det(24)` | `NodeId::FormFM(0).inp_param("det")` |
#### NodeId::FormFM input form
Frequency of the formant
This affects how much lower or higher tones the sound has.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |    440.00 |  440.0Hz | `formfm(0).set().form(440)` | `NodeId::FormFM(0).inp_param("form")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `formfm(0).set().form(0.4296875)` | `NodeId::FormFM(0).inp_param("form")` |
| **mid** | -0.2176 |     97.34 |  97.34Hz | `formfm(0).set().form(97.33759)` | `NodeId::FormFM(0).inp_param("form")` |
| **max** |  0.5647 |  22049.99 |  22050Hz | `formfm(0).set().form(22049.994)` | `NodeId::FormFM(0).inp_param("form")` |
#### NodeId::FormFM input side
Which side the peak of the wave is. Values more towards **0.0** or **1.0** make the base frequency more pronounced

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.2000 |      0.20 |  0.200 | `formfm(0).set().side(0.2)` | `NodeId::FormFM(0).inp_param("side")` |
| **min** |  0.0000 |      0.00 |  0.000 | `formfm(0).set().side(0)` | `NodeId::FormFM(0).inp_param("side")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `formfm(0).set().side(0.5)` | `NodeId::FormFM(0).inp_param("side")` |
| **max** |  1.0000 |      1.00 |  1.000 | `formfm(0).set().side(1)` | `NodeId::FormFM(0).inp_param("side")` |
#### NodeId::FormFM input peak
How high the peak amplitude is. Lower values make the effect more pronounced

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.4000 |      0.40 |  0.400 | `formfm(0).set().peak(0.4)` | `NodeId::FormFM(0).inp_param("peak")` |
| **min** |  0.0000 |      0.00 |  0.000 | `formfm(0).set().peak(0)` | `NodeId::FormFM(0).inp_param("peak")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `formfm(0).set().peak(0.5)` | `NodeId::FormFM(0).inp_param("peak")` |
| **max** |  1.0000 |      1.00 |  1.000 | `formfm(0).set().peak(1)` | `NodeId::FormFM(0).inp_param("peak")` |
### NodeId::SFilter
**Simple Filter**

This is a collection of more or less simple filters.
There are only two parameters: Filter cutoff `freq` and the `res` resonance.

- [input **inp**](#nodeidsfilter-input-inp) - Signal input
- [input **freq**](#nodeidsfilter-input-freq) - Filter cutoff frequency.
- [input **res**](#nodeidsfilter-input-res) - Filter resonance.
- [setting **ftype**](#nodeidsfilter-setting-ftype) - The filter type, there are varying types of filters available. Please consult the node documentation for a complete list. Types: **1p/1pt**=one poles, **12c**=Hal Chamberlin SVF, **12s**=Simper SVF, **24m**=Moog Outputs: **LP**=Low-,**HP**=High-,**BP**=Band-Pass,**NO**=Notch,**PK**=Peak
- output **sig**
Filtered signal output.
#### NodeId::SFilter Help
**Simple Audio Filter Collection**

This is a collection of a few more or less simple filters
of varying types. There are only few parameters for you to change: `freq`
and `res` resonance. You can switch between the types with the `ftype`.
There are currently following filters available:

- **HP 1p** - One pole low-pass filter (6db)
- **HP 1pt** - One pole low-pass filter (6db) (TPT form)
- **LP 1p** - One pole high-pass filter (6db)
- **LP 1pt** - One pole high-pass filter (6db) (TPT form)

The Hal Chamberlin filters are an older state variable filter design,
that is limited to max cutoff frequency of 16kHz. For a more stable
filter use the "12s" variants.

- **LP 12c** - Low-pass Hal Chamberlin state variable filter (12dB)
- **HP 12c** - High-pass Hal Chamberlin state variable filter (12dB)
- **BP 12c** - Band-pass Hal Chamberlin state variable filter (12dB)
- **NO 12c** - Notch Hal Chamberlin state variable filter (12dB)

The (Andrew) Simper state variable filter is a newer design
and stable up to 22kHz at 44.1kHz sampling rate. It's overall more precise
and less quirky than the Hal Chamberlin SVF.

- **LP 12s** - Low-pass Simper state variable filter (12dB)
- **HP 12s** - High-pass Simper state variable filter (12dB)
- **BP 12s** - Band-pass Simper state variable filter (12dB)
- **NO 12s** - Notch Simper state variable filter (12dB)
- **PK 12s** - Peak Simper state variable filter (12dB)

For a more colored filter reach for the Stilson/Moog filter with a 24dB
fall off per octave. Beware high cutoff frequencies for this filter,
as it can become quite unstable.

- **LP 24m** - Low-pass Stilson/Moog filter (24dB)


#### NodeId::SFilter input inp
Signal input

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `sfilter(0).set().inp(0)` | `NodeId::SFilter(0).inp_param("inp")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `sfilter(0).set().inp(-1)` | `NodeId::SFilter(0).inp_param("inp")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `sfilter(0).set().inp(0)` | `NodeId::SFilter(0).inp_param("inp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `sfilter(0).set().inp(1)` | `NodeId::SFilter(0).inp_param("inp")` |
#### NodeId::SFilter input freq
Filter cutoff frequency.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.1184 |   1000.00 |   1000Hz | `sfilter(0).set().freq(1000)` | `NodeId::SFilter(0).inp_param("freq")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `sfilter(0).set().freq(0.4296875)` | `NodeId::SFilter(0).inp_param("freq")` |
| **mid** | -0.2176 |     97.34 |  97.34Hz | `sfilter(0).set().freq(97.33759)` | `NodeId::SFilter(0).inp_param("freq")` |
| **max** |  0.5647 |  22049.99 |  22050Hz | `sfilter(0).set().freq(22049.994)` | `NodeId::SFilter(0).inp_param("freq")` |
#### NodeId::SFilter input res
Filter resonance.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `sfilter(0).set().res(0.5)` | `NodeId::SFilter(0).inp_param("res")` |
| **min** |  0.0000 |      0.00 |  0.000 | `sfilter(0).set().res(0)` | `NodeId::SFilter(0).inp_param("res")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `sfilter(0).set().res(0.5)` | `NodeId::SFilter(0).inp_param("res")` |
| **max** |  1.0000 |      1.00 |  1.000 | `sfilter(0).set().res(1)` | `NodeId::SFilter(0).inp_param("res")` |
#### NodeId::SFilter setting ftype
The filter type, there are varying types of filters available. Please consult the node documentation for a complete list.
Types: **1p/1pt**=one poles, **12c**=Hal Chamberlin SVF,
**12s**=Simper SVF, **24m**=Moog
Outputs: **LP**=Low-,**HP**=High-,**BP**=Band-Pass,**NO**=Notch,**PK**=Peak

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | LP 1p | `sfilter(0).set().ftype(0)` | `NodeId::SFilter(0).inp_param("ftype")` |
| 1 | LP 1pt | `sfilter(0).set().ftype(1)` | `NodeId::SFilter(0).inp_param("ftype")` |
| 2 | HP 1p | `sfilter(0).set().ftype(2)` | `NodeId::SFilter(0).inp_param("ftype")` |
| 3 | HP 1pt | `sfilter(0).set().ftype(3)` | `NodeId::SFilter(0).inp_param("ftype")` |
| 4 | LP 12c | `sfilter(0).set().ftype(4)` | `NodeId::SFilter(0).inp_param("ftype")` |
| 5 | HP 12c | `sfilter(0).set().ftype(5)` | `NodeId::SFilter(0).inp_param("ftype")` |
| 6 | BP 12c | `sfilter(0).set().ftype(6)` | `NodeId::SFilter(0).inp_param("ftype")` |
| 7 | NO 12c | `sfilter(0).set().ftype(7)` | `NodeId::SFilter(0).inp_param("ftype")` |
| 8 | LP 12s | `sfilter(0).set().ftype(8)` | `NodeId::SFilter(0).inp_param("ftype")` |
| 9 | HP 12s | `sfilter(0).set().ftype(9)` | `NodeId::SFilter(0).inp_param("ftype")` |
| 10 | BP 12s | `sfilter(0).set().ftype(10)` | `NodeId::SFilter(0).inp_param("ftype")` |
| 11 | NO 12s | `sfilter(0).set().ftype(11)` | `NodeId::SFilter(0).inp_param("ftype")` |
| 12 | PK 12s | `sfilter(0).set().ftype(12)` | `NodeId::SFilter(0).inp_param("ftype")` |
| 13 | LP 24m | `sfilter(0).set().ftype(13)` | `NodeId::SFilter(0).inp_param("ftype")` |
### NodeId::FVaFilt
**F's Virtual Analog (Stereo) Filter**

This is a collection of virtual analog filters that were implemented
by Fredemus (aka Frederik Halkjær). They behave well when driven hard
but that comes with the price that they are more expensive.

- [input **inp**](#nodeidfvafilt-input-inp) - Signal input
- [input **freq**](#nodeidfvafilt-input-freq) - Filter cutoff frequency.
- [input **res**](#nodeidfvafilt-input-res) - Filter resonance.
- [input **drive**](#nodeidfvafilt-input-drive) - Filter (over) drive.
- [setting **ftype**](#nodeidfvafilt-setting-ftype) - The filter type, there are varying types of filters available: - **Ladder** - **SVF** - **Sallen Key** 
- [setting **smode**](#nodeidfvafilt-setting-smode) - SVF Filter Mode - **LP** - Low pass - **HP** - High pass - **BP1** - Band pass 1 - **BP2** - Band pass 2 - **Notch** - Notch 
- [setting **lslope**](#nodeidfvafilt-setting-lslope) - Ladder Slope Available slopes: **6dB**, **12dB**, **18dB**, **24dB**
- output **sig**
Filtered signal output.
#### NodeId::FVaFilt Help
**Frederik Halkjær Virtual Analog Stereo Filters**

#### NodeId::FVaFilt input inp
Signal input

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `fvafilt(0).set().inp(0)` | `NodeId::FVaFilt(0).inp_param("inp")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `fvafilt(0).set().inp(-1)` | `NodeId::FVaFilt(0).inp_param("inp")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `fvafilt(0).set().inp(0)` | `NodeId::FVaFilt(0).inp_param("inp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `fvafilt(0).set().inp(1)` | `NodeId::FVaFilt(0).inp_param("inp")` |
#### NodeId::FVaFilt input freq
Filter cutoff frequency.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.1184 |   1000.00 |   1000Hz | `fvafilt(0).set().freq(1000)` | `NodeId::FVaFilt(0).inp_param("freq")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `fvafilt(0).set().freq(0.4296875)` | `NodeId::FVaFilt(0).inp_param("freq")` |
| **mid** | -0.2247 |     92.70 |  92.70Hz | `fvafilt(0).set().freq(92.704)` | `NodeId::FVaFilt(0).inp_param("freq")` |
| **max** |  0.5506 |  20000.66 |  20001Hz | `fvafilt(0).set().freq(20000.658)` | `NodeId::FVaFilt(0).inp_param("freq")` |
#### NodeId::FVaFilt input res
Filter resonance.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `fvafilt(0).set().res(0.5)` | `NodeId::FVaFilt(0).inp_param("res")` |
| **min** |  0.0000 |      0.00 |  0.000 | `fvafilt(0).set().res(0)` | `NodeId::FVaFilt(0).inp_param("res")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `fvafilt(0).set().res(0.5)` | `NodeId::FVaFilt(0).inp_param("res")` |
| **max** |  1.0000 |      1.00 |  1.000 | `fvafilt(0).set().res(1)` | `NodeId::FVaFilt(0).inp_param("res")` |
#### NodeId::FVaFilt input drive
Filter (over) drive.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      1.00 | +0.0dB | `fvafilt(0).set().drive(1)` | `NodeId::FVaFilt(0).inp_param("drive")` |
| **min** |  0.0000 |      1.00 | +0.0dB | `fvafilt(0).set().drive(1)` | `NodeId::FVaFilt(0).inp_param("drive")` |
| **mid** |  0.5000 |      3.98 | +12.0dB | `fvafilt(0).set().drive(3.981072)` | `NodeId::FVaFilt(0).inp_param("drive")` |
| **max** |  1.0000 |     15.85 | +24.0dB | `fvafilt(0).set().drive(15.848933)` | `NodeId::FVaFilt(0).inp_param("drive")` |
#### NodeId::FVaFilt setting ftype
The filter type, there are varying types of filters available:
- **Ladder**
- **SVF**
- **Sallen Key**


| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Ladder | `fvafilt(0).set().ftype(0)` | `NodeId::FVaFilt(0).inp_param("ftype")` |
| 1 | SVF | `fvafilt(0).set().ftype(1)` | `NodeId::FVaFilt(0).inp_param("ftype")` |
| 2 | SallenKey | `fvafilt(0).set().ftype(2)` | `NodeId::FVaFilt(0).inp_param("ftype")` |
#### NodeId::FVaFilt setting smode
SVF Filter Mode
- **LP** - Low pass
- **HP** - High pass
- **BP1** - Band pass 1
- **BP2** - Band pass 2
- **Notch** - Notch


| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | LP | `fvafilt(0).set().smode(0)` | `NodeId::FVaFilt(0).inp_param("smode")` |
| 1 | HP | `fvafilt(0).set().smode(1)` | `NodeId::FVaFilt(0).inp_param("smode")` |
| 2 | BP1 | `fvafilt(0).set().smode(2)` | `NodeId::FVaFilt(0).inp_param("smode")` |
| 3 | BP2 | `fvafilt(0).set().smode(3)` | `NodeId::FVaFilt(0).inp_param("smode")` |
| 4 | Notch | `fvafilt(0).set().smode(4)` | `NodeId::FVaFilt(0).inp_param("smode")` |
#### NodeId::FVaFilt setting lslope
Ladder Slope
Available slopes: **6dB**, **12dB**, **18dB**, **24dB**

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Ladder 6dB | `fvafilt(0).set().lslope(0)` | `NodeId::FVaFilt(0).inp_param("lslope")` |
| 1 | Ladder 12dB | `fvafilt(0).set().lslope(1)` | `NodeId::FVaFilt(0).inp_param("lslope")` |
| 2 | Ladder 18dB | `fvafilt(0).set().lslope(2)` | `NodeId::FVaFilt(0).inp_param("lslope")` |
| 3 | Ladder 24dB | `fvafilt(0).set().lslope(3)` | `NodeId::FVaFilt(0).inp_param("lslope")` |
### NodeId::BiqFilt
**Biquad Filter**

This is the implementation of a biquad filter cascade.
It is not meant for fast automation. Please use other nodes
like eg. `SFilter` for that.

- [input **inp**](#nodeidbiqfilt-input-inp) - Signal input
- [input **freq**](#nodeidbiqfilt-input-freq) - Filter cutoff frequency.
- [input **q**](#nodeidbiqfilt-input-q) - Filter Q factor.
- [input **gain**](#nodeidbiqfilt-input-gain) - Filter gain.
- [setting **ftype**](#nodeidbiqfilt-setting-ftype) - 'BtW LP' Butterworth Low-Pass, 'Res' Resonator
- [setting **order**](#nodeidbiqfilt-setting-order) - 
- output **sig**
Filtered signal output.
#### NodeId::BiqFilt Help
**Biquad Filter (Cascade)**

This is the implementation of a biquad filter cascade.
It is not meant for fast automation and might blow up if you
treat it too rough. Please use other nodes like eg. `SFilter` for that.

#### NodeId::BiqFilt input inp
Signal input

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `biqfilt(0).set().inp(0)` | `NodeId::BiqFilt(0).inp_param("inp")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `biqfilt(0).set().inp(-1)` | `NodeId::BiqFilt(0).inp_param("inp")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `biqfilt(0).set().inp(0)` | `NodeId::BiqFilt(0).inp_param("inp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `biqfilt(0).set().inp(1)` | `NodeId::BiqFilt(0).inp_param("inp")` |
#### NodeId::BiqFilt input freq
Filter cutoff frequency.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.1184 |   1000.00 |   1000Hz | `biqfilt(0).set().freq(1000)` | `NodeId::BiqFilt(0).inp_param("freq")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `biqfilt(0).set().freq(0.4296875)` | `NodeId::BiqFilt(0).inp_param("freq")` |
| **mid** | -0.2176 |     97.34 |  97.34Hz | `biqfilt(0).set().freq(97.33759)` | `NodeId::BiqFilt(0).inp_param("freq")` |
| **max** |  0.5647 |  22049.99 |  22050Hz | `biqfilt(0).set().freq(22049.994)` | `NodeId::BiqFilt(0).inp_param("freq")` |
#### NodeId::BiqFilt input q
Filter Q factor.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `biqfilt(0).set().q(0.5)` | `NodeId::BiqFilt(0).inp_param("q")` |
| **min** |  0.0000 |      0.00 |  0.000 | `biqfilt(0).set().q(0)` | `NodeId::BiqFilt(0).inp_param("q")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `biqfilt(0).set().q(0.5)` | `NodeId::BiqFilt(0).inp_param("q")` |
| **max** |  1.0000 |      1.00 |  1.000 | `biqfilt(0).set().q(1)` | `NodeId::BiqFilt(0).inp_param("q")` |
#### NodeId::BiqFilt input gain
Filter gain.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      1.00 | +0.0dB | `biqfilt(0).set().gain(1)` | `NodeId::BiqFilt(0).inp_param("gain")` |
| **min** |  0.0000 |      0.06 | -24.0dB | `biqfilt(0).set().gain(0.063095726)` | `NodeId::BiqFilt(0).inp_param("gain")` |
| **mid** |  0.5000 |      1.00 | +0.0dB | `biqfilt(0).set().gain(1)` | `NodeId::BiqFilt(0).inp_param("gain")` |
| **max** |  1.0000 |     15.85 | +24.0dB | `biqfilt(0).set().gain(15.848933)` | `NodeId::BiqFilt(0).inp_param("gain")` |
#### NodeId::BiqFilt setting ftype
'BtW LP' Butterworth Low-Pass, 'Res' Resonator

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | BtW LP | `biqfilt(0).set().ftype(0)` | `NodeId::BiqFilt(0).inp_param("ftype")` |
| 1 | Res | `biqfilt(0).set().ftype(1)` | `NodeId::BiqFilt(0).inp_param("ftype")` |
#### NodeId::BiqFilt setting order


| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | 1 | `biqfilt(0).set().order(0)` | `NodeId::BiqFilt(0).inp_param("order")` |
| 1 | 2 | `biqfilt(0).set().order(1)` | `NodeId::BiqFilt(0).inp_param("order")` |
| 2 | 3 | `biqfilt(0).set().order(2)` | `NodeId::BiqFilt(0).inp_param("order")` |
| 3 | 4 | `biqfilt(0).set().order(3)` | `NodeId::BiqFilt(0).inp_param("order")` |
### NodeId::PVerb
**Plate Reverb**

This is a simple but yet powerful small plate reverb based on the design by Jon Dattorro.
It should suit your needs from small rooms up to large atmospheric sound scapes.

- [input **in_l**](#nodeidpverb-input-in_l) - Left input channel, will be summed with the right channel. So you can just feed in a mono signal without harm.
- [input **in_r**](#nodeidpverb-input-in_r) - Right input channel, will be summed with the left channel.
- [input **predly**](#nodeidpverb-input-predly) - The pre-delay length for the first reflection.
- [input **size**](#nodeidpverb-input-size) - The size of the simulated room. Goes from a small chamber to a huge hall.
- [input **dcy**](#nodeidpverb-input-dcy) - The decay of the sound. If you set this to **1.0** the         sound will infinitively be sustained. Just be careful feeding in more sound with that.
- [input **ilpf**](#nodeidpverb-input-ilpf) - Input low-pass filter cutoff frequency, for filtering the input before it's fed into the pre-delay.
- [input **ihpf**](#nodeidpverb-input-ihpf) - Input high-pass filter cutoff frequency, for filtering the input before it's fed into the pre-delay.
- [input **dif**](#nodeidpverb-input-dif) - The amount of diffusion inside the reverb tank. Setting this to **0** will disable any kind of diffusion and the reverb will become a more or less simple echo effect.
- [input **dmix**](#nodeidpverb-input-dmix) - The mix between input diffusion and clean output of the pre-delay. Setting this to **0** will not diffuse any input.
- [input **mspeed**](#nodeidpverb-input-mspeed) - The internal LFO speed, that modulates the internal diffusion inside the reverb tank. Keeping this low (< **0.2**) will sound a bit more natural than a fast LFO.
- [input **mshp**](#nodeidpverb-input-mshp) - The shape of the LFO. **0.0** is a down ramp, **1.0** is an up ramp and **0.0** is a triangle. Setting this to **0.5** is a good choice. The extreme values of **0.0** and **1.0** can lead to audible artifacts.
- [input **mdepth**](#nodeidpverb-input-mdepth) - The depth of the LFO change that is applied to the diffusion inside the reverb tank. More extreme values (above **0.2**) will lead to more detuned sounds reverbing inside the tank.
- [input **rlpf**](#nodeidpverb-input-rlpf) - Reverb tank low-pass filter cutoff frequency.
- [input **rhpf**](#nodeidpverb-input-rhpf) - Reverb tank high-pass filter cutoff frequency.
- [input **mix**](#nodeidpverb-input-mix) - Dry/Wet mix between the input and the diffused output.
- output **sig_l**
The left channel of the output signal.
- output **sig_r**
The right channel of the output signal.
#### NodeId::PVerb Help
**Plate Reverb (by Jon Dattorro)**

This is a simple but yet powerful small plate reverb based on the design
by Jon Dattorro. It should suit your needs from small rooms up to large
atmospheric sound scapes. It provides two inputs, and two outputs for
stereo signals. You can also feed a monophonic input, and you will get
a stereo output.

It provides simple low-pass and high-pass filters for the inputs
and another set of them for the internal reverberation tank to control
the bandwidth of the reverbs.

Internal modulation keeps the sound alive and spreads it even more.

Structure of the reverb is:

```text
      Left       Right
        |         |
        \----+----/
             v
           'ilpf'           'ihpf'         'predly'
      Input Low-Pass -> Input High-Pass -> Pre-Delay
                                                   |
           o------------------o--------------\     |
           +------\           +----------\   |     |
           v      |           v          |   |  v--o----> All-Pass Diffusor
     [Left Channel]     [Right Channel]  |   \--x 'dmix'     |
    /> Diffusor 1 |'size' Diffusor 1 <-\ |      ^------------/
    |    Delay 1  |'size'   Delay 1    | |
    |   LPF/HPF   |        LPF/HPF   'rlpf'/'rhpf'
    |  [x Decay]  |'dcy'  [x Decay]    | |               'mspeed'
    o> Diffusor 2 |'size' Diffusor 2 <-o----o-x-----LFO  'mshp'
    |    Delay 2  |'size'   Delay 2      |  | 'mdepth'
    |      |      |            |         |  |
    |      x 'dcy'|            x         |  |
    |      |      \-[feedback]-/         |  |
    |      \--------[feedback]-----------/  |
    \--------------------------------------/

      Multiple Taps into Left/Right Diffusors 1/2 and Delays 1/2
      are then fed to the left and right output channels.
```


#### NodeId::PVerb input in_l
Left input channel, will be summed with the right channel. So you can just feed in a mono signal without harm.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `pverb(0).set().in_l(0)` | `NodeId::PVerb(0).inp_param("in_l")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `pverb(0).set().in_l(-1)` | `NodeId::PVerb(0).inp_param("in_l")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `pverb(0).set().in_l(0)` | `NodeId::PVerb(0).inp_param("in_l")` |
| **max** |  1.0000 |      1.00 |  1.000 | `pverb(0).set().in_l(1)` | `NodeId::PVerb(0).inp_param("in_l")` |
#### NodeId::PVerb input in_r
Right input channel, will be summed with the left channel.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `pverb(0).set().in_r(0)` | `NodeId::PVerb(0).inp_param("in_r")` |
| **min** | -1.0000 |     -1.00 | -1.000 | `pverb(0).set().in_r(-1)` | `NodeId::PVerb(0).inp_param("in_r")` |
| **mid** |  0.0000 |      0.00 |  0.000 | `pverb(0).set().in_r(0)` | `NodeId::PVerb(0).inp_param("in_r")` |
| **max** |  1.0000 |      1.00 |  1.000 | `pverb(0).set().in_r(1)` | `NodeId::PVerb(0).inp_param("in_r")` |
#### NodeId::PVerb input predly
The pre-delay length for the first reflection.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.00ms | `pverb(0).set().predly(0)` | `NodeId::PVerb(0).inp_param("predly")` |
| **min** |  0.0000 |      0.00 |  0.00ms | `pverb(0).set().predly(0)` | `NodeId::PVerb(0).inp_param("predly")` |
| **mid** |  0.5000 |   1250.00 |   1250ms | `pverb(0).set().predly(1250)` | `NodeId::PVerb(0).inp_param("predly")` |
| **max** |  1.0000 |   5000.00 |   5000ms | `pverb(0).set().predly(5000)` | `NodeId::PVerb(0).inp_param("predly")` |
#### NodeId::PVerb input size
The size of the simulated room. Goes from a small chamber to a huge hall.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `pverb(0).set().size(0.5)` | `NodeId::PVerb(0).inp_param("size")` |
| **min** |  0.0000 |      0.00 |  0.000 | `pverb(0).set().size(0)` | `NodeId::PVerb(0).inp_param("size")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `pverb(0).set().size(0.5)` | `NodeId::PVerb(0).inp_param("size")` |
| **max** |  1.0000 |      1.00 |  1.000 | `pverb(0).set().size(1)` | `NodeId::PVerb(0).inp_param("size")` |
#### NodeId::PVerb input dcy
The decay of the sound. If you set this to **1.0** the
        sound will infinitively be sustained. Just be careful feeding in more sound with that.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.2500 |      0.25 |  0.250 | `pverb(0).set().dcy(0.25)` | `NodeId::PVerb(0).inp_param("dcy")` |
| **min** |  0.0000 |      0.00 |  0.000 | `pverb(0).set().dcy(0)` | `NodeId::PVerb(0).inp_param("dcy")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `pverb(0).set().dcy(0.5)` | `NodeId::PVerb(0).inp_param("dcy")` |
| **max** |  1.0000 |      1.00 |  1.000 | `pverb(0).set().dcy(1)` | `NodeId::PVerb(0).inp_param("dcy")` |
#### NodeId::PVerb input ilpf
Input low-pass filter cutoff frequency, for filtering the input before it's fed into the pre-delay.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5647 |  22050.01 |  22050Hz | `pverb(0).set().ilpf(22050.01)` | `NodeId::PVerb(0).inp_param("ilpf")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `pverb(0).set().ilpf(0.4296875)` | `NodeId::PVerb(0).inp_param("ilpf")` |
| **mid** | -0.2176 |     97.34 |  97.34Hz | `pverb(0).set().ilpf(97.33759)` | `NodeId::PVerb(0).inp_param("ilpf")` |
| **max** |  0.5647 |  22049.99 |  22050Hz | `pverb(0).set().ilpf(22049.994)` | `NodeId::PVerb(0).inp_param("ilpf")` |
#### NodeId::PVerb input ihpf
Input high-pass filter cutoff frequency, for filtering the input before it's fed into the pre-delay.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** | -1.5425 |      0.43 |   0.43Hz | `pverb(0).set().ihpf(0.4296875)` | `NodeId::PVerb(0).inp_param("ihpf")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `pverb(0).set().ihpf(0.4296875)` | `NodeId::PVerb(0).inp_param("ihpf")` |
| **mid** | -0.2176 |     97.34 |  97.34Hz | `pverb(0).set().ihpf(97.33759)` | `NodeId::PVerb(0).inp_param("ihpf")` |
| **max** |  0.5647 |  22049.99 |  22050Hz | `pverb(0).set().ihpf(22049.994)` | `NodeId::PVerb(0).inp_param("ihpf")` |
#### NodeId::PVerb input dif
The amount of diffusion inside the reverb tank. Setting this to **0** will disable any kind of diffusion and the reverb will become a more or less simple echo effect.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `pverb(0).set().dif(1)` | `NodeId::PVerb(0).inp_param("dif")` |
| **min** |  0.0000 |      0.00 |  0.000 | `pverb(0).set().dif(0)` | `NodeId::PVerb(0).inp_param("dif")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `pverb(0).set().dif(0.5)` | `NodeId::PVerb(0).inp_param("dif")` |
| **max** |  1.0000 |      1.00 |  1.000 | `pverb(0).set().dif(1)` | `NodeId::PVerb(0).inp_param("dif")` |
#### NodeId::PVerb input dmix
The mix between input diffusion and clean output of the pre-delay. Setting this to **0** will not diffuse any input.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  1.0000 |      1.00 |  1.000 | `pverb(0).set().dmix(1)` | `NodeId::PVerb(0).inp_param("dmix")` |
| **min** |  0.0000 |      0.00 |  0.000 | `pverb(0).set().dmix(0)` | `NodeId::PVerb(0).inp_param("dmix")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `pverb(0).set().dmix(0.5)` | `NodeId::PVerb(0).inp_param("dmix")` |
| **max** |  1.0000 |      1.00 |  1.000 | `pverb(0).set().dmix(1)` | `NodeId::PVerb(0).inp_param("dmix")` |
#### NodeId::PVerb input mspeed
The internal LFO speed, that modulates the internal diffusion inside the reverb tank. Keeping this low (< **0.2**) will sound a bit more natural than a fast LFO.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.0000 |      0.00 |  0.000 | `pverb(0).set().mspeed(0)` | `NodeId::PVerb(0).inp_param("mspeed")` |
| **min** |  0.0000 |      0.00 |  0.000 | `pverb(0).set().mspeed(0)` | `NodeId::PVerb(0).inp_param("mspeed")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `pverb(0).set().mspeed(0.5)` | `NodeId::PVerb(0).inp_param("mspeed")` |
| **max** |  1.0000 |      1.00 |  1.000 | `pverb(0).set().mspeed(1)` | `NodeId::PVerb(0).inp_param("mspeed")` |
#### NodeId::PVerb input mshp
The shape of the LFO. **0.0** is a down ramp, **1.0** is an up ramp and **0.0** is a triangle. Setting this to **0.5** is a good choice. The extreme values of **0.0** and **1.0** can lead to audible artifacts.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `pverb(0).set().mshp(0.5)` | `NodeId::PVerb(0).inp_param("mshp")` |
| **min** |  0.0000 |      0.00 |  0.000 | `pverb(0).set().mshp(0)` | `NodeId::PVerb(0).inp_param("mshp")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `pverb(0).set().mshp(0.5)` | `NodeId::PVerb(0).inp_param("mshp")` |
| **max** |  1.0000 |      1.00 |  1.000 | `pverb(0).set().mshp(1)` | `NodeId::PVerb(0).inp_param("mshp")` |
#### NodeId::PVerb input mdepth
The depth of the LFO change that is applied to the diffusion inside the reverb tank. More extreme values (above **0.2**) will lead to more detuned sounds reverbing inside the tank.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.2000 |      0.20 |  0.200 | `pverb(0).set().mdepth(0.2)` | `NodeId::PVerb(0).inp_param("mdepth")` |
| **min** |  0.0000 |      0.00 |  0.000 | `pverb(0).set().mdepth(0)` | `NodeId::PVerb(0).inp_param("mdepth")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `pverb(0).set().mdepth(0.5)` | `NodeId::PVerb(0).inp_param("mdepth")` |
| **max** |  1.0000 |      1.00 |  1.000 | `pverb(0).set().mdepth(1)` | `NodeId::PVerb(0).inp_param("mdepth")` |
#### NodeId::PVerb input rlpf
Reverb tank low-pass filter cutoff frequency.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5647 |  22050.01 |  22050Hz | `pverb(0).set().rlpf(22050.01)` | `NodeId::PVerb(0).inp_param("rlpf")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `pverb(0).set().rlpf(0.4296875)` | `NodeId::PVerb(0).inp_param("rlpf")` |
| **mid** | -0.2176 |     97.34 |  97.34Hz | `pverb(0).set().rlpf(97.33759)` | `NodeId::PVerb(0).inp_param("rlpf")` |
| **max** |  0.5647 |  22049.99 |  22050Hz | `pverb(0).set().rlpf(22049.994)` | `NodeId::PVerb(0).inp_param("rlpf")` |
#### NodeId::PVerb input rhpf
Reverb tank high-pass filter cutoff frequency.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** | -1.5425 |      0.43 |   0.43Hz | `pverb(0).set().rhpf(0.4296875)` | `NodeId::PVerb(0).inp_param("rhpf")` |
| **min** | -1.0000 |      0.43 |   0.43Hz | `pverb(0).set().rhpf(0.4296875)` | `NodeId::PVerb(0).inp_param("rhpf")` |
| **mid** | -0.2176 |     97.34 |  97.34Hz | `pverb(0).set().rhpf(97.33759)` | `NodeId::PVerb(0).inp_param("rhpf")` |
| **max** |  0.5647 |  22049.99 |  22050Hz | `pverb(0).set().rhpf(22049.994)` | `NodeId::PVerb(0).inp_param("rhpf")` |
#### NodeId::PVerb input mix
Dry/Wet mix between the input and the diffused output.

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `pverb(0).set().mix(0.5)` | `NodeId::PVerb(0).inp_param("mix")` |
| **min** |  0.0000 |      0.00 |  0.000 | `pverb(0).set().mix(0)` | `NodeId::PVerb(0).inp_param("mix")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `pverb(0).set().mix(0.5)` | `NodeId::PVerb(0).inp_param("mix")` |
| **max** |  1.0000 |      1.00 |  1.000 | `pverb(0).set().mix(1)` | `NodeId::PVerb(0).inp_param("mix")` |
### NodeId::Test
****
- [input **f**](#nodeidtest-input-f) - F Test
- [setting **p**](#nodeidtest-setting-p) - An unsmoothed parameter for automated tests.
- [setting **trig**](#nodeidtest-setting-trig) - A trigger input, that will create a short pulse on the `tsig` output.
- output **sig**
The output of p as signal
- output **tsig**
A short trigger pulse will be generated when the `trig` input is triggered.
- output **out2**
A test output that will emit **1.0** if output `sig` is connected.
- output **out3**
A test output that will emit **1.0** if input `f` is connected.
- output **out4**

- output **outc**
Emits a number that defines the out_connected bitmask. Used only for testing!
#### NodeId::Test Help
****
#### NodeId::Test input f
F Test

| | value | denormalized | fmt | build API | [crate::ParamId] |
|-|-------|--------------|-----|-----------|------------------|
| **default** |  0.5000 |      0.50 |  0.500 | `test(0).set().f(0.5)` | `NodeId::Test(0).inp_param("f")` |
| **min** |  0.0000 |      0.00 |  0.000 | `test(0).set().f(0)` | `NodeId::Test(0).inp_param("f")` |
| **mid** |  0.5000 |      0.50 |  0.500 | `test(0).set().f(0.5)` | `NodeId::Test(0).inp_param("f")` |
| **max** |  1.0000 |      1.00 |  1.000 | `test(0).set().f(1)` | `NodeId::Test(0).inp_param("f")` |
#### NodeId::Test setting p
An unsmoothed parameter for automated tests.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Zero | `test(0).set().p(0)` | `NodeId::Test(0).inp_param("p")` |
| 1 | One | `test(0).set().p(1)` | `NodeId::Test(0).inp_param("p")` |
| 2 | Two | `test(0).set().p(2)` | `NodeId::Test(0).inp_param("p")` |
| 3 | Three | `test(0).set().p(3)` | `NodeId::Test(0).inp_param("p")` |
| 4 | Four | `test(0).set().p(4)` | `NodeId::Test(0).inp_param("p")` |
| 5 | Five | `test(0).set().p(5)` | `NodeId::Test(0).inp_param("p")` |
| 6 | Six | `test(0).set().p(6)` | `NodeId::Test(0).inp_param("p")` |
| 7 | Seven | `test(0).set().p(7)` | `NodeId::Test(0).inp_param("p")` |
| 8 | Eigth | `test(0).set().p(8)` | `NodeId::Test(0).inp_param("p")` |
| 9 | Nine | `test(0).set().p(9)` | `NodeId::Test(0).inp_param("p")` |
| 10 | Ten | `test(0).set().p(10)` | `NodeId::Test(0).inp_param("p")` |
#### NodeId::Test setting trig
A trigger input, that will create a short pulse on the `tsig` output.

| setting | fmt | build API | [crate::ParamId] |
|---------|-----|-----------|------------------|
| 0 | Zero | `test(0).set().trig(0)` | `NodeId::Test(0).inp_param("trig")` |
