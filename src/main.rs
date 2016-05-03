#![crate_name = "rl_tic_tac_toe"]
#![feature(slice_patterns, path_ext)]

#[macro_use] extern crate log;
extern crate env_logger;
extern crate rand;

use game::Board;
use game::Cell::{self, X, O};
use player::{Player, RLPlayer, HumanPlayer};
use persist::{estimates_file_exists, persist_rlplayer};


mod game;
mod persist;
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

struct Game<'a> {
    current: PlayerId,
    players: [&'a mut Player; 2],
    board: Board,
}


impl<'a> Game<'a> {
    fn new(player1: &'a mut Player, player2: &'a mut Player) -> Game<'a> {
        Game {
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

    fn current_player(&mut self) -> &mut Player {
        match self.current {
            PlayerId::P1 => self.players[0],
            PlayerId::P2 => self.players[1],
        }
    }

    fn player_action(&mut self) -> Result<(), &'static str> {
        let board = self.board;
        match self.current_player().choose_action(&board) {
            Some((i, j)) => {
                self.current_player_mark_cell(i, j);
                let new_board = self.board;
                self.current_player().update_after_action(&board, &new_board);
                self.next_player();
                self.current_player().ensure_board_has_estimate(new_board);
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

static ESTIMATES_FNAME: &'static str = "rlttt_estimates";
const NUM_GAMES: u64 = 3;

fn main() {
    env_logger::init().unwrap();
    let mut player1 = if estimates_file_exists() {
        match RLPlayer::from_file(PlayerId::P1) {
            Ok(player) => player,
            Err(_) =>  RLPlayer::new(PlayerId::P1, 0.08),
        }
    } else {
        RLPlayer::new(PlayerId::P1, 0.08)
    };

    //let mut player2 = RLPlayer::new(PlayerId::P2, 0.08);
    let mut player2 = HumanPlayer::new(PlayerId::P2);


    let mut p1 = 0;
    {
        let mut game = Game::new(&mut player1, &mut player2);

        for i in 0..NUM_GAMES  {
            println!("Game {}:", i);
            match game.play() {
                GameResult::Wins(pid) => {
                    if pid == PlayerId::P1 {
                        p1 += 1;
                    }

                    println!("{:?} wins!", pid);
                },
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
