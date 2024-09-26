pub mod board_repr;

use board_repr::fen::Fen;

const START_FEN: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 1";

fn main() {
    Fen::draw_fen(START_FEN);
}
