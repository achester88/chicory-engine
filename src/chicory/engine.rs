use crate::chicory::bitboard::postshift;
use crate::chicory::board::Move;
use super::bitboard::{Bitboard};
use super::board::*;
use super::utils::*;
use super::tables::ZobristKeys;

//          (from,  to,    new board)
//pub type Move = (usize, usize, Board, Option<PieceType>);

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    North,
    South,
    West,
    East,
    NOEA,
    NOWE,
    SOWE,
    SOEA,
}

pub struct Engine {
    ray_attacks: Vec<Vec<Bitboard>>,
    pawn_attacks: Vec<Vec<Bitboard>>,
    king_attacks: Vec<Bitboard>,
    knight_attacks: Vec<Bitboard>,
    king_attackables: Vec<Bitboard>,

    pub zobrist_keys: ZobristKeys,
}

impl Engine {
    pub fn new() -> Self {
        let mut new_engine = Engine {
            ray_attacks: gen_ray_attacks(),
            pawn_attacks: gen_pawn_attacks(),
            king_attacks: gen_king_attacks(),
            knight_attacks: gen_knight_attacks(),
            king_attackables: vec![],

            zobrist_keys: ZobristKeys::new(),
        };

        new_engine.king_attackables = new_engine.gen_king_attackables();

        return new_engine;
    }
    //from, to, new board
    pub fn gen_moves(&self, board: &Board) -> Vec<Move> {
        let mut all_moves: Vec<Move> = Vec::with_capacity(64);
        let mut possible: Vec<(usize, Bitboard)> = vec![];
        let mut possible_king: Vec<(usize, Bitboard)> = vec![];

        //######### BASIC MOVE GEN #########

        for i in board.queens[board.turn] {
            possible.push(self.gen_queen_moves(&board, i, board.pieces[board.turn]));
        }

        for i in board.bishops[board.turn] {
            possible.push(self.gen_bishop_moves(&board, i, board.pieces[board.turn]));
        }

        for i in board.rooks[board.turn] {
            possible.push(self.gen_rook_moves(&board, i, board.pieces[board.turn]));
        }

        for i in board.pawns[board.turn] {
            let (from, moves) = self.gen_pawn_moves(&board, i, board.turn);

            if (moves & Bitboard::new(0xff00000000000000) ) != Bitboard::new(0) || (moves & Bitboard::new(0xff)) != Bitboard::new(0) {
                for to in moves {
                    //all_moves.append(&mut (board.promote(from, to, &self.zobrist_keys))); //64 out of range, no piece
                    all_moves.push(Move {from: from, to: to, promote_to: Some(PieceType::Knight), move_type: MoveType::Promotion});
                    all_moves.push(Move {from: from, to: to, promote_to: Some(PieceType::Bishop), move_type: MoveType::Promotion});
                    all_moves.push(Move {from: from, to: to, promote_to: Some(PieceType::Rook), move_type: MoveType::Promotion});
                    all_moves.push(Move {from: from, to: to, promote_to: Some(PieceType::Queen), move_type: MoveType::Promotion});
                }
            } else {
                possible.push((from, moves));
            }
        }

        if !board.check_real.is_empty() {
            for i in board.kings[board.turn] {
                let (from, moves) = self.gen_king_moves(&board, i, board.turn);

                for to in moves {

                    if (Bitboard::new(1 << to) & board.check_full).is_empty() {
                        //let mut new_move = board.move_piece(to, from, &self.zobrist_keys);
                        all_moves.push(Move {from: from, to: to, promote_to: None, move_type: MoveType::Move});
                        //new_move.board.check_real = 0;
                        //new_move.board.check_full = 0;
                        //all_moves.push(new_move);
                    }
                }
            }
        } else {
            for i in board.kings[board.turn] {
                //print_bitboard_pos(board.check_full, i);
                possible_king.push(self.gen_king_moves(&board, i, board.turn));
            }
        }

        for i in board.knights[board.turn] {
            possible.push(self.gen_knight_moves(&board, i, board.turn));
        }

        //######### Castle Logic #########
        let not_check = board.check_real == Bitboard::new(0);
        let can_castle = (board.castling != 0) && not_check;

        if can_castle {
            match board.turn {
                PieceColor::White => {
                    if board.castling & 0b1000 == 0b1000
                        && (board.occupied & Bitboard::new(0xe)).is_empty()
                        && self.can_castle_through(Bitboard::new(0xc), &board)
                    {
                        //queenside
                        //all_moves.push(board.castle(88, &self.zobrist_keys));
                        all_moves.push( Move { from: 4, to: 2, promote_to: None, move_type: MoveType::CastleQueenSide } );
                    }
                    if board.castling & 0b0100 == 0b0100
                        && (board.occupied & Bitboard::new(0x60)).is_empty()
                        && self.can_castle_through(Bitboard::new(0x60), &board)
                    {
                        //all_moves.push(board.castle(80, &self.zobrist_keys));
                        all_moves.push( Move { from: 4, to: 6, promote_to: None, move_type: MoveType::CastleKingSide } );
                    }
                }
                PieceColor::Black => {
                    if board.castling & 0b0010 == 0b0010
                        && (board.occupied & Bitboard::new(0xe00000000000000)).is_empty()
                        && self.can_castle_through(Bitboard::new(0xc00000000000000), &board)
                    {
                        //queenside
                        //all_moves.push(board.castle(88, &self.zobrist_keys));
                        all_moves.push( Move { from: 60, to: 58, promote_to: None, move_type: MoveType::CastleQueenSide } );
                    }
                    if board.castling & 0b0001 == 0b0001
                        && (board.occupied & Bitboard::new(0x6000000000000000)).is_empty()
                        && self.can_castle_through(Bitboard::new(0x6000000000000000), &board)
                    {
                        //kingside
                        //all_moves.push(board.castle(80, &self.zobrist_keys));
                        all_moves.push( Move { from: 60, to: 62, promote_to: None, move_type: MoveType::CastleKingSide } );
                    }
                }
            }
        }

        //######### New Board Gen Loop #########


        for i in 0..possible.len() {
            let (from, moves) = possible[i];

            for to in moves {
                //let to = moves_to[ii];

                if not_check || !(board.check_real & Bitboard::new(1 << to)).is_empty() {
                    //Not in check or to is in (check)
                    //all_moves.push(board.move_piece(to, from, &self.zobrist_keys));
                    all_moves.push(Move {from: from, to: to, promote_to: None, move_type: MoveType::Move } );
                }
            }
        }

        for i in 0..possible_king.len() {
            let (from, moves) = possible_king[i];

            for to in moves {
                //let to = moves_to[ii];
                //println!("S--------[{}]------------", to);

                //if not_check || board.check_real & (1 << to) != 0 {
                    //Not in check or to is in (check)
                    //all_moves.push(board.move_piece(to, from, &self.zobrist_keys));
                let (cr, _) = self.cal_check(&board, to, !board.turn, true);
                //println!("cr: {}, cf: {}, to: {}", cr, cf, to);
                if cr.is_empty() {
                    all_moves.push(Move { from: from, to: to, promote_to: None, move_type: MoveType::Move });
                }
                //}
                //println!("E-------------------------------------------");
            }
        }

        all_moves
    }


    pub fn can_castle_through(&self, pos: Bitboard, board: &Board) -> bool {
        let opp_color = !board.turn;

        for i in pos {
            let attackable_check_pos = self.king_attackables[i];

            let hits = attackable_check_pos & board.pieces[!board.turn];

            for to in hits {
                if !(Bitboard::new(1 << to) & attackable_check_pos).is_empty() {
                    match board.lookup(to) {
                        Some((_, pt)) => {
                            let (_, att) = match pt {
                                PieceType::Pawn => (0, self.pawn_attacks[opp_color as usize][to]), //self.gen_pawn_moves(&board, to, opp_color), //en_pass??
                                PieceType::Knight => self.gen_knight_moves(&board, to, opp_color),
                                PieceType::Bishop => {
                                    self.gen_bishop_moves(&board, to, board.pieces[opp_color])
                                }
                                PieceType::Rook => self.gen_rook_moves(&board, to, board.pieces[opp_color]),
                                PieceType::Queen => {
                                    self.gen_queen_moves(&board, to, board.pieces[opp_color])
                                }
                                PieceType::King => self.gen_king_moves(&board, to, opp_color),
                            };
                            if !(att & Bitboard::new(1 << i)).is_empty() {
                                return false;
                            }
                        },
                        None => {}
                    }

                }
            }
        }

        true
    }

    pub fn gen_knight_moves(&self, board: &Board, sq: usize, turn: PieceColor) -> (usize, Bitboard) {
        let opp = match turn {
            PieceColor::White => board.pieces[PieceColor::Black],
            PieceColor::Black => board.pieces[PieceColor::White],
        };
        let attacks = self.knight_attacks[sq] & (!board.occupied | opp);

        (sq, attacks)
    }

    pub fn gen_king_moves(&self, board: &Board, sq: usize, turn: PieceColor) -> (usize, Bitboard) {
        let opp = match turn {
            PieceColor::White => board.pieces[PieceColor::Black],
            PieceColor::Black => board.pieces[PieceColor::White],
        };

        let attacks = self.king_attacks[sq] & (!board.occupied | opp);

        (sq, attacks)
    }

    pub fn gen_pawn_moves(&self, board: &Board, sq: usize, turn: PieceColor) -> (usize, Bitboard) {
        self.gen_pawn_moves_raw(board, sq, turn, board.pieces[!turn])

    }
    pub fn gen_pawn_moves_raw(&self, board: &Board, sq: usize, turn: PieceColor, opp_pieces: Bitboard) -> (usize, Bitboard) {
        let mut moves = Bitboard::new(0);
        let piece = Bitboard::new(1 << sq);

        if turn == PieceColor::White {
            moves = moves | ( postshift::nort_one(piece) & !board.occupied);
            if !(piece & Bitboard::new(0xff00)).is_empty() {
                moves = moves | (postshift::nort_one(moves) & !board.occupied);
            }

            let mut en_pass: Bitboard = Bitboard::new(0);

            if board.en_passant != 65 {
                en_pass = Bitboard::new(1) << board.en_passant;
            }

            moves = moves
                | (self.pawn_attacks[PieceColor::White as usize][sq]
                    & (opp_pieces | en_pass));
        } else {
            //Black
            moves = moves | (postshift::sout_one(piece) & !board.occupied);
            if !(piece & Bitboard::new(0xff000000000000)).is_empty() {
                moves = moves | (postshift::sout_one(moves) & !board.occupied);
            }

            let mut en_pass: Bitboard = Bitboard::new(0);

            if board.en_passant != 65 {
                en_pass = Bitboard::new(1 << board.en_passant);
            }

            moves = moves
                | (self.pawn_attacks[PieceColor::Black as usize][sq]
                    & (opp_pieces | en_pass));
        }
        
        //
        (sq, moves)
    }

    pub fn gen_rook_moves(&self, board: &Board, sq: usize, pieces: Bitboard) -> (usize, Bitboard) {
        let all_moves = self.gen_ray_attacks(board.occupied, Dir::North, sq)
            | self.gen_ray_attacks(board.occupied, Dir::South, sq)
            | self.gen_ray_attacks(board.occupied, Dir::East, sq)
            | self.gen_ray_attacks(board.occupied, Dir::West, sq);

        let attack = all_moves & !pieces;

        (sq, attack)
    }

    pub fn gen_bishop_moves(&self, board: &Board, sq: usize, pieces: Bitboard) -> (usize, Bitboard) {
        let all_moves = self.gen_ray_attacks(board.occupied, Dir::NOEA, sq)
            | self.gen_ray_attacks(board.occupied, Dir::NOWE, sq)
            | self.gen_ray_attacks(board.occupied, Dir::SOEA, sq)
            | self.gen_ray_attacks(board.occupied, Dir::SOWE, sq);

        let attack = all_moves & !pieces;

        (sq, attack)
    }

    pub fn gen_queen_moves(&self, board: &Board, sq: usize, pieces: Bitboard) -> (usize, Bitboard) {
        let attack =
            self.gen_rook_moves(board, sq, pieces).1 | self.gen_bishop_moves(board, sq, pieces).1;

        (sq, attack)
    }

    pub fn cal_check(&self, board: &Board, pos: usize, opp_color: PieceColor, inculde_pawn_diag: bool) -> (Bitboard, Bitboard) {
        //println!("S--------------------");
        //board.print_board();
        let mut check_real: Bitboard = Bitboard::new(0);
        let mut check_full: Bitboard = Bitboard::new(0);
        let attackable_check_pos = self.king_attackables[pos];

        let hits = attackable_check_pos & board.pieces[opp_color];

        for to in hits {
            if !(Bitboard::new(1 << to) & attackable_check_pos).is_empty() {
                //let (_, pt) = board.lookup(to);
                match board.lookup(to) {
                    Some ((_, pt)) => {
                        let (_, att) = match pt {
                            PieceType::Pawn => {
                                if inculde_pawn_diag {
                                    //(0, self.gen_pawn_moves_raw(&board, to, opp_color, 0xffffffffffffffff).1 & !(1 << to)) //TODO replace with get pawn attacks
                                    // print_bitboard_pos(self.pawn_attacks[opp_color as usize][to], pos);
                                    (0, self.pawn_attacks[opp_color as usize][to])
                                } else {
                                    self.gen_pawn_moves(&board, to, opp_color)
                                }
                            }, //en_pass??
                            PieceType::Knight => self.gen_knight_moves(&board, to, opp_color),
                            PieceType::Bishop => self.gen_bishop_moves(&board, to, board.pieces[opp_color]),
                            PieceType::Rook => self.gen_rook_moves(&board, to, board.pieces[opp_color]),
                            PieceType::Queen => self.gen_queen_moves(&board, to, board.pieces[opp_color]),
                            PieceType::King => self.gen_king_moves(&board, to, opp_color),
                        };
                        if !(att & Bitboard::new(1 << pos)).is_empty() {
                            // println!("HITTTTTT at {}", pos);
                            let (cr, cf) = self.gen_check_info(&board, to, pos);
                            //TODO SET R AND F TO ONLY SAME RANK/FILE OF KING
                            check_real = check_real | cr;
                            check_full = check_full | cf;
                        }
                    },
                    None => {}
                }

            }
        }
        //println!("Check real: 0x{:x}", check_real);
        //println!("Check Full: 0x{:x}", check_full);
        //println!("E--------------------");
        (check_real, check_full)
    }

    fn gen_king_attacks() -> Vec<Bitboard> {
        let mut attacks: Vec<Bitboard> = vec![Bitboard::new(0); 64];

        for i in 0..64 {
            let mut pos = Bitboard::new(1 << i);

            pos |= postshift::east_one(pos) | postshift::west_one(pos);
            pos |= postshift::nort_one(pos) | postshift::sout_one(pos);

            attacks[i] = pos;
        }

        attacks
    }

    pub fn gen_king_attackables(&self) -> Vec<Bitboard> {
        let mut attackables: Vec<Bitboard> = vec![Bitboard::new(0); 64];

        for pos in 0..64 {
            let board = self.ray_attacks[Dir::North as usize][pos]
                | self.ray_attacks[Dir::NOEA as usize][pos]
                | self.ray_attacks[Dir::East as usize][pos]
                | self.ray_attacks[Dir::SOEA as usize][pos]
                | self.ray_attacks[Dir::South as usize][pos]
                | self.ray_attacks[Dir::SOWE as usize][pos]
                | self.ray_attacks[Dir::West as usize][pos]
                | self.ray_attacks[Dir::NOWE as usize][pos]
                | self.knight_attacks[pos]
                | Bitboard::new(1 << pos);

            attackables[pos] = board;
        }

        attackables
    }

    pub fn gen_check_info(&self, board: &Board, pos: usize, king_pos: usize) -> (Bitboard, Bitboard) {
        let mut kingless = board.clone();
        kingless.occupied &= !board.kings[board.turn];
        kingless.kings[board.turn] = Bitboard::new(0);
        kingless.pieces[board.turn] &= !board.kings[board.turn];
        //kingless.recalc_board();

        let mut check_real: Bitboard = Bitboard::new(0); //any piece other than the king need to occupied
        let mut check_full: Bitboard = Bitboard::new(0); //king can not be on

        match board.lookup(pos) {
            Some ((_, pt)) => {
                match pt {
                    PieceType::Pawn => {
                        //TODO ACCOUNT FOR ALL!!! PIECES IN THIS MATCH
                        check_real = Bitboard::new(0); //self pos added on return //1 << pos;//self.gen_pawn_moves(&board, pos, !board.turn);
                        let (_, check_full_pre) = self.gen_pawn_moves(&board, pos, !board.turn);
                        check_full = check_full_pre
                            & !(self.ray_attacks[Dir::North as usize][pos]
                            | self.ray_attacks[Dir::South as usize][pos]);
                    }
                    PieceType::Knight => {
                        (_, check_real) = self.gen_knight_moves(&board, pos, !board.turn);
                        (_, check_full) = self.gen_knight_moves(&kingless, pos, !board.turn);
                    }
                    PieceType::Bishop => {
                        let raf = self.ray_attacks[Dir::NOEA as usize][king_pos]
                            | self.ray_attacks[Dir::NOWE as usize][king_pos]
                            | self.ray_attacks[Dir::SOEA as usize][king_pos]
                            | self.ray_attacks[Dir::SOWE as usize][king_pos];
                        (_, check_real) = self.gen_bishop_moves(&board, pos, board.pieces[!board.turn]);
                        (_, check_full) = self.gen_bishop_moves(&kingless, pos, board.pieces[!board.turn]);
                        check_real = check_real & raf;
                    }
                    PieceType::Rook => {
                        let raf = self.ray_attacks[Dir::North as usize][king_pos]
                            | self.ray_attacks[Dir::South as usize][king_pos]
                            | self.ray_attacks[Dir::East as usize][king_pos]
                            | self.ray_attacks[Dir::West as usize][king_pos];
                        (_, check_real) = self.gen_rook_moves(&board, pos, board.pieces[!board.turn]);
                        (_, check_full) = self.gen_rook_moves(&kingless, pos, board.pieces[!board.turn]);
                        check_real = check_real & raf;
                    }
                    PieceType::Queen => {
                        let raf = self.ray_attacks[Dir::North as usize][king_pos]
                            | self.ray_attacks[Dir::South as usize][king_pos]
                            | self.ray_attacks[Dir::East as usize][king_pos]
                            | self.ray_attacks[Dir::West as usize][king_pos]
                            | self.ray_attacks[Dir::NOEA as usize][king_pos]
                            | self.ray_attacks[Dir::NOWE as usize][king_pos]
                            | self.ray_attacks[Dir::SOEA as usize][king_pos]
                            | self.ray_attacks[Dir::SOWE as usize][king_pos];
                        (_, check_real) = self.gen_queen_moves(&board, pos, board.pieces[!board.turn]);
                        (_, check_full) = self.gen_queen_moves(&kingless, pos, board.pieces[!board.turn]);
                        check_real = check_real & raf;
                    }
                    PieceType::King => {
                        //NOT NEEDED PROBABLE
                        (_, check_real) = self.gen_king_moves(&board, pos, !board.turn);
                        (_, check_full) = self.gen_king_moves(&kingless, pos, !board.turn);
                    }
                }
            },
            None => {}
        }

        (check_real | Bitboard::new(1 << pos), check_full)
    }

    pub fn gen_ray_attacks(&self, occupied: Bitboard, dir: Dir, square: usize) -> Bitboard {
        let set = &self.ray_attacks[dir as usize];
        let mut attack: Bitboard = set[square];
        let block: Bitboard = attack & occupied;
        if block != Bitboard::new(0) {
            let stop: usize;
            if dir_is_pos(dir) {
                stop = bit_scan(block);
            } else {
                stop = bit_scan_neg(block);
            }
            attack = attack ^ set[stop];
        }
        attack
    }
}

fn dir_is_pos(dir: Dir) -> bool {
    match dir {
        Dir::North | Dir::NOWE | Dir::NOEA | Dir::East => true,
        Dir::South | Dir::West | Dir::SOWE | Dir::SOEA => false,
    }
}

fn gen_ray_attacks() -> Vec<Vec<Bitboard>> {
    let mut attacks: Vec<Vec<Bitboard>> = vec![vec![Bitboard::new(0); 64]; 8];

    //North
    let mut nort: Bitboard = Bitboard::new(0x0101010101010100);
    for i in 0..64 {
        attacks[Dir::North as usize][i as usize] = nort;
        nort = nort << 1;
    }
    //South
    let mut south: Bitboard = Bitboard::new(0x80808080808080);
    for i in (0..64).rev() {
        attacks[Dir::South as usize][i as usize] = south;
        south = south >> 1;
    }

    //East
    let mut east: Bitboard;
    for i in 0..8 {
        east = Bitboard::new(0xfe << (8 * i));
        for ii in 0..8 {
            attacks[Dir::East as usize][((i * 8) + ii) as usize] = east;
            east = postshift::east_one(east);
        }
    }

    //West
    let mut west: Bitboard;
    for i in 0..8 {
        west = Bitboard::new(0x7f << (8 * i));
        for ii in 0..8 {
            attacks[Dir::West as usize][((i * 8) + (7 - ii)) as usize] = west;
            west = postshift::west_one(west);
        }
    }

    //noea
    let mut noea;
    for i in 0..8 {
        noea = Bitboard::new(0x8040201008040200 << (8 * i));
        for ii in 0..8 {
            attacks[Dir::NOEA as usize][((i * 8) + ii) as usize] = noea;

            noea = postshift::east_one(noea);
        }
    }

    //nowe
    let mut nowe: Bitboard;
    for i in 0..8 {
        nowe = Bitboard::new(0x102040810204000 << (8 * i));
        for ii in 0..8 {
            attacks[Dir::NOWE as usize][((i * 8) + (7 - ii)) as usize] = nowe;

            nowe = postshift::west_one(nowe);
        }
    }

    //soea
    let mut soea: Bitboard;
    for i in (0..8).rev() {
        soea = Bitboard::new(0x2040810204080 >> (8 * (7 - i)));
        for ii in 0..8 {
            attacks[Dir::SOEA as usize][((i * 8) + ii) as usize] = soea;

            soea = postshift::east_one(soea);
        }
    }

    //sowe
    let mut sowe: Bitboard;
    for i in (0..8).rev() {
        sowe = Bitboard::new(0x40201008040201 >> (8 * (7 - i)));
        for ii in 0..8 {
            attacks[Dir::SOWE as usize][((i * 8) + (7 - ii)) as usize] = sowe;

            sowe = postshift::west_one(sowe);
        }
    }

    attacks
}

fn gen_pawn_attacks() -> Vec<Vec<Bitboard>> {
    let mut attacks: Vec<Vec<Bitboard>> = vec![vec![Bitboard::new(0); 64]; 2];

    for i in 8..56 {
        let bb = Bitboard::new(1 << i);

        attacks[PieceColor::White as usize][i] =
            postshift::no_ea_one(bb) | postshift::no_we_one(bb);
    }

    for i in 8..56 {
        let bb = Bitboard::new(1 << i);

        attacks[PieceColor::Black as usize][i] =
            postshift::so_ea_one(bb) | postshift::so_we_one(bb);
    }

    attacks
}

fn gen_king_attacks() -> Vec<Bitboard> {
    let mut attacks: Vec<Bitboard> = vec![Bitboard::new(0); 64];

    for i in 0..64 {
        let mut pos = Bitboard::new(1 << i);

        pos |= postshift::east_one(pos) | postshift::west_one(pos);
        pos |= postshift::nort_one(pos) | postshift::sout_one(pos);

        attacks[i] = pos;
    }

    attacks
}

fn gen_knight_attacks() -> Vec<Bitboard> {
    let mut attacks: Vec<Bitboard> = vec![Bitboard::new(0); 64];

    for i in 0..64 {
        let pos: Bitboard = Bitboard::new(1 << i);
        let mut attack = Bitboard::new(0);
        attack |= postshift::no_ea_one(postshift::nort_one(pos));
        attack |= postshift::no_we_one(postshift::nort_one(pos));
        attack |= postshift::so_ea_one(postshift::sout_one(pos));
        attack |= postshift::so_we_one(postshift::sout_one(pos));

        attack |= postshift::no_ea_one(postshift::east_one(pos));
        attack |= postshift::no_we_one(postshift::west_one(pos));
        attack |= postshift::so_ea_one(postshift::east_one(pos));
        attack |= postshift::so_we_one(postshift::west_one(pos));

        attacks[i] = attack;
    }

    attacks
}
