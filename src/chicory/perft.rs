use crate::chicory::board::Board;
use crate::chicory::engine::Engine;
use std::sync::mpsc;
use std::thread;
pub fn perft(eng: &Engine, board: Board, depth: usize) -> usize {
    let mut count = 0;

    if depth == 0 {
        return 1;
    }

    let moves = eng.gen_moves(board);
    for m in moves {
        count += perft(eng, m.board, depth - 1);
    }

    count
}
#[allow(dead_code)]
pub fn multi_perft(eng: &Engine, board: Board, depth: usize, thread_count: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let moves = eng.gen_moves(board);

    if depth == 1 {
        return moves.len();
    }

    let mut chunks = vec![vec!(); thread_count];

    let group = moves.len() / thread_count;
    let group_r = moves.len() % thread_count;

    for i in 0..chunks.len() {
        chunks[i] = moves[(group * i)..(group * (i + 1))].to_vec();
    }

    if group_r != 0 {
        chunks[thread_count - 1] =
            moves[(group * (chunks.len() - 1))..((group * (chunks.len())) + group_r)].to_vec();
    }

    let (tx, rx) = mpsc::channel();

    thread::scope(|s| {
        for n in 0..thread_count {
            let set = chunks[n].clone();
            let ctx = tx.clone();
            s.spawn(move || {
                let mut count = 0;

                for m in set {
                    count += perft(eng, m.board, depth - 1);
                }
                ctx.send(count).unwrap();
            });
        }
    });

    let mut sum = 0;
    let mut rec_count = 0;
    for received in rx {
        sum += received;
        rec_count += 1;

        if rec_count >= thread_count {
            break;
        }
    }

    sum
}

pub fn multi_perft_list(
    eng: &Engine,
    board: Board,
    depth: usize,
    thread_count: usize,
) -> (usize, Vec<(String, usize)>) {
    //let mut results: HashMap<String, usize> = HashMap::new();
    let mut result = vec![];
    let mut total = 0;

    let moves = eng.gen_moves(board);
    for m in moves {

        let child_total = multi_perft(&eng, m.board, depth - 1, thread_count);
        result.push((Board::move_to_lan(&m), child_total));
        total += child_total;
    }

    (total, result)
}
