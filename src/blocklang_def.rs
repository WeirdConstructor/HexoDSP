// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::blocklang::{BlockLanguage, BlockUserInput, BlockType};
use std::cell::RefCell;
use std::rc::Rc;

pub fn setup_hxdsp_block_language() -> Rc<RefCell<BlockLanguage>> {
    let mut lang = BlockLanguage::new();

    lang.define(BlockType {
        category: "source".to_string(),
        name: "phse".to_string(),
        rows: 1,
        inputs: vec![Some("f".to_string())],
        outputs: vec![Some("".to_string())],
        area_count: 0,
        user_input: BlockUserInput::None,
        description: "A phasor, returns a saw tooth wave to scan through things or use as modulator.".to_string(),
        color: 2,
    });

    lang.define(BlockType {
        category:       "literals".to_string(),
        name:           "zero".to_string(),
        rows:           1,
        inputs:         vec![],
        outputs:        vec![Some("".to_string())],
        area_count:     0,
        user_input:     BlockUserInput::None,
        description:    "The 0.0 value".to_string(),
        color:          1,
    });

    lang.define(BlockType {
        category:       "literals".to_string(),
        name:           "π".to_string(),
        rows:           1,
        inputs:         vec![],
        outputs:        vec![Some("".to_string())],
        area_count:     0,
        user_input:     BlockUserInput::None,
        description:    "The PI number".to_string(),
        color:          1,
    });

    lang.define(BlockType {
        category:       "literals".to_string(),
        name:           "2π".to_string(),
        rows:           1,
        inputs:         vec![],
        outputs:        vec![Some("".to_string())],
        area_count:     0,
        user_input:     BlockUserInput::None,
        description:    "2 * PI == TAU".to_string(),
        color:          1,
    });

    lang.define(BlockType {
        category:       "literals".to_string(),
        name:           "SR".to_string(),
        rows:           1,
        inputs:         vec![],
        outputs:        vec![Some("".to_string())],
        area_count:     0,
        user_input:     BlockUserInput::None,
        description:    "The sample rate".to_string(),
        color:          1,
    });

    lang.define(BlockType {
        category:       "literals".to_string(),
        name:           "value".to_string(),
        rows:           1,
        inputs:         vec![],
        outputs:        vec![Some("".to_string())],
        area_count:     0,
        user_input:     BlockUserInput::Float,
        description:    "A literal value, typed in by the user.".to_string(),
        color:          1,
    });

    lang.define(BlockType {
        category:       "routing".to_string(),
        name:           "->".to_string(),
        rows:           1,
        inputs:         vec![Some("".to_string())],
        outputs:        vec![Some("".to_string())],
        area_count:     0,
        user_input:     BlockUserInput::None,
        description:    "Forwards the value one block".to_string(),
        color:          6,
    });

    lang.define(BlockType {
        category:       "routing".to_string(),
        name:           "->2".to_string(),
        rows:           2,
        inputs:         vec![Some("".to_string())],
        outputs:        vec![Some("".to_string()), Some("".to_string())],
        area_count:     0,
        user_input:     BlockUserInput::None,
        description:    "Forwards the value one block and sends it to multiple destinations".to_string(),
        color:          6,
    });

    lang.define(BlockType {
        category:       "routing".to_string(),
        name:           "->3".to_string(),
        rows:           3,
        inputs:         vec![Some("".to_string())],
        outputs:        vec![Some("".to_string()), Some("".to_string()), Some("".to_string())],
        area_count:     0,
        user_input:     BlockUserInput::None,
        description:    "Forwards the value one block and sends it to multiple destinations".to_string(),
        color:          6,
    });

    lang.define(BlockType {
        category:       "variables".to_string(),
        name:           "set".to_string(),
        rows:           1,
        inputs:         vec![Some("".to_string())],
        outputs:        vec![],
        area_count:     0,
        user_input:     BlockUserInput::Identifier,
        description:    "Stores into a variable".to_string(),
        color:          2,
    });

    lang.define(BlockType {
        category:       "variables".to_string(),
        name:           "get".to_string(),
        rows:           1,
        inputs:         vec![],
        outputs:        vec![Some("".to_string())],
        area_count:     0,
        user_input:     BlockUserInput::Identifier,
        description:    "Loads a variable".to_string(),
        color:          12,
    });

    lang.define(BlockType {
        category:       "variables".to_string(),
        name:           "if".to_string(),
        rows:           1,
        inputs:         vec![Some("".to_string())],
        outputs:        vec![Some("".to_string())],
        area_count:     2,
        user_input:     BlockUserInput::None,
        description:    "Divides the controlflow based on a true (>= 0.5) \
                         or false (< 0.5) input value.".to_string(),
        color:          0,
    });

    lang.define(BlockType {
        category:       "nodes".to_string(),
        name:           "1pole".to_string(),
        rows:           2,
        inputs:         vec![Some("in".to_string()), Some("f".to_string())],
        outputs:        vec![Some("lp".to_string()), Some("hp".to_string())],
        area_count:     0,
        user_input:     BlockUserInput::None,
        description:    "Runs a simple one pole filter on the input".to_string(),
        color:          8,
    });

    lang.define(BlockType {
        category:       "nodes".to_string(),
        name:           "svf".to_string(),
        rows:           3,
        inputs:         vec![Some("in".to_string()), Some("f".to_string()), Some("r".to_string())],
        outputs:        vec![Some("lp".to_string()), Some("bp".to_string()), Some("hp".to_string())],
        area_count:     0,
        user_input:     BlockUserInput::None,
        description:    "Runs a state variable filter on the input".to_string(),
        color:          8,
    });

    lang.define(BlockType {
        category:       "functions".to_string(),
        name:           "sin".to_string(),
        rows:           1,
        inputs:         vec![Some("".to_string())],
        outputs:        vec![Some("".to_string())],
        area_count:     0,
        user_input:     BlockUserInput::None,
        description:    "Calculates the sine of the input".to_string(),
        color:          16,
    });

    lang.define(BlockType {
        category:       "nodes".to_string(),
        name:           "delay".to_string(),
        rows:           2,
        inputs:         vec![Some("in".to_string()), Some("t".to_string())],
        outputs:        vec![Some("".to_string())],
        area_count:     0,
        user_input:     BlockUserInput::None,
        description:    "Runs a linearly interpolated delay on the input".to_string(),
        color:          8,
    });

    lang.define(BlockType {
        category:       "arithmetics".to_string(),
        name:           "/%".to_string(),
        rows:           2,
        inputs:         vec![Some("a".to_string()), Some("b".to_string())],
        outputs:        vec![Some("div".to_string()), Some("rem".to_string())],
        area_count:     0,
        user_input:     BlockUserInput::None,
        description:    "Computes the integer division and remainder of a / b".to_string(),
        color:          8,
    });

    for fun_name in &["+", "-", "*", "/"] {
        lang.define(BlockType {
            category:       "arithmetics".to_string(),
            name:           fun_name.to_string(),
            rows:           2,
            inputs:
                if fun_name == &"-" || fun_name == &"/" {
                    vec![Some("a".to_string()), Some("b".to_string())]
                } else {
                    vec![Some("".to_string()), Some("".to_string())]
                },
            outputs:        vec![Some("".to_string())],
            area_count:     0,
            user_input:     BlockUserInput::None,
            description:    "A binary arithmetics operation".to_string(),
            color:          4,
        });
    }

    lang.define_identifier("in1");
    lang.define_identifier("in2");
    lang.define_identifier("israte");
    lang.define_identifier("srate");
    lang.define_identifier("alpha");
    lang.define_identifier("beta");
    lang.define_identifier("delta");
    lang.define_identifier("gamma");
    lang.define_identifier("&sig1");
    lang.define_identifier("&sig2");

    Rc::new(RefCell::new(lang))
}
