use std::collections::HashMap;

use game::{Action, GameState};
use super::PlayerId;

pub struct RLPlayer {
    player_id: PlayerId,
    estimates: HashMap<GameState, f64>,
    epsilon: f64, // for small chance of non-greedy move
}

impl RLPlayer {
    fn new(id: PlayerId, eps: f64) -> RLPlayer {
        let mut est = HashMap::new();
        RLPlayer {
            player_id: id,
            estimates: est,
            epsilon: eps,
        }
    }

    pub fn choose_action(&self, state: &GameState) -> (usize, usize) {
        unimplemented!()
    }

    fn lookup_estimate(&self, state: &GameState) -> f64 {
        // if it's in hashmap, assume it's up to date and use it.

        if state.is_won_by(self.player_id) {
            1.
        } else if state.is_won_by(self.player_id.next()) {
            0.
        } else {
            0.5
        }
    }
}
