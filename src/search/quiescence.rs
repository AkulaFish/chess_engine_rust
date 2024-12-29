use crate::{
    evaluation::eval::evaluate,
    move_generation::{move_list::MoveList, moves::MoveType},
};

use super::negamax::Search;

impl<'a> Search<'a> {
    pub fn quiescence(&mut self, mut alpha: i16, beta: i16) -> i16 {
        // evaluate position
        let eval_score = evaluate(self.board);

        // fail hard beta cutoff
        if eval_score >= beta {
            return beta;
        }

        // found better move
        if eval_score > alpha {
            alpha = eval_score;
        }

        // make list of moves
        let mut move_list = MoveList::new();
        self.mg
            .generate_moves(self.board, &mut move_list, MoveType::Capture);

        for move_data in move_list {
            let is_legal = self.board.make_move(move_data, self.mg);
            if !is_legal {
                continue;
            }

            // increment counters
            self.ply += 1;

            let score = -self.quiescence(-beta, -alpha);

            self.ply -= 1;
            self.board.unmake_move();

            // fail hard beta cutoff
            if score >= beta {
                return beta;
            }

            // found better move
            if score > alpha {
                alpha = score;
            }
        }

        // fails low
        alpha
    }
}
