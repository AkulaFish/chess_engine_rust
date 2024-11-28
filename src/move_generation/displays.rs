use std::fmt::{Debug, Display};

use crate::{board_repr::piece::Piece, utils::traits::DisplayExtension};

use super::{move_list::MoveList, moves::Move};

// Move displays
impl DisplayExtension for Move {}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("SOURCE_SQUARE    TARGET_SQUARE   PIECE    CAPTURED_PIECE    PROMOTED_PIECE    EN_PASSANT    CASTLING    DOUBLE_PUSH\n");
        result.push_str(&format!( "      {}               {}         {}             {}                 {}                {}            {}            {}\n",
            self.source_square(), self.target_square(), self.piece(), self.captured_piece(), self.promoted_piece(), self.en_passant() as u8, self.castling() as u8, self.double_push() as u8
        ));

        write!(f, "{}", result)
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("SOURCE_SQUARE    TARGET_SQUARE   PIECE    CAPTURED_PIECE    PROMOTED_PIECE    EN_PASSANT    CASTLING    DOUBLE_PUSH\n");
        result.push_str(&format!( "      {}               {}         {}             {}                 {}                {}            {}            {}\n",
            self.source_square(), self.target_square(), self.piece(), self.captured_piece(), self.promoted_piece(), self.en_passant() as u8, self.castling() as u8, self.double_push() as u8
        ));

        write!(f, "{}", result)
    }
}

// MoveList displays
impl DisplayExtension for MoveList {}

impl Display for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("INDEX    SOURCE_SQUARE    TARGET_SQUARE   PIECE    CAPTURED_PIECE    PROMOTED_PIECE    EN_PASSANT    CASTLING    DOUBLE_PUSH\n");
        result.push_str("----------------------------------------------------------------------------------------------------------------------------\n");

        for (i, m) in self.moves.iter().enumerate() {
            if m.piece() == Piece::None {
                continue;
            }

            result.push_str(&format!( "  {}           {}               {}         {}             {}                 {}                {}            {}            {}\n",
                i, m.source_square(), m.target_square(), m.piece(), m.captured_piece(), m.promoted_piece(), m.en_passant() as u8, m.castling() as u8, m.double_push() as u8
            ));
            result.push_str("----------------------------------------------------------------------------------------------------------------------------\n");
        }

        write!(f, "{}", result)
    }
}

impl Debug for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("INDEX    SOURCE_SQUARE    TARGET_SQUARE   PIECE    CAPTURED_PIECE    PROMOTED_PIECE    EN_PASSANT    CASTLING    DOUBLE_PUSH\n");
        result.push_str("----------------------------------------------------------------------------------------------------------------------------\n");

        for (i, m) in self.moves.iter().enumerate() {
            if m.piece() == Piece::None {
                continue;
            }

            result.push_str(&format!( "  {}           {}               {}         {}             {}                 {}                {}            {}            {}\n",
                i, m.source_square(), m.target_square(), m.piece(), m.captured_piece(), m.promoted_piece(), m.en_passant() as u8, m.castling() as u8, m.double_push() as u8
            ));
            result.push_str("----------------------------------------------------------------------------------------------------------------------------\n");
        }

        write!(f, "{}", result)
    }
}
