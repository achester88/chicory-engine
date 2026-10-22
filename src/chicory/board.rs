#[allow(unused_imports)]
use super::bitboard::{print_bitboard, print_bitboard_pos};
use crate::chicory::bitboard::board_serialize;
use crate::chicory::engine::Engine;
use crate::chicory::utils::bit_scan;
use core::ops::{Index, IndexMut, Not};
use crate::chicory::bitboard::Bitboard;
//use crate::chicory::board::MoveType::Move;
use crate::chicory::tables::ZobristKeys;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl Index<PieceColor> for [Bitboard] {
    type Output = Bitboard;

    fn index(&self, color: PieceColor) -> &Self::Output {
        match color {
            PieceColor::White => &self[0],
            PieceColor::Black => &self[1],
        }
    }
}

 impl IndexMut<PieceColor> for [Bitboard; 2] {
    fn index_mut(&mut self, color: PieceColor) -> &mut Self::Output {
        match color {
            PieceColor::White => &mut self[0],
            PieceColor::Black => &mut self[1],
        }
   }
}

impl Index<PieceColor> for [[Bitboard; 64]; 2] {
    type Output = [Bitboard; 64];

    fn index(&self, color: PieceColor) -> &Self::Output {
        match color {
            PieceColor::White => &self[0],
            PieceColor::Black => &self[1],
        }
    }
}

impl Index<PieceColor> for [[u64; 64]; 2] {
    type Output = [u64; 64];

    fn index(&self, color: PieceColor) -> &Self::Output {
        match color {
            PieceColor::White => &self[0],
            PieceColor::Black => &self[1],
        }
    }
}

impl Index<PieceColor> for [Option<f64>] {
    type Output = Option<f64>;

    fn index(&self, color: PieceColor) -> &Self::Output {
        match color {
            PieceColor::White => &self[0],
            PieceColor::Black => &self[1],
        }
    }
}

impl IndexMut<PieceColor> for [Option<f64>; 2] {
    fn index_mut(&mut self, color: PieceColor) -> &mut Self::Output {
        match color {
            PieceColor::White => &mut self[0],
            PieceColor::Black => &mut self[1],
        }
    }
}

impl Not for PieceColor {
    type Output = PieceColor;

    fn not(self) -> Self::Output {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LastMoveInfo {
    pub capture: Option<PieceType>,
    en_passant: u8,
    castling: u8,
    check_real: Bitboard,
    check_full: Bitboard
}


#[derive(Clone, PartialEq, Debug)]
pub struct Board {
    pub pawns: [Bitboard; 2],
    pub bishops: [Bitboard; 2],
    pub knights: [Bitboard; 2],
    pub rooks: [Bitboard; 2],
    pub queens: [Bitboard; 2],
    pub kings: [Bitboard; 2],

    pub turn: PieceColor,
    pub castling: u8,    //white, black | queenside, kingside QKqk
    pub en_passant: u8,  //position of available en passant
    pub check_real: Bitboard, //TODO USE BOOL AND CAL AS NEEDED
    pub check_full: Bitboard,
    pub half_moves: u16,
    pub full_move: u64,

    pub occupied: Bitboard,
    pub pieces: [Bitboard; 2], //All piece of the same color,
    pub zobrist_hash: u64,
    pub capture: Option<PieceType>,

    //pub last_capture: Vec<Option<PieceType>>,
    // pub last_move_info: Vec<LastMoveInfo>,
    //pub last_castling: Vec<u8>,
    //pub last_en_passant: Vec<u8>,
    //pub last_check_info: Vec<(u64, u64)>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveType {
    // Capture,
    CastleKingSide,
    CastleQueenSide,
    Move,
    Promotion,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub promote_to: Option<PieceType>,
    pub move_type: MoveType
}

impl Board {
    pub fn new(fen_str: &str, engine: &Engine) -> Self {
        let mut wp = 0;
        let mut wb = 0;
        let mut wn = 0;
        let mut wr = 0;
        let mut wq = 0;
        let mut wk = 0;
        let mut bp = 0;
        let mut bb = 0;
        let mut bn = 0;
        let mut br = 0;
        let mut bq = 0;
        let mut bk = 0;
        let mut castling: u8 = 0;
        let mut ep: u8 = 65;
        let mut hm = 0;
        let mut fm = 1;
        let mut f = 0;
        let mut r = 7;
        let fen: Vec<&str> = fen_str.split(" ").collect();

        //position
        let pos = fen[0];
        let pos_vec: Vec<char> = pos.chars().collect();
        for c in pos_vec {
            if c == '/' {
                r -= 1;
                f = 0;
            } else if c.is_numeric() {
                f += c.to_digit(10).unwrap() as usize;
            } else {
                let s = 1 << ((r * 8) + f); //set bit of i;
                                            //(1 << n)
                match c {
                    'P' => wp = wp | s,
                    'B' => wb = wb | s,
                    'N' => wn = wn | s,
                    'R' => wr = wr | s,
                    'Q' => wq = wq | s,
                    'K' => wk = wk | s,

                    'p' => bp = bp | s,
                    'b' => bb = bb | s,
                    'n' => bn = bn | s,
                    'r' => br = br | s,
                    'q' => bq = bq | s,
                    'k' => bk = bk | s,
                    _ => {}
                };

                f += 1;
            }
        }

        //casling
        if fen[2] != "-" {
            let cal: Vec<char> = fen[2].chars().collect();
            for c in cal {
                castling |= match c {
                    'Q' => 1 << 3,
                    'K' => 1 << 2,
                    'q' => 1 << 1,
                    'k' => 1 << 0,
                    _ => 0,
                }
            }
        }

        //En Passant
        if fen[3] != "-" {
            let square: Vec<char> = fen[3].chars().collect();
            let f = (square[0].to_ascii_lowercase() as u8) - 96; //a:0, h:9
            let r = square[1].to_digit(10).unwrap() as u8;
            ep = ((r - 1) * 8) + (f - 1);
        }

        //Halfmove
        if fen.len() > 3 {
            hm = fen[4].parse::<u16>().unwrap();
        }
        //Fullmove
        if fen.len() > 4 {
            fm = fen[5].parse::<u64>().unwrap();
        }

        let mut new_board = Board {
            pawns:   [Bitboard::new(wp), Bitboard::new(bp)],
            bishops: [Bitboard::new(wb), Bitboard::new(bb)],
            knights: [Bitboard::new(wn), Bitboard::new(bn)],
            rooks:   [Bitboard::new(wr), Bitboard::new(br)],
            queens:  [Bitboard::new(wq), Bitboard::new(bq)],
            kings:   [Bitboard::new(wk), Bitboard::new(bk)],
            turn: if fen[1] == "w" {
                PieceColor::White
            } else {
                PieceColor::Black
            },
            castling: 0b1111_0000 | castling,
            check_real: Bitboard::new(0),
            check_full: Bitboard::new(0),
            capture: None,
            en_passant: ep,
            half_moves: hm,
            full_move: fm,
            occupied: Bitboard::new(wp | wb | wn | wr | wq | wk | bp | bb | bn | br | bq | bk),
            pieces: [Bitboard::new(wp | wb | wn | wr | wq | wk), Bitboard::new(bp | bb | bn | br | bq | bk)],
            zobrist_hash: 0,
            // last_move_info: vec![],

        };

        let king_board = new_board.kings[new_board.turn];
        if king_board.val != 0 {
            let king_pos = board_serialize(king_board);

            let (cr, cf) = engine.cal_check(&new_board, king_pos[0], !new_board.turn, false);

            new_board.check_real = cr;
            new_board.check_full = cf;
        }

        new_board.zobrist_hash = engine.zobrist_keys.get_key(&new_board);

        new_board
    }

    pub fn lookup(&self, pos: usize) -> Option<(PieceColor, PieceType)> {
        let board = Bitboard::new((1 << pos));

        let color: PieceColor;

        if board & self.pieces[PieceColor::White] != Bitboard::new(0) {
            color = PieceColor::White;
        } else if board & self.pieces[PieceColor::Black] != Bitboard::new(0) {
            color = PieceColor::Black;
        } else {
            return None;
        }

        if board & (self.pawns[color] | self.bishops[color] | self.knights[color]) != Bitboard::new(0) {
            if board & self.pawns[color] != Bitboard::new(0) {
                Some((color, PieceType::Pawn))
            } else if board & self.bishops[color] != Bitboard::new(0) {
                Some((color, PieceType::Bishop))
            } else {
                Some((color, PieceType::Knight))
            }
        } else {
            if board & self.rooks[color] != Bitboard::new(0) {
                Some((color, PieceType::Rook))
            } else if board & self.queens[color] != Bitboard::new(0) {
                Some((color, PieceType::Queen))
            } else {
                Some((color, PieceType::King))
            }
        }
    }

    pub fn move_piece(&mut self, to: usize, from: usize, zobrist_keys: &ZobristKeys) -> Option<PieceType> {

        let (pc, pt) = self.lookup(from).unwrap(); //From piece should always be there

        //Remove Current Castling State From Hash
        self.zobrist_hash ^= zobrist_keys.white_castling_rights[((self.castling & 0b1100) >> 2) as usize];
        self.zobrist_hash ^= zobrist_keys.black_castling_rights[(self.castling & 0b0011) as usize];  //Black

        //Check if en_passant needs updating

        //CHECK FOR CHECK
        if self.castling != 0 && (pt == PieceType::Rook || pt == PieceType::King) {
            if pt == PieceType::King {
                let values;
                match pc {
                    PieceColor::White => {
                        values = 0b0011_0011;
                    }
                    PieceColor::Black => {
                        values = 0b1100_1100;
                    }
                };
                self.castling &= values;
            } else {
                //if rook cancel side its on
                //check if from matches
                match from {
                    0 => {
                        self.castling &= 0b0111_0111;
                    } //white queenside
                    7 => {
                        self.castling &= 0b1011_1011;
                    } //white kingside
                    56 => {
                        self.castling &= 0b1101_1101;
                    } //black queenside
                    63 => {
                        self.castling &= 0b1110_1110;
                    } //black kingside
                    _ => {}
                };
            }
        }

        let mut capture = match self.lookup(to) {
            Some((old_pc, old_pt)) => {
                self.remove_castling(to, old_pt);
                self.remove_piece(to, &old_pt, old_pc, zobrist_keys);
                //Remove Opps piece from to pos
                Some(old_pt)
            },
            None =>  None
        };

        

        if &pt == &PieceType::Pawn {
            if to == (self.en_passant as usize) {
                if pc == PieceColor::White {
                    self.remove_piece(to - 8, &PieceType::Pawn, PieceColor::Black, zobrist_keys);
                } else {
                    self.remove_piece(to + 8, &PieceType::Pawn, PieceColor::White, zobrist_keys);
                }
                capture = Some(PieceType::Pawn);
            }

            if self.en_passant != 65 {
                self.zobrist_hash ^= zobrist_keys.en_passant[self.en_passant as usize];
            }

            if to > 16 && (to - 16) == from && from > 7 && from < 16 {
                //white
                self.en_passant = (to as u8) - 8; //south_one
                self.zobrist_hash ^= zobrist_keys.en_passant[self.en_passant as usize];
            } else if to < 48 && (to + 16) == from && from > 47 && from < 56 {
                //black
                self.en_passant = (to as u8) + 8; //north_one
                self.zobrist_hash ^= zobrist_keys.en_passant[self.en_passant as usize];
            } else {
                self.en_passant = 65;
            }
        } else {
            if self.en_passant != 65 {
                self.zobrist_hash ^= zobrist_keys.en_passant[self.en_passant as usize];
            }
            self.en_passant = 65;
        }

        //Remove from pos piece
        self.remove_piece(from, &pt, pc, zobrist_keys);
        //Add piece to to pos
        self.add_piece(to, &pt, pc, zobrist_keys);

       // self.recalc_board();
        
        //self.pieces = [white_pieces, black_pieces];

        self.next_turn(zobrist_keys);

        //Add Current Castling State Back to Hash
        self.zobrist_hash ^= zobrist_keys.white_castling_rights[((self.castling & 0b1100) >> 2) as usize];
        self.zobrist_hash ^= zobrist_keys.black_castling_rights[(self.castling & 0b0011) as usize];  //Black

        capture
    }

    pub fn unmove_piece(&mut self, to: usize, from: usize, zobrist_keys: &ZobristKeys, did_capture_occur: Option<PieceType>) {

        let (pc, pt) = match self.lookup(to) {
            Some((old_pc, old_pt)) => { (old_pc, old_pt) },
            None => {
                self.print_board();
                panic!("ERROR: from: {}, from: {}", from, to);
            }
        };

        self.remove_piece(to, &pt, pc, zobrist_keys);

        match did_capture_occur {
            Some(capture_type) => {
            if to == (self.en_passant as usize) && &capture_type == &PieceType::Pawn {
                if pc == PieceColor::White {
                    self.add_piece(to-8, &PieceType::Pawn, PieceColor::Black, zobrist_keys);
                }else {
                    self.add_piece(to+8, &PieceType::Pawn, PieceColor::White, zobrist_keys);
                }
            }
            else {
                self.add_piece(to, &did_capture_occur.unwrap(), !pc, zobrist_keys);
            }
            },
            None => {}
        }

        //Remove from pos piece
        //Add piece to to pos
        self.add_piece(from, &pt, pc, zobrist_keys);

        //self.recalc_board();

        self.untake_turn(zobrist_keys);

    }

    fn remove_piece(&mut self, pos: usize, pt: &PieceType, pc: PieceColor, zobrist_keys: &ZobristKeys) {
        //Remove Opps piece from to pos
        //print_bitboard(self.occupied);
        //print_bitboard_pos(1 << pos, pos);
        self.occupied.val &= !(1 << pos);
        //print_bitboard(self.occupied);
        self.pieces[pc].val &= !(1 << pos);

        match pt {
            PieceType::Pawn => {
                self.pawns[pc] = self.pawns[pc] & !Bitboard::new((1 << pos));
                self.zobrist_hash ^= zobrist_keys.pawns[pc][pos];
                //self.occupied &= !(1 << pos);
            },
            PieceType::Bishop => {
                self.bishops[pc] = self.bishops[pc] & !Bitboard::new(1 << pos);
                self.zobrist_hash ^= zobrist_keys.bishops[pc][pos];
            },
            PieceType::Knight => {
                self.knights[pc] = self.knights[pc] & !Bitboard::new(1 << pos);
                self.zobrist_hash ^= zobrist_keys.knights[pc][pos];
            },
            PieceType::Rook => {
                self.rooks[pc] = self.rooks[pc] & !Bitboard::new(1 << pos);
                self.zobrist_hash ^= zobrist_keys.rooks[pc][pos];
            },
            PieceType::Queen => {
                self.queens[pc] = self.queens[pc] & !Bitboard::new(1 << pos);
                self.zobrist_hash ^= zobrist_keys.queens[pc][pos];
            },
            PieceType::King => {
                self.kings[pc] = self.kings[pc] & !Bitboard::new(1 << pos);
                self.zobrist_hash ^= zobrist_keys.kings[pc][pos];
            },
        };
    }

    fn add_piece(&mut self, pos: usize, pt: &PieceType, pc: PieceColor, zobrist_keys: &ZobristKeys) {

        self.occupied |= Bitboard::new(1 << pos);
        self.pieces[pc] |= Bitboard::new(1 << pos);

        match pt {
            PieceType::Pawn => {
                self.pawns[pc] = self.pawns[pc] | Bitboard::new(1 << pos);
                self.zobrist_hash ^= zobrist_keys.pawns[pc][pos];
            },
            PieceType::Bishop => {
                self.bishops[pc] = self.bishops[pc] | Bitboard::new(1 << pos);
                self.zobrist_hash ^= zobrist_keys.bishops[pc][pos];
            },
            PieceType::Knight => {
                self.knights[pc] = self.knights[pc] | Bitboard::new(1 << pos);
                self.zobrist_hash ^= zobrist_keys.knights[pc][pos];
            },
            PieceType::Rook => {
                self.rooks[pc] = self.rooks[pc] | Bitboard::new(1 << pos);
                self.zobrist_hash ^= zobrist_keys.rooks[pc][pos];
            },
            PieceType::Queen => {
                self.queens[pc] = self.queens[pc] | Bitboard::new(1 << pos);
                self.zobrist_hash ^= zobrist_keys.queens[pc][pos];
            },
            PieceType::King => {
                self.kings[pc] = self.kings[pc] | Bitboard::new(1 << pos);
                self.zobrist_hash ^= zobrist_keys.kings[pc][pos];
            },
        };
    }
    
//     pub fn recalc_board(&mut self) {
//         let white_pieces = self.pawns[PieceColor::White]
//             | self.bishops[PieceColor::White]
//             | self.knights[PieceColor::White]
//             | self.rooks[PieceColor::White]
//             | self.queens[PieceColor::White]
//             | self.kings[PieceColor::White];
//         let black_pieces = self.pawns[PieceColor::Black]
//             | self.bishops[PieceColor::Black]
//             | self.knights[PieceColor::Black]
//             | self.rooks[PieceColor::Black]
//             | self.queens[PieceColor::Black]
//             | self.kings[PieceColor::Black];
//
//         self.occupied = white_pieces | black_pieces;
//         self.pieces = [white_pieces, black_pieces];
// }

    fn remove_castling(&mut self, to: usize, pt: PieceType) {
        if (self.castling & 0b1111) != 0
            && ((1 << to) & (0x8100000000000081u64)) != 0
            && pt == PieceType::Rook
        {
            if to == 0 {
                self.castling &= 0b0111;
            } else if to == 7 {
                self.castling &= 0b1011;
            } else if to == 56 {
                self.castling &= 0b1101;
            } else if to == 63 {
                self.castling &= 0b1110;
            }
        }
    }

    pub fn promote(&mut self, from: usize, to: usize, promot_to: PieceType, zobrist_keys: &ZobristKeys) -> Option<PieceType> {
        self.zobrist_hash ^= zobrist_keys.white_castling_rights[((self.castling & 0b1100) >> 2) as usize];
        self.zobrist_hash ^= zobrist_keys.black_castling_rights[(self.castling & 0b0011) as usize];  //Black

        let (pc, _) = self.lookup(from).unwrap();

        let capture = match self.lookup(to) {
            Some((old_pc, old_pt)) => {
                self.remove_castling(to, old_pt);
                self.remove_piece(to, &old_pt, old_pc, zobrist_keys);
                //Remove Opps piece from to pos
                Some(old_pt)
            },
            None =>  None
        };

        self.zobrist_hash ^= zobrist_keys.white_castling_rights[((self.castling & 0b1100) >> 2) as usize];
        self.zobrist_hash ^= zobrist_keys.black_castling_rights[(self.castling & 0b0011) as usize];  //Black

        self.next_turn(zobrist_keys);

        self.remove_piece(from, &PieceType::Pawn, pc, zobrist_keys);

        if self.en_passant != 65 {
            self.zobrist_hash ^= zobrist_keys.en_passant[self.en_passant as usize];
            self.en_passant = 65;
        }

        self.occupied |= Bitboard::new(1 << to);
        self.pieces[pc] |= Bitboard::new(1 << to);
        match promot_to {
            PieceType::Knight => {
                self.knights[pc] = self.knights[pc] | Bitboard::new(1 << to);
                self.zobrist_hash ^= zobrist_keys.knights[pc][to];
            },
            PieceType::Bishop => {
                self.bishops[pc] = self.bishops[pc] | Bitboard::new(1 << to);
                self.zobrist_hash ^= zobrist_keys.bishops[pc][to];
            },
            PieceType::Rook => {
                self.rooks[pc] = self.rooks[pc] | Bitboard::new(1 << to);
                self.zobrist_hash ^= zobrist_keys.rooks[pc][to];
            },
            PieceType::Queen => {
                self.queens[pc] = self.queens[pc] | Bitboard::new(1 << to);
                self.zobrist_hash ^= zobrist_keys.queens[pc][to];
            },
            _ => { panic!("Promotion of unsuported piece type") }
        }

        //Check for check
        //In case other piece is removed all case need to be run
           // self.recalc_board();

        capture
    }

    pub fn unpromote(&mut self, from: usize, to: usize, promot_to: PieceType, zobrist_keys: &ZobristKeys, did_capture_occur: Option<PieceType>) {

        let (pc, _) = self.lookup(to).unwrap();
        //let (old_pc, old_pt) = new_board.lookup(to);

        self.occupied &= !Bitboard::new(1 << to);
        self.pieces[pc] &= !Bitboard::new(1 << to);

        match did_capture_occur {
            Some(capture_type) => {
                self.add_piece(to, &capture_type, !pc, zobrist_keys);
            },
            None => {}
        }

        self.untake_turn(zobrist_keys);
       self.add_piece(from, &PieceType::Pawn, pc, zobrist_keys);
        match promot_to {
            PieceType::Knight => {
                self.knights[pc] = self.knights[pc] & !Bitboard::new(1 << to);
                self.zobrist_hash ^= zobrist_keys.knights[pc][to];
            },
            PieceType::Bishop => {
                self.bishops[pc] = self.bishops[pc] & !Bitboard::new(1 << to);
                self.zobrist_hash ^= zobrist_keys.bishops[pc][to];
            },
            PieceType::Rook => {
                self.rooks[pc] = self.rooks[pc] & !Bitboard::new(1 << to);
                self.zobrist_hash ^= zobrist_keys.rooks[pc][to];
            },
            PieceType::Queen => {
                self.queens[pc] = self.queens[pc] & !Bitboard::new(1 << to);
                self.zobrist_hash ^= zobrist_keys.queens[pc][to];
            },
            _ => { panic!("Promotion of unsuported piece type") }
        }

        //Check for check
        //In case other piece is removed all case need to be run
        // self.recalc_board();


        //out.push(Move{from, to, board: new_boards[0], promote_to: Some(PieceType::Knight), capture});

    }

    pub fn castle(&mut self, code: u8, zobrist_keys: &ZobristKeys) -> Option<PieceType>  {
        let king_from_pos: usize;
        let rook_from_pos: usize;

        let king_to_pos: usize;
        let rook_to_pos: usize;

        self.zobrist_hash ^= zobrist_keys.white_castling_rights[((self.castling & 0b1100) >> 2) as usize];
        self.zobrist_hash ^= zobrist_keys.black_castling_rights[(self.castling & 0b0011) as usize];  //Black

        match self.turn {
            PieceColor::White => {
                king_from_pos = 4;
                if code == 88 { //Queenside
                    rook_from_pos = 0;

                    rook_to_pos = 3;
                    king_to_pos = 2;
                } else { //KingSide
                    rook_from_pos = 7;

                    rook_to_pos = 5;
                    king_to_pos = 6;
                }

                self.castling &= 0b0011;
            }
            PieceColor::Black => {
                king_from_pos = 60;
                if code == 88 {
                    rook_from_pos = 56;

                    rook_to_pos = 59;
                    king_to_pos = 58;
                } else {
                    rook_from_pos = 63;

                    rook_to_pos = 61;
                    king_to_pos = 62;
                }

                self.castling &= 0b1100;
            }
        }

        if self.en_passant != 65 {
            self.zobrist_hash ^= zobrist_keys.en_passant[self.en_passant as usize];
            self.en_passant = 65;
        }

        self.remove_piece(king_from_pos, &PieceType::King, self.turn, zobrist_keys);
        self.remove_piece(rook_from_pos, &PieceType::Rook, self.turn, zobrist_keys);

        self.add_piece(king_to_pos, &PieceType::King, self.turn, zobrist_keys);
        self.add_piece(rook_to_pos, &PieceType::Rook, self.turn, zobrist_keys);

        // self.recalc_board();

        self.next_turn(zobrist_keys);

        self.zobrist_hash ^= zobrist_keys.white_castling_rights[((self.castling & 0b1100) >> 2) as usize];
        self.zobrist_hash ^= zobrist_keys.black_castling_rights[(self.castling & 0b0011) as usize];  //Black

        return None; //Capture can never occur
    }

    pub fn uncastle(&mut self, code: u8, zobrist_keys: &ZobristKeys) {
        let king_from_pos: usize;
        let rook_from_pos: usize;

        let king_to_pos: usize;
        let rook_to_pos: usize;

        match self.turn {
            PieceColor::Black => { //Flip turns (if white's turn last move was for black)
                king_from_pos = 4;
                if code == 88 { //Queenside
                    rook_from_pos = 0;

                    rook_to_pos = 3;
                    king_to_pos = 2;
                } else { //KingSide
                    rook_from_pos = 7;

                    rook_to_pos = 5;
                    king_to_pos = 6;
                }

                //self.castling &= 0b0011;
            }
            PieceColor::White => {
                king_from_pos = 60;
                if code == 88 {
                    rook_from_pos = 56;

                    rook_to_pos = 59;
                    king_to_pos = 58;
                } else {
                    rook_from_pos = 63;

                    rook_to_pos = 61;
                    king_to_pos = 62;
                }

                //self.castling &= 0b1100;
            }
        }


        self.remove_piece(king_to_pos, &PieceType::King, !self.turn, zobrist_keys);
        self.remove_piece(rook_to_pos, &PieceType::Rook, !self.turn, zobrist_keys);

        self.add_piece(king_from_pos, &PieceType::King, !self.turn, zobrist_keys);
        self.add_piece(rook_from_pos, &PieceType::Rook, !self.turn, zobrist_keys);

        // self.recalc_board();

        self.untake_turn(zobrist_keys);

        //new_board
        //Move{from: code as usize, to: code as usize, board: new_board, promote_to: None, capture: false}

    }

    fn next_turn(&mut self, zobrist_keys: &ZobristKeys) {
        if self.turn == PieceColor::Black {
            self.turn = PieceColor::White;
            self.full_move += 1;
        } else {
            self.turn = PieceColor::Black;
        };

        self.zobrist_hash ^= zobrist_keys.black_turn;

        self.half_moves = self.half_moves + 1;
    }

    fn untake_turn(&mut self, zobrist_keys: &ZobristKeys) {
        if self.turn == PieceColor::White {
            self.turn = PieceColor::Black;
            self.full_move -= 1;
        } else {
            self.turn = PieceColor::White;
        };

        self.zobrist_hash ^= zobrist_keys.black_turn;

        self.half_moves = self.half_moves - 1;
    }

    pub fn lan_to_pos(code: &str) -> usize {
        let square: Vec<char> = code.chars().collect();
        let f = (square[0].to_ascii_lowercase() as u8) - 96; //a:0, h:9
        let r = square[1].to_digit(10).unwrap() as u8;

        (((r - 1) * 8) + (f - 1)) as usize
    }

    pub fn pos_to_lan(pos: usize) -> String {
        let c = (((pos as u8) % 8) + 97) as char;
        let n = (((pos as u8) / 8) + 1).to_string();

        let mut out = c.to_string();
        out.push_str(&n);

        out
    }

    pub fn move_to_lan(cur_move: &Move) -> String {

        let promo_to = match cur_move.promote_to {
            Some(x) => match x {
                PieceType::Queen => String::from("q"),
                PieceType::Rook => String::from("r"),
                PieceType::Bishop => String::from("b"),
                PieceType::Knight => String::from("n"),
                _ => String::from(""),
            },
            None => String::from(""),
        };

        [Board::pos_to_lan(cur_move.from), Board::pos_to_lan(cur_move.to), promo_to].join("")
    }

    pub fn lan_to_move(str: &str) -> Move { //NOTE: ASSUMES THERE WAS NO CAPTURE
        let from: usize;
        let to: usize;
        let mut promote_to = None;

        from = Board::lan_to_pos(&str[0..2]);
        to = Board::lan_to_pos(&str[2..4]);

        if str.len() == 5 {
            promote_to = Some(match str.chars().nth(4).unwrap() {
                'n' | 'N' => PieceType::Knight,
                'b' | 'B' => PieceType::Bishop,
                'r' | 'R' => PieceType::Rook,
                'q' | 'Q' => PieceType::Queen,
                _ => { panic!("invalid lan_to_move"); }
            });

            return Move {
                from,
                to,
                promote_to,
                move_type: MoveType::Promotion
            }
        }
          return Move { from, to, promote_to, move_type: MoveType::Move };
    }

   pub fn is_legal(&self, engine: &Engine) -> bool {
        let king_pos = board_serialize(self.kings[!self.turn])[0];
        let (cr, _) = engine.cal_check(&self, king_pos, self.turn, true); 
        if cr.val != 0 {
            return false;
        }
        
        return true;
   } 

    pub fn make_move(&mut self, engine: &Engine, move_: Move) -> LastMoveInfo { //returns if a capture happened
        //self.last_castling.push(self.castling);
        //self.last_en_passant.push(self.en_passant);
        //self.last_capture.push(self.capture);
        //self.last_check_info.push((self.check_real, self.check_full));
        let lmi = LastMoveInfo {
            castling: self.castling,
            en_passant: self.en_passant,
            capture: self.capture,
            check_real: self.check_real,
            check_full: self.check_full,
        };

        let did_capture_occur: Option<PieceType> = match move_.move_type {
            MoveType::Move => self.move_piece(move_.to, move_.from, &engine.zobrist_keys),
            MoveType::CastleKingSide => self.castle(80, &engine.zobrist_keys),
            MoveType::CastleQueenSide => self.castle(88, &engine.zobrist_keys),
            MoveType::Promotion => self.promote(move_.from, move_.to, move_.promote_to.unwrap(), &engine.zobrist_keys),
        };

        self.capture = did_capture_occur;

        if self.kings[self.turn] != Bitboard::new(0) {
            let king_pos = bit_scan(self.kings[self.turn]);//board_serialize(self.kings[self.turn]);
            let (cr, cf) = engine.cal_check(&self, king_pos, !self.turn, false);
            self.check_real = cr;
            self.check_full = cf;
        } else {
            self.check_real.val = 0;
            self.check_full.val = 0;
        }
        return lmi;
    }

    pub fn unmake_move(&mut self, move_: Move, engine: &Engine, last_move_info: LastMoveInfo) {

        //Remove Current Castling State From Hash
        self.zobrist_hash ^= &engine.zobrist_keys.white_castling_rights[((self.castling & 0b1100) >> 2) as usize];
        self.zobrist_hash ^= &engine.zobrist_keys.black_castling_rights[(self.castling & 0b0011) as usize];  //Black

        if self.en_passant != 65 {
           self.zobrist_hash ^= engine.zobrist_keys.en_passant[self.en_passant as usize];
        }

        let did_capture_occur = self.capture;//self.last_capture.pop().expect("Expected at least one capture state");
        // let last_move_info = self.last_move_info.pop().expect("Expected at least one previous state");
        self.capture = last_move_info.capture;
        self.castling = last_move_info.castling;
        self.en_passant = last_move_info.en_passant;
        self.check_full = last_move_info.check_full;
        self.check_real = last_move_info.check_real;
        //Add Current Castling State Back to Hash
        self.zobrist_hash ^= &engine.zobrist_keys.white_castling_rights[((self.castling & 0b1100) >> 2) as usize];
        self.zobrist_hash ^= &engine.zobrist_keys.black_castling_rights[(self.castling & 0b0011) as usize];  //Black

        match move_.move_type {
            MoveType::Move => self.unmove_piece(move_.to, move_.from, &engine.zobrist_keys, did_capture_occur),
            MoveType::CastleKingSide => self.uncastle(80, &engine.zobrist_keys),
            MoveType::CastleQueenSide => self.uncastle(88, &engine.zobrist_keys),
            MoveType::Promotion => self.unpromote(move_.from, move_.to, move_.promote_to.unwrap(), &engine.zobrist_keys, did_capture_occur),

        };

        if self.en_passant != 65 {
           self.zobrist_hash ^= engine.zobrist_keys.en_passant[self.en_passant as usize];
        }


        // return did_capture_occur.is_some();
    }

    #[allow(dead_code)]
    pub fn print_board(&self) {
        let set = [
            ["P", "N", "B", "R", "Q", "K", "p", "n", "b", "r", "q", "k"],
            ["󰡙", "󰡘", "󰡜", "󰡛", "󰡚", "󰡗", "", "", "", "", "", ""],
        ];

        let n = 0; //Replace with cmd arg

        println!(
            "\n{} to move:",
            if self.turn == PieceColor::White {
                "White"
            } else {
                "Black"
            }
        );
        println!("-----");
        for r in [7, 6, 5, 4, 3, 2, 1, 0] {
            //cant be bothered
            for f in 0..8 {
                let i = (r * 8) + f;

                if ((self.pawns[PieceColor::White] >> i) & Bitboard::new(1)) == Bitboard::new(1) {
                    print!("{} ", set[n][0]);
                } else if ((self.bishops[PieceColor::White] >> i) & Bitboard::new(1)) == Bitboard::new(1) {
                    print!("{} ", set[n][2]);
                } else if ((self.knights[PieceColor::White] >> i) & Bitboard::new(1)) == Bitboard::new(1) {
                    print!("{} ", set[n][1]);
                } else if ((self.rooks[PieceColor::White] >> i) & Bitboard::new(1)) == Bitboard::new(1) {
                    print!("{} ", set[n][3]);
                } else if ((self.queens[PieceColor::White] >> i) & Bitboard::new(1)) == Bitboard::new(1) {
                    print!("{} ", set[n][4]);
                } else if ((self.kings[PieceColor::White] >> i) & Bitboard::new(1)) == Bitboard::new(1) {
                    print!("{} ", set[n][5]);
                } else if ((self.pawns[PieceColor::Black] >> i) & Bitboard::new(1)) == Bitboard::new(1) {
                    print!("{} ", set[n][6]);
                } else if ((self.bishops[PieceColor::Black] >> i) & Bitboard::new(1)) == Bitboard::new(1) {
                    print!("{} ", set[n][8]);
                } else if ((self.knights[PieceColor::Black] >> i) & Bitboard::new(1)) == Bitboard::new(1) {
                    print!("{} ", set[n][7]);
                } else if ((self.rooks[PieceColor::Black] >> i) & Bitboard::new(1)) == Bitboard::new(1) {
                    print!("{} ", set[n][9]);
                } else if ((self.queens[PieceColor::Black] >> i) & Bitboard::new(1)) == Bitboard::new(1) {
                    print!("{} ", set[n][10]);
                } else if ((self.kings[PieceColor::Black] >> i) & Bitboard::new(1)) == Bitboard::new(1) {
                    print!("{} ", set[n][11]);
                } else {
                    if i == self.en_passant {
                        print!("# ");
                    } else {
                        print!(". ");
                    }
                }
            }
            println!();
        }
        println!("-----");
        println!(
            "Castling Rights: {}{} {}{}",
            if self.castling & 0b1000 != 0 {
                "Q"
            } else {
                "-"
            },
            if self.castling & 0b0100 != 0 {
                "K"
            } else {
                "-"
            },
            if self.castling & 0b0010 != 0 {
                "q"
            } else {
                "-"
            },
            if self.castling & 0b0001 != 0 {
                "k"
            } else {
                "-"
            }
        );
        println!(
            "Temp Castling Rights: {}{} {}{}",
            if self.castling & 0b1000_0000 != 0 {
                "Q"
            } else {
                "-"
            },
            if self.castling & 0b0100_0000 != 0 {
                "K"
            } else {
                "-"
            },
            if self.castling & 0b0010_0000 != 0 {
                "q"
            } else {
                "-"
            },
            if self.castling & 0b0001_0000 != 0 {
                "k"
            } else {
                "-"
            }
        );
        println!("Halfmoves: {}", self.half_moves);
        println!("Fullmoves: {}", self.full_move);

        println!();
    }
}
