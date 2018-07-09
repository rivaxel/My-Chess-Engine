use std::collections::HashMap;
use chess_board::ChessBoard;

static N: i64 = -10;
static E: i64 = 1;
static S: i64 = 10;
static W: i64 = -1;

static A1: i64 = 91;
static H1: i64 = 98;
static A8: i64 = 21;
static H8: i64 = 28;

pub struct MoveGenerator {
    directions: HashMap<char, Vec<i64>>,
}

impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        let mut directions: HashMap<char, Vec<i64>> = HashMap::new();

        directions.insert('P', vec![N, N+N, N+W, N+E]);
        directions.insert('N', vec![N+N+E, E+N+E, E+S+E, S+S+E, S+S+W, W+S+W, W+N+W, N+N+W]);
        directions.insert('B', vec![N+E, S+E, S+W, N+W]);
        directions.insert('R', vec![N, E, S, W]);
        directions.insert('Q', vec![N, E, S, W, N+E, S+E, S+W, N+W]);
        directions.insert('K', vec![N, E, S, W, N+E, S+E, S+W, N+W]);

        MoveGenerator {
            directions,
        }
    }

    pub fn generate_next_moves_from_board(&self, current_position: &ChessBoard) -> Vec<ChessBoard> {
        let mut new_next_positions: Vec<ChessBoard> = Vec::new();

        // starts here

        for (index, &piece) in current_position.board_vector.iter().enumerate() {
            let index = index as i64;

            if !piece.is_uppercase() {
                continue;
            }

            let mut piece_directions = self.directions[&piece].clone();

            while let Some(piece_direction) = piece_directions.pop() {
                let mut destination = index;

                loop {
                    destination = destination + piece_direction;

                    let mut new_position = current_position.clone();

                    new_position.decrease_en_passant_lifetime();

                    let destination_piece = current_position.get_square(destination);

                    if destination_piece.is_whitespace() || destination_piece.is_uppercase() {
                        break;
                    }

                    let mut pawn_promotion = false;

                    if piece == 'P' {
                        if (piece_direction == N || piece_direction == N + N) && destination_piece != '.' {
                            break;
                        }

                        if piece_direction == N + N {
                            if index < (A1 + N) || current_position.get_square(index + N) != '.' {
                                break;
                            } else {
                                new_position.set_en_passant_square(119 - (index + N));
                            }
                        }

                        if piece_direction == N + W || piece_direction == N + E {
                            if new_position.is_en_passant_active() && destination as usize == new_position.en_passant_square {
                                new_position.make_en_passant_move(index, destination);

                                if !self.detect_check(&new_position) {
                                    new_next_positions.push(new_position.clone_and_rotate());
                                }
                                break;
                            }

                            if destination_piece == '.' {
                                break;
                            }
                        }

                        if A8 <= destination && destination <= H8 {
                            pawn_promotion = true;
                        }
                    }

                    new_position.make_move(index, destination);

                    if piece == 'K' {
                        new_position.set_castling_rights(false, false);
                    }

                    if piece == 'R' {
                        self.enforce_castling_rules(&current_position,
                                                    &mut new_position,
                                                    &mut new_next_positions,
                                                    index, destination);
                    }

                    if !self.detect_check(&new_position) {
                        if pawn_promotion {
                            for promoted_piece in vec!['R', 'N', 'B', 'Q'] {
                                new_position.promote_pawn(destination as usize, promoted_piece);
                                new_next_positions.push(new_position.clone_and_rotate());
                            }
                        } else {
                            new_next_positions.push(new_position.clone_and_rotate());
                        }
                    }

                    if vec!['P', 'N', 'K'].contains(&piece) || destination_piece.is_lowercase() {
                        break;
                    }
                }
            }
        }
        new_next_positions
    }

    fn enforce_castling_rules(&self, current_position: &ChessBoard, new_position: &mut ChessBoard,
                              new_next_positions: &mut Vec<ChessBoard>,
                              index: i64, destination: i64) {
        let mut board_after_castling = new_position.clone();
        let mut castling_made = false;

        if !self.detect_check(&board_after_castling) && current_position.get_square(destination) == '.' {  
            if index == A1 && new_position.get_left_castling() {
                new_position.set_left_castling(false);

                if current_position.get_square(destination + E) == 'K' {
                    castling_made = true;

                    board_after_castling.make_move(destination + E, destination + W);
                }
            } else if index == H1 && new_position.get_right_castling() {
                new_position.set_right_castling(false);

                if current_position.get_square(destination + W) == 'K' {
                    castling_made = true;

                    board_after_castling.make_move(destination + W, destination + E);
                }
            }   
        }

        if castling_made {
            board_after_castling.set_castling_rights(false, false);
            board_after_castling.add_castling_bonus();

            if !self.detect_check(&board_after_castling)
                && !self.detect_rook_check(&new_position, destination) {
                new_next_positions.push(board_after_castling.clone_and_rotate());
            }
        }
    }

    fn find_king_index(&self, current_position: ChessBoard) -> i64 {
        let mut king_index = 0;

        for (index, &piece) in current_position.board_vector.iter().enumerate() {
            if piece == 'K' {
                king_index = index;
                break;
            }
        }

        king_index as i64
    }

    pub fn detect_check(&self, current_position: &ChessBoard) -> bool {
        let king_index = self.find_king_index(current_position.clone());

        self.is_square_under_attack(&current_position, king_index)
    }

    fn detect_rook_check(&self, current_position: &ChessBoard, rook_index: i64) -> bool {
        self.is_square_under_attack(&current_position, rook_index)
    }

    fn is_square_under_attack(&self, current_position: &ChessBoard, square_index: i64) -> bool {
        for &piece in ['P', 'N', 'B', 'R', 'Q', 'K'].into_iter() {
            for &piece_direction in &self.directions[&piece] {
                let mut destination = square_index;

                loop {
                    destination = destination + piece_direction;

                    let destination_piece = current_position.get_square(destination);

                    if destination_piece.is_whitespace() || destination_piece.is_uppercase() {
                        break;
                    }

                    if piece == 'P' {
                        if piece_direction == N || piece_direction == N + N {
                            if destination_piece != '.' {
                                break;
                            }
                        }

                        if piece_direction == N + N {
                            if square_index < (A1 + N) || current_position.get_square(square_index + N) != '.' {
                                break;
                            }
                        }

                        if piece_direction == N + W || piece_direction == N + E {
                            if destination_piece == '.' {
                                break;
                            }
                        }
                    }

                    if vec!['P', 'N', 'B', 'R', 'Q', 'K'].contains(&piece)
                        && piece.to_lowercase().to_string() == destination_piece.to_string() {
                        return true;
                    }

                    if vec!['P', 'N', 'K'].contains(&piece) || destination_piece.is_lowercase() {
                        break;
                    }
                }
            }
        }
        false
    }

    pub fn calculate_perft_depth(&mut self, current_board: &ChessBoard, depth: usize, notate: bool) -> usize {
        if depth == 0 {
            1
        } else {
            let moves = self.generate_next_moves_from_board(current_board);
            let mut nodes = 0;

            for some_move in moves {
                let move_notation = if notate {
                    some_move.last_move.clone()
                } else {
                    "None".to_string()
                };

                let partial_nodes = self.calculate_perft_depth(&some_move, depth - 1, false);

                if notate {
                    println!("{}: {}", move_notation, partial_nodes);
                }

                nodes += partial_nodes;
            }

            nodes
        }
    }
}

#[cfg(test)]
mod perft_move_generation_tests {
    use chess_board::ChessBoard;
    use move_generator::MoveGenerator;

    #[test]
    fn perft_test_with_initial_position() {
        let mut move_generator = MoveGenerator::new();
        let mut test_board = ChessBoard::new();

        test_board.setup_initial_position();

        //depth0
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 0, false), 1);
        //depth1
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 1, false), 20);
        //depth2
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 2, false), 400);
        //depth3
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 3, false), 8902);
        //depth4
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 4, false), 197281);
        //depth5
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 5, false), 4865609);
    }

    #[test]
    fn perft_test_with_position_2() {
        let mut move_generator = MoveGenerator::new();
        let mut test_board = ChessBoard::new();

        test_board.setup_position_2();

        //depth0
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 0, false), 1);
        //depth1
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 1, false), 48);
        //depth2
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 2, false), 2039);
        //depth3
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 3, false), 97862);
        //depth4
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 4, false), 4085603);
    }

    #[test]
    fn perft_test_with_position_3() {
        let mut move_generator = MoveGenerator::new();
        let mut test_board = ChessBoard::new();

        test_board.setup_position_3();

        //depth0
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 0, false), 1);
        //depth1
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 1, false), 14);
        //depth2
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 2, false), 191);
        //depth3
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 3, false), 2812);
        //depth4
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 4, false), 43238);
        //depth5
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 5, false), 674624);
        //depth6
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 6, false), 11030083);
    }

    #[test]
    fn perft_test_with_position_4() {
        let mut move_generator = MoveGenerator::new();
        let mut test_board = ChessBoard::new();

        test_board.setup_position_4();

        //depth0
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 0, false), 1);
        //depth1
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 1, false), 6);
        //depth2
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 2, false), 264);
        //depth3
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 3, false), 9467);
        //depth4
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 4, false), 422333);
        //depth5
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 5, false), 15833292);
    }

    #[test]
    fn perft_test_with_position_5() {
        let mut move_generator = MoveGenerator::new();
        let mut test_board = ChessBoard::new();

        test_board.setup_position_5();

        //depth0
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 0, false), 1);
        //depth1
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 1, false), 44);
        //depth2
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 2, false), 1486);
        //depth3
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 3, false), 62379);
        //depth4
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 4, false), 2103487);
    }

    #[test]
    fn perft_test_with_position_6() {
        let mut move_generator = MoveGenerator::new();
        let mut test_board = ChessBoard::new();

        test_board.setup_position_6();

        //depth0
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 0, false), 1);
        //depth1
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 1, false), 46);
        //depth2
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 2, false), 2079);
        //depth3
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 3, false), 89890);
        //depth4
        assert_eq!(move_generator.calculate_perft_depth(&test_board, 4, false), 3894594);
    }
}
