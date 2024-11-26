use board_repr::fen::Fen;
use move_generation::{generator::MoveGenerator, move_list::MoveList};

pub mod board_repr;
pub mod move_generation;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const FEN_1: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
const EN_PASSANT: &str = "r2qk2r/pp3ppp/2nb1n2/3pp3/1PpPP1b1/P4NP1/2PQBP1P/RNB2RK1 b kq b3 0 9";
const EMPTY: &str = "8/8/8/8/8/8/8/8 w - - 0 1";
const PROMOTION: &str = "8/P7/8/8/8/8/p7/8 b - - 0 1";
const CASTLING: &str = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";

fn main() {
    let board = Fen::to_board(EN_PASSANT);
    board.display();

    let mg = MoveGenerator::new();
    let mut move_list = MoveList::new();

    mg.generate_moves(&board, &mut move_list);
    move_list.display();
}
