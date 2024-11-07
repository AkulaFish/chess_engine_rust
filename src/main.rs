pub mod board_repr;
pub mod move_generation;

use std::{
    io::{stdin, stdout},
    u64,
};

use board_repr::{board::BitBoard, fen::Fen, game_state, piece::Color, square::Square};
use move_generation::tables::{
    generate_bishop_attacks, generate_blocker_table, generate_rook_attacks,
    get_bishop_relevant_occupancy_mask, get_king_attack_mask, get_knight_attack_mask,
    get_pawn_attack_mask, get_rook_relevant_occupancy_mask,
};
use strum::IntoEnumIterator;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let rook_mask = get_rook_relevant_occupancy_mask(Square::A1);
    let bishop_mask = get_bishop_relevant_occupancy_mask(Square::A1);

    for index in 0..400 {
        let bb = generate_blocker_table(index, bishop_mask);
        bb.debug();
        let mut s = String::new();
        stdin().read_line(&mut s);
    }
}
