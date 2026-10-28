use std::ops::{BitAnd, Shl, Shr, BitOr, BitAndAssign, BitOrAssign, BitXor};
use std::ops::Not;

pub const NOTAFILE: Bitboard = Bitboard {val: 0xfefefefefefefefe}; // ~0x0101010101010101
pub const NOTHFILE: Bitboard = Bitboard {val: 0x7f7f7f7f7f7f7f7f}; // ~0x8080808080808080

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Bitboard {
    pub val: u64
}

impl Bitboard {
    pub fn new(val: u64) -> Self {
        Bitboard {val}
    }

    pub fn is_empty(&self) -> bool {
        self.val == 0
    }

    pub fn count_ones(&self) -> usize {
        self.val.count_ones() as usize
    }

    pub fn bit_scan(&self) -> usize {
        if !self.is_empty() {
            return self.val.trailing_zeros() as usize;
        }

        0
    }
}

impl Iterator for Bitboard {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.val == 0 {
            return None;
        }

        let val = self.val.trailing_zeros();

        self.val &= self.val - 1;

        return Some(val as usize);
    }
}

impl BitAnd<Bitboard> for Bitboard {
    type Output = Bitboard;
    fn bitand(self, other: Bitboard) -> Bitboard {
        Bitboard {val: self.val & other.val}
    }
}
impl BitAnd<Bitboard> for &Bitboard {
    type Output = Bitboard;
    fn bitand(self, other: Bitboard) -> Bitboard {
        Bitboard {val: self.val & other.val}
    }
}

impl BitAnd<&Bitboard> for Bitboard {
    type Output = Bitboard;
    fn bitand(self, other: &Bitboard) -> Bitboard {
        Bitboard {val: self.val & other.val}
    }
}

impl BitAnd<&Bitboard> for &Bitboard {
    type Output = Bitboard;
    fn bitand(self, other: &Bitboard) -> Bitboard {
        Bitboard {val: self.val & other.val}
    }
}

impl BitAndAssign<Bitboard> for Bitboard {
    fn bitand_assign(&mut self, other: Bitboard) {
        self.val &= other.val;
    }
}

impl BitOrAssign<Bitboard> for Bitboard {
    fn bitor_assign(&mut self, other: Bitboard) {
        self.val |= other.val;
    }
}

impl BitOr<Bitboard> for Bitboard {
    type Output = Bitboard;
    fn bitor(self, other: Bitboard) -> Bitboard {
        Bitboard {val: self.val | other.val}
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Bitboard {
        Bitboard {val: !self.val}
    }
}


impl Shr<u8> for Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: u8) -> Self::Output {
        Bitboard{ val: self.val >> rhs }
    }
}

impl Shl<u8> for Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: u8) -> Self::Output {
        Bitboard{ val: self.val << rhs }
    }
}

impl BitXor<Bitboard> for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, other: Bitboard) -> Bitboard {
        Bitboard {val: self.val ^ other.val}
    }
}

pub mod postshift {
    use super::{Bitboard, NOTAFILE, NOTHFILE};

    pub fn east_one(b: Bitboard) -> Bitboard {

        (b << 1) & NOTAFILE
    }
    pub fn no_ea_one(b: Bitboard) -> Bitboard {

        (b << 9) & NOTAFILE
    }
    pub fn so_ea_one(b: Bitboard) -> Bitboard {
        (b >> 7) & NOTAFILE
    }
    pub fn west_one(b: Bitboard) -> Bitboard {

        (b >> 1) & NOTHFILE
    }
    pub fn so_we_one(b: Bitboard) -> Bitboard {

        b >> 9 & NOTHFILE
    }
    pub fn no_we_one(b: Bitboard) -> Bitboard {

        (b << 7) & NOTHFILE
    }
    pub fn sout_one(b: Bitboard) -> Bitboard {

        b >> 8
    }
    pub fn nort_one(b: Bitboard) -> Bitboard {
        b << 8
    }
}

/*
pub fn board_serialize(bitboard: Bitboard) -> Vec<usize> {
    let mut board = bitboard;
    let mut pos: Vec<usize> = vec![];

    while board.val != 0 {
        let i = bit_scan(board);
        pos.push(i);

        board.val = board.val & (board.val - 1)
    }
    pos
}
 */

#[allow(dead_code)]
pub fn print_bitboard(bitboard: Bitboard) {
    let mut output = String::new();
    for r in (0..8).rev() {
        for f in 0..8 {
            let i = (r * 8) + f;

            if ((bitboard.val >> i) & 1) == 1 {
                output.push_str("1 ");
            } else {
                output.push_str(". ");
            }
        }
        output.push_str("\n");
    }
    println!("-----\n{}-----\n", output);
}
#[allow(dead_code)]
pub fn print_bitboard_pos(bitboard: Bitboard, pos: usize) {
    let mut output = String::new();
    for r in (0..8).rev() {
        for f in 0..8 {
            let i = (r * 8) + f;

            if i == pos {
                output.push_str("X ");
            } else if ((bitboard.val >> i) & 1) == 1 {
                output.push_str("1 ");
            } else {
                output.push_str(". ");
            }
        }

        if r == 4 {
            output.push_str("    ");
            output.push_str(&pos.to_string());
        }
        output.push_str("\n");
    }
    println!("-----\n{}-----\n", output);
}
