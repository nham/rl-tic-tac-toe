use std::collections::HashMap;

use game::{Action, GameState};
use super::PlayerId;

pub trait Player {
    fn new(id: PlayerId) -> Self where Self: Sized;
    fn choose_action(&self, state: &GameState) -> (usize, usize);
}


struct RLPlayer {
    player_id: PlayerId,
    estimates: HashMap<GameState, f64>,
}

impl Player for RLPlayer {
    fn new(id: PlayerId) -> RLPlayer {
        let mut est = HashMap::new();
        RLPlayer {
            player_id: id,
            estimates: est,
        }
    }

    fn choose_action(&self, state: &GameState) -> (usize, usize) {
        (0, 0)
    }

    fn lookup_estimate(&self, state: &GameState) -> f64 {
    }
}
