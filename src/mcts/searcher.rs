use std::{
    sync::atomic::AtomicBool,
    time::{Duration, Instant},
};

use crate::board::{Board, MOVE_DOWN, MOVE_LEFT, MOVE_RIGHT, MOVE_UP};

pub struct Searcher<'a> {
    root_pos: Board,
    start_time: Instant,
    time_limit: Duration,
    stopped: &'a AtomicBool,
}

impl<'a> Searcher<'a> {
    pub fn new(board: Board, move_time: u64, stopped: &'a AtomicBool) -> Self {
        Searcher {
            root_pos: board,
            start_time: Instant::now(),
            time_limit: Duration::from_millis(move_time),
            stopped,
        }
    }

    fn time_spent(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn search(&mut self) -> u8 {
        todo!()
    }
}
