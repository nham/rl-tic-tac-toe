use rand;
use rand::distributions::{IndependentSample, Range};
use std::collections::{HashMap, hash_map};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

use game::Board;
use super::PlayerId;

const ALPHA: f64 = 0.1;

// temporal difference learner
pub struct RLPlayer {
    player_id: PlayerId,
    estimates: HashMap<Board, f64>,
    pub epsilon: f64, // for small chance of non-greedy move
    rng: rand::ThreadRng,
    pub alpha: f64, // "step size parameter"
}

type CellCoords = (usize, usize);

impl RLPlayer {
    pub fn new(id: PlayerId, eps: f64) -> RLPlayer {
        RLPlayer {
            player_id: id,
            estimates: HashMap::new(),
            epsilon: eps,
            rng: rand::thread_rng(),
            alpha: ALPHA,
        }
    }

    pub fn from_file(id: PlayerId) -> io::Result<RLPlayer> {
        let f = try!(File::open(super::ESTIMATES_FNAME));
        let mut file = BufReader::new(&f);

        let mut first = String::new();
        try!(file.read_line(&mut first));
        let epsilon: f64 = FromStr::from_str( (&first).trim() ).unwrap();

        let mut second = String::new();
        try!(file.read_line(&mut second));
        let alpha: f64 = FromStr::from_str( (&second).trim() ).unwrap();

        let mut map = HashMap::new();

        for line in file.lines() {
            let line_str = match line {
                Ok(s) => s,
                Err(e) => return Err(e),
            };

            let mut segments = line_str.split(' ');
            let board: Board = match segments.next() {
                Some(s) => FromStr::from_str(s).unwrap(),
                _ => panic!("iono wat"),
            };

            let val: f64 = match segments.next() {
                Some(s) => FromStr::from_str(s).unwrap(),
                _ => panic!("iono wat"),
            };

            map.insert(board, val);
        }

        println!("first line is: {}", first);
        println!("second line is: {}", second);

        Ok(RLPlayer {
            player_id: id,
            estimates: map,
            epsilon: epsilon,
            rng: rand::thread_rng(),
            alpha: alpha,
        })
    }

    //
    fn exploratory_action(&mut self, state: &Board) -> Option<CellCoords> {
        let mut max_val = ::std::f64::MIN;
        let mut actions_values = Vec::new();
        let mut all_same_value = true;
        debug!("STARTING loop exploratory_action");
        for (i, j) in state.available_choices() {
            let mut candidate = state.clone();
            candidate.set_cell(i, j, self.player_id.as_cell());

            let estimate = self.estimate_and_add(candidate);
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
        debug!("exploratory action: {} to choose from, chose {:?} (k = {})", actions_values.len(), actions_values[k].1, k);
        Some(actions_values[k].1)
    }

    fn greedy_action(&mut self, state: &Board) -> Option<CellCoords> {
        let mut max_val = ::std::f64::MIN;
        let mut max_action: Option<(usize, usize)> = None;
        debug!("\nSTARTING loop greedy_action\n======");
        self.print_estimates();
        for (i, j) in state.available_choices() {
            let mut candidate = state.clone();
            candidate.set_cell(i, j, self.player_id.as_cell());

            let estimate = self.estimate_and_add(candidate);

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

    fn estimate_and_add(&mut self, state: Board) -> f64 {
        match self.lookup_estimate(&state) {
            (val, true) => val,
            (val, false) => {
                debug!("adding {:?} to map",  &state);
                self.add_estimate(state.clone(), val);
                val
            },
        }
    }

    fn estimate(&self, state: &Board) -> f64 {
        self.lookup_estimate(state).0
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
        debug!("add_estimate: state = {:?}", &state);
        self.estimates.insert(state, value);
    }

    fn update_estimate(&mut self, state: &Board, value: f64) -> Result<(), &'static str> {
        if let Some(estimate) = self.estimates.get_mut(state) {
            debug!("update_estimate: state = {:?}", state);
            *estimate = value;
            Ok(())
        } else {
            Err("estimate not set, call add_estimate instead")
        }
    }

    fn calc_new_estimate(&self, state1: &Board, state2: &Board) -> f64 {
        let estimate1 = self.estimate(state1);
        let estimate2 = self.estimate(state2);
        estimate1 + self.alpha * (estimate2 - estimate1)
    }

    pub fn print_estimates(&self) {
        debug!("estimates ({:?}):", self.player_id);
        for (k, v) in self.estimates.iter() {
            debug!("  {:?} {:?}", k, v);
        }
    }

    pub fn get_estimates(&self) -> Estimates {
        Estimates::new(self.estimates.iter())
    }
}

struct Estimates<'a> {
    iter: hash_map::Iter<'a, Board, f64>,
}

impl<'a> Estimates<'a> {
    fn new(iter: hash_map::Iter<'a, Board, f64>) -> Estimates<'a> {
        Estimates { iter: iter }
    }
}

impl<'a> Iterator for Estimates<'a> {
    type Item = (&'a Board, f64);
    fn next(&mut self) -> Option<(&'a Board, f64)> {
        self.iter.next().map(|(board, val)| (board, *val))
    }
}


pub trait Player {
    fn choose_action(&mut self, state: &Board) -> Option<CellCoords>;
    fn update_after_action(&mut self, state1: &Board, state2: &Board);
}

impl Player for RLPlayer {
    fn choose_action(&mut self, state: &Board) -> Option<CellCoords> {
        debug!("STARTING choose_action");
        self.estimate_and_add(*state);
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

    fn update_after_action(&mut self, state1: &Board, state2: &Board) {
        let new_estimate = self.calc_new_estimate(state1, state2);
        debug!("update_after_action: new_estimate = {}", new_estimate);
        self.update_estimate(state1, new_estimate).unwrap();
    }

}
