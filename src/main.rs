#![feature(slice_patterns)]

#[macro_use] extern crate log;
extern crate env_logger;
extern crate rand;

use game::GameState;
use game::TTTCell::{self, X, O};
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

    fn as_cellstate(&self) -> TTTCell {
        match *self{
            PlayerId::P1 => X,
            PlayerId::P2 => O,
        }
    }

}

// TODO: impl Display
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
            debug!("{:?}", self.gamestate);

            match self.advance_state() {
                Err(e) => debug!("{}", e),
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

    fn reset(&mut self) {
        self.current = PlayerId::P1;
        self.gamestate = GameState::new();
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

    pub fn update_estimates(&mut self) {
        for i in 0..2 {
            self.players[i].update_estimates();
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


const NUM_GAMES: u64 = 8;

fn main() {
    env_logger::init().unwrap();
    let mut player1 = RLPlayer::new(PlayerId::P1, 0.95);
    let mut player2 = RLPlayer::new(PlayerId::P2, 0.08);
    let mut game = TTTGame::new(&mut player1, &mut player2);

    let mut p1 = 0;
    for _ in 0..NUM_GAMES  {
        match game.play() {
            GameResult::Wins(PlayerId::P1) => { p1 += 1; },
            _ => {},
        }
        game.reset();
        debug!("-----");
    }

    game.update_estimates();

    println!("Played {} games.", NUM_GAMES);
    println!("Wins: P1: {}, P2: {}", p1, NUM_GAMES - p1);
}
