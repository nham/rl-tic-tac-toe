use std::collections::HashMap;

use game::{Action, GameState};
use super::PlayerId;

pub struct RLPlayer {
    player_id: PlayerId,
    estimates: HashMap<GameState, f64>,
}

impl RLPlayer {
    fn new(id: PlayerId) -> RLPlayer {
        let mut est = HashMap::new();
        RLPlayer {
            player_id: id,
            estimates: est,
        }
    }

    pub fn choose_action(&self, state: &GameState) -> (usize, usize) {
        unimplemented!()
    }

    fn lookup_estimate(&self, state: &GameState) -> f64 {
        if state.is_won_by(self.player_id) {
            1.
        } else if state.is_won_by(self.player_id.next()) {
            0.
        } else {
            // should check hashmap first tho
            0.5
        }
    }
}
