use super::utils;

pub const NOTAFILE: u64 = 0xfefefefefefefefe; // ~0x0101010101010101
pub const NOTHFILE: u64 = 0x7f7f7f7f7f7f7f7f; // ~0x8080808080808080

struct Bitboard {
    board: u64
}

impl Bitboard {
    pub fn new(val: u64) -> Self {
        Bitboard {board: val}
    }
}

pub mod postshift {
    use super::{NOTAFILE, NOTHFILE};

    pub fn east_one(b: u64) -> u64 {
        (b << 1) & NOTAFILE
    }
    pub fn no_ea_one(b: u64) -> u64 {
        (b << 9) & NOTAFILE
    }
    pub fn so_ea_one(b: u64) -> u64 {
        (b >> 7) & NOTAFILE
    }
    pub fn west_one(b: u64) -> u64 {
        (b >> 1) & NOTHFILE
    }
    pub fn so_we_one(b: u64) -> u64 {
        (b >> 9) & NOTHFILE
    }
    pub fn no_we_one(b: u64) -> u64 {
        (b << 7) & NOTHFILE
    }
    pub fn sout_one(b: u64) -> u64 {
        b >> 8
    }
    pub fn nort_one(b: u64) -> u64 {
        b << 8
    }
}

pub fn board_serialize(bitboard: u64) -> Vec<usize> {
    let mut board = bitboard;
    let mut pos: Vec<usize> = vec![];

    while board != 0 {
        let i = utils::bit_scan(board);
        pos.push(i);

        board = board & (board - 1)
    }
    pos
}
#[allow(dead_code)]
pub fn print_bitboard(bitboard: u64) {
    let mut output = String::new();
    for r in (0..8).rev() {
        for f in 0..8 {
            let i = (r * 8) + f;

            if ((bitboard >> i) & 1) == 1 {
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
pub fn print_bitboard_pos(bitboard: u64, pos: usize) {
    let mut output = String::new();
    for r in (0..8).rev() {
        for f in 0..8 {
            let i = (r * 8) + f;

            if i == pos {
                output.push_str("X ");
            } else if ((bitboard >> i) & 1) == 1 {
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
