use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

use super::engine::Move;
use super::board::{Board, PieceColor};

pub enum Flag {
    EXACT,
    ALPHA,
    BETA,
}

pub struct Entry {
    pub zobrist_hash: u64,
    pub depth: usize,
    pub flag: Flag,
    pub eval: i32,
    //pub ancient: bool,
    pub move_info: Move,
}

#[derive(Debug)]
pub struct ZobristKeys {
   pub pawns: [[u64; 64]; 2],
   pub bishops: [[u64; 64]; 2],
   pub knights: [[u64; 64]; 2],
   pub rooks: [[u64; 64]; 2],
   pub queens: [[u64; 64]; 2],
   pub kings: [[u64; 64]; 2],

   pub white_castling_rights: [u64; 4],
   pub black_castling_rights: [u64; 4],
   pub en_passant: [u64; 64],

   pub black_turn: u64
}

impl ZobristKeys {
    pub fn new() -> Self {
        let white_castling_rights = [
            RandomState::new().build_hasher().finish() as u64,
            RandomState::new().build_hasher().finish() as u64,
            RandomState::new().build_hasher().finish() as u64,
            RandomState::new().build_hasher().finish() as u64
        ];
        let black_castling_rights = [
            RandomState::new().build_hasher().finish() as u64,
            RandomState::new().build_hasher().finish() as u64,
            RandomState::new().build_hasher().finish() as u64,
            RandomState::new().build_hasher().finish() as u64
        ];

        let black_turn = RandomState::new().build_hasher().finish() as u64;

        
        ZobristKeys{
            pawns: gen_color_board_random(),
            bishops: gen_color_board_random(),
            knights: gen_color_board_random(),
            rooks: gen_color_board_random(),
            queens: gen_color_board_random(),
            kings: gen_color_board_random(),

            white_castling_rights: white_castling_rights,
            black_castling_rights: black_castling_rights,

            en_passant: gen_color_board_random()[0],

            black_turn: black_turn
        }
    }

    pub fn get_key(&self, board: Board) -> u64 {
        let mut zorb: u64 = 0;

        xor_board(&mut zorb, board.pawns, self.pawns);
        xor_board(&mut zorb, board.bishops, self.bishops);
        xor_board(&mut zorb, board.knights, self.knights);
        xor_board(&mut zorb, board.rooks, self.rooks);
        xor_board(&mut zorb, board.queens, self.queens);
        xor_board(&mut zorb, board.kings, self.kings);

        zorb ^= self.white_castling_rights[((board.castling & 0b1100) >> 2) as usize]; //White
        zorb ^= self.black_castling_rights[(board.castling & 0b0011) as usize];  //Black

        if board.en_passant != 65 {
            zorb ^=  self.en_passant[board.en_passant as usize];
        }

        if board.turn == PieceColor::Black {
            zorb ^= self.black_turn;
        }

        zorb
    }

}

fn gen_color_board_random() -> [[u64; 64]; 2] {
    let mut arr = [[0; 64]; 2];

    for ii in 0..2 {
            for i in 0..arr[ii].len() {
                arr[ii][i] = RandomState::new().build_hasher().finish() as u64;
            }
        }

    arr
}

fn xor_board(zorb: &mut u64, bitboards: [u64; 2], keys: [[u64; 64]; 2]) {
   for color in 0..2 {
       let mut bitboard = bitboards[color];
       while bitboard != 0 {
           let i = bitboard.trailing_zeros() as usize;
           *zorb ^= keys[color][i];
           bitboard ^= 1 << i;
       }
   }

}
