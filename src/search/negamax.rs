use crate::{
    board_repr::board::Board,
    move_generation::{
        generator::MoveGenerator,
        move_list::MoveList,
        moves::{Move, MoveType},
    },
};

pub struct Search<'a> {
    pub board: &'a mut Board,
    pub mg: &'a MoveGenerator,

    pub nodes: u32,
    pub ply: u32,
    pub best_move: Option<Move>,
}

impl<'a> Search<'a> {
    pub fn new(board: &'a mut Board, mg: &'a MoveGenerator) -> Self {
        let nodes = 0;
        let ply = 0;
        let best_move = None;
        Self {
            board,
            mg,

            nodes,
            ply,
            best_move,
        }
    }
}

impl<'a> Search<'a> {
    pub fn alpha_beta(&mut self, mut alpha: i16, beta: i16, depth: i8) -> i16 {
        // base condition
        if depth == 0 {
            return self.quiescence(alpha, beta);
        }

        // init variables
        let mut best_move_so_far: Move = Move::default();
        let mut legal_moves_count = 0;
        let init_alpha = alpha;
        let is_king_in_check = self.board.is_king_in_check(self.mg);

        // update number of nodes traversed
        self.nodes += 1;

        // make list of moves
        let mut move_list = MoveList::new();
        self.mg
            .generate_moves(self.board, &mut move_list, MoveType::All);

        for move_data in move_list {
            let is_legal = self.board.make_move(move_data, self.mg);
            if !is_legal {
                continue;
            }

            // increment counters
            self.ply += 1;
            legal_moves_count += 1;

            let score = -self.alpha_beta(-beta, -alpha, depth - 1);

            self.ply -= 1;
            self.board.unmake_move();

            // fail hard beta cutoff
            if score >= beta {
                return beta;
            }

            // found better move
            if score > alpha {
                alpha = score;

                // if root node
                if self.ply == 0 {
                    best_move_so_far = move_data;
                }
            }
        }

        if legal_moves_count == 0 {
            // Mate
            if is_king_in_check {
                // We need to add ply here to ensure that mate with fewer number of moves
                // going to have higher score.
                return -30000i16 + self.ply as i16;
            }

            // Stalemate
            return 0;
        }

        if init_alpha != alpha {
            self.best_move = Some(best_move_so_far)
        }

        // fails low
        alpha
    }
}
