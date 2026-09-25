use chicory::chicory::board::Board;
use chicory::chicory::engine::Engine;
use chicory::chicory::perft::*;
use std::collections::HashMap;

const THREAD_COUNT: usize = 8;

#[allow(dead_code)]

fn perft_from_string(eng: &Engine, board: Board, list: String, depth: usize) -> bool {
    let lines: Vec<&str> = list.split("\n").collect();

    let mut expected: HashMap<String, usize> = HashMap::new();
    let mut results: HashMap<String, usize> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        expected.insert(parts[0].to_string(), parts[1].parse::<usize>().unwrap());
    }

    let moves = eng.gen_moves(board);
    for m in moves {
        results.insert(
            Board::move_to_lan(&m),
            multi_perft(&eng, m.board, depth - 1, THREAD_COUNT),
        );
    }

    let mut pass = true;

    for key in results.keys() {
        let result = results.get(key).unwrap();

        match expected.get(key) {
            Some(x) => {
                if result == x {
                    println!("{} | ==", key);
                } else {
                    pass = false;
                    println!("{} | E: {}, R: {}", key, x, result);
                }
                expected.remove(key);
            }
            None => {
                pass = false;
                println!("{} | Additional", key);
            }
        }
    }

    for key in expected.keys() {
        pass = false;
        print!("{} | Missing ", key);
    }

    pass
}

#[test]
fn ip_one() {
    let engine = Engine::new();
    let board = Board::new(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        &engine,
    );
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 20);
}

#[test]
fn ip_two() {
    let engine = Engine::new();
    let board = Board::new(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        &engine,
    );
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 400);
}

#[test]
fn ip_three() {
    let engine = Engine::new();
    let board = Board::new(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        &engine,
    );
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 8902);
}

#[test]
fn ip_four() {
    let engine = Engine::new();
    let board = Board::new(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        &engine,
    );
    let count = multi_perft(&engine, board, 4, THREAD_COUNT);

    assert_eq!(count, 197281);
}

#[test]
fn ip_five() {
    let engine = Engine::new();
    let board = Board::new(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        &engine,
    );
    let count = multi_perft(&engine, board, 5, THREAD_COUNT);

    assert_eq!(count, 4865609);
}

#[test]
fn p2_one() {
    let engine = Engine::new();
    let board = Board::new(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1",
        &engine,
    );
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 48);
}

#[test]
fn p2_two() {
    let engine = Engine::new();
    let board = Board::new(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1",
        &engine,
    );
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 2039);
}

#[test]
fn p2_three() {
    let engine = Engine::new();
    let board = Board::new(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1",
        &engine,
    );
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 97862);
}

#[test]
fn p3_one() {
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 14);
}

#[test]
fn p3_two() {
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 191);
}

#[test]
fn p3_three() {
    //E.P?
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 2812);
}

#[test]
fn p4_one() {
    let engine = Engine::new();
    let board = Board::new(
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        &engine,
    );
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 6);
}

#[test]
fn p4_two() {
    let engine = Engine::new();
    let board = Board::new(
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        &engine,
    );
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 264);
}

#[test]
fn p4_three() {
    let engine = Engine::new();
    let board = Board::new(
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        &engine,
    );
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 9467);
}

#[test]
fn p5_one() {
    let engine = Engine::new();
    let board = Board::new(
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        &engine,
    );
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 44);
}

#[test]
fn p5_two() {
    let engine = Engine::new();
    let board = Board::new(
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        &engine,
    );
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 1486);
}

#[test]
fn p5_three() {
    let engine = Engine::new();
    let board = Board::new(
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        &engine,
    );
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 62379);
}

#[test]
fn p6_one() {
    let engine = Engine::new();
    let board = Board::new(
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        &engine,
    );
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 46);
}

#[test]
fn p6_two() {
    let engine = Engine::new();
    let board = Board::new(
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        &engine,
    );
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 2079);
}

#[test]
fn p6_three() {
    let engine = Engine::new();
    let board = Board::new(
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        &engine,
    );
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 89890);
}

#[test]
fn p6_four() {
    let engine = Engine::new();
    let board = Board::new(
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        &engine,
    );
    let count = multi_perft(&engine, board, 4, THREAD_COUNT);

    assert_eq!(count, 3894594);
}
