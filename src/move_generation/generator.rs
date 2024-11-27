use crate::board_repr::{
    bit_board::BitBoard,
    board::Board,
    piece::{Color, Piece},
    square::Square,
};

use super::{
    magics::{Magic, BISHOP_TABLE_SIZE, ROOK_TABLE_SIZE},
    move_list::MoveList,
    moves::{Move, MoveType},
};

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

impl MoveGenerator {
    pub fn generate_moves(&self, board: &Board, move_list: &mut MoveList, move_type: MoveType) {
        // Special case. Generate pawn moves
        self.generate_pawn_moves(board, move_list, move_type);

        // Special case. Generate castle moves
        if move_type == MoveType::All || move_type == MoveType::Quite {
            self.generate_castling_moves(board, move_list);
        }

        // Generate moves for other pieces
        self.generate_piece_moves(
            Piece::WhiteKing.to_color(board.active_color),
            board,
            move_list,
            move_type,
        );
        self.generate_piece_moves(
            Piece::WhiteQueen.to_color(board.active_color),
            board,
            move_list,
            move_type,
        );
        self.generate_piece_moves(
            Piece::WhiteRook.to_color(board.active_color),
            board,
            move_list,
            move_type,
        );
        self.generate_piece_moves(
            Piece::WhiteBishop.to_color(board.active_color),
            board,
            move_list,
            move_type,
        );
        self.generate_piece_moves(
            Piece::WhiteKnight.to_color(board.active_color),
            board,
            move_list,
            move_type,
        );
    }

    pub fn generate_piece_moves(
        &self,
        piece: Piece,
        board: &Board,
        move_list: &mut MoveList,
        move_type: MoveType,
    ) {
        let occupancy = board.get_occupancies(Color::Both);
        let friendly_occupancy = board.get_occupancies(board.active_color);
        let opponent_occupancy = board.get_occupancies(board.active_color.opposite());

        let mut piece_bitboard = board.bitboards[piece.to_color(board.active_color) as usize];

        while !piece_bitboard.empty() {
            let source_square = piece_bitboard.lsb_bit_square();
            piece_bitboard.pop_bit_value(source_square);

            let attack = match piece {
                Piece::WhiteKing | Piece::BlackKing => self.get_king_attack(source_square),
                Piece::WhiteKnight | Piece::BlackKnight => self.get_knight_attack(source_square),
                Piece::WhiteRook | Piece::BlackRook => {
                    self.get_rook_attack(source_square, occupancy)
                }
                Piece::WhiteBishop | Piece::BlackBishop => {
                    self.get_bishop_attack(source_square, occupancy)
                }
                Piece::WhiteQueen | Piece::BlackQueen => {
                    self.get_queen_attack(source_square, occupancy)
                }
                _ => panic!("Can not generate moves for this piece: {}", piece),
            };

            let moves = match move_type {
                MoveType::All => attack & !friendly_occupancy,
                MoveType::Quite => attack & occupancy,
                MoveType::Capture => attack & opponent_occupancy,
            };

            self.add_move(board, piece, source_square, moves, move_list);
        }
    }

    pub fn generate_pawn_moves(
        &self,
        board: &Board,
        move_list: &mut MoveList,
        move_type: MoveType,
    ) {
        // TODO: I don't like that ranks are reversed, fix this later.
        let next_rank = match board.active_color {
            Color::White => -1,
            Color::Black => 1,
            Color::Both => panic!("Active color can not be BOTH."),
        };
        let pawns_rank = match board.active_color {
            Color::White => 6,
            Color::Black => 1,
            Color::Both => panic!("Active color can not be BOTH."),
        };
        let rotations_count = (64 + 8 * next_rank) as u32;

        let piece = Piece::WhitePawn.to_color(board.active_color);
        let mut piece_board = board.bitboards[piece as usize];
        let occupancy = board.get_occupancies(Color::Both);
        let opponent_occupancy = board.get_occupancies(board.active_color.opposite());
        let empty_squares = !occupancy;

        while !piece_board.empty() {
            let mut moves = BitBoard::default();
            let source_square = piece_board.lsb_bit_square();
            piece_board.pop_bit_value(source_square);
            let next_square = source_square.add_rank(next_rank);

            // Generate quite pawn moves
            if move_type == MoveType::All || move_type == MoveType::Quite {
                let single_push = next_square.get_bitboard() & empty_squares;
                let double_push = if source_square.rank() == pawns_rank && !single_push.empty() {
                    next_square.get_bitboard().rotate_left(rotations_count) & empty_squares
                } else {
                    BitBoard::default()
                };
                moves |= single_push | double_push;
            }

            // Generate pawn captures
            if move_type == MoveType::All || move_type == MoveType::Capture {
                let targets = self.get_pawn_attack(source_square, board.active_color);
                let attacks = targets & opponent_occupancy;
                let en_passant_attack = match board.en_passant_target {
                    Some(square) => square.get_bitboard() & targets,
                    None => BitBoard::default(),
                };
                moves |= attacks | en_passant_attack;
            }

            // Add move to the move list
            self.add_move(board, piece, source_square, moves, move_list);
        }
    }

    pub fn generate_castling_moves(&self, board: &Board, move_list: &mut MoveList) {
        let occupancy = board.get_occupancies(Color::Both);
        let source_square = board.bitboards[Piece::WhiteKing.to_color(board.active_color) as usize]
            .lsb_bit_square();

        let white_kingside_blockers = Square::F1.get_bitboard() | Square::G1.get_bitboard();
        let white_queenside_blockers =
            Square::B1.get_bitboard() | Square::C1.get_bitboard() | Square::D1.get_bitboard();

        let black_kingside_blockers = Square::F8.get_bitboard() | Square::G8.get_bitboard();
        let black_queenside_blockers =
            Square::B8.get_bitboard() | Square::C8.get_bitboard() | Square::D8.get_bitboard();

        //White
        if board.active_color == Color::White {
            // Kingside
            if board.castle_settings.can_white_castle_king {
                let is_kingside_blocked = !(white_kingside_blockers & occupancy).empty();

                if !is_kingside_blocked
                    && !self.is_square_attacked(Square::F1, Color::Black, board)
                    && !self.is_square_attacked(Square::G1, Color::Black, board)
                {
                    self.add_move(
                        board,
                        Piece::WhiteKing,
                        source_square,
                        Square::G1.get_bitboard(),
                        move_list,
                    );
                }
            }
            // Queenside
            if board.castle_settings.can_white_castle_queen {
                let is_queenside_blocked = !(white_queenside_blockers & occupancy).empty();

                if !is_queenside_blocked
                    && !self.is_square_attacked(Square::B1, Color::Black, board)
                    && !self.is_square_attacked(Square::C1, Color::Black, board)
                    && !self.is_square_attacked(Square::D1, Color::Black, board)
                {
                    self.add_move(
                        board,
                        Piece::WhiteKing,
                        source_square,
                        Square::C1.get_bitboard(),
                        move_list,
                    );
                }
            }
        }

        // Black
        if board.active_color == Color::Black {
            // Kingside
            if board.castle_settings.can_black_castle_king {
                let is_kingside_blocked = !(black_kingside_blockers & occupancy).empty();

                if !is_kingside_blocked
                    && !self.is_square_attacked(Square::F8, Color::White, board)
                    && !self.is_square_attacked(Square::G8, Color::White, board)
                {
                    self.add_move(
                        board,
                        Piece::BlackKing,
                        source_square,
                        Square::G8.get_bitboard(),
                        move_list,
                    );
                }
            }
            // Queenside
            if board.castle_settings.can_black_castle_queen {
                let is_queenside_blocked = !(black_queenside_blockers & occupancy).empty();

                if !is_queenside_blocked
                    && !self.is_square_attacked(Square::B8, Color::White, board)
                    && !self.is_square_attacked(Square::C8, Color::White, board)
                    && !self.is_square_attacked(Square::D8, Color::White, board)
                {
                    self.add_move(
                        board,
                        Piece::BlackKing,
                        source_square,
                        Square::C8.get_bitboard(),
                        move_list,
                    );
                }
            }
        }
    }

    pub fn add_move(
        &self,
        board: &Board,
        piece: Piece,
        source_square: Square,
        to_bitboard: BitBoard,
        move_list: &mut MoveList,
    ) {
        let mut to_bb = to_bitboard;
        let is_king = piece == Piece::WhiteKing || piece == Piece::BlackKing;
        let is_pawn = piece == Piece::WhitePawn || piece == Piece::BlackPawn;
        let promotion_rank = match board.active_color {
            Color::White => 0,
            Color::Black => 7,
            Color::Both => panic!("Piece can not be of color BOTH"),
        };

        while !to_bb.empty() {
            let to_square = to_bb.lsb_bit_square();
            to_bb.pop_bit_value(to_square);
            let is_en_passant = match board.en_passant_target {
                Some(en_passant_square) => en_passant_square == to_square,
                None => false,
            };
            let castling = is_king && ((to_square as i8 - source_square as i8).abs() == 2);
            let double_push = is_pawn && ((to_square as i8 - source_square as i8).abs() == 16);
            let is_promotion = is_pawn && to_square.rank() == promotion_rank;
            let captured_piece = board.piece_by_square[to_square as usize];

            if is_promotion {
                let promotion_pieces = match board.active_color {
                    Color::White => [
                        Piece::WhiteRook,
                        Piece::WhiteQueen,
                        Piece::WhiteBishop,
                        Piece::WhiteKnight,
                    ],
                    Color::Black => [
                        Piece::BlackRook,
                        Piece::BlackQueen,
                        Piece::BlackBishop,
                        Piece::BlackKnight,
                    ],
                    Color::Both => panic!("Piece can not be of color BOTH"),
                };
                for promotion_piece in promotion_pieces {
                    let move_data = Move::encode_move(
                        source_square,
                        to_square,
                        piece,
                        captured_piece,
                        promotion_piece,
                        is_en_passant,
                        castling,
                        double_push,
                    );
                    move_list.add_move(move_data);
                }
            } else {
                let move_data = Move::encode_move(
                    source_square,
                    to_square,
                    piece,
                    captured_piece,
                    Piece::None,
                    is_en_passant,
                    castling,
                    double_push,
                );
                move_list.add_move(move_data);
            }
        }
    }

    // Returns if square is attacked by given color
    pub fn is_square_attacked(&self, square: Square, color: Color, board: &Board) -> bool {
        // Is attacked by pawns
        let pawn_attack = self.get_pawn_attack(square, board.active_color);
        let pawn_index = Piece::BlackPawn.to_color(color) as usize;
        if !(pawn_attack & board.bitboards[pawn_index]).empty() {
            return true;
        }

        // Is attacked by knight
        let knight_attack = self.get_knight_attack(square);
        let knight_index = Piece::BlackKnight.to_color(color) as usize;
        if !(knight_attack & board.bitboards[knight_index]).empty() {
            return true;
        }

        // Is attacked by king
        let king_attack = self.get_king_attack(square);
        let king_index = Piece::BlackKing.to_color(color) as usize;
        if !(king_attack & board.bitboards[king_index]).empty() {
            return true;
        }

        // Is attacked by bishop
        let bishop_attack = self.get_bishop_attack(square, board.get_occupancies(Color::Both));
        let bishop_index = Piece::BlackBishop.to_color(color) as usize;
        if !(bishop_attack & board.bitboards[bishop_index]).empty() {
            return true;
        }

        // Is attacked by rook
        let rook_attack = self.get_rook_attack(square, board.get_occupancies(Color::Both));
        let rook_index = Piece::BlackRook.to_color(color) as usize;
        if !(rook_attack & board.bitboards[rook_index]).empty() {
            return true;
        }

        // Is attacked by rook
        let queen_attack = self.get_queen_attack(square, board.get_occupancies(Color::Both));
        let queen_index = Piece::BlackQueen.to_color(color) as usize;
        if !(queen_attack & board.bitboards[queen_index]).empty() {
            return true;
        }

        false
    }
}
