use std::collections::HashMap;

use game::{Action, GameState};

pub trait Player {
    fn new() -> Self where Self: Sized;
    fn choose_action(&self, state: &GameState) -> (usize, usize);
}


struct RLPlayer {
    x: (),
}

impl Player for RLPlayer {
    fn new() -> RLPlayer {
        RLPlayer {
            x: (),
        }
    }

    fn choose_action(&self, state: &GameState) -> (usize, usize) {
        (0, 0)
    }
}

