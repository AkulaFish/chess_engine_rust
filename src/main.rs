use board_repr::{fen::Fen, piece::Color, square::Square};
use move_generation::generator::MoveGenerator;

pub mod board_repr;
pub mod move_generation;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const FEN_1: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
const EN_PASSAN: &str = "r2qk2r/pp3ppp/2nb1n2/3pp3/1PpPP1b1/P4NP1/2PQBP1P/RNB2RK1 b kq b3 0 9";
const EMPTY: &str = "8/8/8/8/8/8/8/8 w - - 0 1";

fn main() {
    let board = Fen::to_board(EN_PASSAN);
    let mg = MoveGenerator::new();
    let is_attacked = board.is_square_attacked(Square::G5, Color::White, &mg);
    board.display();

    println!("{}", is_attacked);
}
