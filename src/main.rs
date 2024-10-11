pub mod board_repr;
pub mod move_generation;

use std::u64;

use board_repr::{board::BitBoard, fen::Fen, game_state, piece::Color, square::Square};
use move_generation::tables::{
    get_bishop_relevant_occupancy_mask, get_king_attack_mask, get_knight_attack_mask,
    get_pawn_attack_mask,
};

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let bb = get_bishop_relevant_occupancy_mask(Square::D5);
    bb.debug();
}
