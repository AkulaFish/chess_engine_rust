use std::ops::BitAnd;

#[derive(Debug, Default, Clone, Copy)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn from(bitmap: u64) -> Self {
        Self(bitmap)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
