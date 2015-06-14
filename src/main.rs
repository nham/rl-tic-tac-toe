#![crate_name = "rl_tic_tac_toe"]
#![feature(slice_patterns, path_ext)]

#[macro_use] extern crate log;
extern crate env_logger;
extern crate rand;

use game::Board;
use game::Cell::{self, X, O};
use player::RLPlayer;

use std::fs::{PathExt, File};
use std::io::{self, Write};
use std::path::Path;

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
        debug!("play, board: {:?}", self.board);
        loop {
            match self.player_action() {
                Err(e) => debug!("{}", e),
                _ => {},
            }

            debug!("play, board: {:?}", self.board);

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
        // FIXME: bad because it copies
        let board = self.board; // appeasing the borrow checker
        match self.current_player().choose_action(&board) {
            Some((i, j)) => {
                self.current_player_mark_cell(i, j);
                // FIXME: bad because it copies
                let new_board = self.board;
                self.current_player().update_after_action(&board, &new_board);
                self.current_player().print_estimates();
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

    fn is_drawn(&self) -> bool {
        self.board.is_drawn()
    }

    fn is_won(&self) -> Option<PlayerId> {
        self.board.is_won()
    }
}

// most recent is always written as rlttt_estimates
// as more estimates get persisted, we keep a record of files,
// rlttt_estimates.1, rlttt_estimates.2, ...

static ESTIMATES_FNAME: &'static str = "rlttt_estimates";

fn persist_rlplayer(player: &RLPlayer) -> io::Result<()> {
    let path = Path::new(ESTIMATES_FNAME);

    if path.is_file() {
        panic!("file 'rlttt_estimates' already exists");
    }

    // create
    let mut f = try!(File::create(path));

    let line = format!("epsilon: {:?}, alpha: {:?}\n",
                       player.epsilon, player.alpha);
    try!(f.write_all(line.as_bytes()));

    for (board, val) in player.get_estimates() {
        let line = format!("{:?} {:?}\n", board, val);
        try!(f.write_all(line.as_bytes()));
    }

    Ok(())

}


const NUM_GAMES: u64 = 500;

fn main() {
    env_logger::init().unwrap();
    let mut player1 = RLPlayer::new(PlayerId::P1, 0.08);
    let mut player2 = RLPlayer::new(PlayerId::P2, 0.08);


    let mut p1 = 0;
    {
        let mut game = TTTGame::new(&mut player1, &mut player2);

        for _ in 0..NUM_GAMES  {
            match game.play() {
                GameResult::Wins(PlayerId::P1) => { p1 += 1; },
                _ => {},
            }
            game.reset();
            debug!("------------------------");
            debug!("------------------------");
        }
    }

    println!("Played {} games.", NUM_GAMES);
    println!("Wins: P1: {}, P2: {}", p1, NUM_GAMES - p1);

    // persist player 1. idea is we are really just trying to train a player to play as X
    // later we'll train one to exclusively play as O? or how about 1 player that is trained
    // to play both? options to consider
    match persist_rlplayer(&player1) {
        Err(e) => panic!("Error persisting player 1: {:?}", e),
        _ => {},
    }
}
