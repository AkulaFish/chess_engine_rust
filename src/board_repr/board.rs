use std::fmt::Debug;

use strum::IntoEnumIterator;

use super::bit_board::BitBoard;
use super::piece::{Color, Piece};
use super::square::Square;

#[derive(Debug, Default)]
pub struct CastleAvailability {
    pub can_white_castle_queen: bool,
    pub can_white_castle_king: bool,

    pub can_black_castle_queen: bool,
    pub can_black_castle_king: bool,
}

#[derive()]
pub struct Board {
    pub bitboards: [BitBoard; 12],
    pub occupancy: [BitBoard; 3],
    pub active_color: Color,
    pub castle_settings: CastleAvailability,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,
    pub piece_by_square: [Piece; 64],
}

impl Board {
    pub fn new(
        bitboards: [BitBoard; 12],
        active_color: Color,
        castle_settings: CastleAvailability,
        en_passant_target: Option<Square>,
        halfmove_clock: u8,
        fullmove_number: u8,
    ) -> Self {
        let piece_by_square = Self::init_piece_by_square(bitboards);
        let occupancy = Self::init_occupancy(bitboards);
        Self {
            bitboards,
            occupancy,
            active_color,
            castle_settings,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
            piece_by_square,
        }
    }

    pub fn get_occupancies(&self, color: Color) -> BitBoard {
        self.occupancy[color as usize]
    }

    pub fn init_occupancy(bitboards: [BitBoard; 12]) -> [BitBoard; 3] {
        let mut white_occupancy = BitBoard::default();
        for i in 0..6 {
            white_occupancy |= bitboards[i];
        }

        let mut black_occupancy = BitBoard::default();
        for i in 6..12 {
            black_occupancy |= bitboards[i];
        }

        [
            white_occupancy,
            black_occupancy,
            white_occupancy | black_occupancy,
        ]
    }

    pub fn init_piece_by_square(bitboards: [BitBoard; 12]) -> [Piece; 64] {
        let mut result = [Piece::None; 64];
        for square in Square::iter() {
            for piece in Piece::iter().filter(|p| p != &Piece::None) {
                let occ = bitboards[piece as usize];
                let square_bb = square.get_bitboard();
                if !(occ & square_bb).empty() {
                    result[square as usize] = piece;
                    break;
                }
            }
        }

        result
    }

    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        self.bitboards[piece as usize] |= square.get_bitboard();
    }
}
