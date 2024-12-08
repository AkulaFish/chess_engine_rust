use super::game_state::GameState;

const MAX_MOVES: usize = 2048;

pub struct History {
    pub history: [GameState; MAX_MOVES],
    pub count: usize,
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

impl History {
    pub fn new() -> Self {
        Self {
            history: [GameState::default(); MAX_MOVES],
            count: 0,
        }
    }

    pub fn push(&mut self, game_state: GameState) {
        self.history[self.count] = game_state;
        self.count += 1;
    }

    pub fn pop(&mut self) -> GameState {
        self.count -= 1;
        self.history[self.count]
    }
}
