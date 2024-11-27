use crate::board_repr::{piece::Piece, square::Square};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveType {
    Quite,
    Capture,
    All,
}

#[derive(Clone, Copy)]
pub struct Move {
    data: u32,
}

/*
BINARY REPRESENTATION                DATA               HEX REPRESENTATION

0000 0000 0000 0000 0000 0011 1111   source square      0x3f
0000 0000 0000 0000 1111 1100 0000   target square      0xfc0
0000 0000 0000 1111 0000 0000 0000   piece              0xf000
0000 0000 1111 0000 0000 0000 0000   captured piece     0xf0000
0000 1111 0000 0000 0000 0000 0000   promoted piece     0xf00000
0001 0000 0000 0000 0000 0000 0000   en-passant flag    0x1000000
0010 0000 0000 0000 0000 0000 0000   castling flag      0x2000000
0100 0000 0000 0000 0000 0000 0000   double push flag   0x4000000
*/
impl Move {
    #[allow(clippy::too_many_arguments)]
    pub fn encode_move(
        source_square: Square,
        target_square: Square,
        piece: Piece,
        captured_piece: Piece,
        promoted_piece: Piece,
        en_passant: bool,
        castling: bool,
        double_push: bool,
    ) -> Self {
        let data = (source_square as u32)
            | (target_square as u32) << 6
            | (piece as u32) << 12
            | (captured_piece as u32) << 16
            | (promoted_piece as u32) << 20
            | (en_passant as u32) << 24
            | (castling as u32) << 25
            | (double_push as u32) << 26;

        Self { data }
    }

    pub fn source_square(&self) -> Square {
        Square::get_by_index((self.data & 0x3f) as u8)
    }

    pub fn target_square(&self) -> Square {
        Square::get_by_index(((self.data & 0xfc0) >> 6) as u8)
    }

    pub fn piece(&self) -> Piece {
        Piece::get_by_index(((self.data & 0xf000) >> 12) as u8)
    }

    pub fn captured_piece(&self) -> Piece {
        Piece::get_by_index(((self.data & 0xf0000) >> 16) as u8)
    }

    pub fn promoted_piece(&self) -> Piece {
        Piece::get_by_index(((self.data & 0xf00000) >> 20) as u8)
    }

    pub fn en_passant(&self) -> bool {
        self.data & 0x1000000 > 0
    }

    pub fn castling(&self) -> bool {
        self.data & 0x2000000 > 0
    }

    pub fn double_push(&self) -> bool {
        self.data & 0x4000000 > 0
    }
}

impl Default for Move {
    fn default() -> Self {
        Self::encode_move(
            Square::A8,
            Square::A8,
            Piece::None,
            Piece::None,
            Piece::None,
            false,
            false,
            false,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_encoding_and_decoding() {
        let move_one = Move::encode_move(
            Square::H1,
            Square::A8,
            Piece::WhiteRook,
            Piece::BlackKnight,
            Piece::BlackBishop,
            true,
            false,
            true,
        );

        assert_eq!(move_one.source_square(), Square::H1);
        assert_eq!(move_one.target_square(), Square::A8);
        assert_eq!(move_one.piece(), Piece::WhiteRook);
        assert_eq!(move_one.captured_piece(), Piece::BlackKnight);
        assert_eq!(move_one.promoted_piece(), Piece::BlackBishop);
        assert!(move_one.en_passant());
        assert!(!move_one.castling());
        assert!(move_one.double_push());
    }
}
