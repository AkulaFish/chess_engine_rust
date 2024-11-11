use std::sync::Arc;

use board_repr::square::Square;
use move_generation::{generator::MoveGenerator, tables::get_rook_relevant_occupancy_mask};
use strum::IntoEnumIterator;

pub mod board_repr;
pub mod move_generation;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {}
