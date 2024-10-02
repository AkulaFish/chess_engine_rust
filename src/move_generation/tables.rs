use crate::board_repr::{board::BitBoard, piece::Color, square::Square};
use arr_macro::arr;
use strum::IntoEnumIterator;

//////////////////
//    CONSTS    //
//////////////////

/*
    8   .  1  1  1  1  1  1  1
    7   .  1  1  1  1  1  1  1
    6   .  1  1  1  1  1  1  1
    5   .  1  1  1  1  1  1  1
    4   .  1  1  1  1  1  1  1
    3   .  1  1  1  1  1  1  1
    2   .  1  1  1  1  1  1  1
    1   .  1  1  1  1  1  1  1
        a  b  c  d  e  f  g  h
*/
const NOT_A_FILE: u64 = 0xfefefefefefefefe;

/*
    8   1  1  1  1  1  1  1  .
    7   1  1  1  1  1  1  1  .
    6   1  1  1  1  1  1  1  .
    5   1  1  1  1  1  1  1  .
    4   1  1  1  1  1  1  1  .
    3   1  1  1  1  1  1  1  .
    2   1  1  1  1  1  1  1  .
    1   1  1  1  1  1  1  1  .
        a  b  c  d  e  f  g  h
*/
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

/*
    8   .  .  1  1  1  1  1  1
    7   .  .  1  1  1  1  1  1
    6   .  .  1  1  1  1  1  1
    5   .  .  1  1  1  1  1  1
    4   .  .  1  1  1  1  1  1
    3   .  .  1  1  1  1  1  1
    2   .  .  1  1  1  1  1  1
    1   .  .  1  1  1  1  1  1
        a  b  c  d  e  f  g  h
*/
const NOT_AB_FILE: u64 = 0xfcfcfcfcfcfcfcfc;

/*
    8   1  1  1  1  1  1  .  .
    7   1  1  1  1  1  1  .  .
    6   1  1  1  1  1  1  .  .
    5   1  1  1  1  1  1  .  .
    4   1  1  1  1  1  1  .  .
    3   1  1  1  1  1  1  .  .
    2   1  1  1  1  1  1  .  .
    1   1  1  1  1  1  1  .  .
        a  b  c  d  e  f  g  h
*/
const NOT_GH_FILE: u64 = 0x3f3f3f3f3f3f3f3f;

////////////////////////////////
//    GENERATE PAWN TABLES    //
////////////////////////////////

pub fn generate_pawn_attack_masks() -> [[BitBoard; 64]; 2] {
    // TODO: This might be slow, so I want to consider other options later
    let mut pawn_tables: [[BitBoard; 64]; 2] =
        [arr![BitBoard::default(); 64], arr![BitBoard::default(); 64]];

    Square::iter().for_each(|s| {
        pawn_tables[Color::White as usize][s.index() as usize] =
            get_pawn_attack_mask(Color::White, s);

        pawn_tables[Color::Black as usize][s.index() as usize] =
            get_pawn_attack_mask(Color::Black, s);
    });

    pawn_tables
}

pub fn get_pawn_attack_mask(color: Color, square: Square) -> BitBoard {
    let mut bitboard = BitBoard::default();
    let mut attack = BitBoard::default();

    bitboard.set_bit_value(square);

    if color == Color::White {
        //If on the left edge of the board - do not add pawn diagonal left attack square
        if !(bitboard & NOT_A_FILE).empty() {
            attack |= bitboard >> 9;
        }
        //If on the right edge of the board - do not add pawn diagonal right attack square
        if !(bitboard & NOT_H_FILE).empty() {
            attack |= bitboard >> 7;
        }
    }

    if color == Color::Black {
        //If on the left edge of the board - do not add pawn diagonal left attack square
        if !(bitboard & NOT_A_FILE).empty() {
            attack |= bitboard << 9;
        }
        //If on the right edge of the board - do not add pawn diagonal right attack square
        if !(bitboard & NOT_H_FILE).empty() {
            attack |= bitboard << 7;
        }
    }

    attack
}
