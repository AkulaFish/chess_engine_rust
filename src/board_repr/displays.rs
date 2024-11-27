use std::fmt::{Debug, Display};

use strum::IntoEnumIterator;

use crate::{
    board_repr::{piece::Piece, square::Square},
    utils::traits::DisplayExtension,
};

use super::{bit_board::BitBoard, board::Board};

// Board displays
impl DisplayExtension for Board {}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for rank in 0..8 {
            result.push_str(&format!("{}  ", 8 - rank));
            for file in 0..8 {
                let square = Square::get_by_index(rank * 8 + file);
                let mut match_found = false;
                for piece in Piece::iter().filter(|p| p != &Piece::None) {
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
                for piece in Piece::iter().filter(|p| p != &Piece::None) {
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

// BitBoard displays
impl DisplayExtension for BitBoard {}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for rank in 0..8 {
            result.push_str(&format!("{}  ", 8 - rank));
            for file in 0..8 {
                let square = Square::get_by_index(rank * 8 + file);
                result.push_str(&format!(
                    " {} ",
                    if self.get_bit_value(square) { "1" } else { "." }
                ));
            }
            result.push('\n')
        }
        result.push_str("    a  b  c  d  e  f  g  h\n\n");
        result.push_str(&format!("    Decimal: {}\n", self.value()));
        result.push_str(&format!("    Hex: {:x}\n\n", self.value()));

        write!(f, "{}", result)
    }
}

impl Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for rank in 0..8 {
            result.push_str(&format!("{}  ", 8 - rank));
            for file in 0..8 {
                let square = Square::get_by_index(rank * 8 + file);
                result.push_str(&format!(
                    " {} ",
                    if self.get_bit_value(square) { "1" } else { "." }
                ));
            }
            result.push('\n')
        }
        result.push_str("    a  b  c  d  e  f  g  h\n\n");
        result.push_str(&format!("    Decimal: {}\n", self.value()));
        result.push_str(&format!("    Hex: {:x}\n\n", self.value()));

        write!(f, "{}", result)
    }
}
