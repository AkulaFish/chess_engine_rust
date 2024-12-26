use uci::uci::UCI;

pub mod board_repr;
pub mod evaluation;
pub mod move_generation;
pub mod search;
pub mod uci;
pub mod utils;

const _START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const _POSITION_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
const _POSITION_4: &str = "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1";
const _POSITION_5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/2N5/PPP2nPP/RNBQK2R b KQ - 1 8";
const _POSITION_6: &str =
    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
const _TRICKY_POSITION: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
const _EN_PASSANT: &str = "r2qk2r/pp3ppp/2nb1n2/3pp3/1PpPP1b1/P4NP1/2PQBP1P/RNB2RK1 b kq b3 0 9";
const _EMPTY: &str = "8/8/8/8/8/8/8/8 w - - 0 1";
const _PROMOTION: &str = "8/P7/8/8/8/8/p7/8 b - - 0 1";
const _CASTLING: &str = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";

fn main() {
    UCI::uci_loop();
}
