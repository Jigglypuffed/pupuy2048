use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use crate::{
    board::Board,
    mcts::searcher::Searcher,
};

pub fn command_newgame(board: &mut Board) {
    *board = Board::new([[0; 4]; 4])
}

pub fn command_is_ready() {
    println!("readyok")
}

pub fn command_position(params: &[&str], board: &mut Board) {
    if let Some(fen) = params.get(0) {
        *board = Board::from_fen(fen)
    } else {
        panic!("Invalid command format")
    };
    println!("{:?}", board);
}

pub fn command_move(params: &[&str], board: &mut Board) {
    match params.get(0) {
        Some(&"l") => {
            board.move_left();
        }
        Some(&"r") => {
            board.move_right();
        }
        Some(&"u") => {
            board.move_up();
        }
        Some(&"d") => {
            board.move_down();
        }
        _ => panic!("Invalid command format"),
    }
    println!("{:?}", board);
}

pub fn command_add(params: &[&str], board: &mut Board) {
    let sq = match params[0] {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "a" => 10,
        "b" => 11,
        "c" => 12,
        "d" => 13,
        "e" => 14,
        "f" => 15,
        _ => panic!(),
    };
    match params[1] {
        "0" => board.add_tile(sq, 0),
        "1" => board.add_tile(sq, 1),
        "2" => board.add_tile(sq, 2),
        "3" => board.add_tile(sq, 3),
        "4" => board.add_tile(sq, 4),
        "5" => board.add_tile(sq, 5),
        "6" => board.add_tile(sq, 6),
        "7" => board.add_tile(sq, 7),
        "8" => board.add_tile(sq, 8),
        "9" => board.add_tile(sq, 9),
        "a" => board.add_tile(sq, 10),
        "b" => board.add_tile(sq, 11),
        "c" => board.add_tile(sq, 12),
        "d" => board.add_tile(sq, 13),
        "e" => board.add_tile(sq, 14),
        "f" => board.add_tile(sq, 15),
        _ => panic!(),
    };
}

pub fn command_go(
    params: &[&str],
    board: &Board,
    stopped: Arc<AtomicBool>,
    mst: &mut JoinHandle<()>,
) {
    stopped.store(false, Ordering::Relaxed);

    let mut move_time = u64::MAX;

    let mut params = params.iter();
    while let Some(p) = params.next() {
        match *p {
            "time" => {
                move_time = params
                    .next()
                    .unwrap()
                    .parse::<u64>()
                    .expect("Incorrect time format")
            }
            _ => {}
        }
    }

    let board = board.clone();
    let stopped = Arc::clone(&stopped);

    *mst = thread::spawn(move || {
        Searcher::new(board, move_time, &*stopped).search();
    });
}

pub fn command_stop(stopped: Arc<AtomicBool>) {
    stopped.store(true, std::sync::atomic::Ordering::Relaxed);
}
