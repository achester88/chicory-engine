mod chicory;

use crate::chicory::board::PieceColor;
use crate::chicory::perft::multi_perft_list;
use board::*;
use chicory::eval::eval;
use chicory::*;
use engine::*;
use eval::minmax;
use std::io::{stdin, stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use uci_interface::*;

//use std::fs::OpenOptions;
//use std::io::prelude::*;
//use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let engine = Arc::new(Engine::new()); //replace with ref or something :(
    let stop_calculation = Arc::new(AtomicBool::new(false));
    let finished_calculation = Arc::new(AtomicBool::new(false));

    let mut interface = UciInterface::new(engine.clone());

    //let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    //let log_name = current_time.to_string() + " _log.txt";

    /*
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true) // Creates the file if it doesn't exist
        .open(log_name).unwrap();
    */

    loop {
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let commands: Vec<&str> = input.trim_end().split(" ").collect();

        let cmd_out = match commands[0] {
            "uci" => interface.uci(),
            "isready" => interface.isready(),
            "position" => interface.position(commands),
            "go" => interface.go(commands),
            "ucinewgame" => interface.uci_new_game(),
            "stop" => interface.stop(),
            "quit" => interface.quit(),
            "setoption" => interface.set_option(commands),
            "perft" => interface.perft(commands),
            _ => None,
        };

        if cmd_out.is_some() {
            match cmd_out.unwrap() {
                Cmd::Go(search_info) => {
                    let stop_calculation_clone = Arc::clone(&stop_calculation);
                    let finished_calculation_clone = Arc::clone(&finished_calculation);
                    let eng = engine.clone();
                    let board_ref = interface.current_board.clone();
                    let positions_reached_ref = interface.positions_reached.clone();
                    let transposition_table_ref = interface.transposition_table.clone();

                    thread::spawn(move || {
                        let move_timer = Instant::now();
                        let board = { board_ref.lock().unwrap().clone() };
                        let cal_board = board.unwrap();
                        let mut positions_reached = positions_reached_ref.lock().unwrap();
                        let mut transposition_table = transposition_table_ref.lock().unwrap();

                        //println!("{:?}", cal_board);

                        let mut time_per_move = 0.0;

                        let (current_time, per_move_time): (Option<f64>, Option<f64>);

                        match cal_board.turn {
                            PieceColor::White => {
                                (current_time, per_move_time) = (
                                    search_info.current_time[PieceColor::White],
                                    search_info.per_move_time[PieceColor::White],
                                );
                            }
                            PieceColor::Black => {
                                (current_time, per_move_time) = (
                                    search_info.current_time[PieceColor::Black],
                                    search_info.per_move_time[PieceColor::Black],
                                );
                            }
                        }

                        if search_info.movetime.is_some() {
                            time_per_move = search_info.movetime.unwrap();
                        } else if current_time.is_some() {
                            time_per_move =
                                (current_time.unwrap() / 20.0) + (per_move_time.unwrap_or(0.0) / 2.0);
                            // base / 20 + increment / 2
                        }

                        let mut cur_best_move = None;

                        let stop_depth: usize;

                        if search_info.depth.is_some() {
                            stop_depth = search_info.depth.unwrap();
                        } else {
                            stop_depth = interface.max_search_depth
                        }

                        let mut depth = 1;
                        while !&stop_calculation_clone.load(Ordering::Relaxed)
                            && (stop_depth == 0 || depth <= stop_depth || time_per_move == 0.0)
                        {
                            let (_, best_move, _, _) = minmax(
                                &eng,
                                cal_board,
                                depth,
                                i32::MIN,
                                i32::MAX,
                                cal_board.turn,
                                1,
                                &stop_calculation_clone,
                                time_per_move,
                                move_timer,
                                &mut positions_reached,
                                &mut transposition_table,
                                cur_best_move,
                                true,
                                false
                            );

                            if time_per_move != 0.0
                                && (move_timer.elapsed().as_millis() as f64) > time_per_move
                            {
                                //break;
                                if depth == 1 {
                                    cur_best_move = best_move;//We need some move
                                } else {
                                    break;
                                }
                            } else {
                                cur_best_move = best_move;
                            }

                            depth += 1;
                        }

                        finished_calculation_clone.store(true, Ordering::Relaxed);

                        //let (_, _, board, _) = cur_best_move.unwrap(); //*best_move_lock;
                        println!("bestmove {}", Board::move_to_lan(&cur_best_move.unwrap()));
                        *board_ref.lock().unwrap() = Some(cur_best_move.unwrap().board);

                        //let zh = cur_best_move.unwrap().board.zobrist_hash;
                        
                        /*
                        if positions_reached.contains_key(&zh) {
                            let current_count = *positions_reached.get(&zh).unwrap();
                            positions_reached.insert(zh, current_count + 1);
                        } else {
                            positions_reached.insert(zh, 1);
                        }
                        */

                        //println!("info string HM: {:?}", positions_reached);

                        stop_calculation_clone.store(false, Ordering::Relaxed);
                    });
                }
                Cmd::Set(board) => { //
                    //writeln!(file, "----POS READ AS: {:?}", board);
                    println!("info score cp {}", if board.turn == PieceColor::White {eval(&board)} else {-eval(&board)});
                }

                Cmd::Stop => {
                    stop_calculation.store(true, Ordering::Relaxed);
                }
                Cmd::Perft(depth) => {
                    let eng = engine.clone();

                    let board_ref = interface.current_board.clone();

                    thread::spawn(move || {
                        let board = { board_ref.lock().unwrap().clone() };
                        let cal_board = board.unwrap();

                        let time = Instant::now();
                        let (count, list) = multi_perft_list(&eng, cal_board, depth, 8);
                        let stop_time = time.elapsed().as_millis();

                        for (moves, nodes) in list {
                            println!("{}:  {}", moves, nodes);
                        }
                        println!("\n-----------------------------------\n");
                        println!("Total Nodes   : {}", count);
                        println!("Total Time    : {}(s)", stop_time / 1000);
                        println!(
                            "Nodes per Sec : {},",
                            (count as f64 / stop_time as f64) * 1000.0
                        );
                    });
                },
            }
        }
    }
}
