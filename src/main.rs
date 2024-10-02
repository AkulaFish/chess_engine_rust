pub mod board_repr;
pub mod move_generation;

use std::u64;

use board_repr::{board::BitBoard, fen::Fen, game_state, piece::Color, square::Square};
use move_generation::tables::get_pawn_attack_mask;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let mut bb = BitBoard::default();

    for rank in 0..8 {
        for file in 0..6 {
            let square = Square::get_nth(rank * 8 + file);
            bb.set_bit_value(square);
        }
    }

    bb.debug();
}
