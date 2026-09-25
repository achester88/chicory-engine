use board::Board;
use chicory::chicory::{board, engine};
use engine::{Move, Engine};

pub fn fen_arr(eng: &Engine, from: usize, new_boards: Vec<(usize, &str)>) -> Vec<Move> {
    let mut boards: Vec<Move> = vec![];
    //let eng = engine::Engine::new(); //Only for testing

    for (to, fen) in new_boards.iter() {
        boards.push((Move{from, to: *to, board: Board::new(fen, eng), promote_to: None, capture: false}));
    }

    boards
}

pub fn assert_fen_arr(eng_arr: &mut Vec<Move>, expc_arr: &mut Vec<Move>) {
    for i in 0..eng_arr.len() {
        //let mut board2 = board.clone();

        //Would be better to add proper check, but would need to change all fen values in all test
        eng_arr[i].board.castling &= 0b0000_1111;
        eng_arr[i].promote_to = None;
        eng_arr[i].capture = false;
        //eng_arr[i].board.zobrist_hash = 0;
    }

    for i in 0..expc_arr.len() {
        //let (from, to, board) = m;
        //let mut board2 = board.clone();
        expc_arr[i].board.castling &= 0b0000_1111;

    }

    assert_eq!(eng_arr, expc_arr);
}
