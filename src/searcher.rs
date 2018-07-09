use std::cmp::{max, min};
use chess_board::ChessBoard;
use move_generator::MoveGenerator;

pub struct Searcher {
    move_generator: MoveGenerator,
    pub next_move: ChessBoard,
    pub game_status: String,
    pub game_result: String,
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher {
            move_generator: MoveGenerator::new(),
            next_move: ChessBoard::new(),
            game_status: String::new(),
            game_result: String::new(),
        }
    }

    pub fn run_negamax(&mut self, current_node: &ChessBoard, depth: usize) {
        self.negamax(current_node, depth, true);
    }

    fn negamax(&mut self, current_node: &ChessBoard, depth: usize, root: bool) -> i64 {
        if depth == 0 {
            return current_node.get_score();
        }

        let mut best_move = ChessBoard::new();
        let mut max = i64::min_value() + 1;

        let next_moves: Vec<ChessBoard> = self.move_generator
            .generate_next_moves_from_board(current_node);

        if root && next_moves.len() == 0 {
            self.game_status = "Finished".to_string();

            if self.move_generator.detect_check(current_node) {
                if current_node.get_current_color() {
                    self.game_result = "Black Won!".to_string();
                } else {
                    self.game_result = "White Won!".to_string();
                }
            } else {
                self.game_result = "Draw".to_string();
            }

            return 0
        } else if next_moves.len() == 0 {
            if self.move_generator.detect_check(current_node) {
                return i64::min_value() + 10000 - depth as i64
            } else {
                return i64::max_value() - 10000 + depth as i64
            }
        }

        for chess_move in &next_moves {
            let chess_move_score = -self.negamax(chess_move, depth - 1, false);

            if chess_move_score > max {
                best_move = chess_move.clone();
                max = chess_move_score;
            }
        }

        if root {
            self.next_move = best_move.clone();
        }

        max
    }

    pub fn run_alpha_beta_pruning(&mut self, current_node: &ChessBoard, depth: usize) {
        let alpha = i64::min_value() + 1;
        let beta = i64::max_value();

        if depth % 2 == 0 {
            self.alpha_beta_pruning(current_node, depth, alpha, beta, true, true, true);
        } else {
            self.alpha_beta_pruning(current_node, depth, alpha, beta, true, true, false);
        }
    }

    fn alpha_beta_pruning(&mut self, current_node: &ChessBoard, depth: usize, alpha: i64, beta: i64, maximizing_player: bool, root: bool, even: bool) -> i64 {
        if depth == 0 {
            if even {
                return current_node.get_score()
            } else {
                return -current_node.get_score()
            }

        }

        let mut best_move_score = i64::min_value();

        let next_moves: Vec<ChessBoard> = self.move_generator
            .generate_next_moves_from_board(current_node);

        if root && next_moves.len() == 0 {
            self.game_status = "Finished".to_string();

            if self.move_generator.detect_check(current_node) {
                if current_node.get_current_color() {
                    self.game_result = "Black Won!".to_string();
                } else {
                    self.game_result = "White Won!".to_string();
                }
            } else {
                self.game_result = "Draw".to_string();
            }

            return 0
        } else if next_moves.len() == 0 {
            if maximizing_player {
                if self.move_generator.detect_check(current_node) {
                    return i64::min_value() + 10000 - depth as i64
                } else {
                    return i64::max_value() - 10000 + depth as i64
                }
            } else {
                if self.move_generator.detect_check(current_node) {
                    return i64::max_value() - 10000 + depth as i64
                } else {
                    return i64::min_value() + 10000 - depth as i64
                }
            }
        }

        let mut v;

        if maximizing_player {        
            v = i64::min_value() + 1;
            let mut alpha = alpha;

            for chess_move in &next_moves {
                v = max(v, self.alpha_beta_pruning(chess_move, depth - 1, alpha, beta, false, false, even));
                alpha = max(alpha, v);

                if root {
                    if v > best_move_score {
                        best_move_score = v;
                        self.next_move = chess_move.clone();
                    }
                }

                if beta <= alpha {
                    break;
                }
            }
        } else {
            v = i64::max_value();
            let mut beta = beta;

            for chess_move in &next_moves {
                v = min(v, self.alpha_beta_pruning(chess_move, depth - 1, alpha, beta, true, false, even));
                beta = min(beta, v);

                if beta <= alpha {
                    break;
                }
            }
        }

        v
    }

}
