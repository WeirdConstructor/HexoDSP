# -1.0 == 21        => A0
# -0.1 == 69 - 12   => A3
#  0.0 == 69        => A4
#  0.1 == 69 + 12   => A5
#  0.2 == 69 + 12   => A6
#  0.3 == 69 + 12   => A7
#  0.4 == 69 + 12   => A8
#  0.5 == 69 + 12   => A9
#  0.6 == 69 + 12   => A10
#  0.7 == 69 + 12   => A11
#  0.7 == 69 + 12   => A12
#  0.9 == 69 + 12   => A13
#  1.0 == 69 + 12   => A14
!f2note = { 69.0 + _ * 120.0 };
!note2pitch = { 440.0 * 2.0 ^ ((_ - 69) / 12) };

!f2p = { 440.0 * 2.0 ^ (_ * 10.0); };

!pitch2note = {
    12.0 * std:num:log2[_ / 440.0] + 69.0
};

!note2f = { _ / 120.0 - 0.575 };

!p2f = { 0.1 * std:num:log2[_ / 440.0] };

#    ((($x as f32).max(0.01) / 440.0).log2() / 10.0)

!in = (1.0 / 12.0) / 10.0;
std:displayln :f2note ~ f2note in;
std:displayln :note2pitch ~ note2pitch ~ f2note in;
std:displayln :f2p "      "~ f2p in;
!f = f2p in;
std:displayln :pitch2note "      "~ pitch2note f;
std:displayln :note2f "      "~ note2f ~ pitch2note f;
std:displayln :p2f "         "~ p2f f;

!eucDiv = {
    !a = int _;
    !b = int _1;
    !div = a / b;
    !mod = a % b;
    if mod < 0 { .div -= 1; };
    div
};

# gets the index in the 24 half semitone array
!get_note_idx = {
    !num = _ * 240.0; # twice the resolution, for quater notes and a more even quantization
    !r  = int ~ std:num:floor num;
    !o  = eucDiv r 24;
    .r -= o * 24;
    std:displayln ~ $F "       in={:8.5} r={:2} o={:2}" _ r o;
    r
};

# std:displayln :for674_12 ~ note2f (67.4 - 12.0);
# std:displayln :for674_12 ~ get_note_idx ~ note2f (67.4 - 12.0);
# 
# std:displayln :for675_12 ~ note2f (67.5 - 12.0);
# std:displayln :for675_12 ~ get_note_idx ~ note2f (67.5 - 12.0);
# 
# std:displayln :for68 "   " ~ note2f (68.0 - 12.0);
# std:displayln :for68 "   " ~ get_note_idx ~ note2f (68.0 - 12.0);
# 
# std:displayln :for679 "  " ~ note2f (67.9 - 0.0);
# std:displayln :for679 "  " ~ get_note_idx ~ note2f (67.9 - 0.0);
# 
# std:displayln :for69_115 ~ note2f (69.0 - 11.5);
# std:displayln :for69_115 ~ get_note_idx ~ note2f (69.0 - 11.5);
# 
# std:displayln :for69_12 "" ~ note2f (69.0 - 11.5);
# std:displayln :for69_12 "" ~ get_note_idx ~ note2f (69.0 - 11.5);
# 
# std:displayln :for69 "   " ~ note2f (69.0 - 0.0);
# std:displayln :for69 "   " ~ get_note_idx ~ note2f (69.0 - 0.0);
# 
# std:displayln :for69+114 ~ note2f (69.0 + 11.4);
# std:displayln :for69+114 ~ get_note_idx ~ note2f (69.0 + 11.4);

iter f $[
    $[10, 0, 0],
    $[-10, 0, 0],
    $[0, 1, -11],
    $[0, 1,  6],
    $[0, 1,  0],
    $[0,  -11,  -5],
    $[0, -1,  0],
#    $[0, -1, 0],
#    $[0, -1, -1],
#    $[0, -1, 1],
#    $[-1, -1, 0],
#    $[-1, 1, 0],
#    $[1, -1, 0],
#    $[1, 1, 0],
#    $[2, -1, 1],
#    $[2, 1, 1],
    $[2, -11, 0],
    $[2, 11, 0],
    $[-9, -1, 0],
] {
    !num =
          float[f.0] * 0.1
        + (float[f.1] / 120.0)
        + (float[f.2] / 1200.0);
    #-0.2, -0.3, -0.4, -0.5, 0.2, 0.3, 0.4, 0.5, -1.0, 1.0] {
    std:displayln ~
        $F "o={:2} n={:2} c={:2} | {:6.3} => {:3}"
            f.0 f.1 f.2 num (get_note_idx num);
    std:displayln[];
};

# Taken from VCV Rack Fundamental Modules Quantizer.cpp
# Under GPL-3.0-or-later
#
#	void process(const ProcessArgs& args) override {
#		bool playingNotes[12] = {};
#		int channels = std::max(inputs[PITCH_INPUT].getChannels(), 1);
#
#		for (int c = 0; c < channels; c++) {
#			float pitch = inputs[PITCH_INPUT].getVoltage(c);
#			int range = std::floor(pitch * 24); // 1.1 => 26
#			int octave = eucDiv(range, 24);     // 26 => 1
#			range -= octave * 24;               // 26 => 2
#			int note = ranges[range] + octave * 12;
#			playingNotes[eucMod(note, 12)] = true;
#			pitch = float(note) / 12;
#			outputs[PITCH_OUTPUT].setVoltage(pitch, c);
#		}
#		outputs[PITCH_OUTPUT].setChannels(channels);
#		std::memcpy(this->playingNotes, playingNotes, sizeof(playingNotes));
#	}
#
#	void updateRanges() {
#		// Check if no notes are enabled
#		bool anyEnabled = false;
#		for (int note = 0; note < 12; note++) {
#			if (enabledNotes[note]) {
#				anyEnabled = true;
#				break;
#			}
#		}
#		// Find closest notes for each range
#		for (int i = 0; i < 24; i++) { // => Oversampling, for taking care of the rounding?!
#			int closestNote = 0;
#			int closestDist = INT_MAX;
#			for (int note = -12; note <= 24; note++) {
#				int dist = std::abs((i + 1) / 2 - note);
#				// Ignore enabled state if no notes are enabled
#				if (anyEnabled && !enabledNotes[eucMod(note, 12)]) {
#					continue;
#				}
#				if (dist < closestDist) {
#					closestNote = note;
#					closestDist = dist;
#				}
#				else {
#					// If dist increases, we won't find a better one.
#					break;
#				}
#			}
#			ranges[i] = closestNote;
#		}
#	}


#std:displayln ~ get_note_offs ~ note2f (0.1 + (4.0 / 12.0) / 10.0);
