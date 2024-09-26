use crate::board_repr::piece::Piece;
use std::str::FromStr;

use super::{
    board::BitBoard,
    game_state::{CastleAvailability, GameState},
    piece::Color,
    square::Square,
};

pub type BitBoardMap = [BitBoard; 12];

#[derive(Debug)]
pub struct Fen;

impl Fen {
    pub fn draw_fen(fen: &str) {
        let mut board = String::new();

        for s in fen.chars() {
            if s == ' ' || s == 'w' {
                break;
            }

            if s == '/' {
                board.push('\n');
                continue;
            }

            if s.is_digit(10) {
                let d = s.to_digit(10).unwrap();
                for _ in 0..d {
                    board.push_str("  □");
                }
                continue;
            }

            let piece = Piece::from_str(&s.to_string()).expect("Invalid piece value");
            board.push_str(&format!("  {}", piece.to_string()));
        }

        println!("{}", board);
    }

    pub fn to_game_state(fen: &str) -> GameState {
        let parts = fen.split(" ");
        let mut active_color: Color = Color::White;
        let mut castle_settings = CastleAvailability::default();
        let mut en_passant_target = String::from("-");
        let mut halfmove_clock = 0;
        let mut fullmove_number: u8 = 1u8;
        let mut bitboards: BitBoardMap = BitBoardMap::default();

        for (i, part) in parts.enumerate() {
            match i {
                0 => {
                    let mut square_counter = 1; // 0 == A1, 1 == B2 ...
                    for rank in part.split('/') {
                        for piece in rank.chars() {
                            if piece.is_digit(10) {
                                square_counter += piece.to_digit(10).unwrap();
                            }

                            let board_piece = Piece::from_str(&piece.to_string())
                                .expect("Unknown piece value in FEN configuration");
                            let square = Square::get_nth(square_counter as usize);
                            let piece_bitboard = square.get_bitboard();
                            let piece_index = board_piece as usize;
                            bitboards[piece_index] = bitboards[piece_index] & piece_bitboard;
                        }
                    }
                }
                1 => {
                    if part == "w" {
                        active_color = Color::White;
                    } else {
                        active_color = Color::Black;
                    }
                }
                2 => {
                    for c in part.chars() {
                        match c {
                            'K' => castle_settings.can_white_castle_king = true,
                            'k' => castle_settings.can_black_castle_king = true,
                            'Q' => castle_settings.can_white_castle_queen = true,
                            'q' => castle_settings.can_black_castle_queen = true,
                            _ => panic!("Unknown castle setting"),
                        }
                    }
                }
                3 => {
                    en_passant_target = String::from(part);
                }
                4 => {
                    let clock: u8 = part.parse().expect("Clock value must be positive integer");
                    halfmove_clock = clock;
                }
                5 => {
                    let number: u8 = part
                        .parse()
                        .expect("Fullmove number must be positive integer");
                    fullmove_number = number;
                }
                _ => panic!("test"),
            }
        }

        GameState {
            bitboards,
            active_color,
            castle_settings,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
        }
    }
}
