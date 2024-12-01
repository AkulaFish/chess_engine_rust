use crate::move_generation::{generator::MoveGenerator, moves::Move};

use super::{
    board::Board,
    piece::{Color, Piece},
    square::Square,
};

// Board manipulations
impl Board {
    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        let square_bb = square.get_bitboard();
        self.bitboards[piece as usize] |= square_bb;
        self.piece_by_square[square as usize] = piece;
        self.occupancy[piece.color() as usize] |= square_bb;
    }

    pub fn remove_piece(&mut self, square: Square, piece: Piece) {
        let square_bb = square.get_bitboard();
        self.bitboards[piece as usize] &= !square_bb;
        self.piece_by_square[square as usize] = Piece::None;
        self.occupancy[piece.color() as usize] &= !square_bb;
    }

    pub fn move_piece(&mut self, source_square: Square, target_square: Square, piece: Piece) {
        self.remove_piece(source_square, piece);
        self.set_piece(target_square, piece);
    }

    // source_square: source square of the move we want to reverse
    // target_square: target square of the move we want to reverse
    pub fn reverse_move(&mut self, source_square: Square, target_square: Square, piece: Piece) {
        self.remove_piece(target_square, piece);
        self.set_piece(source_square, piece);
    }
}

impl Board {
    #[inline(always)]
    pub fn make_move(&mut self, move_data: Move, mg: &MoveGenerator) -> bool {
        let mut game_state = self.game_state;
        game_state.next_move = move_data;
        self.history.push(game_state);

        let source_square = move_data.source_square();
        let target_square = move_data.target_square();
        let piece = move_data.piece();
        let captured_piece = move_data.captured_piece();
        let promoted_piece = move_data.promoted_piece();
        let en_passant = move_data.en_passant();
        let castling = move_data.castling();
        let double_push = move_data.double_push();
        let en_passant_square = Square::get_by_index(target_square as u8 ^ 8);

        let is_capture = captured_piece != Piece::None;
        let is_promotion = promoted_piece != Piece::None;

        self.game_state.halfmove_clock += 1;

        if self.game_state.en_passant_target.is_some() {
            self.game_state.en_passant_target = None;
        }

        if is_capture {
            self.remove_piece(target_square, captured_piece);
            self.game_state.halfmove_clock = 0;

            if captured_piece.is_rook() {
                self.castle_settings_mut().set_by_square(target_square);
            }
        }

        if !piece.is_pawn() {
            self.move_piece(source_square, target_square, piece);
        } else {
            self.remove_piece(source_square, piece);
            self.set_piece(
                target_square,
                if is_promotion { promoted_piece } else { piece },
            );

            if en_passant {
                self.remove_piece(en_passant_square, piece.to_color(self.opponent_color()));
            }

            if double_push {
                self.game_state.en_passant_target = Some(en_passant_square);
            }
        }

        if piece.is_king() || piece.is_rook() {
            self.castle_settings_mut().set_by_square(source_square);
        }

        if castling {
            let rook = Piece::WhiteRook.to_color(self.active_color());
            match target_square {
                Square::G1 => self.move_piece(Square::H1, Square::F1, rook),
                Square::C1 => self.move_piece(Square::A1, Square::D1, rook),
                Square::G8 => self.move_piece(Square::H8, Square::F8, rook),
                Square::C8 => self.move_piece(Square::A8, Square::D8, rook),
                _ => panic!(
                    "Target square of castle move does not match any possible castle squares."
                ),
            }
        }

        self.game_state.active_color = self.opponent_color();

        if self.active_color() == Color::Black {
            self.game_state.fullmove_number += 1;
        }

        let king_square = self.bitboards[Piece::WhiteKing.to_color(self.opponent_color()) as usize]
            .lsb_bit_square();
        let is_legal = !mg.is_square_attacked(king_square, self.active_color(), self);
        if !is_legal {
            self.unmake_move();
        }

        is_legal
    }

    #[inline(always)]
    pub fn unmake_move(&mut self) {
        self.game_state = self.history.pop();

        let move_data = self.game_state.next_move;

        let source_square = move_data.source_square();
        let target_square = move_data.target_square();
        let piece = move_data.piece();
        let captured_piece = move_data.captured_piece();
        let promoted_piece = move_data.promoted_piece();
        let en_passant = move_data.en_passant();
        let castling = move_data.castling();
        let en_passant_square = Square::get_by_index(target_square as u8 ^ 8);

        if promoted_piece == Piece::None {
            self.reverse_move(source_square, target_square, piece);
        } else {
            self.remove_piece(target_square, promoted_piece);
            self.set_piece(
                source_square,
                Piece::WhitePawn.to_color(self.active_color()),
            );
        }

        if castling {
            let rook = Piece::WhiteRook.to_color(self.active_color());
            match target_square {
                Square::G1 => self.reverse_move(Square::H1, Square::F1, rook),
                Square::C1 => self.reverse_move(Square::A1, Square::D1, rook),
                Square::G8 => self.reverse_move(Square::H8, Square::F8, rook),
                Square::C8 => self.reverse_move(Square::A8, Square::D8, rook),
                _ => panic!(
                    "Target square of castle move does not match any possible castle squares."
                ),
            };
        }

        if captured_piece != Piece::None {
            self.set_piece(target_square, captured_piece);
        }

        if en_passant {
            self.set_piece(
                en_passant_square,
                Piece::WhitePawn.to_color(self.opponent_color()),
            );
        }
    }
}
