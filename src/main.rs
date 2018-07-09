use std::time::SystemTime;

pub mod chess_board;
pub mod move_generator;
pub mod searcher;
pub mod uci_interface;
pub mod game;

fn main() {
    uci_interface::uci_loop();
    
    let now = SystemTime::now();

    let mut new_searcher = searcher::Searcher::new();

    let mut new_board = chess_board::ChessBoard::new();

    new_board.setup_initial_position();

    new_board.print_board_from_white_perspective(true);

    let toggle_alpha_beta = true;

    if toggle_alpha_beta {
        new_searcher.run_alpha_beta_pruning(&new_board, 5);
    } else {
        new_searcher.run_negamax(&new_board, 5);
    }

    new_searcher.next_move.print_last_move_with_moving_side_info();
    new_searcher.next_move.print_board_from_white_perspective(true);

    loop {
        let new_move = new_searcher.next_move.clone();

        if toggle_alpha_beta {
            new_searcher.run_alpha_beta_pruning(&new_move, 5);
        } else {
            new_searcher.run_negamax(&new_move, 5);
        }

        if new_searcher.game_status == "Finished" {
            println!("{}", new_searcher.game_result);
            
            break;
        }

        new_searcher.next_move.print_last_move_with_moving_side_info();
        new_searcher.next_move.print_board_from_white_perspective(true);
    }

    match now.elapsed() {
        Ok(elapsed) => {
            println!("Time elapsed: {}", elapsed.as_secs());
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
    
}
