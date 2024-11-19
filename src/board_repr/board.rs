use std::fmt::{Debug, Display};

use strum::IntoEnumIterator;

use crate::move_generation::generator::MoveGenerator;

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

pub struct Board {
    pub bitboards: [BitBoard; 12],
    pub active_color: Color,
    pub castle_settings: CastleAvailability,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,
}

impl Board {
    pub fn display(&self) {
        println!("{}", self);
    }

    pub fn debug(&self) {
        println!("{:?}", self);
    }

    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        self.bitboards[piece as usize] |= square.get_bitboard();
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for rank in 0..8 {
            result.push_str(&format!("{}  ", 8 - rank));
            for file in 0..8 {
                let square = Square::get_by_index(rank * 8 + file);
                let mut match_found = false;
                for piece in Piece::iter() {
                    if !(square.get_bitboard() & self.bitboards[piece as usize]).empty() {
                        result.push_str(&format!(" {} ", piece));
                        match_found = true;
                        break;
                    }
                }
                if !match_found {
                    result.push_str(" . ");
                }
            }
            result.push('\n')
        }
        result.push_str("    a  b  c  d  e  f  g  h\n\n");

        write!(f, "{}", result)
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for rank in 0..8 {
            result.push_str(&format!("{}  ", 8 - rank));
            for file in 0..8 {
                let square = Square::get_by_index(rank * 8 + file);
                let mut match_found = false;
                for piece in Piece::iter() {
                    if !(square.get_bitboard() & self.bitboards[piece as usize]).empty() {
                        result.push_str(&format!(" {} ", piece));
                        match_found = true;
                        break;
                    }
                }
                if !match_found {
                    result.push_str(" . ");
                }
            }
            result.push('\n')
        }
        result.push_str("    a  b  c  d  e  f  g  h\n\n");

        write!(f, "{}", result)
    }
}

impl Board {
    pub fn get_occupancies(&self, color: Color) -> BitBoard {
        let (start, end) = match color {
            Color::White => (0, 6),
            Color::Black => (6, 12),
            Color::Both => (0, 12),
        };
        let mut result = BitBoard::default();
        for i in start..end {
            result |= self.bitboards[i];
        }
        result
    }

    // Returns if square is attacked by given color
    pub fn is_square_attacked(&self, square: Square, color: Color, mg: &MoveGenerator) -> bool {
        let defender_color = color.opposite();
        // Is attacked by pawns
        let pawn_attack = mg.get_pawn_attack(square, defender_color);
        let pawn_index = Piece::BlackPawn.to_color(color) as usize;
        if !(pawn_attack & self.bitboards[pawn_index]).empty() {
            return true;
        }

        // Is attacked by knight
        let knight_attack = mg.get_knight_attack(square);
        let knight_index = Piece::BlackKnight.to_color(color) as usize;
        if !(knight_attack & self.bitboards[knight_index]).empty() {
            return true;
        }

        // Is attacked by king
        let king_attack = mg.get_king_attack(square);
        let king_index = Piece::BlackKing.to_color(color) as usize;
        if !(king_attack & self.bitboards[king_index]).empty() {
            return true;
        }

        // Is attacked by bishop
        let bishop_attack = mg.get_bishop_attack(square, self.get_occupancies(Color::Both));
        let bishop_index = Piece::BlackBishop.to_color(color) as usize;
        if !(bishop_attack & self.bitboards[bishop_index]).empty() {
            return true;
        }

        // Is attacked by rook
        let rook_attack = mg.get_rook_attack(square, self.get_occupancies(Color::Both));
        let rook_index = Piece::BlackRook.to_color(color) as usize;
        rook_attack.debug();
        self.get_occupancies(Color::Both).debug();
        if !(rook_attack & self.bitboards[rook_index]).empty() {
            return true;
        }

        // Is attacked by rook
        let queen_attack = mg.get_queen_attack(square, self.get_occupancies(Color::Both));
        let queen_index = Piece::BlackQueen.to_color(color) as usize;
        if !(queen_attack & self.bitboards[queen_index]).empty() {
            return true;
        }

        false
    }
}
