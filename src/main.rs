#![feature(slice_patterns)]

extern crate rand;

use game::GameState;
use game::CellState::{self, X, O};
use player::RLPlayer;

mod game;
mod player;

#[derive(Copy, Clone, Debug)]
pub enum PlayerId { P1, P2 }

impl PlayerId {
    fn next(&self) -> PlayerId {
        match *self {
            PlayerId::P1 => PlayerId::P2,
            PlayerId::P2 => PlayerId::P1,
        }
    }

    fn as_cellstate(&self) -> CellState {
        match *self{
            PlayerId::P1 => X,
            PlayerId::P2 => O,
        }
    }

}

#[derive(Debug)]
enum GameResult {
    Wins(PlayerId),
    Draw,
}

struct TTTGame<'a> {
    current: PlayerId,
    players: [&'a mut RLPlayer; 2],
    gamestate: GameState,
}


impl<'a> TTTGame<'a> {
    fn new(player1: &'a mut RLPlayer, player2: &'a mut RLPlayer) -> TTTGame<'a> {
        TTTGame {
            current: PlayerId::P1,
            players: [player1, player2],
            gamestate: GameState::new(),
        }
    }

    fn play(&mut self) -> GameResult {
        loop {
            match self.advance_state() {
                Err(e) => println!("{}", e),
                _ => {},
            }

            if let Some(winner) = self.is_won() {
                return GameResult::Wins(winner)
            }
            
            if self.is_drawn() {
                return GameResult::Draw
            }
        }
    }

    fn current_player(&mut self) -> &mut RLPlayer {
        match self.current {
            PlayerId::P1 => self.players[0],
            PlayerId::P2 => self.players[1],
        }
    }

    fn advance_state(&mut self) -> Result<(), &'static str> {
        let state = self.gamestate; // choose_action() borrows mutably, so this is on a
                                    // separate line
        match self.current_player().choose_action(&state) {
            Some((i, j)) => {
                let state = self.current.as_cellstate();
                self.gamestate.act_upon(&(i, j, state));
                self.current = self.current.next();
                Ok(())
            },
            None => Err("No actions left. Cannot advance state."),
        }
    }
    
    fn is_drawn(&self) -> bool {
        self.gamestate.is_drawn()
    }

    fn is_won(&self) -> Option<PlayerId> {
        if self.gamestate.is_won_by_X() {
            Some(PlayerId::P1)
        } else if self.gamestate.is_won_by_O() {
            Some(PlayerId::P2)
        } else {
            None
        }
    }
}


fn main() {
    let mut player1 = RLPlayer::new(PlayerId::P1, 0.1);
    let mut player2 = RLPlayer::new(PlayerId::P2, 0.05);
    let mut game = TTTGame::new(&mut player1, &mut player2);

    let result = game.play();
    println!("{:?}", result);
}
