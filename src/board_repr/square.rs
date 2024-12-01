use strum_macros::{Display, EnumIter, EnumString, FromRepr};

use super::bit_board::BitBoard;

const SQUARE_BITBOARDS: [u64; 64] = init_bb_squares();

const fn init_bb_squares() -> [u64; 64] {
    let mut bb_squares = [0; 64];
    let mut i = 0;

    while i < 64 {
        bb_squares[i] = 1u64 << i;
        i += 1;
    }

    bb_squares
}

#[rustfmt::skip]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, EnumString, Display, FromRepr)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

impl Square {
    pub fn index(&self) -> u8 {
        *self as u8
    }

    pub fn rank(&self) -> u8 {
        self.index() / 8
    }

    pub fn file(&self) -> u8 {
        self.index() % 8
    }

    pub fn from_file_and_rank(file: u8, rank: u8) -> Self {
        Self::get_by_index(rank * 8 + file)
    }

    pub fn get_by_index(n: u8) -> Self {
        Self::from_repr(n).unwrap_or_else(|| panic!("Square index out of range: {n}"))
    }

    pub fn get_bitboard(&self) -> BitBoard {
        BitBoard::from(SQUARE_BITBOARDS[*self as usize])
    }

    pub fn add_rank(&self, increment: i8) -> Self {
        Self::from_file_and_rank(self.file(), (self.rank() as i8 + increment) as u8)
    }

    pub fn add_file(&self, increment: i8) -> Self {
        Self::from_file_and_rank(self.file() + increment as u8, self.rank())
    }
}

//////////////////
//  Unit Tests  //
//////////////////
#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Square;

    #[test]
    fn test_get_by_index() {
        for square in Square::iter() {
            let s = Square::get_by_index(square.index());
            assert!(s == square)
        }
    }

    #[test]
    fn test_from_file_and_rank() {
        for square in Square::iter() {
            let s = Square::from_file_and_rank(square.file(), square.rank());
            assert!(s == square)
        }
    }
}
