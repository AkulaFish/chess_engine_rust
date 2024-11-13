use super::bit_board::BitBoard;
use super::piece::Color;
use super::square::Square;

#[derive(Debug, Default)]
pub struct CastleAvailability {
    pub can_white_castle_queen: bool,
    pub can_white_castle_king: bool,

    pub can_black_castle_queen: bool,
    pub can_black_castle_king: bool,
}

#[derive(Debug)]
pub struct GameState {
    pub bitboards: [BitBoard; 12],
    pub active_color: Color,
    pub castle_settings: CastleAvailability,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,
}
