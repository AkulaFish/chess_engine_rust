use std::{
    io::{stdin, stdout, Write},
    str::FromStr,
};

use crate::{
    board_repr::{board::Board, fen::Fen, piece::Piece, square::Square},
    move_generation::{
        generator::MoveGenerator,
        move_list::MoveList,
        moves::{Move, MoveType},
    },
    search::negamax::Search,
    utils::traits::DisplayExtension,
    _START_FEN,
};

pub struct UCI;

impl UCI {
    pub fn uci_loop() {
        let mut board = Fen::to_board(_START_FEN);
        let mg = MoveGenerator::new();

        UCI::id();
        UCI::uciok();

        loop {
            let _ = stdout().flush();
            let mut command = String::new();
            stdin()
                .read_line(&mut command)
                .expect("Can't take user input");
            command = command.trim().to_string();

            if command == "\n" {
                continue;
            }

            if command == "isready" {
                UCI::readyok();
            }

            if command == "uci" {
                UCI::id();
                UCI::uciok();
            }

            if command == "ucinewgame" {
                UCI::parse_position("position startpos", &mut board, &mg);
            }

            if command.starts_with("position") {
                UCI::parse_position(&command, &mut board, &mg);
                continue;
            }

            if command.starts_with("display") {
                board.display();
                continue;
            }

            if command.starts_with("go") {
                UCI::parse_go(&command, &mut board, &mg);
                continue;
            }

            if command == "quit" {
                break;
            }
        }
    }

    pub fn parse_move(move_str: &str, board: &Board, mg: &MoveGenerator) -> Option<Move> {
        let (source_square_str, parts) = move_str.split_at(2);
        let (target_square_str, promoted) = parts.split_at(2);

        let Ok(source_square) = Square::from_str(&source_square_str.to_uppercase()) else {
            return None;
        };
        let Ok(target_square) = Square::from_str(&target_square_str.to_uppercase()) else {
            return None;
        };
        let promoted_piece = Piece::from_str(promoted).unwrap_or(Piece::None);

        let mut move_list = MoveList::new();
        mg.generate_moves(board, &mut move_list, MoveType::All);

        move_list.moves.into_iter().find(|x| {
            x.source_square() == source_square
                && x.target_square() == target_square
                && (x.promoted_piece() == promoted_piece
                    || x.promoted_piece() == promoted_piece.opposite_color())
        })
    }

    pub fn parse_position(command: &str, board: &mut Board, mg: &MoveGenerator) {
        let parts: Vec<&str> = command.split_whitespace().collect();

        let mut moves: Vec<&str> = vec![];
        let mut fen = String::new();
        let mut parse_moves = false;
        let mut parse_fen = false;

        for part in parts {
            match part {
                "position" => continue,
                "startpos" => {
                    fen = _START_FEN.to_string();
                    continue;
                }
                "fen" => {
                    parse_fen = true;
                    parse_moves = false;
                    continue;
                }
                "moves" => {
                    parse_fen = false;
                    parse_moves = true;
                    continue;
                }
                _ => (),
            }

            if parse_moves {
                moves.push(part.trim());
            }

            if parse_fen {
                fen.push_str(&format!("{} ", part));
            }
        }

        board.from_fen(fen.trim());

        for move_string in moves {
            let move_data = UCI::parse_move(move_string, board, mg);

            if let Some(m) = move_data {
                board.make_move(m, mg);
            }
        }
    }

    pub fn parse_go(command: &str, board: &mut Board, mg: &MoveGenerator) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        let mut depth = 0;

        let mut set_depth = false;

        for part in parts {
            if set_depth {
                depth = part.parse().unwrap_or(6);
            }

            match part.to_lowercase().as_str() {
                "go" => (),
                "depth" => set_depth = true,
                _ => (),
            }
        }

        let mut search = Search::new(board, mg);
        search.alpha_beta(-32000, 32000, depth);
        if let Some(move_data) = search.best_move {
            UCI::info(move_data, depth);
            UCI::bestmove(move_data);
        }
    }
}

impl UCI {
    pub fn id() {
        println!("id name ChessDiplomaEngine");
        println!("id author AkulaFish");
    }

    pub fn readyok() {
        println!("readyok");
    }

    pub fn uciok() {
        println!("uciok");
    }

    pub fn bestmove(move_data: Move) {
        println!("bestmove {}", move_data.to_uci_string())
    }

    pub fn info(move_data: Move, depth: i8) {
        println!("info depth {depth} pv {}", move_data.to_uci_string());
    }
}
