use super::PlayerId;

// (row, column). Top-left is (0, 0), bottom-right is (2, 2)
type Action = (usize, usize, TTTCell);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum TTTCell { X, O, Nil }

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GameState {
    state: [[TTTCell; 3]; 3],
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            state: [[TTTCell::Nil; 3]; 3],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> &TTTCell {
        if row > 2 || col > 2 {
            panic!("GameState::get index out of bounds, row: {}, col: {}",
                   row, col);
        }
        &self.state[row][col]
    }

    pub fn is_nil(&self, row: usize, col: usize) -> bool {
        if row > 2 || col > 2 {
            panic!("GameState::is_nil index out of bounds, row: {}, col: {}",
                   row, col);
        }

        match *self.get(row, col) {
            TTTCell::Nil => true,
            _ => false,
        }
    }

    pub fn act_upon(&mut self, &(i, j, state): &Action) {
        if i > 2 || i > 2 {
            panic!("GameState::act_upon index out of bounds, row: {}, col: {}",
                   i, j);
        }
        self.state[i][j] = state;
    }

    pub fn as_array(&self) -> &[[TTTCell; 3]; 3] {
        &self.state
    }

    pub fn available_choices(&self) -> NilIter {
        NilIter::new(self)
    }

    pub fn is_drawn(&self) -> bool {
        for row in self.state.iter() {
            for cell in row.iter() {
                match *cell {
                    TTTCell::Nil => return false,
                    _ => {},
                }
            }
        }
        true
    }

    pub fn is_won_by_X(&self) -> bool {
        use game::TTTCell::X;
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
        use game::TTTCell::O;
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

    pub fn is_won_by(&self, id: PlayerId) -> bool {
        match id {
            PlayerId::P1 => self.is_won_by_X(),
            _            => self.is_won_by_O(),
        }
    }
}

struct NilIter<'a> {
    count: usize,
    state: &'a GameState,
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
    fn new(state: &'a GameState) -> NilIter<'a> {
        NilIter { count: 0, state: state }
    }
}
