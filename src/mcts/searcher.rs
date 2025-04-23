use std::{
    sync::atomic::AtomicBool,
    time::{Duration, Instant},
};

use rand::Rng;

use crate::board::{Board, GameState, Move, MOVE_DOWN, MOVE_LEFT, MOVE_RIGHT, MOVE_UP};

use super::node::Node;

pub struct Searcher<'a> {
    board: Board,
    start_time: Instant,
    time_limit: Duration,
    stopped: &'a AtomicBool,
    tree: Vec<Node>,
    selection_line: Vec<usize>,
    history: Vec<Board>,
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
            history: vec![],
        }
    }

    fn time_spent(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn search(&mut self) -> Move {
        loop {
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

        let root = &self.tree[0];
        let mut best_move = 0;
        let mut best_score = f32::MIN;
        println!("{:?}", root.children_indices);
        for i in 0..4 {
            // Illegal/unexplored move
            if 1 << i & root.moves == 0 || root.children_indices[i] == usize::MAX {
                continue;
            }

            let node = &self.tree[root.children_indices[i]];
            let score = node.rewards;
            if score > best_score {
                best_score = score;
                best_move = 1 << i;
            }
        }

        println!(
            "{0}",
            match best_move {
                MOVE_LEFT => "bestmove l",
                MOVE_RIGHT => "bestmove r",
                MOVE_UP => "bestmove u",
                MOVE_DOWN => "bestmove d",
                _ => "",
            }
        );
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

                println!("{:?}, {:?}, {:?}", i, best_move_shift, best_score);
                let score = self.ucb1(&node, i);
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

            self.selection_line.push(next_idx);
            self.history.push(self.board.clone());
            self.board.apply_move(1 << best_move_shift);
            node_idx = next_idx;
        }

        Some(node_idx)
    }

    fn expand(&mut self, node_idx: usize) {
        let node = &mut self.tree[node_idx];
        let unexplored: Vec<_> = (0..4)
            .filter(|&i| node.moves_to_explore & (1 << i) != 0)
            .collect();
        if unexplored.is_empty() {
            return;
        }
        let move_shift = unexplored[rand::thread_rng().gen_range(0..unexplored.len())];
        let move_to_expand = 1 << move_shift;

        // Mark as already explored
        node.moves_to_explore &= !move_to_expand;

        self.history.push(self.board.clone());
        self.board.apply_move(move_to_expand);
        self.board.add_random_tile();

        let new_node_idx = self.tree.len();
        self.tree.push(Node::new(&self.board));
        self.tree[node_idx].children_indices[move_shift] = new_node_idx;

        let idx_to_explore = self.tree[node_idx].children_indices[move_shift];
        self.tree[idx_to_explore].index = self.tree.len() - 1;
    }

    fn rollout(&mut self) -> f32 {
        let state = self.board.get_game_state();

        if state == GameState::Lost || self.history.len() > 30 {
            return self.board.evaluate()
        }

        let legal: Vec<_> = (0..4)
            .filter(|&i| self.board.get_legal_moves() & (1 << i) != 0)
            .collect();
        let mv = 1 << legal[rand::thread_rng().gen_range(0..legal.len())];

        self.history.push(self.board.clone());
        self.board.apply_move(mv);
        self.board.add_random_tile();
        self.rollout()
    }

    fn backpropagate(&mut self, result: f32) {
        while let Some(node_idx) = self.selection_line.pop() {
            let node = &mut self.tree[node_idx];
            node.rewards += result;
            node.visits += 1;
            self.board = self.history.pop().unwrap();
        }
    }

    fn ucb1(&self, node: &Node, move_shift: usize) -> f32 {
        let child_idx = node.children_indices[move_shift];
        if child_idx == usize::MAX {
            return 500000.0;
        }

        let child = &self.tree[child_idx];
        if node.visits == 0 || child.visits == 0 {
            return 500000.0;
        }

        child.rewards / child.visits as f32
            + f32::sqrt(2.0 * f32::ln(node.visits as f32) / child.visits as f32)
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
