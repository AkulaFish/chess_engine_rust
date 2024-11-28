use std::io::stdin;

use board_repr::fen::Fen;
use move_generation::{generator::MoveGenerator, move_list::MoveList, moves::MoveType};
use utils::traits::DisplayExtension;

pub mod board_repr;
pub mod move_generation;
pub mod utils;

const _START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const _FEN_1: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
const _EN_PASSANT: &str = "r2qk2r/pp3ppp/2nb1n2/3pp3/1PpPP1b1/P4NP1/2PQBP1P/RNB2RK1 b kq b3 0 9";
const _EMPTY: &str = "8/8/8/8/8/8/8/8 w - - 0 1";
const _PROMOTION: &str = "8/P7/8/8/8/8/p7/8 b - - 0 1";
const _CASTLING: &str = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";

fn main() {
    let mut board = Fen::to_board(_START_FEN);
    let mg = MoveGenerator::new();

    loop {
        board.display();
        let mut move_list = MoveList::new();
        mg.generate_moves(&board, &mut move_list, MoveType::All);
        move_list.display();

        println!("Please input your move index.");
        let mut index = String::new();
        stdin().read_line(&mut index).expect("Failed to read line");
        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let m = move_list.moves[index];
        board.make_move(m, &mg);
    }
}
