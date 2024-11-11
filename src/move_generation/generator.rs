use crate::{board_repr::board::BitBoard, move_generation::tables::generate_rook_attack_masks};

use super::{
    magics::{BISHOP_TABLE_SIZE, ROOK_TABLE_SIZE},
    tables::{
        generate_bishop_attack_masks, generate_king_attack_masks, generate_knight_attack_masks,
        generate_pawn_attack_masks,
    },
};

pub struct MoveGenerator {
    pub king: [BitBoard; 64],
    pub pawn: [[BitBoard; 64]; 2],
    pub rook: [BitBoard; ROOK_TABLE_SIZE],
    pub bishop: [BitBoard; BISHOP_TABLE_SIZE],
    pub knight: [BitBoard; 64],
}

impl MoveGenerator {
    pub fn new() -> Self {
        let king = generate_king_attack_masks();
        let pawn = generate_pawn_attack_masks();
        let knight = generate_knight_attack_masks();
        let bishop = generate_bishop_attack_masks();
        let rook = generate_rook_attack_masks();

        Self {
            king,
            pawn,
            knight,
            bishop,
            rook,
        }
    }
}
