use std::collections::HashMap;
use crate::chicory::board::{Board, PieceColor};
use crate::chicory::engine::Engine;
use crate::chicory::tables::Entry;
use std::sync::Arc;
use std::sync::Mutex;

type TimeInfo = [Option<u128>; 2];

#[derive(Debug, Clone, PartialEq)]
pub enum Cmd {
    Set(Board),
    Stop,
    Go(SearchInfo),
    Perft(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchInfo {
    pub current_time: TimeInfo,
    pub per_move_time: TimeInfo,
    pub depth: Option<usize>,
    pub nodes: Option<usize>,
    pub movetime: Option<u128>,
    pub infinite: bool,
}

pub struct UciInterface {
    pub current_board: Arc<Mutex<Option<Board>>>,
    //current_move: usize,
    pub max_search_depth: usize,
    engine: Arc<Engine>,

    pub positions_reached: Arc<Mutex<HashMap<u64, usize>>>,
    pub transposition_table: Arc<Mutex<HashMap<u64, Entry>>>
}
impl UciInterface {
    pub fn new(engine: Arc<Engine>) -> Self {
        UciInterface {
            current_board: Arc::new(Mutex::new(None)),
            max_search_depth: 99,
            engine: engine,

            positions_reached: Arc::new(Mutex::new(HashMap::new())),
            transposition_table: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn uci(&mut self) -> Option<Cmd> {
        let name = env!("CARGO_PKG_NAME");
        let authors = env!("CARGO_PKG_AUTHORS");
        let version = env!("CARGO_PKG_VERSION");
        println!("id name {} {}", name, version);
        println!("id author {}", authors);
        println!(
            "option name MaxSearchDepth type spin default {} min 1 max 99",
            self.max_search_depth
        );
        println!("uciok");

        None
    }

    pub fn isready(&mut self) -> Option<Cmd> {
        println!("readyok");

        None
    }

    pub fn position(&mut self, command: Vec<&str>) -> Option<Cmd> {
        let mut i = 1;

        let mut cur_board = self.current_board.lock().unwrap().clone();

            if command[i] == "startpos" {
                cur_board = Some(Board::new(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                    &self.engine,
                ));

                i += 1;
            } else {
                i += 1;
                let mut fen_tokens = vec![];

                //collect all of fen string
                while i < command.len() {
                    if command[i] == "moves" {
                        break;
                    } else {
                        fen_tokens.push(command[i]);
                        i += 1;
                    }
                }

                //println!("FEN||{:?}||", &fen_tokens.join(" "));
                cur_board = Some(Board::new(&fen_tokens.join(" "), &self.engine));
            }

            if i < command.len() && command[i] == "moves" {
                i += 1;
                while i < command.len() {
                    cur_board = Some(
                        cur_board
                            .unwrap()
                            .make_move(&command[i], &self.engine),
                    );

                    i += 1;
                }
            }

        //println!("READ BOARD AS||{:?}||", cur_board);
        *self.current_board.lock().unwrap() = cur_board;

        Some(Cmd::Set(cur_board.unwrap()))
    }

    pub fn go(&mut self, command: Vec<&str>) -> Option<Cmd> {
        let mut search_info = SearchInfo {
            current_time: [None, None],
            per_move_time: [None, None],
            depth: None,
            nodes: None,
            movetime: None,
            infinite: false,
        };

        let mut i = 1;

        while (i + 1) < command.len() {
            match command[i + 1].parse::<u128>() {
                Ok(val) => {
                    match command[i] {
                        "wtime" => search_info.current_time[PieceColor::White] = Some(val),
                        "btime" => search_info.current_time[PieceColor::Black] = Some(val),
                        "winc" => search_info.per_move_time[PieceColor::White] = Some(val),
                        "binc" => search_info.per_move_time[PieceColor::Black] = Some(val),
                        "depth" => search_info.depth = Some(val as usize), //search x plies only.
                        "nodes" => search_info.nodes = Some(val as usize), //search x nodes only,
                        "movetime" => search_info.movetime = Some(val), //search exactly x mseconds
                        _ => {}
                    }
                    i += 2;
                }
                Err(_) => {
                    if command[i] == "infinite" {
                        search_info.infinite = true;
                    }
                    i += 1;
                }
            }
        }

        Some(Cmd::Go(search_info))
    }
    pub fn uci_new_game(&mut self) -> Option<Cmd> {
        *self.current_board.lock().unwrap() = None;

        None
    }

    pub fn stop(&mut self) -> Option<Cmd> {
        Some(Cmd::Stop)
    }

    pub fn quit(&mut self) -> Option<Cmd> {
        std::process::exit(0);
    }

    pub fn set_option(&mut self, command: Vec<&str>) -> Option<Cmd> {
        if command[1] == "name" {
            match command[2] {
                "MaxSearchDepth" => {
                    if command[3] == "value" {
                        self.max_search_depth = command[4].parse().unwrap();
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn perft(&mut self, command: Vec<&str>) -> Option<Cmd> {
        let depth = command[1].parse::<usize>().unwrap();

        Some(Cmd::Perft(depth))
    }
}
