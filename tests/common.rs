use board::{Board, Move};
use chicory::chicory::{board, engine};
use chicory::chicory::board::PieceType;
use engine::Engine;

#[derive(Debug, PartialEq)]
pub struct OldMove {
    pub from: usize,
    pub to: usize,
    pub board: Board,
    pub promote_to: Option<PieceType>,
    pub capture: bool,
}

pub fn fen_arr(eng: &Engine, from: usize, new_boards: Vec<(usize, &str)>) -> Vec<OldMove> {
    let mut boards: Vec<OldMove> = vec![];
    //let eng = engine::Engine::new(); //Only for testing

    for (to, fen) in new_boards.iter() {
        boards.push((OldMove{from, to: *to, board: Board::new(fen, eng), promote_to: None, capture: false}));
    }

    boards
}

pub fn assert_fen_arr(eng_arr: &mut Vec<OldMove>, expc_arr: &mut Vec<OldMove>) {
    for i in 0..eng_arr.len() {
        //let mut board2 = board.clone();

        //Would be better to add proper check, but would need to change all fen values in all test
        eng_arr[i].board.castling &= 0b0000_1111;
        eng_arr[i].promote_to = None;
        eng_arr[i].capture = false;
        // eng_arr[i].board.last_move_info = vec![];
        eng_arr[i].board.capture = None;

        //eng_arr[i].board.zobrist_hash = 0;
    }

    for i in 0..expc_arr.len() {
        //let (from, to, board) = m;
        //let mut board2 = board.clone();
        expc_arr[i].board.castling &= 0b0000_1111;

    }

    assert_eq!(eng_arr, expc_arr);
}

pub fn moves_to_oldmoves(eng: &Engine, init_board: &Board, moves: Vec<Move>) -> Vec<OldMove> {
    let mut oldmoves: Vec<OldMove> = vec![];
    for m in moves {
        let mut mboard = init_board.clone();
        mboard.make_move(eng, m);

        oldmoves.push(OldMove {from: m.from, to: m.to, board: mboard, promote_to: m.promote_to, capture: false } );
    }
    return oldmoves;
}

pub fn gen_oldmoves(eng: &Engine, board: &Board) -> Vec<OldMove> {
    let moves: Vec<Move> = eng.gen_moves(board);
    return moves_to_oldmoves(eng, board, moves);
}
