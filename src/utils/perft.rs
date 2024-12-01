use std::time::Instant;

use crate::{
    board_repr::board::Board,
    move_generation::{generator::MoveGenerator, move_list::MoveList, moves::MoveType},
};

pub fn perft_test(board: &mut Board, mg: &MoveGenerator, depth: i8) {
    let mut total_time = 0u128;
    let mut total_nodes = 0u64;

    let mut move_list = MoveList::new();
    mg.generate_moves(board, &mut move_list, MoveType::All);

    for i in 0..move_list.count {
        let now = Instant::now();
        let mut leaf_nodes = 0;
        let move_data = move_list.moves[i as usize];

        if board.make_move(move_data, mg) {
            leaf_nodes += perft(board, mg, depth - 1);

            // Measure time and speed
            let elapsed = now.elapsed().as_millis();

            // Add tot totals for final calculation at the very end.
            total_time += elapsed;
            total_nodes += leaf_nodes;

            board.unmake_move();

            // Print the results.
            println!(
                "{i}: {}{}: {leaf_nodes} ({elapsed} ms)",
                move_data.source_square(),
                move_data.target_square()
            );
        }
    }

    println!("Total time spent: {total_time} ms");
    println!("Total nodes: {total_nodes} nodes");
}

fn perft(board: &mut Board, mg: &MoveGenerator, depth: i8) -> u64 {
    let mut nodes = 0u64;
    let mut move_list = MoveList::new();

    if depth <= 0 {
        return 1;
    }

    mg.generate_moves(board, &mut move_list, MoveType::All);

    for i in 0..move_list.count {
        let move_data = move_list.moves[i as usize];

        if board.make_move(move_data, mg) {
            nodes += perft(board, mg, depth - 1);

            board.unmake_move();
        }
    }

    nodes
}
