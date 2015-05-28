#![feature(slice_patterns)]

use game::{Action, GameState};
use game::CellState::{X, O};
use player::Player;

mod game;
mod player;

#[derive(Copy, Clone)]
enum PlayerId { P1, P2 }

impl PlayerId {
    fn next(&self) -> PlayerId {
        match *self {
            PlayerId::P1 => PlayerId::P2,
            PlayerId::P2 => PlayerId::P1,
        }
    }
}

enum GameResult {
    Wins(PlayerId),
    Draw,
}

struct TTTGame<'a> {
    current: PlayerId,
    players: [&'a Player; 2],
    gamestate: GameState,
}


impl<'a> TTTGame<'a> {
    fn play(&mut self) -> GameResult {
        loop {
            self.advance_state();

            if let Some(winner) = self.is_won() {
                return GameResult::Wins(winner)
            }
            
            if self.is_drawn() {
                return GameResult::Draw
            }
        }
    }

    fn current_player(&self) -> &Player {
        match self.current {
            PlayerId::P1 => self.players[0],
            PlayerId::P2 => self.players[1],
        }
    }

    fn advance_state(&mut self) {
        let action = self.current_player().choose_action(&self.gamestate);
        self.gamestate.act_upon(&action);
        self.current = self.current.next();
    }
    
    fn is_drawn(&self) -> bool {
        self.gamestate.is_full()
    }

    fn is_won(&self) -> Option<PlayerId> {
        match *self.gamestate.as_array() {
            [[X, X, X], _, _]
            | [_, [X, X, X], _]
            | [_, _, [X, X, X]]
            | [[X, _, _], [X, _, _], [X, _, _]]
            | [[_, X, _], [_, X, _], [_, X, _]]
            | [[_, _, X], [_, _, X], [_, _, X]]
            | [[X, _, _], [_, X, _], [_, _, X]]
            | [[_, _, X], [_, X, _], [X, _, _]] => Some(PlayerId::P1),

            [[O, O, O], _, _]
            | [_, [O, O, O], _]
            | [_, _, [O, O, O]]
            | [[O, _, _], [O, _, _], [O, _, _]]
            | [[_, O, _], [_, O, _], [_, O, _]]
            | [[_, _, O], [_, _, O], [_, _, O]]
            | [[O, _, _], [_, O, _], [_, _, O]]
            | [[_, _, O], [_, O, _], [O, _, _]] => Some(PlayerId::P2),

            _ => None,
        }
    }
}


fn main() {
    println!("Hello, world!");
}
