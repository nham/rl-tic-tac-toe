use game::{Action, GameState};

pub trait Player {
    fn take_turn(&self, state: &GameState) -> Action;
}

