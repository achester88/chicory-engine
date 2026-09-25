#[allow(unused_imports)]
use super::bitboard::{print_bitboard, print_bitboard_pos};
use crate::chicory::bitboard::board_serialize;
use crate::chicory::engine::Engine;
use crate::chicory::engine::Move;
use core::ops::{Index, IndexMut, Not};
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

impl Index<PieceColor> for [u64] {
    type Output = u64;

    fn index(&self, color: PieceColor) -> &Self::Output {
        match color {
            PieceColor::White => &self[0],
            PieceColor::Black => &self[1],
        }
    }
}

 impl IndexMut<PieceColor> for [u64; 2] {
    fn index_mut(&mut self, color: PieceColor) -> &mut Self::Output {
        match color {
            PieceColor::White => &mut self[0],
            PieceColor::Black => &mut self[1],
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

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Board {
    pub pawns: [u64; 2],
    pub bishops: [u64; 2],
    pub knights: [u64; 2],
    pub rooks: [u64; 2],
    pub queens: [u64; 2],
    pub kings: [u64; 2],

    pub turn: PieceColor,
    pub castling: u8,    //white, black | queenside, kingside QKqk
    pub en_passant: u8,  //position of available en passant
    pub check_real: u64, //TODO USE BOOL AND CAL AS NEEDED
    pub check_full: u64,
    pub half_moves: u16,
    pub full_move: u64,

    pub occupied: u64,
    pub pieces: [u64; 2], //All piece of the same color,
    pub zobrist_hash: u64,
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
            pawns: [wp, bp],
            bishops: [wb, bb],
            knights: [wn, bn],
            rooks: [wr, br],
            queens: [wq, bq],
            kings: [wk, bk],
            turn: if fen[1] == "w" {
                PieceColor::White
            } else {
                PieceColor::Black
            },
            castling: 0b1111_0000 | castling,
            check_real: 0,
            check_full: 0,
            en_passant: ep,
            half_moves: hm,
            full_move: fm,
            occupied: wp | wb | wn | wr | wq | wk | bp | bb | bn | br | bq | bk,
            pieces: [wp | wb | wn | wr | wq | wk, bp | bb | bn | br | bq | bk],
            zobrist_hash: 0,
        };

        let king_board = new_board.kings[new_board.turn];
        if king_board != 0 {
            let king_pos = board_serialize(king_board);

            let (cr, cf) = engine.cal_check(&new_board, king_pos[0], !new_board.turn);

            new_board.check_real = cr;
            new_board.check_full = cf;
        }

        new_board.zobrist_hash = engine.zobrist_keys.get_key(new_board);

        new_board
    }

    pub fn lookup(&self, pos: usize) -> Option<(PieceColor, PieceType)> {
        let board = 1 << pos;

        let color: PieceColor;

        if board & self.pieces[PieceColor::White] != 0 {
            color = PieceColor::White;
        } else if board & self.pieces[PieceColor::Black] != 0 {
            color = PieceColor::Black;
        } else {
            return None;
        }

        if board & (self.pawns[color] | self.bishops[color] | self.knights[color]) != 0 {
            if board & self.pawns[color] != 0 {
                Some((color, PieceType::Pawn))
            } else if board & self.bishops[color] != 0 {
                Some((color, PieceType::Bishop))
            } else {
                Some((color, PieceType::Knight))
            }
        } else {
            if board & self.rooks[color] != 0 {
                Some((color, PieceType::Rook))
            } else if board & self.queens[color] != 0 {
                Some((color, PieceType::Queen))
            } else {
                Some((color, PieceType::King))
            }
        }
    }

    pub fn move_piece(&self, to: usize, from: usize, zobrist_keys: &ZobristKeys) -> Move {
        let mut new_board = self.clone();

        let (pc, pt) = self.lookup(from).unwrap(); //To piece should always be there

        //Remove Current Castling State From Hash
        new_board.zobrist_hash ^= zobrist_keys.white_castling_rights[((new_board.castling & 0b1100) >> 2) as usize];
        new_board.zobrist_hash ^= zobrist_keys.black_castling_rights[(new_board.castling & 0b0011) as usize];  //Black

        //Check if en_passant needs updating
        new_board.en_passant_check(to, from, &pt, zobrist_keys);

        //CHECK FOR CHECK
        if new_board.castling != 0 && (pt == PieceType::Rook || pt == PieceType::King) {
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
                new_board.castling &= values;
            } else {
                //if rook cancel side its on
                //check if from matches
                match from {
                    0 => {
                        new_board.castling &= 0b0111_0111;
                    } //white queenside
                    7 => {
                        new_board.castling &= 0b1011_1011;
                    } //white kingside
                    56 => {
                        new_board.castling &= 0b1101_1101;
                    } //black queenside
                    63 => {
                        new_board.castling &= 0b1110_1110;
                    } //black kingside
                    _ => {}
                };
            }
        }

        let capture = match self.lookup(to) {
            Some((old_pc, old_pt)) => {
                new_board.remove_castling(to, old_pt);
                new_board.remove_piece(to, &old_pt, old_pc, zobrist_keys);
                //Remove Opps piece from to pos
                true
            },
            None =>  false
        };

        //Remove from pos piece
        new_board.remove_piece(from, &pt, pc, zobrist_keys);
        //Add piece to to pos
        new_board.add_piece(to, &pt, pc, zobrist_keys);

        new_board.recalc_board();

        new_board.next_turn(zobrist_keys);

        //Add Current Castling State Back to Hash
        new_board.zobrist_hash ^= zobrist_keys.white_castling_rights[((new_board.castling & 0b1100) >> 2) as usize];
        new_board.zobrist_hash ^= zobrist_keys.black_castling_rights[(new_board.castling & 0b0011) as usize];  //Black

        //new_board
        Move{from, to, board: new_board, promote_to: None, capture}
    }

    fn en_passant_check(&mut self, to: usize, from: usize, pt: &PieceType, zobrist_keys: &ZobristKeys) {

        if pt == &PieceType::Pawn {
            if to == self.en_passant as usize {
                //remove pawn at en_pass
                match self.turn {
                    PieceColor::White => {
                        self.pawns[PieceColor::Black] = self.pawns[PieceColor::Black] & !(1 << self.en_passant - 8);
                        self.zobrist_hash ^= zobrist_keys.pawns[PieceColor::Black][(self.en_passant - 8) as usize];

                    }
                    PieceColor::Black => {
                        self.pawns[PieceColor::White] = self.pawns[PieceColor::White] & !(1 << self.en_passant + 8);
                        self.zobrist_hash ^= zobrist_keys.pawns[PieceColor::White][(self.en_passant + 8) as usize];
                    }
                };
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
            self.en_passant = 65;
        }


    }

    fn remove_piece(&mut self, pos: usize, pt: &PieceType, pc: PieceColor, zobrist_keys: &ZobristKeys) {
        //Remove Opps piece from to pos
        match pt {
            PieceType::Pawn => {
                self.pawns[pc] = self.pawns[pc] & !(1 << pos);
                self.zobrist_hash ^= zobrist_keys.pawns[pc][pos];
            },
            PieceType::Bishop => {
                self.bishops[pc] = self.bishops[pc] & !(1 << pos);
                self.zobrist_hash ^= zobrist_keys.bishops[pc][pos];
            },
            PieceType::Knight => {
                self.knights[pc] = self.knights[pc] & !(1 << pos);
                self.zobrist_hash ^= zobrist_keys.knights[pc][pos];
            },
            PieceType::Rook => {
                self.rooks[pc] = self.rooks[pc] & !(1 << pos);
                self.zobrist_hash ^= zobrist_keys.rooks[pc][pos];
            },
            PieceType::Queen => {
                self.queens[pc] = self.queens[pc] & !(1 << pos);
                self.zobrist_hash ^= zobrist_keys.queens[pc][pos];
            },
            PieceType::King => {
                self.kings[pc] = self.kings[pc] & !(1 << pos);
                self.zobrist_hash ^= zobrist_keys.kings[pc][pos];
            },
        };
    }

    fn add_piece(&mut self, pos: usize, pt: &PieceType, pc: PieceColor, zobrist_keys: &ZobristKeys) {
        match pt {
            PieceType::Pawn => {
                self.pawns[pc] = self.pawns[pc] | (1 << pos);
                self.zobrist_hash ^= zobrist_keys.pawns[pc][pos];
            },
            PieceType::Bishop => {
                self.bishops[pc] = self.bishops[pc] | (1 << pos);
                self.zobrist_hash ^= zobrist_keys.bishops[pc][pos];
            },
            PieceType::Knight => {
                self.knights[pc] = self.knights[pc] | (1 << pos);
                self.zobrist_hash ^= zobrist_keys.knights[pc][pos];
            },
            PieceType::Rook => {
                self.rooks[pc] = self.rooks[pc] | (1 << pos);
                self.zobrist_hash ^= zobrist_keys.rooks[pc][pos];
            },
            PieceType::Queen => {
                self.queens[pc] = self.queens[pc] | (1 << pos);
                self.zobrist_hash ^= zobrist_keys.queens[pc][pos];
            },
            PieceType::King => {
                self.kings[pc] = self.kings[pc] | (1 << pos);
                self.zobrist_hash ^= zobrist_keys.kings[pc][pos];
            },
        };
    }

    pub fn recalc_board(&mut self) {
        let white_pieces = self.pawns[PieceColor::White]
            | self.bishops[PieceColor::White]
            | self.knights[PieceColor::White]
            | self.rooks[PieceColor::White]
            | self.queens[PieceColor::White]
            | self.kings[PieceColor::White];
        let black_pieces = self.pawns[PieceColor::Black]
            | self.bishops[PieceColor::Black]
            | self.knights[PieceColor::Black]
            | self.rooks[PieceColor::Black]
            | self.queens[PieceColor::Black]
            | self.kings[PieceColor::Black];

        self.occupied = white_pieces | black_pieces;
        self.pieces = [white_pieces, black_pieces];
    }

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

    pub fn promote(&self, from: usize, to: usize, zobrist_keys: &ZobristKeys) -> Vec<Move> {
        let mut new_board = self.clone();

        new_board.zobrist_hash ^= zobrist_keys.white_castling_rights[((new_board.castling & 0b1100) >> 2) as usize];
        new_board.zobrist_hash ^= zobrist_keys.black_castling_rights[(new_board.castling & 0b0011) as usize];  //Black

        let (pc, _) = new_board.lookup(from).unwrap();
        //let (old_pc, old_pt) = new_board.lookup(to);

        let capture = match self.lookup(to) {
            Some((old_pc, old_pt)) => {
                new_board.remove_castling(to, old_pt);
                new_board.remove_piece(to, &old_pt, old_pc, zobrist_keys);
                //Remove Opps piece from to pos
                true
            },
            None =>  false
        };

        new_board.zobrist_hash ^= zobrist_keys.white_castling_rights[((new_board.castling & 0b1100) >> 2) as usize];
        new_board.zobrist_hash ^= zobrist_keys.black_castling_rights[(new_board.castling & 0b0011) as usize];  //Black

        //new_board.remove_castling(to, old_pt);

        new_board.next_turn(zobrist_keys);

        //new_board.remove_piece(to, &old_pt, old_pc);
        new_board.remove_piece(from, &PieceType::Pawn, pc, zobrist_keys);

        if new_board.en_passant != 65 {
            new_board.zobrist_hash ^= zobrist_keys.en_passant[self.en_passant as usize];
            new_board.en_passant = 65;
        }
        //new_board.zobrist_hash ^= zobrist_keys.en_passant[self.en_passant as usize];

        let mut new_boards = vec![new_board.clone(); 4];

        new_boards[0].knights[pc] = new_board.knights[pc] | (1 << to);
        new_boards[0].zobrist_hash ^= zobrist_keys.knights[pc][to];

        new_boards[1].bishops[pc] = new_board.bishops[pc] | (1 << to);
        new_boards[1].zobrist_hash ^= zobrist_keys.bishops[pc][to];

        new_boards[2].rooks[pc] = new_board.rooks[pc] | (1 << to);
        new_boards[2].zobrist_hash ^= zobrist_keys.rooks[pc][to];

        new_boards[3].queens[pc] = new_board.queens[pc] | (1 << to);
        new_boards[3].zobrist_hash ^= zobrist_keys.queens[pc][to];

        //Check for check
        let mut out: Vec<Move> = vec![];
        //In case other piece is removed all case need to be run
        for i in 0..4 {
            new_boards[i].recalc_board();

            //new_boards[i].en_passant = 65;

        }

        out.push(Move{from, to, board: new_boards[0], promote_to: Some(PieceType::Knight), capture});
        out.push(Move{from, to, board: new_boards[1], promote_to: Some(PieceType::Bishop), capture});
        out.push(Move{from, to, board: new_boards[2], promote_to: Some(PieceType::Rook), capture});
        out.push(Move{from, to, board: new_boards[3], promote_to: Some(PieceType::Queen), capture});

        out
    }

    fn promote_pawn_to(&mut self, from: usize, to: usize, pt: PieceType, zobrist_keys: &ZobristKeys) -> Move {
        let mut new_board = self.clone();

        let (pc, _) = new_board.lookup(from).unwrap();
        //let (old_pc, old_pt) = new_board.lookup(to);

        let capture = match self.lookup(to) {
            Some((old_pc, old_pt)) => {
                new_board.remove_castling(to, old_pt);
                new_board.remove_piece(to, &old_pt, old_pc, zobrist_keys);
                //Remove Opps piece from to pos
                true
            },
            None =>  false
        };


        new_board.next_turn(zobrist_keys);

        new_board.remove_piece(from, &PieceType::Pawn, pc, zobrist_keys);

        new_board.add_piece(to, &pt, pc, zobrist_keys);

        new_board.recalc_board();
        new_board.en_passant = 65;

        //new_board
        Move{from, to, board: new_board, promote_to: Some(pt), capture}
    }

    pub fn castle(&self, code: u8, zobrist_keys: &ZobristKeys) -> Move {
        let mut new_board = self.clone();

        let king_from_pos: usize;
        let rook_from_pos: usize;

        let king_to_pos: usize;
        let rook_to_pos: usize;

        new_board.zobrist_hash ^= zobrist_keys.white_castling_rights[((new_board.castling & 0b1100) >> 2) as usize];
        new_board.zobrist_hash ^= zobrist_keys.black_castling_rights[(new_board.castling & 0b0011) as usize];  //Black

        match new_board.turn {
            PieceColor::White => {
                king_from_pos = 4;
                if code == 88 {
                    rook_from_pos = 0;

                    rook_to_pos = 3;
                    king_to_pos = 2;
                } else {
                    rook_from_pos = 7;

                    rook_to_pos = 5;
                    king_to_pos = 6;
                }

                new_board.castling &= 0b0011;
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

                new_board.castling &= 0b1100;
            }
        }

        if new_board.en_passant != 65 {
            new_board.zobrist_hash ^= zobrist_keys.en_passant[self.en_passant as usize];
            new_board.en_passant = 65;
        }

        new_board.remove_piece(king_from_pos, &PieceType::King, new_board.turn, zobrist_keys);
        new_board.remove_piece(rook_from_pos, &PieceType::Rook, new_board.turn, zobrist_keys);

        new_board.add_piece(king_to_pos, &PieceType::King, new_board.turn, zobrist_keys);
        new_board.add_piece(rook_to_pos, &PieceType::Rook, new_board.turn, zobrist_keys);

        new_board.recalc_board();

        new_board.next_turn(zobrist_keys);

        new_board.zobrist_hash ^= zobrist_keys.white_castling_rights[((new_board.castling & 0b1100) >> 2) as usize];
        new_board.zobrist_hash ^= zobrist_keys.black_castling_rights[(new_board.castling & 0b0011) as usize];  //Black

        //new_board
        Move{from: code as usize, to: code as usize, board: new_board, promote_to: None, capture: false}
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
        //let (Move{from, to, board: new_board, promote_to: promo_type, _}) = cur_move;

        if cur_move.to == 80 {
            return match !cur_move.board.turn {
                PieceColor::White => String::from("e1g1"),
                PieceColor::Black => String::from("e8g8"),
            };
        } else if cur_move.to == 88 {
            return match !cur_move.board.turn {
                PieceColor::White => String::from("e1c1"),
                PieceColor::Black => String::from("e8c8"),
            };
        }

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

    pub fn make_move(&mut self, str: &str, engine: &Engine) -> Board {
        //let mut new_board: Board;


        /*
        if str == "O-O" {
            new_board = self.castle(80);
        } else if str == "O-O-O" {
            new_board = self.castle(88);
        } else {
            let from = Board::lan_to_pos(&str[0..2]);
            let to = Board::lan_to_pos(&str[2..4]);
            new_board = self.move_piece(to, from);
        }
        */
        let mut new_move: Move;

        if str.len() == 5 {
            let from = Board::lan_to_pos(&str[0..2]);
            let to = Board::lan_to_pos(&str[2..4]);
            new_move = match str.chars().nth(4).unwrap() {
                'n' | 'N' => self.promote_pawn_to(from, to, PieceType::Knight, &engine.zobrist_keys),
                'b' | 'B' => self.promote_pawn_to(from, to, PieceType::Bishop, &engine.zobrist_keys),
                'r' | 'R' => self.promote_pawn_to(from, to, PieceType::Rook, &engine.zobrist_keys),
                'q' | 'Q' => self.promote_pawn_to(from, to, PieceType::Queen, &engine.zobrist_keys),
                _ => { Move {from: 100, to: 100, board: self.clone(), promote_to: None, capture: false} }
            }
        } else {
            let from = Board::lan_to_pos(&str[0..2]);
            let to = Board::lan_to_pos(&str[2..4]);

            if str == "e1g1" && self.kings[PieceColor::White] & 0x10 != 0 {
                new_move = self.castle(80, &engine.zobrist_keys);
            } else if  str == "e1c1" && self.kings[PieceColor::White] & 0x10 != 0 {
                new_move = self.castle(88, &engine.zobrist_keys);
            } else if str == "e8g8" && self.kings[PieceColor::Black] & 0x1000000000000000 != 0 {
                new_move = self.castle(80, &engine.zobrist_keys);
            } else if  str == "e8c8" && self.kings[PieceColor::Black] & 0x1000000000000000 != 0 {
                new_move = self.castle(88, &engine.zobrist_keys);
            } else {
                new_move = self.move_piece(to, from, &engine.zobrist_keys);
            }

            //"e1c1" | "e8c8" //=> self.castle(88, &engine.zobrist_keys),


        }

        let king_pos = board_serialize(new_move.board.kings[!self.turn]);
        if king_pos.len() > 0 {
            let (cr, cf) = engine.cal_check(&new_move.board, king_pos[0], self.turn);
            new_move.board.check_real = cr;
            new_move.board.check_full = cf;
        } else {
            new_move.board.check_real = 0;
            new_move.board.check_full = 0;
        }

        return new_move.board;
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

                if ((self.pawns[PieceColor::White] >> i) & 1) == 1 {
                    print!("{} ", set[n][0]);
                } else if ((self.bishops[PieceColor::White] >> i) & 1) == 1 {
                    print!("{} ", set[n][2]);
                } else if ((self.knights[PieceColor::White] >> i) & 1) == 1 {
                    print!("{} ", set[n][1]);
                } else if ((self.rooks[PieceColor::White] >> i) & 1) == 1 {
                    print!("{} ", set[n][3]);
                } else if ((self.queens[PieceColor::White] >> i) & 1) == 1 {
                    print!("{} ", set[n][4]);
                } else if ((self.kings[PieceColor::White] >> i) & 1) == 1 {
                    print!("{} ", set[n][5]);
                } else if ((self.pawns[PieceColor::Black] >> i) & 1) == 1 {
                    print!("{} ", set[n][6]);
                } else if ((self.bishops[PieceColor::Black] >> i) & 1) == 1 {
                    print!("{} ", set[n][8]);
                } else if ((self.knights[PieceColor::Black] >> i) & 1) == 1 {
                    print!("{} ", set[n][7]);
                } else if ((self.rooks[PieceColor::Black] >> i) & 1) == 1 {
                    print!("{} ", set[n][9]);
                } else if ((self.queens[PieceColor::Black] >> i) & 1) == 1 {
                    print!("{} ", set[n][10]);
                } else if ((self.kings[PieceColor::Black] >> i) & 1) == 1 {
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
