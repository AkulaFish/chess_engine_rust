use board_repr::{board::BitBoard, square::Square};
use move_generation::{
    generator::MoveGenerator, magics::BISHOP_MAGICS, tables::get_bishop_relevant_occupancy_mask,
};

pub mod board_repr;
pub mod move_generation;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let mg = MoveGenerator::new();

    let mask = get_bishop_relevant_occupancy_mask(Square::A8);
    let mut blocker = BitBoard::default();
    blocker.set_bit_value(Square::D5);

    let magic = BISHOP_MAGICS[Square::A8.index() as usize];
    let block = blocker & mask;
    let index = (block.value().wrapping_mul(magic) >> (64 - mask.count_ones())) as usize;
    let result = mg.bishop[index];
    result.debug();
}
