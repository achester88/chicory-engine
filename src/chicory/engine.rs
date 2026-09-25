use super::bitboard::*;
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub board: Board,
    pub promote_to: Option<PieceType>,
    pub capture: bool
}

pub struct Engine {
    ray_attacks: Vec<Vec<u64>>,
    pawn_attacks: Vec<Vec<u64>>,
    king_attacks: Vec<u64>,
    knight_attacks: Vec<u64>,

    pub zobrist_keys: ZobristKeys,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            ray_attacks: gen_ray_attacks(),
            pawn_attacks: gen_pawn_attacks(),
            king_attacks: gen_king_attacks(),
            knight_attacks: gen_knight_attacks(),

            zobrist_keys: ZobristKeys::new(),
        }
    }
    //from, to, new board
    pub fn gen_moves(&self, board: Board) -> Vec<Move> {
        let mut all_moves: Vec<Move> = vec![];
        let mut possible: Vec<(usize, u64)> = vec![];

        //######### BASIC MOVE GEN #########

        let queens = board_serialize(board.queens[board.turn]);
        for i in queens {
            possible.push(self.gen_queen_moves(&board, i, board.pieces[board.turn]));
        }

        let bishops = board_serialize(board.bishops[board.turn]);
        for i in bishops {
            possible.push(self.gen_bishop_moves(&board, i, board.pieces[board.turn]));
        }

        let rooks = board_serialize(board.rooks[board.turn]);
        for i in rooks {
            possible.push(self.gen_rook_moves(&board, i, board.pieces[board.turn]));
        }

        let pawns = board_serialize(board.pawns[board.turn]);
        for i in pawns {
            let (from, moves) = self.gen_pawn_moves(&board, i, board.turn);

            if moves & 0xff00000000000000 != 0 || moves & 0xff != 0 {
                let prmo_moves = board_serialize(moves);
                for to in prmo_moves {
                    all_moves.append(&mut (board.promote(from, to, &self.zobrist_keys))); //64 out of range, no piece
                }
            } else {
                possible.push((from, moves));
            }
        }

        let kings = board_serialize(board.kings[board.turn]);
        if board.check_real != 0 {
            for i in kings {
                let (from, moves) = self.gen_king_moves(&board, i, board.turn);
                let moves_to = board_serialize(moves);

                for i in 0..moves_to.len() {
                    let to = moves_to[i];

                    if (1 << to) & board.check_full == 0 {
                        let mut new_move = board.move_piece(to, from, &self.zobrist_keys);
                        new_move.board.check_real = 0;
                        new_move.board.check_full = 0;
                        all_moves.push(new_move);
                    }
                }
            }
        } else {
            for i in kings {
                possible.push(self.gen_king_moves(&board, i, board.turn));
            }
        }

        let knight = board_serialize(board.knights[board.turn]);
        for i in knight {
            possible.push(self.gen_knight_moves(&board, i, board.turn));
        }

        //######### Castle Logic #########
        let not_check = board.check_real == 0;
        let can_castle = (board.castling != 0) && not_check;

        if can_castle {
            match board.turn {
                PieceColor::White => {
                    if board.castling & 0b1000 == 0b1000
                        && (board.occupied & 0xe == 0)
                        && self.can_castle_through(0xc, &board)
                    {
                        //queenside
                        all_moves.push(board.castle(88, &self.zobrist_keys));
                    }
                    if board.castling & 0b0100 == 0b0100
                        && (board.occupied & 0x60 == 0)
                        && self.can_castle_through(0x60, &board)
                    {
                        all_moves.push(board.castle(80, &self.zobrist_keys));
                    }
                }
                PieceColor::Black => {
                    if board.castling & 0b0010 == 0b0010
                        && (board.occupied & 0xe00000000000000 == 0)
                        && self.can_castle_through(0xc00000000000000, &board)
                    {
                        //queenside
                        all_moves.push(board.castle(88, &self.zobrist_keys));
                    }
                    if board.castling & 0b0001 == 0b0001
                        && (board.occupied & 0x6000000000000000 == 0)
                        && self.can_castle_through(0x6000000000000000, &board)
                    {
                        //kingside
                        all_moves.push(board.castle(80, &self.zobrist_keys));
                    }
                }
            }
        }

        let king_pos = board_serialize(board.kings[!board.turn]);

        //######### New Board Gen Loop #########
        for i in 0..possible.len() {
            let (from, moves) = possible[i];
            let moves_to = board_serialize(moves);

            for ii in 0..moves_to.len() {
                let to = moves_to[ii];

                if not_check || board.check_real & (1 << to) != 0 {
                    //Not in check or to is in (check)
                    all_moves.push(board.move_piece(to, from, &self.zobrist_keys));
                }
            }
        }

        //Calcs check
        let mut i = 0;

        while i < all_moves.len() {
            let new_board = &mut all_moves[i].board;

            //enemy check
            if king_pos.len() > 0 {
                let (cr, cf) = self.cal_check(new_board, king_pos[0], board.turn);
                new_board.check_real = cr;
                new_board.check_full = cf;
            } else {
                new_board.check_real = 0;
                new_board.check_full = 0;
            }

            //self check
            let self_king_pos = board_serialize(new_board.kings[board.turn]);
            if self_king_pos.len() > 0 {
                //let self_king_pos = board_serialize(new_board.kings[board.turn]);
                if self.cal_check(&new_board, self_king_pos[0], !board.turn) != (0, 0) {
                    all_moves.remove(i);
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        all_moves
    }

    pub fn can_castle_through(&self, pos: u64, board: &Board) -> bool {
        let sq = board_serialize(pos);
        let opp_color = !board.turn;

        for i in sq {
            let attackable_check_pos = self.gen_king_attackables(i);

            let hits = attackable_check_pos & board.pieces[!board.turn];

            let hits_pos = board_serialize(hits);

            for to in hits_pos {
                if (1 << to) & attackable_check_pos != 0 {
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
                            if (att & (1 << i)) != 0 {
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

    pub fn gen_knight_moves(&self, board: &Board, sq: usize, turn: PieceColor) -> (usize, u64) {
        let opp = match turn {
            PieceColor::White => board.pieces[PieceColor::Black],
            PieceColor::Black => board.pieces[PieceColor::White],
        };
        let attacks = self.knight_attacks[sq] & (!board.occupied | opp);

        (sq, attacks)
    }

    pub fn gen_king_moves(&self, board: &Board, sq: usize, turn: PieceColor) -> (usize, u64) {
        let opp = match turn {
            PieceColor::White => board.pieces[PieceColor::Black],
            PieceColor::Black => board.pieces[PieceColor::White],
        };

        let attacks = self.king_attacks[sq] & (!board.occupied | opp);

        (sq, attacks)
    }

    pub fn gen_pawn_moves(&self, board: &Board, sq: usize, turn: PieceColor) -> (usize, u64) {
        let mut moves = 0;
        let piece = 1 << sq;

        if turn == PieceColor::White {
            moves = moves | (postshift::nort_one(piece) & !board.occupied);
            if (piece & 0xff00) != 0 {
                moves = moves | (postshift::nort_one(moves) & !board.occupied);
            }

            let mut en_pass: u64 = 0;

            if board.en_passant != 65 {
                en_pass = 1 << board.en_passant;
            }

            moves = moves
                | (self.pawn_attacks[PieceColor::White as usize][sq]
                    & (board.pieces[PieceColor::Black as usize] | en_pass));
        } else {
            //Black
            moves = moves | (postshift::sout_one(piece) & !board.occupied);
            if (piece & 0xff000000000000) != 0 {
                moves = moves | (postshift::sout_one(moves) & !board.occupied);
            }

            let mut en_pass: u64 = 0;

            if board.en_passant != 65 {
                en_pass = 1 << board.en_passant;
            }

            moves = moves
                | (self.pawn_attacks[PieceColor::Black as usize][sq]
                    & (board.pieces[PieceColor::White as usize] | en_pass));
        }
        
        //
        (sq, moves)
    }

    pub fn gen_rook_moves(&self, board: &Board, sq: usize, pieces: u64) -> (usize, u64) {
        let all_moves = self.gen_ray_attacks(board.occupied, Dir::North, sq)
            | self.gen_ray_attacks(board.occupied, Dir::South, sq)
            | self.gen_ray_attacks(board.occupied, Dir::East, sq)
            | self.gen_ray_attacks(board.occupied, Dir::West, sq);

        let attack = all_moves & !pieces;

        (sq, attack)
    }

    pub fn gen_bishop_moves(&self, board: &Board, sq: usize, pieces: u64) -> (usize, u64) {
        let all_moves = self.gen_ray_attacks(board.occupied, Dir::NOEA, sq)
            | self.gen_ray_attacks(board.occupied, Dir::NOWE, sq)
            | self.gen_ray_attacks(board.occupied, Dir::SOEA, sq)
            | self.gen_ray_attacks(board.occupied, Dir::SOWE, sq);

        let attack = all_moves & !pieces;

        (sq, attack)
    }

    pub fn gen_queen_moves(&self, board: &Board, sq: usize, pieces: u64) -> (usize, u64) {
        let attack =
            self.gen_rook_moves(board, sq, pieces).1 | self.gen_bishop_moves(board, sq, pieces).1;

        (sq, attack)
    }

    pub fn cal_check(&self, board: &Board, pos: usize, opp_color: PieceColor) -> (u64, u64) {
        let mut check_real = 0;
        let mut check_full = 0;
        let attackable_check_pos = self.gen_king_attackables(pos);

        let hits = attackable_check_pos & board.pieces[opp_color];

        let hits_pos = board_serialize(hits);

        for to in hits_pos {
            if (1 << to) & attackable_check_pos != 0 {
                //let (_, pt) = board.lookup(to);
                match board.lookup(to) {
                    Some ((_, pt)) => {
                        let (_, att) = match pt {
                            PieceType::Pawn => self.gen_pawn_moves(&board, to, opp_color), //en_pass??
                            PieceType::Knight => self.gen_knight_moves(&board, to, opp_color),
                            PieceType::Bishop => self.gen_bishop_moves(&board, to, board.pieces[opp_color]),
                            PieceType::Rook => self.gen_rook_moves(&board, to, board.pieces[opp_color]),
                            PieceType::Queen => self.gen_queen_moves(&board, to, board.pieces[opp_color]),
                            PieceType::King => self.gen_king_moves(&board, to, opp_color),
                        };
                        if (att & (1 << pos)) != 0 {
                            let (cr, cf) = self.gen_check_info(&board, to, pos);
                            //TODO SET R AND F TO ONLY SAME RANK/FILE OF KING
                            check_real |= cr;
                            check_full |= cf;
                        }
                    },
                    None => {}
                }

            }
        }

        (check_real, check_full)
    }

    pub fn gen_king_attackables(&self, pos: usize) -> u64 {
        let board = self.ray_attacks[Dir::North as usize][pos]
            | self.ray_attacks[Dir::NOEA as usize][pos]
            | self.ray_attacks[Dir::East as usize][pos]
            | self.ray_attacks[Dir::SOEA as usize][pos]
            | self.ray_attacks[Dir::South as usize][pos]
            | self.ray_attacks[Dir::SOWE as usize][pos]
            | self.ray_attacks[Dir::West as usize][pos]
            | self.ray_attacks[Dir::NOWE as usize][pos]
            | self.knight_attacks[pos]
            | (1 << pos);

        board
    }

    pub fn gen_check_info(&self, board: &Board, pos: usize, king_pos: usize) -> (u64, u64) {
        let mut kingless = board.clone();
        kingless.kings[board.turn] = 0;
        kingless.recalc_board();

        let mut check_real: u64 = 0; //any piece other than the king need to occupied
        let mut check_full: u64 = 0; //king can not be on

        match board.lookup(pos) {
            Some ((_, pt)) => {
                match pt {
                    PieceType::Pawn => {
                        //TODO ACCOUNT FOR ALL!!! PIECES IN THIS MATCH
                        check_real = 0; //self pos added on return //1 << pos;//self.gen_pawn_moves(&board, pos, !board.turn);
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

        (check_real | (1 << pos), check_full)
    }

    pub fn gen_ray_attacks(&self, occupied: u64, dir: Dir, square: usize) -> u64 {
        let set = &self.ray_attacks[dir as usize];
        let mut attack: u64 = set[square];
        let block = attack & occupied;
        if block != 0 {
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

fn gen_ray_attacks() -> Vec<Vec<u64>> {
    let mut attacks: Vec<Vec<u64>> = vec![vec![0; 64]; 8];

    //North
    let mut nort: u64 = 0x0101010101010100;
    for i in 0..64 {
        attacks[Dir::North as usize][i as usize] = nort;
        nort <<= 1;
    }
    //South
    let mut south: u64 = 0x80808080808080;
    for i in (0..64).rev() {
        attacks[Dir::South as usize][i as usize] = south;
        south >>= 1;
    }

    //East
    let mut east: u64;
    for i in 0..8 {
        east = 0xfe << (8 * i);
        for ii in 0..8 {
            attacks[Dir::East as usize][((i * 8) + ii) as usize] = east;
            east = postshift::east_one(east);
        }
    }

    //West
    let mut west: u64;
    for i in 0..8 {
        west = 0x7f << (8 * i);
        for ii in 0..8 {
            attacks[Dir::West as usize][((i * 8) + (7 - ii)) as usize] = west;
            west = postshift::west_one(west);
        }
    }

    //noea
    let mut noea;
    for i in 0..8 {
        noea = 0x8040201008040200 << (8 * i);
        for ii in 0..8 {
            attacks[Dir::NOEA as usize][((i * 8) + ii) as usize] = noea;

            noea = postshift::east_one(noea);
        }
    }

    //nowe
    let mut nowe: u64;
    for i in 0..8 {
        nowe = 0x102040810204000 << (8 * i);
        for ii in 0..8 {
            attacks[Dir::NOWE as usize][((i * 8) + (7 - ii)) as usize] = nowe;

            nowe = postshift::west_one(nowe);
        }
    }

    //soea
    let mut soea: u64;
    for i in (0..8).rev() {
        soea = 0x2040810204080 >> (8 * (7 - i));
        for ii in 0..8 {
            attacks[Dir::SOEA as usize][((i * 8) + ii) as usize] = soea;

            soea = postshift::east_one(soea);
        }
    }

    //sowe
    let mut sowe: u64;
    for i in (0..8).rev() {
        sowe = 0x40201008040201 >> (8 * (7 - i));
        for ii in 0..8 {
            attacks[Dir::SOWE as usize][((i * 8) + (7 - ii)) as usize] = sowe;

            sowe = postshift::west_one(sowe);
        }
    }

    attacks
}

fn gen_pawn_attacks() -> Vec<Vec<u64>> {
    let mut attacks: Vec<Vec<u64>> = vec![vec![0; 64]; 2];

    for i in 8..56 {
        let bb = 1 << i;

        attacks[PieceColor::White as usize][i] =
            postshift::no_ea_one(bb) | postshift::no_we_one(bb);
    }

    for i in 8..56 {
        let bb = 1 << i;

        attacks[PieceColor::Black as usize][i] =
            postshift::so_ea_one(bb) | postshift::so_we_one(bb);
    }

    attacks
}

fn gen_king_attacks() -> Vec<u64> {
    let mut attacks: Vec<u64> = vec![0; 64];

    for i in 0..64 {
        let mut pos = 1 << i;

        pos |= postshift::east_one(pos) | postshift::west_one(pos);
        pos |= postshift::nort_one(pos) | postshift::sout_one(pos);

        attacks[i] = pos;
    }

    attacks
}

fn gen_knight_attacks() -> Vec<u64> {
    let mut attacks: Vec<u64> = vec![0; 64];

    for i in 0..64 {
        let pos = 1 << i;
        let mut attack = 0;
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
