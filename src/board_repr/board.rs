use strum::IntoEnumIterator;

use crate::move_generation::moves::Move;

use super::bit_board::BitBoard;
use super::game_state::{CastleAvailability, GameState};
use super::piece::{Color, Piece};
use super::square::Square;

const MAX_MOVES: usize = 2048;

pub struct History {
    pub history: [GameState; MAX_MOVES],
    pub count: usize,
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

impl History {
    pub fn new() -> Self {
        Self {
            history: [GameState::default(); MAX_MOVES],
            count: 0,
        }
    }

    pub fn push(&mut self, game_state: GameState) {
        self.history[self.count] = game_state;
        self.count += 1;
    }

    pub fn pop(&mut self) -> GameState {
        self.count -= 1;
        self.history[self.count]
    }
}

#[derive()]
pub struct Board {
    pub bitboards: [BitBoard; 12],
    pub occupancy: [BitBoard; 2],
    pub game_state: GameState,
    pub piece_by_square: [Piece; 64],
    pub history: History,
}

impl Board {
    pub fn new(
        bitboards: [BitBoard; 12],
        active_color: Color,
        castle_settings: CastleAvailability,
        en_passant_target: Option<Square>,
        halfmove_clock: u8,
        fullmove_number: u8,
    ) -> Self {
        let piece_by_square = Self::init_piece_by_square(bitboards);
        let occupancy = Self::init_occupancy(bitboards);
        let next_move = Move::default();
        let history = History::new();
        let game_state = GameState {
            active_color,
            castle_settings,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
            next_move,
        };
        Self {
            bitboards,
            occupancy,
            game_state,
            history,
            piece_by_square,
        }
    }

    pub fn init_occupancy(bitboards: [BitBoard; 12]) -> [BitBoard; 2] {
        let mut white_occupancy = BitBoard::default();
        for bb in bitboards.iter().take(6) {
            white_occupancy |= *bb;
        }

        let mut black_occupancy = BitBoard::default();
        for bb in bitboards.iter().skip(6) {
            black_occupancy |= *bb;
        }

        [white_occupancy, black_occupancy]
    }

    pub fn init_piece_by_square(bitboards: [BitBoard; 12]) -> [Piece; 64] {
        let mut result = [Piece::None; 64];
        for square in Square::iter() {
            for piece in Piece::iter().filter(|p| p != &Piece::None) {
                let occ = bitboards[piece as usize];
                let square_bb = square.get_bitboard();
                if !(occ & square_bb).empty() {
                    result[square as usize] = piece;
                    break;
                }
            }
        }

        result
    }

    pub fn get_occupancies(&self, color: Color) -> BitBoard {
        if color == Color::Both {
            return self.occupancy[Color::White as usize] | self.occupancy[Color::Black as usize];
        }
        self.occupancy[color as usize]
    }

    pub fn active_color(&self) -> Color {
        self.game_state.active_color
    }

    pub fn opponent_color(&self) -> Color {
        self.game_state.active_color.opposite()
    }

    pub fn castle_settings_mut(&mut self) -> &mut CastleAvailability {
        &mut self.game_state.castle_settings
    }

    pub fn castle_settings(&self) -> &CastleAvailability {
        &self.game_state.castle_settings
    }
}
