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
    let mut pawn_tables = [arr![BitBoard::default(); 64], arr![BitBoard::default(); 64]];

    Square::iter().for_each(|s| {
        pawn_tables[Color::White as usize][s.index() as usize] =
            get_pawn_attack_mask(Color::White, s);

        pawn_tables[Color::Black as usize][s.index() as usize] =
            get_pawn_attack_mask(Color::Black, s);
    });

    pawn_tables
}

pub fn get_pawn_attack_mask(color: Color, square: Square) -> BitBoard {
    let bitboard = square.get_bitboard();
    let mut attack = BitBoard::default();

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

//////////////////////////////////
//    GENERATE KNIGHT TABLES    //
//////////////////////////////////

pub fn generate_knight_attack_masks() -> [BitBoard; 64] {
    let mut knight_tables = arr![BitBoard::default(); 64];

    for square in Square::iter() {
        knight_tables[square.index() as usize] = get_knight_attack_mask(square);
    }

    knight_tables
}

pub fn get_knight_attack_mask(square: Square) -> BitBoard {
    let bitboard = square.get_bitboard();
    let mut attack = BitBoard::default();

    if !(bitboard & NOT_A_FILE).empty() {
        attack |= bitboard >> 17;
        attack |= bitboard << 15;
    }

    if !(bitboard & NOT_H_FILE).empty() {
        attack |= bitboard >> 15;
        attack |= bitboard << 17;
    }

    if !(bitboard & NOT_AB_FILE).empty() {
        attack |= bitboard >> 10;
        attack |= bitboard << 6;
    }

    if !(bitboard & NOT_GH_FILE).empty() {
        attack |= bitboard >> 6;
        attack |= bitboard << 10;
    }

    attack
}

////////////////////////////////
//    GENERATE KNIG TABLES    //
////////////////////////////////

pub fn generate_king_attack_masks() -> [BitBoard; 64] {
    let mut king_attacks = arr![BitBoard::default(); 64];

    for s in Square::iter() {
        king_attacks[s.index() as usize] = get_king_attack_mask(s);
    }

    king_attacks
}

pub fn get_king_attack_mask(square: Square) -> BitBoard {
    let bitboard = square.get_bitboard();
    let mut attack = BitBoard::default();

    if !(bitboard & NOT_A_FILE).empty() {
        // left attack
        attack |= bitboard >> 1;
        // top left attack
        attack |= bitboard >> 9;
        // bottom left attack
        attack |= bitboard << 7;
    }

    if !(bitboard & NOT_H_FILE).empty() {
        // right attack
        attack |= bitboard << 1;
        // top right attack
        attack |= bitboard >> 7;
        // bottom right attack;
        attack |= bitboard << 9;
    }

    // top attack
    attack |= bitboard >> 8;
    // bottom attack
    attack |= bitboard << 8;

    attack
}

//////////////////////////////////
//    GENERATE BISHOP TABLES    //
//////////////////////////////////

pub fn get_bishop_relevant_occupancy_mask(square: Square) -> BitBoard {
    // TODO: this is pbly rook attacks
    let mut occupancy = BitBoard::default();
    let tr = square.rank();
    let tf = square.file();

    let mut r = tr + 1;
    let mut f = tf + 1;
    loop {
        if r > 6 || f > 6 {
            break;
        }
        let square_bb = Square::from_file_and_rank(f, r).get_bitboard();
        occupancy |= square_bb;
        r += 1;
        f += 1;
    }

    r = tr + 1;
    f = if tf == 0 { 0 } else { tf - 1 };
    loop {
        if r > 6 || f < 1 {
            break;
        }
        let square_bb = Square::from_file_and_rank(f, r).get_bitboard();
        occupancy |= square_bb;
        r += 1;
        f -= 1;
    }

    r = if tr == 0 { 0 } else { tr - 1 };
    f = tf + 1;
    loop {
        if r < 1 || f > 6 {
            break;
        }
        let square_bb = Square::from_file_and_rank(f, r).get_bitboard();
        occupancy |= square_bb;
        r -= 1;
        f += 1;
    }

    r = if tr == 0 { 0 } else { tr - 1 };
    f = if tf == 0 { 0 } else { tf - 1 };
    loop {
        if r < 1 || f < 1 {
            break;
        }
        let square_bb = Square::from_file_and_rank(f, r).get_bitboard();
        occupancy |= square_bb;
        r -= 1;
        f -= 1;
    }

    occupancy
}
