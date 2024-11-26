use std::fmt::Display;

use crate::board_repr::piece::Piece;

use super::moves::Move;

const MAX_LEGAL_MOVES: usize = 255;

pub struct MoveList {
    pub moves: [Move; MAX_LEGAL_MOVES],
    pub count: u8,
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

impl Display for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("SOURCE_SQUARE    TARGET_SQUARE   PIECE    CAPTURED_PIECE    PROMOTED_PIECE    EN_PASSANT    CASTLING    DOUBLE_PUSH\n");
        result.push_str("-------------------------------------------------------------------------------------------------------------------\n");

        for m in self.moves {
            if m.piece() == Piece::None {
                continue;
            }

            result.push_str(&format!( "      {}               {}         {}             {}                 {}                {}            {}            {}\n",
                m.source_square(), m.target_square(), m.piece(), m.captured_piece(), m.promoted_piece(), m.en_passant() as u8, m.castling() as u8, m.double_push() as u8
            ));
            result.push_str("-------------------------------------------------------------------------------------------------------------------\n");
        }

        write!(f, "{}", result)
    }
}
