use super::PlayerId;

// (row, column). Top-left is (0, 0), bottom-right is (2, 2)
type Action = (usize, usize, Cell);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Cell { X, O, Nil }

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Board {
    state: [[Cell; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            state: [[Cell::Nil; 3]; 3],
        }
    }

    fn check_index_out_of_bounds(method: &'static str, row: usize, col: usize) {
        if row > 2 || col > 2 {
            panic!("Board::{} index out of bounds, row: {}, col: {}",
                   method, row, col);
        }
    }

    pub fn get(&self, row: usize, col: usize) -> &Cell {
        Board::check_index_out_of_bounds("get", row, col);
        &self.state[row][col]
    }

    pub fn is_nil(&self, row: usize, col: usize) -> bool {
        Board::check_index_out_of_bounds("is_nil", row, col);

        match *self.get(row, col) {
            Cell::Nil => true,
            _ => false,
        }
    }

    pub fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        Board::check_index_out_of_bounds("act_upon", row, col);
        self.state[row][col] = cell;
    }

    pub fn as_array(&self) -> &[[Cell; 3]; 3] {
        &self.state
    }

    pub fn available_choices(&self) -> NilIter {
        NilIter::new(self)
    }

    pub fn is_drawn(&self) -> bool {
        for row in self.state.iter() {
            for cell in row.iter() {
                match *cell {
                    Cell::Nil => return false,
                    _ => {},
                }
            }
        }
        true
    }

    pub fn is_won_by(&self, id: PlayerId) -> bool {
        match self.is_won() {
            Some(player) => id == player,
            None => false,
        }
    }

    // Assumes that only one player has won (which is only meaningful way
    // to call this)
    pub fn is_won(&self) -> Option<PlayerId> {
        use game::Cell::{X, O};
        match *self.as_array() {
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

struct NilIter<'a> {
    count: usize,
    state: &'a Board,
}

impl<'a> Iterator for NilIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut row: usize;
        let mut col: usize;
        loop {
            if self.count >= 9 { return None; }

            row = self.count / 3;
            col = self.count % 3;

            if !self.state.is_nil(row, col) {
                self.count += 1;
            } else {
                break
            }
        }
        let result = Some((row, col));
        self.count += 1;
        result
    }
}

impl <'a> NilIter<'a> {
    fn new(state: &'a Board) -> NilIter<'a> {
        NilIter { count: 0, state: state }
    }
}
