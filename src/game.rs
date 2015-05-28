use std::hash::Hash;


// (row, column). Top-left is (0, 0), bottom-right is (2, 2)
pub type Action = (usize, usize, CellState);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum CellState { X, O, Nil }

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct GameState {
    state: [[CellState; 3]; 3],
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            state: [[CellState::Nil; 3]; 3],
        }
    }

    pub fn act_upon(&mut self, &(i, j, state): &Action) {
        self.state[i][j] = state;
    }

    pub fn as_array(&self) -> &[[CellState; 3]; 3] {
        &self.state
    }

    pub fn is_drawn(&self) -> bool {
        for row in self.state.iter() {
            for cell in row.iter() {
                match *cell {
                    CellState::Nil => return false,
                    _ => {},
                }
            }
        }
        true
    }

    pub fn is_won_by_X(&self) -> bool {
        use game::CellState::X;
        match *self.as_array() {
            [[X, X, X], _, _]
            | [_, [X, X, X], _]
            | [_, _, [X, X, X]]
            | [[X, _, _], [X, _, _], [X, _, _]]
            | [[_, X, _], [_, X, _], [_, X, _]]
            | [[_, _, X], [_, _, X], [_, _, X]]
            | [[X, _, _], [_, X, _], [_, _, X]]
            | [[_, _, X], [_, X, _], [X, _, _]] => true,
            _ => false,
        }
    }

    pub fn is_won_by_O(&self) -> bool {
        use game::CellState::O;
        match *self.as_array() {
            [[O, O, O], _, _]
            | [_, [O, O, O], _]
            | [_, _, [O, O, O]]
            | [[O, _, _], [O, _, _], [O, _, _]]
            | [[_, O, _], [_, O, _], [_, O, _]]
            | [[_, _, O], [_, _, O], [_, _, O]]
            | [[O, _, _], [_, O, _], [_, _, O]]
            | [[_, _, O], [_, O, _], [O, _, _]] => true,
            _ => false,
        }
    }
}
