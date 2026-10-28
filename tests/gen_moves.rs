use bitboard::print_bitboard_pos;
use board::Board;
use chicory::chicory::{bitboard, board, engine};
use chicory::chicory::bitboard::Bitboard;
use engine::Engine;

#[test]
fn pawn_empty() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/8/8/4P3/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_pawn_moves(&board, 20, board.turn);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x10000000));
}

#[test]
fn pawn_start() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/8/8/8/2P5/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_pawn_moves(&board, 10, board.turn);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x4040000));
}

#[test]
fn bishop_empty() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/8/8/4B3/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_bishop_moves(&board, 20, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x1824428002844));
}

#[test]
fn knight_empty() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/8/8/4N3/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_knight_moves(&board, 20, board.turn);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x2844004428));
}

#[test]
fn rook_empty() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/8/8/4R3/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_rook_moves(&board, 20, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x1010101010ef1010));
}

#[test]
fn queen_empty() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/8/8/4Q3/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_queen_moves(&board, 20, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x1011925438ef3854));
}

#[test]
fn king_empty() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/8/8/4K3/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_king_moves(&board, 20, board.turn);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x38283800));
}

#[test]
fn pawn_capture() {
    let eng = Engine::new();
    let board = Board::new("8/8/p1p5/1P6/8/8/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_pawn_moves(&board, 33, board.turn);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x70000000000));
}

#[test]
fn knight_capture() {
    let eng = Engine::new();
    let board = Board::new("8/8/2p1p3/1p3p2/3N4/1p3p2/2p1p3/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_knight_moves(&board, 27, board.turn);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x142200221400));
}

#[test]
fn bishop_capture() {
    let eng = Engine::new();
    let board = Board::new("8/8/1p3p2/8/3B4/4p3/8/p7 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_bishop_moves(&board, 27, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x221400140201));
}

#[test]
fn rook_capture() {
    let eng = Engine::new();
    let board = Board::new("3p4/8/8/8/p2Rp3/8/3p4/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_rook_moves(&board, 27, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x808080817080800));
}

#[test]
fn queen_capture() {
    let eng = Engine::new();
    let board = Board::new("3p3p/8/1p6/8/p2Qp3/2p5/3p1p2/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_queen_moves(&board, 27, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x88482a1c171c2800));
}

#[test]
fn king_capture() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/2ppp3/2pKp3/2ppp3/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_king_moves(&board, 27, board.turn);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x1c141c0000));
}

#[test]
fn pawn_block() {
    let eng = Engine::new();
    let board = Board::new("8/8/4PpP1/5P2/8/8/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_pawn_moves(&board, 37, board.turn);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x0));
}

#[test]
fn bishop_block() {
    let eng = Engine::new();
    let board = Board::new("8/8/4P1P1/5B2/4P1P1/8/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_bishop_moves(&board, 37, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x0));
}

#[test]
fn rook_block() {
    let eng = Engine::new();
    let board = Board::new("8/8/4PPP1/4PQP1/4PPP1/8/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_rook_moves(&board, 37, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x0));
}

#[test]
fn queen_block() {
    let eng = Engine::new();
    let board = Board::new("8/8/4PPP1/4PQP1/4PPP1/8/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_queen_moves(&board, 37, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x0));
}

#[test]
fn king_block() {
    let eng = Engine::new();
    let board = Board::new("8/8/4PPP1/4PKP1/4PPP1/8/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_king_moves(&board, 37, board.turn);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x0));
}

#[test]
fn pawn_corner() {
    let eng = Engine::new();
    let board = Board::new("7P/8/8/8/8/8/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_pawn_moves(&board, 63, board.turn);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x0));
}

#[test]
fn knight_corner() {
    let eng = Engine::new();
    let board = Board::new("7N/8/8/8/8/8/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_knight_moves(&board, 63, board.turn);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x20400000000000));
}

#[test]
fn bishop_corner() {
    let eng = Engine::new();
    let board = Board::new("7B/8/8/8/8/8/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_bishop_moves(&board, 63, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x40201008040201));
}

#[test]
fn rook_corner() {
    let eng = Engine::new();
    let board = Board::new("7B/8/8/8/8/8/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_rook_moves(&board, 63, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x7f80808080808080));
}

#[test]
fn queen_corner() {
    let eng = Engine::new();
    let board = Board::new("7B/8/8/8/8/8/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_queen_moves(&board, 63, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x7fc0a09088848281));
}

#[test]
fn king_corner() {
    let eng = Engine::new();
    let board = Board::new("7K/8/8/8/8/8/8/8 w - - 0 1", &eng);
    let (pos, bb) = eng.gen_king_moves(&board, 63, board.turn);
    print_bitboard_pos(bb, pos);

    assert_eq!(bb, Bitboard::new(0x40c0000000000000));
}
