use std::{
    sync::atomic::AtomicBool,
    time::{Duration, Instant},
};

use crate::board::{Board, GameState, Move};

use super::node::Node;

pub struct Searcher<'a> {
    board: Board,
    start_time: Instant,
    time_limit: Duration,
    stopped: &'a AtomicBool,
    tree: Vec<Node>,
    selection_line: Vec<usize>,
}

impl<'a> Searcher<'a> {
    pub fn new(board: Board, move_time: u64, stopped: &'a AtomicBool) -> Self {
        Searcher {
            board,
            start_time: Instant::now(),
            time_limit: Duration::from_millis(move_time),
            stopped,
            tree: vec![Node::new(&board)],
            selection_line: vec![],
        }
    }

    fn time_spent(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn search(&mut self) -> Move {
        let best_move = 0;
        for _ in 0..30 {
            if let Some(selected_node_idx) = self.select() {
                if self.tree[selected_node_idx].state != GameState::Lost {
                    self.expand(selected_node_idx);
                }
            }

            let result = self.rollout();
            self.backpropagate(result);

            if self.time_spent() > self.time_limit
                || self.stopped.load(std::sync::atomic::Ordering::Relaxed)
            {
                break;
            }
        }
        best_move
    }

    fn select(&mut self) -> Option<usize> {
        self.selection_line.clear();
        let mut node_idx = 0;
        loop {
            let node = &self.tree[node_idx];

            if node.state == GameState::Lost {
                return None;
            }

            let mut best_move_shift = 0;
            let mut best_score = 0.0;

            for i in 0..4 {
                if 1 << i & node.moves == 0 {
                    continue;
                }

                let score = 0.0;
                if score > best_score {
                    best_score = score;
                    best_move_shift = i;
                }
            }

            let next_idx: usize = node.children_indices[best_move_shift];

            // This node isn't expanded
            if next_idx == usize::MAX {
                break;
            }

            self.selection_line.push(best_move_shift);
            self.board.apply_move(1 << best_move_shift);
            node_idx = node.children_indices[best_move_shift.trailing_zeros() as usize]
        }

        Some(node_idx)
    }

    fn expand(&mut self, node_idx: usize) {
        todo!()
    }

    fn rollout(&mut self) -> f32 {
        todo!()
    }

    fn backpropagate(&mut self, result: f32) {
        todo!()
    }
}

#[test]
fn selection_test() {
    let stopped = AtomicBool::new(false);
    let mut a = Searcher::new(Board::from_fen("0000/1211/2232/2222"), 3000, &stopped);
    let selected_idx = a.select();
    assert_eq!(selected_idx, Some(0));
    assert_eq!(a.selection_line, [])
}
