use chicory::chicory::board::Board;
use chicory::chicory::engine::{Engine, Move};

mod common;
use crate::common::assert_fen_arr;
use common::fen_arr;

#[test]
fn pawn_base() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/8/8/3p4/2P5/8 w - - 0 1", &eng);
    let mut moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    assert_fen_arr(
        &mut moves,
        &mut fen_arr(
            &eng,
            10,
            vec![
                (18, "8/8/8/8/8/2Pp4/8/8 b - - 1 1"),
                (19, "8/8/8/8/8/3P4/8/8 b - - 1 1"),
                (26, "8/8/8/8/2P5/3p4/8/8 b - c3 1 1"),
            ],
        ),
    );
}

#[test]
fn knight_base() {
    let eng = Engine::new();
    let board = Board::new("8/8/2p2n2/8/2pNp3/1r6/4p3/8 w - - 0 1", &eng);
    let mut moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    assert_fen_arr(
        &mut moves,
        &mut fen_arr(
                &eng,
            27,
            vec![
                (10, "8/8/2p2n2/8/2p1p3/1r6/2N1p3/8 b - - 1 1"),
                (12, "8/8/2p2n2/8/2p1p3/1r6/4N3/8 b - - 1 1"),
                (17, "8/8/2p2n2/8/2p1p3/1N6/4p3/8 b - - 1 1"),
                (21, "8/8/2p2n2/8/2p1p3/1r3N2/4p3/8 b - - 1 1"),
                (33, "8/8/2p2n2/1N6/2p1p3/1r6/4p3/8 b - - 1 1"),
                (37, "8/8/2p2n2/5N2/2p1p3/1r6/4p3/8 b - - 1 1"),
                (42, "8/8/2N2n2/8/2p1p3/1r6/4p3/8 b - - 1 1"),
                (44, "8/8/2p1Nn2/8/2p1p3/1r6/4p3/8 b - - 1 1"),
            ],
        ),
    );
}

#[test]
fn queen_base() {
    let eng = Engine::new();
    let board = Board::new("8/1K6/6p1/8/1p2Q3/4p3/8/8 w - - 0 1", &eng);
    let mut moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    //Queen
    let mut fen_moves = fen_arr(
        &eng,
        28,
        vec![
            (1, "8/1K6/6p1/8/1p6/4p3/8/1Q6 b - - 1 1"),
            (7, "8/1K6/6p1/8/1p6/4p3/8/7Q b - - 1 1"),
            (10, "8/1K6/6p1/8/1p6/4p3/2Q5/8 b - - 1 1"),
            (14, "8/1K6/6p1/8/1p6/4p3/6Q1/8 b - - 1 1"),
            (19, "8/1K6/6p1/8/1p6/3Qp3/8/8 b - - 1 1"),
            (20, "8/1K6/6p1/8/1p6/4Q3/8/8 b - - 1 1"),
            (21, "8/1K6/6p1/8/1p6/4pQ2/8/8 b - - 1 1"),
            (25, "8/1K6/6p1/8/1Q6/4p3/8/8 b - - 1 1"),
            (26, "8/1K6/6p1/8/1pQ5/4p3/8/8 b - - 1 1"),
            (27, "8/1K6/6p1/8/1p1Q4/4p3/8/8 b - - 1 1"),
            (29, "8/1K6/6p1/8/1p3Q2/4p3/8/8 b - - 1 1"),
            (30, "8/1K6/6p1/8/1p4Q1/4p3/8/8 b - - 1 1"),
            (31, "8/1K6/6p1/8/1p5Q/4p3/8/8 b - - 1 1"),
            (35, "8/1K6/6p1/3Q4/1p6/4p3/8/8 b - - 1 1"),
            (36, "8/1K6/6p1/4Q3/1p6/4p3/8/8 b - - 1 1"),
            (37, "8/1K6/6p1/5Q2/1p6/4p3/8/8 b - - 1 1"),
            (42, "8/1K6/2Q3p1/8/1p6/4p3/8/8 b - - 1 1"),
            (44, "8/1K6/4Q1p1/8/1p6/4p3/8/8 b - - 1 1"),
            (46, "8/1K6/6Q1/8/1p6/4p3/8/8 b - - 1 1"),
            (52, "8/1K2Q3/6p1/8/1p6/4p3/8/8 b - - 1 1"),
            (60, "4Q3/1K6/6p1/8/1p6/4p3/8/8 b - - 1 1"),
        ],
    );

    //King
    fen_moves.append(&mut fen_arr(
        &eng,
        49,
        vec![
            (40, "8/8/K5p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
            (41, "8/8/1K4p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
            (42, "8/8/2K3p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
            (48, "8/K7/6p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
            (50, "8/2K5/6p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
            (56, "K7/8/6p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
            (57, "1K6/8/6p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
            (58, "2K5/8/6p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
        ],
    ));

    assert_fen_arr(&mut moves, &mut fen_moves);
}

//en_passant
#[test]
fn pawn_en_passant() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/3Pp3/8/8/8/8 w - e6 0 1", &eng);
    let mut moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    assert_fen_arr(
        &mut moves,
        &mut fen_arr(
            &eng,
            35,
            vec![
                (43, "8/8/3P4/4p3/8/8/8/8 b - - 1 1"),
                (44, "8/8/4P3/8/8/8/8/8 b - - 1 1"),
            ],
        ),
    );
}

#[test]
fn pawn_promote() {
    let eng = Engine::new();
    let board = Board::new("8/1P6/8/8/8/8/8/8 w - - 0 1", &eng);
    let mut moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    assert_fen_arr(
        &mut moves,
        &mut fen_arr(
            &eng,
            49,
            vec![
                (57, "1N6/8/8/8/8/8/8/8 b - - 1 1"),
                (57, "1B6/8/8/8/8/8/8/8 b - - 1 1"),
                (57, "1R6/8/8/8/8/8/8/8 b - - 1 1"),
                (57, "1Q6/8/8/8/8/8/8/8 b - - 1 1"),
            ],
        ),
    );
}

#[test]
fn pawn_promote_black() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/8/8/8/1p6/8 b - - 1 1", &eng);
    let mut moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    assert_fen_arr(
        &mut moves,
        &mut fen_arr(
            &eng,
            09,
            vec![
                (01, "8/8/8/8/8/8/8/1n6 w - - 2 2"),
                (01, "8/8/8/8/8/8/8/1b6 w - - 2 2"),
                (01, "8/8/8/8/8/8/8/1r6 w - - 2 2"),
                (01, "8/8/8/8/8/8/8/1q6 w - - 2 2"),
            ],
        ),
    );
}

#[test]
fn pawn_capture_promote() {
    let eng = Engine::new();
    let board = Board::new("1p6/2P5/8/8/8/8/8/8 w - - 0 1", &eng);
    let mut moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    let mut fen_moves: Vec<Move> = vec![];

    fen_moves.append(&mut fen_arr(
        &eng,
        50,
        vec![
            (57, "1N6/8/8/8/8/8/8/8 b - - 1 1"),
            (57, "1B6/8/8/8/8/8/8/8 b - - 1 1"),
            (57, "1R6/8/8/8/8/8/8/8 b - - 1 1"),
            (57, "1Q6/8/8/8/8/8/8/8 b - - 1 1"),
        ],
    ));

    fen_moves.append(&mut fen_arr(
        &eng,
        50,
        vec![
            (58, "1pN5/8/8/8/8/8/8/8 b - - 1 1"),
            (58, "1pB5/8/8/8/8/8/8/8 b - - 1 1"),
            (58, "1pR5/8/8/8/8/8/8/8 b - - 1 1"),
            (58, "1pQ5/8/8/8/8/8/8/8 b - - 1 1"),
        ],
    ));

    assert_fen_arr(&mut moves, &mut fen_moves);
}

#[test]
fn king_check() {
    let eng = Engine::new();
    let mut board = Board::new("7b/8/8/4K3/7B/8/8/8 w - - 0 1", &eng);
    //board.check = 0x8040201000000000;

    let (check_real, check_full) = eng.gen_check_info(&board, 63, 0);

    board.check_real = check_real;
    board.check_full = check_full;

    let mut moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    let mut fen_moves = fen_arr(
        &eng,
        36,
        vec![
            (28, "7b/8/8/8/4K2B/8/8/8 b - - 1 1"),
            (29, "7b/8/8/8/5K1B/8/8/8 b - - 1 1"),
            (35, "7b/8/8/3K4/7B/8/8/8 b - - 1 1"),
            (37, "7b/8/8/5K2/7B/8/8/8 b - - 1 1"),
            (43, "7b/8/3K4/8/7B/8/8/8 b - - 1 1"),
            (44, "7b/8/4K3/8/7B/8/8/8 b - - 1 1"),
        ],
    );

    fen_moves.append(&mut fen_arr(
        &eng,
        31,
        vec![(45, "7b/8/5B2/4K3/8/8/8/8 b - - 1 1")],
    ));

    assert_fen_arr(&mut moves, &mut fen_moves);
}

#[test]
fn king_to_check() {
    let eng = Engine::new();
    let board = Board::new("5k2/8/4P3/8/8/8/8/3K4 w - - 0 1", &eng);

    let mut moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    let mut fen_moves: Vec<Move> = vec![];

    let mut checked_board = Board::new("5k2/4P3/8/8/8/8/8/3K4 b - - 1 1", &eng);

    checked_board.check_real = 0x10000000000000; //check_real;
    checked_board.check_full = 0x2000000000000000;

    fen_moves.push(Move{from: 44, to: 52, board: checked_board, promote_to: None, capture: false});

    fen_moves.append(&mut fen_arr(
        &eng,
        03,
        vec![
            (02, "5k2/8/4P3/8/8/8/8/2K5 b - - 1 1"),
            (04, "5k2/8/4P3/8/8/8/8/4K3 b - - 1 1"),
            (10, "5k2/8/4P3/8/8/8/2K5/8 b - - 1 1"),
            (11, "5k2/8/4P3/8/8/8/3K4/8 b - - 1 1"),
            (12, "5k2/8/4P3/8/8/8/4K3/8 b - - 1 1"),
        ],
    ));

    assert_fen_arr(&mut moves, &mut fen_moves);
}

#[test]
fn king_to_check_next() {
    let eng = Engine::new();
    let board = Board::new("5k2/8/4P3/8/8/8/8/3K4 w - - 0 1", &eng);

    let mut moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    let mut fen_moves: Vec<Move> = vec![];

    let mut checked_board = Board::new("5k2/4P3/8/8/8/8/8/3K4 b - - 1 1", &eng);

    checked_board.check_real = 0x10000000000000; //check_real;
    checked_board.check_full = 0x2000000000000000;

    fen_moves.push(Move{from: 44, to: 52, board: checked_board, promote_to: None, capture: false});

    fen_moves.append(&mut fen_arr(
        &eng,
        03,
        vec![
            (02, "5k2/8/4P3/8/8/8/8/2K5 b - - 1 1"),
            (04, "5k2/8/4P3/8/8/8/8/4K3 b - - 1 1"),
            (10, "5k2/8/4P3/8/8/8/2K5/8 b - - 1 1"),
            (11, "5k2/8/4P3/8/8/8/3K4/8 b - - 1 1"),
            (12, "5k2/8/4P3/8/8/8/4K3/8 b - - 1 1"),
        ],
    ));

    assert_fen_arr(&mut moves, &mut fen_moves);

    let mut next_moves = eng.gen_moves(moves[0].board);

    assert_fen_arr(
        &mut next_moves,
        &mut fen_arr(
            &eng,
            61,
            vec![
                (52, "8/4k3/8/8/8/8/8/3K4 w - - 2 2"),
                (53, "8/4Pk2/8/8/8/8/8/3K4 w - - 2 2"),
                (54, "8/4P1k1/8/8/8/8/8/3K4 w - - 2 2"),
                (60, "4k3/4P3/8/8/8/8/8/3K4 w - - 2 2"),
                (62, "6k1/4P3/8/8/8/8/8/3K4 w - - 2 2"),
            ],
        ),
    );
}

#[test]
fn king_castling() {
    let eng = Engine::new();
    let board = Board::new("4k3/8/8/8/8/8/8/4K2R w K - 0 1", &eng);
    let mut moves = eng.gen_moves(board);

    let mut fen_moves: Vec<Move>;

    println!("Moves: {:?}", moves);

    fen_moves = fen_arr(&eng,80, vec![(80, "4k3/8/8/8/8/8/8/5RK1 b - - 1 1")]);

    fen_moves.append(&mut fen_arr(
        &eng,
        7,
        vec![
            (5, "4k3/8/8/8/8/8/8/4KR2 b - - 1 1"),
            (6, "4k3/8/8/8/8/8/8/4K1R1 b - - 1 1"),
            (15, "4k3/8/8/8/8/8/7R/4K3 b - - 1 1"),
            (23, "4k3/8/8/8/8/7R/8/4K3 b - - 1 1"),
            (31, "4k3/8/8/8/7R/8/8/4K3 b - - 1 1"),
            (39, "4k3/8/8/7R/8/8/8/4K3 b - - 1 1"),
            (47, "4k3/8/7R/8/8/8/8/4K3 b - - 1 1"),
            (55, "4k3/7R/8/8/8/8/8/4K3 b - - 1 1"),
        ],
    ));

    let mut checked_board = Board::new("4k2R/8/8/8/8/8/8/4K3 b - - 1 1", &eng);

    checked_board.check_real = 0xe000000000000000; //check_real;
    checked_board.check_full = 0x7f80808080808080;

    fen_moves.push(Move{from: 7, to: 63, board: checked_board, promote_to: None, capture: false});

    fen_moves.append(&mut fen_arr(
        &eng,
        04,
        vec![
            (3, "4k3/8/8/8/8/8/8/3K3R b - - 1 1"),
            (5, "4k3/8/8/8/8/8/8/5K1R b - - 1 1"),
            (11, "4k3/8/8/8/8/8/3K4/7R b - - 1 1"),
            (12, "4k3/8/8/8/8/8/4K3/7R b - - 1 1"),
            (13, "4k3/8/8/8/8/8/5K2/7R b - - 1 1"),
        ],
    ));

    assert_fen_arr(&mut moves, &mut fen_moves);
}

#[test]
fn black_king_castling() {
    let eng = Engine::new();
    let board = Board::new("r1P1k3/P7/8/8/8/8/P7/R3K3 b Qq - 0 1", &eng);
    //let (_, _,start) = eng.gen_moves(board)[0];

    let mut fen_moves: Vec<Move>;

    println!("Moves: {:?}", board);

    let mut moves = eng.gen_moves(board);

    fen_moves = fen_arr(
        &eng,
        56,
        vec![
            (48, "2P1k3/r7/8/8/8/8/P7/R3K3 w Q - 1 2"),
            (57, "1rP1k3/P7/8/8/8/8/P7/R3K3 w Q - 1 2"),
            (58, "2r1k3/P7/8/8/8/8/P7/R3K3 w Q - 1 2"),
        ],
    );

    fen_moves.append(&mut fen_arr(
        &eng,
        60,
        vec![
            (51, "r1P5/P2k4/8/8/8/8/P7/R3K3 w Q - 1 2"),
            (52, "r1P5/P3k3/8/8/8/8/P7/R3K3 w Q - 1 2"),
            (53, "r1P5/P4k2/8/8/8/8/P7/R3K3 w Q - 1 2"),
            (59, "r1Pk4/P7/8/8/8/8/P7/R3K3 w Q - 1 2"),
            (61, "r1P2k2/P7/8/8/8/8/P7/R3K3 w Q - 1 2"),
        ],
    ));

    assert_fen_arr(&mut moves, &mut fen_moves);
}

#[test]
fn queenside_blocked_castling() {
    let eng = Engine::new();
    let board = Board::new("r3k3/1P6/P7/8/8/8/8/4K3 w q - 0 1", &eng);

    let mut fen_moves: Vec<Move>;

    println!("Moves: {:?}", eng.gen_moves(board)[8].board);

    let mut moves = eng.gen_moves(eng.gen_moves(board)[8].board);

    fen_moves = fen_arr(
        &eng,
        56,
        vec![
            (48, "4k3/rP6/8/8/8/8/8/4K3 w - - 2 2"),
            (57, "1r2k3/PP6/8/8/8/8/8/4K3 w - - 2 2"),
            (58, "2r1k3/PP6/8/8/8/8/8/4K3 w - - 2 2"),
            (59, "3rk3/PP6/8/8/8/8/8/4K3 w - - 2 2"),
        ],
    );

    fen_moves.append(&mut fen_arr(
        &eng,
        60,
        vec![
            (51, "r7/PP1k4/8/8/8/8/8/4K3 w - - 2 2"),
            (52, "r7/PP2k3/8/8/8/8/8/4K3 w - - 2 2"),
            (53, "r7/PP3k2/8/8/8/8/8/4K3 w - - 2 2"),
            (59, "r2k4/PP6/8/8/8/8/8/4K3 w - - 2 2"),
            (61, "r4k2/PP6/8/8/8/8/8/4K3 w - - 2 2"),
        ],
    ));

    assert_fen_arr(&mut moves, &mut fen_moves);
}

#[test]
fn discovered_check() {
    let eng = Engine::new();
    let board = Board::new("8/1k6/8/8/8/5P2/6B1/8 w - - 0 1", &eng);

    let mut moves = eng.gen_moves(eng.gen_moves(board)[3].board);

    let mut fen_moves = fen_arr(
        &eng,
        49,
        vec![
            (40, "8/8/k7/8/5P2/8/6B1/8 w - - 2 2"),
            (41, "8/8/1k6/8/5P2/8/6B1/8 w - - 2 2"),
            (48, "8/k7/8/8/5P2/8/6B1/8 w - - 2 2"),
            (50, "8/2k5/8/8/5P2/8/6B1/8 w - - 2 2"),
            (57, "1k6/8/8/8/5P2/8/6B1/8 w - - 2 2"),
            (58, "2k5/8/8/8/5P2/8/6B1/8 w - - 2 2"),
        ],
    );

    assert_fen_arr(&mut moves, &mut fen_moves);
}

#[test]
fn king_capture_check() {
    let eng = Engine::new();
    let board = Board::new("8/1k6/8/8/8/2K5/6r1/8 b - - 1 1", &eng);

    let mut moves = eng.gen_moves(eng.gen_moves(board)[3].board);

    let mut fen_moves = fen_arr(
        &eng,
        18,
        vec![
            (10, "8/1k6/8/8/8/8/2K5/8 b - - 3 2"),
            (17, "8/1k6/8/8/8/1K6/2r5/8 b - - 3 2"),
            (19, "8/1k6/8/8/8/3K4/2r5/8 b - - 3 2"),
            (25, "8/1k6/8/8/1K6/8/2r5/8 b - - 3 2"),
            (27, "8/1k6/8/8/3K4/8/2r5/8 b - - 3 2"),
        ],
    );

    println!("Moves: {:?}", eng.gen_moves(board)[3]);
    assert_fen_arr(&mut moves, &mut fen_moves);
}

//Double Check
#[test]
fn king_capture_double_check() {
    let eng = Engine::new();
    let board = Board::new("8/1k6/8/5q2/8/2K5/2r5/8 b - - 1 1", &eng);

    let mut moves = eng.gen_moves(eng.gen_moves(board)[16].board);

    let mut fen_moves = fen_arr(
        &eng,
        18,
        vec![
            (10, "8/1k6/5q2/8/8/8/2K5/8 b - - 3 2"),
            (17, "8/1k6/5q2/8/8/1K6/2r5/8 b - - 3 2"),
            (19, "8/1k6/5q2/8/8/3K4/2r5/8 b - - 3 2"),
            (25, "8/1k6/5q2/8/1K6/8/2r5/8 b - - 3 2"),
        ],
    );

    assert_fen_arr(&mut moves, &mut fen_moves);
}

#[test]
fn black_promotion_check() {
    let eng = Engine::new();
    let board = Board::new("8/1P6/3k4/8/8/7K/8/8 w - - 1 1", &eng);

    let mut moves = eng.gen_moves(eng.gen_moves(board)[3].board);

    let mut fen_moves = fen_arr(
        &eng,
        43,
        vec![
            (34, "1Q6/8/8/2k5/8/7K/8/8 w - - 3 2"),
            (35, "1Q6/8/8/3k4/8/7K/8/8 w - - 3 2"),
            (42, "1Q6/8/2k5/8/8/7K/8/8 w - - 3 2"),
            (44, "1Q6/8/4k3/8/8/7K/8/8 w - - 3 2"),
            (51, "1Q6/3k4/8/8/8/7K/8/8 w - - 3 2"),
            (52, "1Q6/4k3/8/8/8/7K/8/8 w - - 3 2"),
        ],
    );

    println!("Moves: {:?}", eng.gen_moves(board)[3]);
    assert_fen_arr(&mut moves, &mut fen_moves);
}

#[test]
fn blocked_castle_att() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/2b5/4p3/8/8/4K2R b K - 0 1", &eng);

    let mut moves = eng.gen_moves(eng.gen_moves(board)[11].board);

    let mut fen_moves = fen_arr(&eng,80, vec![(80, "8/8/8/2b5/8/4p3/8/5RK1 b - - 2 2")]);

    fen_moves.append(&mut fen_arr(
        &eng,
        7,
        vec![
            (5, "8/8/8/2b5/8/4p3/8/4KR2 b - - 2 2"),
            (6, "8/8/8/2b5/8/4p3/8/4K1R1 b - - 2 2"),
            (15, "8/8/8/2b5/8/4p3/7R/4K3 b - - 2 2"),
            (23, "8/8/8/2b5/8/4p2R/8/4K3 b - - 2 2"),
            (31, "8/8/8/2b5/7R/4p3/8/4K3 b - - 2 2"),
            (39, "8/8/8/2b4R/8/4p3/8/4K3 b - - 2 2"),
            (47, "8/8/7R/2b5/8/4p3/8/4K3 b - - 2 2"),
            (55, "8/7R/8/2b5/8/4p3/8/4K3 b - - 2 2"),
            (63, "7R/8/8/2b5/8/4p3/8/4K3 b - - 2 2"),
        ],
    ));

    fen_moves.append(&mut fen_arr(
        &eng,
        4,
        vec![
            (3, "8/8/8/2b5/8/4p3/8/3K3R b - - 2 2"),
            (5, "8/8/8/2b5/8/4p3/8/5K1R b - - 2 2"),
            (12, "8/8/8/2b5/8/4p3/4K3/7R b - - 2 2"),
        ],
    ));

    assert_fen_arr(&mut moves, &mut fen_moves);
}
