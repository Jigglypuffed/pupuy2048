use std::{process::exit, sync::{atomic::AtomicBool, Arc}};

use board::Board;
use interface::*;

mod board;
mod interface;
mod mcts;

fn main() {
    let mut board = Board::new([[0; 4]; 4]);
    let stopped = Arc::new(AtomicBool::new(false));
    let mut mst = std::thread::spawn(|| {});
    loop {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");
        let tokens: Vec<_> = buf.split_ascii_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }
        let cmd = &tokens[0];
        let params = &tokens[1..];
        match cmd.to_owned() {
            "newgame" => command_newgame(&mut board),
            "isready" => command_is_ready(),
            "move" => command_move(params, &mut board),
            "add" => command_add(params, &mut board),
            "go" => command_go(params, &mut board, Arc::clone(&stopped), &mut mst),
            "position" => command_position(params, &mut board),
            "stop" => command_stop(Arc::clone(&stopped)),
            "exit" => exit(0),
            _ => continue,
        }
    }
}
