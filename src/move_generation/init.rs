use super::{
    generator::MoveGenerator,
    tables::{
        generate_bishop_attack_masks, generate_king_attack_masks, generate_knight_attack_masks,
        generate_pawn_attack_masks, generate_rook_attack_masks,
    },
};

impl Default for MoveGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveGenerator {
    pub fn new() -> Self {
        let king = generate_king_attack_masks();
        let pawn = generate_pawn_attack_masks();
        let knight = generate_knight_attack_masks();
        let (bishop, bishop_magics) = generate_bishop_attack_masks();
        let (rook, rook_magics) = generate_rook_attack_masks();

        Self {
            king,
            pawn,
            knight,
            bishop,
            rook,
            bishop_magics,
            rook_magics,
        }
    }
}
