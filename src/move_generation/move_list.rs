use crate::board_repr::piece::Piece;

use super::moves::Move;

const MAX_LEGAL_MOVES: usize = 255;

pub struct MoveList {
    pub moves: [Move; MAX_LEGAL_MOVES],
    pub count: u8,
}

impl Default for MoveList {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [Move::default(); MAX_LEGAL_MOVES],
            count: 0,
        }
    }

    pub fn add_move(&mut self, m: Move) {
        self.moves[self.count as usize] = m;
        self.count += 1;
    }

    pub fn display(&self) {
        println!("{}", self)
    }
}

impl IntoIterator for MoveList {
    type Item = Move;
    type IntoIter = MoveListIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            move_list: self,
            index: 0,
        }
    }
}

pub struct MoveListIterator {
    move_list: MoveList,
    index: usize,
}

impl Iterator for MoveListIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.move_list.moves[self.index];
        self.index += 1;
        if result.piece() == Piece::None {
            return None;
        }
        Some(result)
    }
}
