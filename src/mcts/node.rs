use crate::board::{Board, GameState, Move};

#[derive(Debug)]
pub struct Node {
    pub state: GameState,
    pub moves: Move,
    pub moves_to_explore: Move,
    pub visits: u32,
    pub rewards: f32,
    pub index: usize,
    pub children_indices: [usize; 4],
}

impl Node {
    pub fn new(board: &Board) -> Self {
        let legal_moves = board.get_legal_moves();
        Self {
            state: board.get_game_state(),
            moves: legal_moves,
            moves_to_explore: legal_moves.count_ones() as Move,
            visits: 0,
            rewards: 0.0,
            index: 0,
            children_indices: [usize::MAX; 4],
        }
    }
}
