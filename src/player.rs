use game::{Action, GameState};

pub trait Player {
    fn choose_action(&self, state: &GameState) -> Action;
}

