use crate::board_repr::{board::Board, piece::Color, square::Square};

type Psqt = [i16; 64];

#[rustfmt::skip]
const KING_SCORE: Psqt = [
    0, 0,  0,  0,  0,  0,  0, 0,
    0, 0,  5,  5,  5,  5,  0, 0,
    0, 5,  5, 10, 10,  5,  5, 0,
    0, 5, 10, 20, 20, 10,  5, 0,
    0, 5, 10, 20, 20, 10,  5, 0,
    0, 0,  5, 10, 10,  5,  0, 0,
    0, 5,  5, -5, -5,  0,  5, 0,
    0, 0,  5,  0, 15,  0, 10, 0,
];

#[rustfmt::skip]
const ROOK_SCORE: Psqt = [
    50, 50, 50, 50, 50, 50, 50, 50,
    50, 50, 50, 50, 50, 50, 50, 50,
    0,  0, 10, 20, 20, 10,  0,  0,
    0,  0, 10, 20, 20, 10,  0,  0,
    0,  0, 10, 20, 20, 10,  0,  0,
    0,  0, 10, 20, 20, 10,  0,  0,
    0,  0, 10, 20, 20, 10,  0,  0,
    0,  0,  0, 20, 20,  0,  0,  0,
];

#[rustfmt::skip]
const BISHOP_SCORE: Psqt = [
    0,  0,   0,  0,  0,   0,  0, 0,
    0,  0,   0,  0,  0,   0,  0, 0,
    0,  0,   0, 10, 10,   0,  0, 0,
    0,  0,  10, 20, 20,  10,  0, 0,
    0,  0,  10, 20, 20,  10,  0, 0,
    0, 10,   0,  0,  0,   0, 10, 0,
    0, 30,   0,  0,  0,   0, 30, 0,
    0,  0, -10,  0,  0, -10,  0, 0,
];

#[rustfmt::skip]
const KNIGHT_SCORE: Psqt = [
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,  10,  10,   0,   0,  -5,
    -5,   5,  20,  20,  20,  20,   5,  -5,
    -5,  10,  20,  30,  30,  20,  10,  -5,
    -5,  10,  20,  30,  30,  20,  10,  -5,
    -5,   5,  20,  10,  10,  20,   5,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5, -10,   0,   0,   0,   0, -10,  -5,
];

#[rustfmt::skip]
const PAWN_SCORE: Psqt = [
    90,  90,  90,  90,  90,  90,  90,  90,
    30,  30,  30,  40,  40,  30,  30,  30,
    20,  20,  20,  30,  30,  30,  20,  20,
    10,  10,  10,  20,  20,  10,  10,  10,
     5,   5,  10,  20,  20,   5,   5,   5,
     0,   0,   0,   5,   5,   0,   0,   0,
     0,   0,   0, -10, -10,   0,   0,   0,
     0,   0,   0,   0,   0,   0,   0,   0,
];

#[rustfmt::skip]
pub const FLIP: [usize; 64] = [
    56, 57, 58, 59, 60, 61, 62, 63,
    48, 49, 50, 51, 52, 53, 54, 55,
    40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39,
    24, 25, 26, 27, 28, 29, 30, 31,
    16, 17, 18, 19, 20, 21, 22, 23,
     8,  9, 10, 11, 12, 13, 14, 15,
     0,  1,  2,  3,  4,  5,  6,  7,
];

/*
 * Evaluates the position on the board.
 * Negative value if black is winning, positive - if white is winning.
 */
pub fn evaluate(board: &Board) -> i16 {
    let mut score = 0i16;

    for (square_index, piece) in board.piece_by_square.iter().enumerate() {
        let square = Square::get_by_index(square_index as u8);
        score += piece.eval_value();
        let psqt = match piece {
            p if p.is_king() => KING_SCORE,
            p if p.is_rook() => ROOK_SCORE,
            p if p.is_bishop() => BISHOP_SCORE,
            p if p.is_knight() => KNIGHT_SCORE,
            p if p.is_pawn() => PAWN_SCORE,
            _ => [0; 64],
        };
        let square_index = if piece.color() == Color::White {
            square as usize
        } else {
            FLIP[square as usize]
        };

        score += psqt[square_index];
    }

    score
}
