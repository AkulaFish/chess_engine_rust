pub mod board_repr;
pub mod move_generation;

use std::u64;

use board_repr::{board::BitBoard, fen::Fen, game_state, piece::Color, square::Square};
use move_generation::tables::{
    get_bishop_relevant_occupancy_mask, get_king_attack_mask, get_knight_attack_mask,
    get_pawn_attack_mask, get_rook_relevant_occupancy_mask,
};
use strum::IntoEnumIterator;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    for square in Square::iter() {
        let bb = get_rook_relevant_occupancy_mask(square);
        bb.debug();
    }
}
