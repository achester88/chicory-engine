use chicory::chicory::board::{Board, PieceColor};
use chicory::chicory::engine::{Engine, Move};
use chicory::chicory::tables::ZobristKeys;

#[test]
fn hash_move_unmove() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/3Rr3/8/8/8/8 w - - 0 1", &eng);

    println!("======= INIT =======");
    board.print_board();
    println!("=====================");

    let init_hash = board.zobrist_hash;

    let step_1_moves = eng.gen_moves(board);
    let step_1 = step_1_moves[8];

    let step_2_moves = eng.gen_moves(step_1.board);
    let step_2 = step_2_moves[0];

    let step_3_moves = eng.gen_moves(step_2.board);
    let step_3 = step_3_moves[4];

    let step_4_moves = eng.gen_moves(step_3.board);
    let step_4 = step_4_moves[10];


    println!("\n======= FINAL =======");
    step_4.board.print_board();
    println!("=====================");

    let final_hash = step_4.board.zobrist_hash;

    println!("init  hash: {:?}", init_hash);
    println!("final hash: {:?}", final_hash);

    assert_eq!(init_hash, final_hash);
}
