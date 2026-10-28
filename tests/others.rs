use chicory::chicory::board::{Board};
use chicory::chicory::engine::{Engine};

#[test]
fn hash_move_unmove() {
    let eng = Engine::new();
    let mut board = Board::new("8/8/8/3Rr3/8/8/8/8 w - - 0 1", &eng);

    println!("======= INIT =======");
    board.print_board();
    println!("=====================");

    let init_hash = board.zobrist_hash;

    let step_1_moves = eng.gen_moves(&board);
    let step_1 = step_1_moves[8];

    board.make_move(&eng, step_1);
    println!("Mid Hash: {}", board.zobrist_hash);

    let step_2_moves = eng.gen_moves(&board);
    let step_2 = step_2_moves[0];

    board.make_move(&eng, step_2);

    let step_3_moves = eng.gen_moves(&board);
    let step_3 = step_3_moves[4];

    board.make_move(&eng, step_3);

    let step_4_moves = eng.gen_moves(&board);
    let step_4 = step_4_moves[10];

    board.make_move(&eng, step_4);

    println!("\n======= FINAL =======");
    board.print_board();
    println!("=====================");

    let final_hash = board.zobrist_hash;

    println!("init  hash: {:?}", init_hash);
    println!("final hash: {:?}", final_hash);

    assert_eq!(init_hash, final_hash);
}
