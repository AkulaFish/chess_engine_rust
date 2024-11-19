use crate::board_repr::{bit_board::BitBoard, piece::Color, square::Square};

use super::magics::{Magic, BISHOP_TABLE_SIZE, ROOK_TABLE_SIZE};

pub struct MoveGenerator {
    pub king: [BitBoard; 64],
    pub pawn: [[BitBoard; 64]; 2],
    pub rook: [BitBoard; ROOK_TABLE_SIZE],
    pub bishop: [BitBoard; BISHOP_TABLE_SIZE],
    pub knight: [BitBoard; 64],
    pub rook_magics: [Magic; 64],
    pub bishop_magics: [Magic; 64],
}

impl MoveGenerator {
    pub fn get_bishop_attack(&self, square: Square, blocker: BitBoard) -> BitBoard {
        let magic = self.bishop_magics[square as usize];
        let index = magic.index(blocker);
        self.bishop[index]
    }

    pub fn get_rook_attack(&self, square: Square, blocker: BitBoard) -> BitBoard {
        let magic = self.rook_magics[square as usize];
        let index = magic.index(blocker);
        self.rook[index]
    }

    pub fn get_queen_attack(&self, square: Square, blocker: BitBoard) -> BitBoard {
        let bishop_attack = self.get_bishop_attack(square, blocker);
        let rook_attack = self.get_rook_attack(square, blocker);

        rook_attack | bishop_attack
    }

    pub fn get_king_attack(&self, square: Square) -> BitBoard {
        self.king[square as usize]
    }

    pub fn get_knight_attack(&self, square: Square) -> BitBoard {
        self.knight[square as usize]
    }

    pub fn get_pawn_attack(&self, square: Square, color: Color) -> BitBoard {
        self.pawn[color as usize][square.index() as usize]
    }
}
