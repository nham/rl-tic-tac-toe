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
        unimplemented!()
    }
}
