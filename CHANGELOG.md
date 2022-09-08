0.2.1 (unreleased)
==================

* Feature: Added PM input to the Sin node for phase modulation/distortion.
This allows to get a more linear FM sound from that oscillator.
* Change: Changed `Ad` envelope node to be properly retriggerable. Refactored
out it's DSP code to `synfx_dsp::EnvRetrigAD`.

0.2.0 (2022-08-28)
==================

* Documentation: Added a guide in the hexodsp::dsp module documentation
about implementing new DSP nodes.
* Bugfix: TriSawLFO (TsLFO) node did output too high values if the `rev`
parameter was changed or modulated at runtime.
* Bugfix: Found a bug in cubic interpolation in the sample player and
similar bugs in the delay line (and all-pass & comb filters). Refactored
the cubic interpolation and tested it seperately now.
* Change: Moved DSP code over to `synfx-dsp` crate.
* Feature: Matrix::get\_connections() returns information about the connections
to the adjacent cells.
* Feature: Added the MatrixCellChain abstraction for easy creation of DSP
chains on the hexagonal Matrix.
* Feature: Added Scope DSP node and NodeConfigurator/Matrix API for retrieving
the scope handles for access to it's capture buffers.
* Feature: Added WBlockDSP visual programming language utilizing the `synfx-dsp-jit` crate.
* Feature: Added the `FormFM` node that was contributed by Dimas Leenman (aka Skythedragon).
* Feature: Added `MidiP` node for MIDI pitch/note input.
* Feature: Added `MidiCC` node for MIDI CC input.
* Feature: Added `ExtA` to `ExtF` nodes for plugin parameter access.
* Feature: Added `Inp` input node for the two audio channels.
