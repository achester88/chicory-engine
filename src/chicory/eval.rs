use std::collections::HashMap;
use crate::chicory::bitboard::board_serialize;
use crate::chicory::board::{Board, PieceColor};
use crate::chicory::engine::{Engine, Move};
use crate::chicory::tables::{Entry, Flag};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::MutexGuard;
use std::time::Instant;

const BLACK_PAWN_PS_TABLE: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5,
    10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20,
    -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
];

const BLACK_KNIGHT_PS_TABLE: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10,
    0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10,
    5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];

const BLACK_BISHOP_PS_TABLE: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10,
    -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];

const BLACK_ROOK_PS_TABLE: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
    0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0,
    0, 5, 5, 0, 0, 0,
];

const BLACK_QUEEN_PS_TABLE: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

const BLACK_KING_MID_PS_TABLE: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];

const BLACK_KING_END_PS_TABLE: [i32; 64] = [
    -50, -40, -30, -20, -20, -30, -40, -50, -30, -20, -10, 0, 0, -10, -20, -30, -30, -10, 20, 30,
    30, 20, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30,
    -10, 20, 30, 30, 20, -10, -30, -30, -30, 0, 0, 0, 0, -30, -30, -50, -30, -30, -30, -30, -30,
    -30, -50,
];

const WHITE_PAWN_PS_TABLE: [i32; 64] = reverse_array(BLACK_PAWN_PS_TABLE);
const WHITE_KNIGHT_PS_TABLE: [i32; 64] = reverse_array(BLACK_KNIGHT_PS_TABLE);
const WHITE_BISHOP_PS_TABLE: [i32; 64] = reverse_array(BLACK_BISHOP_PS_TABLE);
const WHITE_ROOK_PS_TABLE: [i32; 64] = reverse_array(BLACK_ROOK_PS_TABLE);
const WHITE_QUEEN_PS_TABLE: [i32; 64] = reverse_array(BLACK_QUEEN_PS_TABLE);
const WHITE_KING_MID_PS_TABLE: [i32; 64] = reverse_array(BLACK_KING_MID_PS_TABLE);
const WHITE_KING_END_PS_TABLE: [i32; 64] = reverse_array(BLACK_KING_END_PS_TABLE);

pub fn minmax(
    eng: &Engine,
    board: Board,
    depth: usize,
    mut alpha: i32,
    mut beta: i32,
    turn: PieceColor,
    par_moves: usize,
    stop_calculation: &AtomicBool,
    time_per_move: f64,
    move_timer: Instant,
    positions_reached:  &mut MutexGuard<HashMap<u64, usize>>,
    transposition_table: &mut MutexGuard<HashMap<u64, Entry>>,
    eval_first: Option<Move>,
    top: bool,
    capture: bool
) -> (i32, Option<Move>, usize, Vec<Move>) {
    let test_start = Instant::now();

    let init_pos_count: usize;

        if top {
            init_pos_count = 0; //If were at the top i.e. haven't made a move, we can ignore our
                                //pos_count
        } else {
        match positions_reached.get(&board.zobrist_hash) {
            Some(x) => {

                if x >= &2 {
                    //println!("depth: {}", depth);
                    return (0, None, 1, vec![]);
                    //init_pos_count = x.clone();
                } else {
                    init_pos_count = x.clone();
                    positions_reached.insert(board.zobrist_hash, init_pos_count + 1);

                }
            },
            None => {
                positions_reached.insert(board.zobrist_hash, 1);
                init_pos_count = 0;
            }
        }
        }

    if depth == 0 {
        if capture {// || board.check_real != 0 {
            if init_pos_count == 0 {
                positions_reached.remove(&board.zobrist_hash);
            } else {
                positions_reached.insert(board.zobrist_hash, init_pos_count);
            }

            return stopping_search(&eng, board, alpha, beta, turn, par_moves, stop_calculation, time_per_move, move_timer, true, 0) //(eval(&board), None, 1);
        } else {

            if init_pos_count == 0 {
                positions_reached.remove(&board.zobrist_hash);
            } else {
                positions_reached.insert(board.zobrist_hash, init_pos_count);
            }

            return (eval(&board), None, 1, vec![])
        }
    }

    let mut best = match turn {
        PieceColor::White => i32::MIN,
        PieceColor::Black => i32::MAX,
    };

    if transposition_table.contains_key(&board.zobrist_hash) {
        let entry = transposition_table.get(&board.zobrist_hash).unwrap();
        if entry.zobrist_hash == board.zobrist_hash {
            if entry.depth >= depth {
                //TODO CHECK IF TURN "WORKS"

                match entry.flag {
                    Flag::ALPHA => {
                        beta = beta.min(entry.eval);
                    },
                    Flag::BETA => {
                        alpha = alpha.max(entry.eval);
                    },
                    Flag::EXACT => {
                        if init_pos_count == 0 {
                            positions_reached.remove(&board.zobrist_hash);
                        } else {
                            positions_reached.insert(board.zobrist_hash, init_pos_count);
                        }

                        return (entry.eval, Some(entry.move_info), 1, vec![]);
                    }
                }

                if alpha >= beta {
                    if init_pos_count == 0 {
                        positions_reached.remove(&board.zobrist_hash);
                    } else {
                        positions_reached.insert(board.zobrist_hash, init_pos_count);
                    }

                    return (entry.eval, Some(entry.move_info), 1, vec![]);

                }

            }
        }
    }


    let mut moves = order_moves(eng.gen_moves(board));

    if moves.len() == 0 {

        if init_pos_count == 0 {
            positions_reached.remove(&board.zobrist_hash);
        } else {
            positions_reached.insert(board.zobrist_hash, init_pos_count);
        }

        if board.check_real == 0 { //Stalemate
            return (0, None, 1, vec![]);
        }

        return match turn {
            PieceColor::White => (i32::MIN, None, 1, vec![]),
            PieceColor::Black => (i32::MAX, None, 1, vec![]),
        };
    }

    let mut best_move = moves[0];

    let mut best_pv: Vec<Move> = vec![];

    let total_nodes = par_moves * moves.len();

    let mut node_count = 0;

    if eval_first.is_some() {
        moves.insert(0, eval_first.unwrap());
    }

    let mut early_stop = false;

    let mut flag = Flag::EXACT;

    for m in moves {

        let (score, _, nodes, pv) = minmax(
            &eng,
            m.board,
            depth - 1,
            alpha,
            beta,
            !turn,
            total_nodes,
            stop_calculation,
            time_per_move,
            move_timer,
            positions_reached,
            transposition_table,
            None,
            false,
            m.capture
        );
        node_count += nodes;

        match turn {
            PieceColor::White => {
                if score > best {
                    best = score;
                    best_move = m;
                    best_pv = pv
                }
                alpha = alpha.max(score);
            }
            PieceColor::Black => {
                if score < best {
                    best = score;
                    best_move = m;
                    best_pv = pv;
                }
                beta = beta.min(score);
            }
        }

        if beta <= alpha {
            flag = match turn {
                PieceColor::White => Flag::BETA,
                PieceColor::Black => Flag::ALPHA,
            };

            break;
        }

        if stop_calculation.load(Ordering::Relaxed) || ((depth > 4 || top) && (time_per_move != 0.0 && (move_timer.elapsed().as_millis() as f64) > time_per_move))
        {

            early_stop = true;
            break;
        }
    }


    best_pv.insert(0, best_move);

    if top && !early_stop {

        let mut pv_str = String::from("");

        for m in &mut *best_pv {
            pv_str.push_str(" ");
            pv_str.push_str(&Board::move_to_lan(&m));
        }

        println!(
            "info depth {} nodes {} score cp {} time {} pv{}",
            depth,
            node_count,
            if board.turn == PieceColor::White {best} else {-best},
            test_start.elapsed().as_millis(),
            pv_str//Board::move_to_lan(&best_move)
        );
    }

    if !early_stop {
        transposition_table.insert(board.zobrist_hash, Entry{
            zobrist_hash: board.zobrist_hash,
            depth: depth,
            flag: flag,
            eval: best,
            //ancient: false,
            move_info: best_move,
        });
    }

    if init_pos_count == 0 {
        positions_reached.remove(&board.zobrist_hash);
    } else {
        positions_reached.insert(board.zobrist_hash, init_pos_count);
    }

    (best, Some(best_move), node_count, best_pv)
}

pub fn stopping_search(
    eng: &Engine,
    board: Board,
    mut alpha: i32,
    mut beta: i32,
    turn: PieceColor,
    par_moves: usize,
    stop_calculation: &AtomicBool,
    time_per_move: f64,
    move_timer: Instant,
    top: bool,
    depth: usize,
) -> (i32, Option<Move>, usize, Vec<Move>) {

    let score = eval(&board);

    match turn {

        PieceColor::White => {

            if score >= beta {
                return (beta, None, 1, vec![]);
            }

            alpha = alpha.max(score);
        }

        PieceColor::Black => {

            if score <= alpha {
                return (alpha, None, 1, vec![]);
            }

            beta = beta.min(score);
        }
    }

    let moves = order_moves(eng.gen_moves(board));

    if moves.is_empty() {

        if board.check_real == 0 {
            return (0, None, 1, vec![]);
        }

        return match turn {
            PieceColor::White => (i32::MIN, None, 1, vec![]),
            PieceColor::Black => (i32::MAX, None, 1, vec![]),
        };
    }

    if moves.len() == 0 {
        if board.check_real == 0 { //Stalemate
            return (0, None, 1, vec![]);
        }
        return match turn {
            PieceColor::White => (i32::MIN, None, 1, vec![]),
            PieceColor::Black => (i32::MAX, None, 1, vec![]),
        };
    }

    //let mut best_move = moves[0];

    let total_nodes: usize = par_moves.saturating_mul(moves.len());

    let mut node_count = 0;

    for m in moves {
        if m.capture { //|| m.board.check_real != 0
            let (score, _, nodes, _pv) = stopping_search(
                &eng,
                m.board,
                alpha,
                beta,
                !turn,
                total_nodes,
                stop_calculation,
                time_per_move,
                move_timer,
                false,
                depth + 1
            );
        node_count += nodes;


            match turn {
                PieceColor::White => {
                    if score >= beta {
                        return (beta, None, 0, vec![]);
                    }
                    alpha = alpha.max(score);
                },
                PieceColor::Black => {
                    if score <= alpha {
                        return (alpha, None, 0, vec![]);
                    }
                    beta = beta.min(score);
                }
            }


        if stop_calculation.load(Ordering::Relaxed)
            || (top)
            && (time_per_move != 0.0 && (move_timer.elapsed().as_millis() as f64) > time_per_move)
        {
            break;
        }
            }
    }

    match turn {
        PieceColor::White => (alpha, None, node_count + 1, vec![]),
        PieceColor::Black => (beta, None, node_count + 1, vec![]),
    }
    //(best_score, None, 0)
}

fn order_moves(mut moves: Vec<Move>) -> Vec<Move> {
    let mut out: Vec<Move> = vec![];

    let mut i = 0;
    while i < moves.len() {
        if moves[i].promote_to.is_some() {
            out.push(moves[i]);
            moves.remove(i);
        } else {
            i += 1;
        }
    }
    i = 0;
    while i < moves.len() {
        if moves[i].capture {
            out.push(moves[i]);
            moves.remove(i);
        } else {
            i += 1;
        }
    }
    out.append(&mut moves);
    out
}

pub fn eval(board: &Board) -> i32 {
    let mut score = 0;

    let white_mat_score = ((board.pawns[PieceColor::White].count_ones() as i32) * 100)
        + ((board.knights[PieceColor::White].count_ones() as i32) * 320)
        + ((board.bishops[PieceColor::White].count_ones() as i32) * 330)
        + ((board.rooks[PieceColor::White].count_ones() as i32) * 500)
        + ((board.queens[PieceColor::White].count_ones() as i32) * 900);

    let black_mat_score = ((board.pawns[PieceColor::Black].count_ones() as i32) * 100)
        + ((board.knights[PieceColor::Black].count_ones() as i32) * 320)
        + ((board.bishops[PieceColor::Black].count_ones() as i32) * 330)
        + ((board.rooks[PieceColor::Black].count_ones() as i32) * 500)
        + ((board.queens[PieceColor::Black].count_ones() as i32) * 900);

    score += white_mat_score - black_mat_score;

    score += ((board_serialize(board.kings[PieceColor::White]).len() as i32)
        - (board_serialize(board.kings[PieceColor::Black]).len() as i32))
        * 20000;

    score += bit_cal(board.pawns[PieceColor::White], WHITE_PAWN_PS_TABLE)
        - bit_cal(board.pawns[PieceColor::Black], BLACK_PAWN_PS_TABLE);
    score += bit_cal(board.knights[PieceColor::White], WHITE_KNIGHT_PS_TABLE)
        - bit_cal(board.knights[PieceColor::Black], BLACK_KNIGHT_PS_TABLE);
    score += bit_cal(board.bishops[PieceColor::White], WHITE_BISHOP_PS_TABLE)
        - bit_cal(board.bishops[PieceColor::Black], BLACK_BISHOP_PS_TABLE);
    score += bit_cal(board.rooks[PieceColor::White], WHITE_ROOK_PS_TABLE)
        - bit_cal(board.rooks[PieceColor::Black], BLACK_ROOK_PS_TABLE);
    score += bit_cal(board.queens[PieceColor::White], WHITE_QUEEN_PS_TABLE)
        - bit_cal(board.queens[PieceColor::Black], BLACK_QUEEN_PS_TABLE);

    if white_mat_score <= 1000 {
        score += bit_cal(board.kings[PieceColor::White], WHITE_KING_END_PS_TABLE);
    } else {
        let endgame_level: f32 = (white_mat_score as f32 - 4000.0) / 3000.0; //(pms - game max) / (game max - 1000)
        score += ( ((bit_cal(board.kings[PieceColor::White], WHITE_KING_MID_PS_TABLE) as f32
            * (1.0 - endgame_level))
            + (bit_cal(board.kings[PieceColor::White], WHITE_KING_END_PS_TABLE) as f32 * endgame_level))
            / 2.0) as i32
    }

    if black_mat_score <= 1000 {
        score -= bit_cal(board.kings[PieceColor::Black], BLACK_KING_END_PS_TABLE);
    } else {
        let endgame_level: f32 = (black_mat_score as f32 - 4000.0) / 3000.0; //(pms - game max) / (game max - 1000)
        score -= (((bit_cal(board.kings[PieceColor::Black], BLACK_KING_MID_PS_TABLE) as f32
            * (1.0 - endgame_level))
            + (bit_cal(board.kings[PieceColor::Black], BLACK_KING_END_PS_TABLE) as f32 * endgame_level))
            / 2.0) as i32
    }

    score
}

fn bit_cal(mut bitboard: u64, table: [i32; 64]) -> i32 {
    let mut score = 0;
    while bitboard != 0 {
        let i = bitboard.trailing_zeros() as usize;
        score += table[i];
        bitboard ^= 1 << i;
    }

    score
}

const fn reverse_array<T: Copy, const N: usize>(array: [T; N]) -> [T; N] {
    let mut out = array;
    let mut i = 0;
    while i < N {
        out[i] = array[N - 1 - i];
        i += 1;
    }

    out
}
