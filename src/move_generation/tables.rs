use crate::board_repr::{bit_board::BitBoard, piece::Color, square::Square};
use strum::IntoEnumIterator;

use super::magics::{BISHOP_MAGICS, BISHOP_TABLE_SIZE, ROOK_MAGICS, ROOK_TABLE_SIZE};

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
    let mut pawn_tables = [[BitBoard::default(); 64], [BitBoard::default(); 64]];

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
    let mut knight_tables = [BitBoard::default(); 64];

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
    let mut king_attacks = [BitBoard::default(); 64];

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

pub fn generate_bishop_attack_masks() -> [BitBoard; BISHOP_TABLE_SIZE] {
    let mut offset = 0;
    let mut table = [BitBoard::default(); BISHOP_TABLE_SIZE];
    for square in Square::iter() {
        let mask = get_bishop_relevant_occupancy_mask(square);
        let bits = mask.count_ones();
        let permutations = 2u64.pow(bits);
        let shift = 64 - bits;
        let magic_number = BISHOP_MAGICS[square.index() as usize];

        let blockers = generate_blockers(mask);
        let attacks = generate_bishop_attacks(square, &blockers);

        for i in 0..permutations {
            let blocker_board = blockers[i as usize];
            let block = blocker_board & mask;
            let index = ((block.value().wrapping_mul(magic_number) >> shift) + offset) as usize;

            if table[index] == BitBoard::default() {
                table[index] = attacks[i as usize]
            } else {
                panic!("Error while initializing magic piece attacks.")
            }
        }
        offset += permutations;
    }

    table
}
pub fn generate_bishop_attacks(square: Square, blockers: &[BitBoard]) -> Vec<BitBoard> {
    let mut attacks: Vec<BitBoard> = Vec::new();

    for blocker in blockers {
        let attack = generate_bishop_attack(square, *blocker);
        attacks.push(attack);
    }

    attacks
}

pub fn generate_bishop_attack(square: Square, blocker: BitBoard) -> BitBoard {
    let mut occupancy = BitBoard::default();
    let tr = square.rank();
    let tf = square.file();

    let top_ranks = (0..tr).rev();
    let bottom_ranks = (tr + 1)..8;
    let right_files = (tf + 1)..8;
    let left_files = (0..tf).rev();

    for (r, f) in top_ranks.clone().zip(right_files.clone()) {
        let square_bb = Square::from_file_and_rank(f, r).get_bitboard();
        occupancy |= square_bb;

        if !(square_bb & blocker).empty() {
            break;
        }
    }

    for (r, f) in top_ranks.zip(left_files.clone()) {
        let square_bb = Square::from_file_and_rank(f, r).get_bitboard();
        occupancy |= square_bb;

        if !(square_bb & blocker).empty() {
            break;
        }
    }

    for (r, f) in bottom_ranks.clone().zip(right_files) {
        let square_bb = Square::from_file_and_rank(f, r).get_bitboard();
        occupancy |= square_bb;

        if !(square_bb & blocker).empty() {
            break;
        }
    }

    for (r, f) in bottom_ranks.zip(left_files) {
        let square_bb = Square::from_file_and_rank(f, r).get_bitboard();
        occupancy |= square_bb;

        if !(square_bb & blocker).empty() {
            break;
        }
    }

    occupancy
}

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

//////////////////////////////////
//    GENERATE ROOK TABLES      //
//////////////////////////////////

pub fn generate_rook_attack_masks() -> [BitBoard; ROOK_TABLE_SIZE] {
    let mut offset = 0;
    let mut table = [BitBoard::default(); ROOK_TABLE_SIZE];
    for square in Square::iter() {
        let mask = get_rook_relevant_occupancy_mask(square);
        let bits = mask.count_ones();
        let permutations = 2u64.pow(bits);
        let shift = 64 - bits;
        let magic_number = ROOK_MAGICS[square.index() as usize];

        let blockers = generate_blockers(mask);
        let attacks = generate_rook_attacks(square, &blockers);

        for i in 0..permutations {
            let blocker_board = blockers[i as usize];
            let block = blocker_board & mask;
            let index = ((block.value().wrapping_mul(magic_number) >> shift) + offset) as usize;

            if table[index] == BitBoard::default() {
                table[index] = attacks[i as usize]
            } else {
                panic!("Error while initializing magic piece attacks.")
            }
        }
        offset += permutations;
    }

    table
}
pub fn generate_rook_attacks(square: Square, blockers: &[BitBoard]) -> Vec<BitBoard> {
    let mut attacks: Vec<BitBoard> = Vec::new();

    for blocker in blockers {
        let attack = generate_rook_attack(square, *blocker);
        attacks.push(attack);
    }

    attacks
}

pub fn generate_rook_attack(square: Square, blocker: BitBoard) -> BitBoard {
    let mut occupancy = BitBoard::default();
    let tr = square.rank();
    let tf = square.file();

    for r in (0..tr).rev() {
        let square_bb = Square::from_file_and_rank(tf, r).get_bitboard();
        occupancy |= square_bb;

        if !(square_bb & blocker).empty() {
            break;
        }
    }

    for r in (tr + 1)..8 {
        let square_bb = Square::from_file_and_rank(tf, r).get_bitboard();
        occupancy |= square_bb;

        if !(square_bb & blocker).empty() {
            break;
        }
    }

    for f in (0..tf).rev() {
        let square_bb = Square::from_file_and_rank(f, tr).get_bitboard();
        occupancy |= square_bb;

        if !(square_bb & blocker).empty() {
            break;
        }
    }

    for f in (tf + 1)..8 {
        let square_bb = Square::from_file_and_rank(f, tr).get_bitboard();
        occupancy |= square_bb;

        if !(square_bb & blocker).empty() {
            break;
        }
    }

    occupancy
}

pub fn get_rook_relevant_occupancy_mask(square: Square) -> BitBoard {
    // TODO: this is pbly rook attacks
    let mut occupancy = BitBoard::default();
    let tr = square.rank();
    let tf = square.file();

    let mut r = 1;
    loop {
        if r > 6 {
            break;
        }

        if r == tr {
            r += 1;
            continue;
        }

        let square_bb = Square::from_file_and_rank(tf, r).get_bitboard();
        occupancy |= square_bb;
        r += 1;
    }

    let mut f = 1;
    loop {
        if f > 6 {
            break;
        }

        if f == tf {
            f += 1;
            continue;
        }

        let square_bb = Square::from_file_and_rank(f, tr).get_bitboard();
        occupancy |= square_bb;
        f += 1;
    }

    occupancy
}

//////////////////////////////////////
//    GENERATE BLOCKERS TABLES      //
//////////////////////////////////////

pub fn generate_blockers(mask: BitBoard) -> Vec<BitBoard> {
    let d: BitBoard = mask;
    let mut bb_blocker_boards = Vec::new();
    let mut n: BitBoard = BitBoard::default();

    // Carry-Rippler
    // https://www.chessprogramming.org/Traversing_Subsets_of_a_Set
    loop {
        bb_blocker_boards.push(n);
        n = n.wrapping_sub(d) & d;
        if n.empty() {
            break;
        }
    }

    bb_blocker_boards
}
