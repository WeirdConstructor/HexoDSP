// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum CellDir {
    TR,
    BR,
    B,
    BL,
    TL,
    T,
    /// Center
    C
}

impl CellDir {
    pub fn from(edge: u8) -> Self {
        match edge {
            0 => CellDir::TR,
            1 => CellDir::BR,
            2 => CellDir::B,
            3 => CellDir::BL,
            4 => CellDir::TL,
            5 => CellDir::T,
            _ => CellDir::C,
        }
    }

    pub fn flip(&self) -> Self {
        match self {
            CellDir::TR => CellDir::BL,
            CellDir::BR => CellDir::TL,
            CellDir::B  => CellDir::T,
            CellDir::BL => CellDir::TR,
            CellDir::TL => CellDir::BR,
            CellDir::T  => CellDir::B,
            CellDir::C  => CellDir::T,
        }
    }

    #[inline]
    pub fn is_output(&self) -> bool {
        let e = self.as_edge();
        e <= 2
    }

    #[inline]
    pub fn is_input(&self) -> bool {
        !self.is_output()
    }

    #[inline]
    pub fn as_edge(&self) -> u8 {
        *self as u8
    }

    pub fn as_menu_pos(&self) -> (i32, i32) {
        match self {
            // out 1 - TR
            CellDir::TR => (0, 1),
            // out 2 - BR
            CellDir::BR => (1, 1),
            // out 3 - B
            CellDir::B  => (0, 1),
            // in 3 - BL
            CellDir::BL => (-1, 1),
            // in 2 - TL
            CellDir::TL => (-1, 0),
            // in 1 - T
            CellDir::T  => (0, -1),
            _           => (0, 0),
        }
    }

    pub fn as_offs(&self, x: usize) -> (i32, i32) {
        let even = x % 2 == 0;
        match self {
            // out 1 - TR
            CellDir::TR => (1, if even { -1 } else { 0 }),
            // out 2 - BR
            CellDir::BR => (1, if even { 0 } else { 1 }),
            // out 3 - B
            CellDir::B  => (0, 1),
            // in 3 - BL
            CellDir::BL => (-1, if even { 0 } else { 1 }),
            // in 2 - TL
            CellDir::TL => (-1, if even { -1 } else { 0 }),
            // in 1 - T
            CellDir::T  => (0, -1),
            _           => (0, 0),
        }
    }

    /// If it returns 0 they are not adjacent,
    /// if it returns 1 the data flow direction is pos_a => pos_b
    /// if it returns -1 the data flow direction is pos_b => pos_a
    pub fn are_adjacent(pos_a: (usize, usize), pos_b: (usize, usize)) -> Option<CellDir> {
        let ipos_a = (pos_a.0 as i32, pos_a.1 as i32);
        let ipos_b = (pos_b.0 as i32, pos_b.1 as i32);

        let (ox, oy) = CellDir::T.as_offs(pos_a.0);
        if ipos_b == (ipos_a.0 + ox, ipos_a.1 + oy) {
            return Some(CellDir::T);
        }
        let (ox, oy) = CellDir::TL.as_offs(pos_a.0);
        if ipos_b == (ipos_a.0 + ox, ipos_a.1 + oy) {
            return Some(CellDir::TL);
        }
        let (ox, oy) = CellDir::BL.as_offs(pos_a.0);
        if ipos_b == (ipos_a.0 + ox, ipos_a.1 + oy) {
            return Some(CellDir::BL);
        }

        let (ox, oy) = CellDir::TR.as_offs(pos_a.0);
        if ipos_b == (ipos_a.0 + ox, ipos_a.1 + oy) {
            return Some(CellDir::TR);
        }
        let (ox, oy) = CellDir::BR.as_offs(pos_a.0);
        if ipos_b == (ipos_a.0 + ox, ipos_a.1 + oy) {
            return Some(CellDir::BR);
        }
        let (ox, oy) = CellDir::B.as_offs(pos_a.0);
        if ipos_b == (ipos_a.0 + ox, ipos_a.1 + oy) {
            return Some(CellDir::B);
        }

        None
    }
}
