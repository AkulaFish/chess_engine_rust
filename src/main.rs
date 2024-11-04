pub mod board_repr;
pub mod move_generation;

use std::u64;

use board_repr::{board::BitBoard, fen::Fen, game_state, piece::Color, square::Square};
use move_generation::tables::{
    generate_bishop_attacks, generate_rook_attacks, get_bishop_relevant_occupancy_mask,
    get_king_attack_mask, get_knight_attack_mask, get_pawn_attack_mask,
    get_rook_relevant_occupancy_mask,
};
use strum::IntoEnumIterator;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let mut blocker = BitBoard::default();
    blocker.set_bit_value(Square::C5);
    blocker.set_bit_value(Square::G7);
    blocker.set_bit_value(Square::A1);
    blocker.set_bit_value(Square::F2);
    blocker.debug();
    let bb = generate_bishop_attacks(Square::D4, blocker);
    bb.debug();
}
