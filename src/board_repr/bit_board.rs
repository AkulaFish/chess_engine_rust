use std::fmt::{Debug, Display};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, Shr};

use super::square::Square;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn from(bitmap: u64) -> Self {
        Self(bitmap)
    }

    pub fn empty(&self) -> bool {
        self.0 == 0
    }

    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn lsb_index(&self) -> u32 {
        self.0.trailing_zeros()
    }

    pub fn value(self) -> u64 {
        self.0
    }

    pub fn set_bit_value(&mut self, square: Square) {
        self.0 |= 1u64 << square.index();
    }

    pub fn pop_bit_value(&mut self, square: Square) {
        self.0 &= !(1u64 << square.index());
    }

    pub fn get_bit_value(&self, square: Square) -> bool {
        (self.0 & (1u64 << square.index())) != 0
    }

    pub fn display(&self) {
        println!("{}", self);
    }

    pub fn debug(&self) {
        println!("{:?}", self);
    }

    pub fn wrapping_add(&self, rhs: BitBoard) -> Self {
        Self::from(self.0.wrapping_add(rhs.0))
    }

    pub fn wrapping_sub(&self, rhs: BitBoard) -> Self {
        Self::from(self.0.wrapping_sub(rhs.0))
    }

    pub fn wrapping_mul(&self, rhs: BitBoard) -> Self {
        Self::from(self.0.wrapping_mul(rhs.0))
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for rank in 0..8 {
            result.push_str(&format!("{}  ", 8 - rank));
            for file in 0..8 {
                let square = Square::get_by_index(rank * 8 + file);
                result.push_str(&format!(
                    " {} ",
                    if self.get_bit_value(square) { "1" } else { "." }
                ));
            }
            result.push('\n')
        }
        result.push_str("    a  b  c  d  e  f  g  h\n\n");
        result.push_str(&format!("    Decimal: {}\n", self.0));
        result.push_str(&format!("    Hex: {:x}\n\n", self.0));

        write!(f, "{}", result)
    }
}

impl Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for rank in 0..8 {
            result.push_str(&format!("{}  ", 8 - rank));
            for file in 0..8 {
                let square = Square::get_by_index(rank * 8 + file);
                result.push_str(&format!(
                    " {} ",
                    if self.get_bit_value(square) { "1" } else { "." }
                ));
            }
            result.push('\n')
        }
        result.push_str("    a  b  c  d  e  f  g  h\n\n");
        result.push_str(&format!("    Decimal: {}\n", self.0));
        result.push_str(&format!("    Hex: {:x}\n\n", self.0));

        write!(f, "{}", result)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAnd<u64> for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: u64) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = self.0 & rhs.0;
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOr<u64> for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: u64) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.0 | rhs.0;
    }
}

impl BitOrAssign<u64> for BitBoard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 = self.0 | rhs;
    }
}

impl Shl<usize> for BitBoard {
    type Output = BitBoard;

    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<usize> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

//////////////////
//  Unit Tests  //
//////////////////

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_get_bit_value_empty_board() {
        let bb = BitBoard::default();

        for square in Square::iter() {
            let bit = bb.get_bit_value(square);
            assert!(!bit);
        }
    }

    #[test]
    fn test_get_bit_value_full_board() {
        let bb = BitBoard::from(u64::MAX);

        for square in Square::iter() {
            let bit = bb.get_bit_value(square);
            assert!(bit);
        }
    }

    #[test]
    fn test_set_bit_value() {
        let mut bb = BitBoard::default();

        for square in Square::iter() {
            bb.set_bit_value(square);
        }

        assert_eq!(bb.0, u64::MAX);
    }

    #[test]
    fn test_bit_and() {
        let bb = BitBoard::default();
        let bbf = BitBoard::from(u64::MAX);

        assert_eq!(bb & bbf, BitBoard::default());
        assert_eq!(bb & u64::MAX, BitBoard::default());
    }

    #[test]
    fn test_bit_or() {
        let bb = BitBoard::default();
        let bbf = BitBoard::from(u64::MAX);

        assert_eq!(bb | bbf, BitBoard::from(u64::MAX));
        assert_eq!(bb | u64::MAX, BitBoard::from(u64::MAX));
    }

    #[test]
    fn test_bit_shift_left() {
        let bb = BitBoard::from(15u64);

        assert_eq!(bb << 5, BitBoard::from(15u64 << 5));
    }

    #[test]
    fn test_bit_shift_right() {
        let bb = BitBoard::from(15u64);

        assert_eq!(bb >> 5, BitBoard::from(15u64 >> 5));
    }
}
