use crate::board_repr::piece::Piece;
use std::str::FromStr;

use super::{
    bit_board::BitBoard,
    board::{CastleAvailability, GameState},
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

            if s.is_ascii_digit() {
                let d = s.to_digit(10).unwrap();
                for _ in 0..d {
                    board.push_str("  □");
                }
                continue;
            }

            let piece = Piece::from_str(&s.to_string()).expect("Invalid piece value");
            board.push_str(&format!("  {}", piece));
        }

        println!("{}", board);
    }

    /// This method converts FEN notation to the GameState struct used later by the engine
    /// Notice that this function does not support update version of spec
    /// (new version skips the en-passant parameter if it's "-")
    pub fn to_game_state(fen: &str) -> GameState {
        let parts = fen.split(" ");
        let mut active_color: Color = Color::White;
        let mut castle_settings = CastleAvailability::default();
        let mut en_passant_target: Option<Square> = None;
        let mut halfmove_clock = 0;
        let mut fullmove_number: u8 = 1u8;
        let mut bitboards: BitBoardMap = BitBoardMap::default();

        for (i, part) in parts.enumerate() {
            match i {
                0 => {
                    let mut square_counter = 0usize;
                    for rank in part.split('/') {
                        for piece in rank.chars() {
                            if piece.is_ascii_digit() {
                                square_counter += piece.to_digit(10).unwrap() as usize;
                                continue;
                            }

                            let board_piece =
                                Piece::from_str(&piece.to_string()).unwrap_or_else(|_| {
                                    panic!("Unknown piece value in FEN configuration: {}", piece)
                                });
                            let square = Square::get_by_index(square_counter);
                            let piece_bitboard = square.get_bitboard();
                            bitboards[board_piece.value() as usize] |= piece_bitboard;

                            square_counter += 1;
                        }
                    }
                }
                1 => match part {
                    "w" => active_color = Color::White,
                    "b" => active_color = Color::Black,
                    _ => panic!("Undefined color in FEN notation"),
                },
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
                    if part == "-" {
                        continue;
                    }

                    en_passant_target = Some(
                        Square::from_str(&part.to_uppercase())
                            .expect("Unknown en passant target square in Fen notation"),
                    );
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
                _ => panic!("Unknown FEN configuration parameter"),
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
