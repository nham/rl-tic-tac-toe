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

    fn greedy_action(&self, state: &GameState) -> Option<(usize, usize)> {
        let max_val = ::std::f64::MIN;
        let mut max_action: Option<(usize, usize)> = None;
        for (i, j) in state.available_choices() {
            let mut candidate = state.clone();
            candidate.act_upon(&(i, j, self.player_id.as_cellstate()));

            if self.estimate(candidate) > max_val {
                max_action = Some((i, j));
            }
        }
    }

    fn estimate(&mut self, state: GameState) -> f64 {
        match self.lookup_estimate(&state) {
            (val, true) => val,
            (val, false) => {
                self.add_estimate(state.clone(), val);
                val
            },
        }
    }

    // (estimate, whether it's in the hash map)
    fn lookup_estimate(&self, state: &GameState) -> (f64, bool) {
        // if it's in hashmap, assume it's up to date and use it.
        if let Some(&value) = self.estimates.get(state) {
            (value, true)
        } else if state.is_won_by(self.player_id) {
            (1., false)
        } else if state.is_won_by(self.player_id.next()) {
            (0., false)
        } else {
            (0.5, false)
        }
    }

    fn add_estimate(&mut self, state: GameState, value: f64) {
        self.estimates.insert(state, value);
    }
}
