#![feature(slice_patterns)]

#[macro_use] extern crate log;
extern crate env_logger;
extern crate rand;

use game::Board;
use game::Cell::{self, X, O};
use player::RLPlayer;

mod game;
mod player;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlayerId { P1, P2 }

impl PlayerId {
    fn next(&self) -> PlayerId {
        match *self {
            PlayerId::P1 => PlayerId::P2,
            PlayerId::P2 => PlayerId::P1,
        }
    }

    fn as_cell(&self) -> Cell {
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
    board: Board,
}


impl<'a> TTTGame<'a> {
    fn new(player1: &'a mut RLPlayer, player2: &'a mut RLPlayer) -> TTTGame<'a> {
        TTTGame {
            current: PlayerId::P1,
            players: [player1, player2],
            board: Board::new(),
        }
    }

    fn play(&mut self) -> GameResult {
        loop {
            debug!("{:?}", self.board);

            match self.player_action() {
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
        self.board = Board::new();
    }

    fn current_player(&mut self) -> &mut RLPlayer {
        match self.current {
            PlayerId::P1 => self.players[0],
            PlayerId::P2 => self.players[1],
        }
    }

    fn player_action(&mut self) -> Result<(), &'static str> {
        let board = self.board; // appeasing the borrow checker
        match self.current_player().choose_action(&board) {
            Some((i, j)) => {
                self.current_player_mark_cell(i, j);
                self.next_player();
                Ok(())
            },
            None => Err("No remaining actions."),
        }
    }

    fn current_player_mark_cell(&mut self, row: usize, col: usize) {
        let mark = self.current.as_cell();
        self.board.set_cell(row, col, mark);
    }

    fn next_player(&mut self) {
        self.current = self.current.next();
    }

    pub fn update_estimates(&mut self) {
        for i in 0..2 {
            self.players[i].update_estimates();
        }
    }
    
    fn is_drawn(&self) -> bool {
        self.board.is_drawn()
    }

    fn is_won(&self) -> Option<PlayerId> {
        self.board.is_won()
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
        game.update_estimates();
        game.reset();
        debug!("-----");
    }

    println!("Played {} games.", NUM_GAMES);
    println!("Wins: P1: {}, P2: {}", p1, NUM_GAMES - p1);
}
