use super::board::BitBoard;
use super::piece::Color;

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
    pub en_passant_target: String,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,
}
