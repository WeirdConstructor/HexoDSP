0.2.0 (unreleased)
==================

* Documentation: Added a guide in the hexodsp::dsp module documentation
about implementing new DSP nodes.
* Bugfix: TriSawLFO (TsLFO) node did output too high values if the `rev`
parameter was changed or modulated at runtime.
* Bugfix: Found a bug in cubic interpolation in the sample player and
similar bugs in the delay line (and all-pass & comb filters). Refactored
the cubic interpolation and tested it seperately now.
* Feature: Matrix::get\_connections() returns information about the connections
to the adjacent cells.
* Feature: Added the MatrixCellChain abstraction for easy creation of DSP
chains on the hexagonal Matrix.
