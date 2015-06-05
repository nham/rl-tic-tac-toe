use rand;
use rand::distributions::{IndependentSample, Range};
use std::collections::HashMap;

use game::Board;
use super::PlayerId;

pub struct RLPlayer {
    player_id: PlayerId,
    estimates: HashMap<Board, f64>,
    epsilon: f64, // for small chance of non-greedy move
    rng: rand::ThreadRng,
}

type CellCoords = (usize, usize);

impl RLPlayer {
    pub fn new(id: PlayerId, eps: f64) -> RLPlayer {
        RLPlayer {
            player_id: id,
            estimates: HashMap::new(),
            epsilon: eps,
            rng: rand::thread_rng(),
        }
    }

    pub fn choose_action(&mut self, state: &Board) -> Option<CellCoords> {
        let between = Range::new(0., 1.);
        let k = between.ind_sample(&mut self.rng);
        if k < self.epsilon {
            debug!("Player {:?} -- exploratory action", self.player_id);
            self.exploratory_action(state)
        } else {
            debug!("Player {:?} -- greedy action", self.player_id);
            self.greedy_action(state)
        }
    }

    //
    fn exploratory_action(&mut self, state: &Board) -> Option<CellCoords> {
        let mut max_val = ::std::f64::MIN;
        let mut actions_values = Vec::new();
        let mut all_same_value = true;
        for (i, j) in state.available_choices() {
            let mut candidate = state.clone();
            candidate.set_cell(i, j, self.player_id.as_cell());

            let estimate = self.estimate(candidate);
            actions_values.push( (estimate, (i, j)) );

            if estimate > max_val {
                max_val = estimate;
                all_same_value = false;
            }
        }

        if !all_same_value {
            actions_values = actions_values.into_iter()
                                           .filter(|x| x.0 == max_val)
                                           .collect::<Vec<_>>();
        }

        // choose random element in actions_values to return
        let between = Range::new(0, actions_values.len());
        let k = between.ind_sample(&mut self.rng);
        debug!("{} to choose from, chose k = {}", actions_values.len(), k);
        Some(actions_values[k].1)
    }

    fn greedy_action(&mut self, state: &Board) -> Option<CellCoords> {
        let mut max_val = ::std::f64::MIN;
        let mut max_action: Option<(usize, usize)> = None;
        for (i, j) in state.available_choices() {
            let mut candidate = state.clone();
            candidate.set_cell(i, j, self.player_id.as_cell());

            let estimate = self.estimate(candidate);

            if estimate > max_val {
                max_val = estimate;
                max_action = Some((i, j));
            }
        }

        match max_action {
            Some(action) => Some(action),
            None => None,
        }
    }

    fn estimate(&mut self, state: Board) -> f64 {
        match self.lookup_estimate(&state) {
            (val, true) => val,
            (val, false) => {
                self.add_estimate(state.clone(), val);
                val
            },
        }
    }

    // (estimate, whether it's in the hash map)
    fn lookup_estimate(&self, state: &Board) -> (f64, bool) {
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

    fn add_estimate(&mut self, state: Board, value: f64) {
        self.estimates.insert(state, value);
    }

    pub fn update_estimates(&mut self) {
        // need a history, 
    }
}
