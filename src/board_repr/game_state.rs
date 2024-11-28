use crate::move_generation::moves::Move;

use super::{piece::Color, square::Square};

#[derive(Default, Clone, Copy)]
pub struct CastleAvailability {
    pub can_white_castle_queen: bool,
    pub can_white_castle_king: bool,

    pub can_black_castle_queen: bool,
    pub can_black_castle_king: bool,
}

impl CastleAvailability {
    pub fn set_by_square(&mut self, square: Square) {
        match square {
            // White
            Square::A1 => self.can_white_castle_queen = false,
            Square::E1 => {
                self.can_white_castle_king = false;
                self.can_white_castle_queen = false;
            }
            Square::H1 => self.can_white_castle_king = false,

            // Black
            Square::A8 => self.can_black_castle_queen = false,
            Square::E8 => {
                self.can_black_castle_king = false;
                self.can_black_castle_queen = false;
            }
            Square::H8 => self.can_black_castle_king = false,
            _ => (),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct GameState {
    pub active_color: Color,
    pub castle_settings: CastleAvailability,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,
    pub next_move: Move,
}
