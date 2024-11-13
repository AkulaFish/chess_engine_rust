use crate::board_repr::{board::BitBoard, piece::Color, square::Square};

use super::{
    magics::{BISHOP_MAGICS, BISHOP_TABLE_SIZE, ROOK_MAGICS, ROOK_TABLE_SIZE},
    tables::get_bishop_relevant_occupancy_mask,
};

pub struct MoveGenerator {
    pub king: [BitBoard; 64],
    pub pawn: [[BitBoard; 64]; 2],
    pub rook: [BitBoard; ROOK_TABLE_SIZE],
    pub bishop: [BitBoard; BISHOP_TABLE_SIZE],
    pub knight: [BitBoard; 64],
}

impl MoveGenerator {
    pub fn get_bishop_attack(&self, square: Square, blocker: BitBoard) -> BitBoard {
        let mask = get_bishop_relevant_occupancy_mask(square);
        let magic = BISHOP_MAGICS[square.index() as usize];

        let block = blocker & mask;
        let index = block.value().wrapping_rem_euclid(magic) >> (64 - mask.count_ones());
        self.bishop[index as usize]
    }

    pub fn get_rook_attack(&self, square: Square, blocker: BitBoard) -> BitBoard {
        let mask = get_bishop_relevant_occupancy_mask(square);
        let magic = ROOK_MAGICS[square.index() as usize];

        let block = blocker & mask;
        let index = block.value().wrapping_rem_euclid(magic) >> (64 - mask.count_ones());
        self.rook[index as usize]
    }

    pub fn get_queen_attack(&self, square: Square, blocker: BitBoard) -> BitBoard {
        let bishop_attack = self.get_bishop_attack(square, blocker);
        let rook_attack = self.get_rook_attack(square, blocker);

        rook_attack | bishop_attack
    }

    pub fn get_kind_attack(&self, square: Square) -> BitBoard {
        self.king[square.index() as usize]
    }

    pub fn get_knight_attack(&self, square: Square) -> BitBoard {
        self.knight[square.index() as usize]
    }

    pub fn get_pawn_attack(&self, square: Square, color: Color) -> BitBoard {
        self.pawn[color as usize][square.index() as usize]
    }
}
