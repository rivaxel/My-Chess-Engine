use std::io::{stdin, BufRead};
use std::str::SplitWhitespace;
use game::Game;

const ENGINE_NAME: &str = "Snow Crust";

pub fn uci_loop() {
    let mut current_game = Game::new();

    let console_input = stdin();

    for line in console_input.lock().lines() {
        let line = line.unwrap_or("null input".to_string());
        let mut token_sequence = line.split_whitespace();

        if let Some(first_token) = token_sequence.next() {
            match first_token {
                "uci"        => uci(),
                "isready"    => println!("readyok"),
                "ucinewgame" => reset_game(&current_game),
                "position"   => set_position(&mut current_game, &mut token_sequence),
                "go"         => go(&mut current_game),
                "setoption"  => set_option(),
                "quit"       => break,
                _            => println!("Unrecognised Token: {}", first_token),
            }
        }
    }
}

fn uci() {
    println!("id name {}", ENGINE_NAME);
    println!("id author Ugur Mislina Gul");
    println!("uciok");
}

fn reset_game(game: &Game) {

}

fn set_position(game: &mut Game, token_sequence: &mut SplitWhitespace) {
    match token_sequence.next() {
        Some(argument) => {
            match argument {
                "fen"      => paraphrase_fen_token(game, token_sequence),
                "startpos" => paraphrase_startpos_token(game, token_sequence),
                _          => panic!("Error: Invalid fen string!!!"), 
            }
        },
        None => panic!("Error: Invalid fen string!!!"),
    }
}

fn paraphrase_fen_token(game: &mut Game, token_sequence: &mut SplitWhitespace) {
    let mut fen_string = String::new();
    let mut moves = Vec::new();

    while let Some(token) = token_sequence.next() {
        match token {
            "moves" => {
                while let Some(chess_move) = token_sequence.next() {
                    moves.push(chess_move.to_string());
                }
            },
            _ => {
                fen_string.push_str(token);
                fen_string.push(' ');
            },
        }
    }

    game.build_board_from_fen_string(fen_string);

    for next_move in moves {
        game.accept_move(next_move);
    }
}

fn paraphrase_startpos_token(game: &mut Game, token_sequence: &mut SplitWhitespace) {
    let mut moves = Vec::new();

    while let Some(token) = token_sequence.next() {
        match token {
            "moves" => {
                while let Some(chess_move) = token_sequence.next() {
                    moves.push(chess_move.to_string());
                }
            },
            _ => {},
        }
    }

    game.setup_initial_position();

    for next_move in moves {
        game.accept_move(next_move);
    }
}

fn go(game: &mut Game) {
    game.find_best_move();
}

fn set_option() {

}