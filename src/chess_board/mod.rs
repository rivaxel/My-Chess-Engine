mod evaluation;

pub struct ChessBoard {
    pub board_vector: Vec<char>,
    current_color: bool,
    white_castling_rights: (bool, bool),
    black_castling_rights: (bool, bool),
    pub en_passant_square: usize,
    en_passant_lifetime: usize,
    pub last_move: String,
    score: i64,
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        ChessBoard {
            board_vector: Vec::new(),
            current_color: true,
            white_castling_rights: (true, true),
            black_castling_rights: (true, true),
            en_passant_square: 0,
            en_passant_lifetime: 0,
            last_move: String::new(),
            score: 0,
        }
    }

    pub fn from(board_vector: Vec<char>) -> ChessBoard {
        ChessBoard {
            board_vector: board_vector.clone(),
            current_color: true,
            white_castling_rights: (true, true),
            black_castling_rights: (true, true),
            en_passant_square: 0,
            en_passant_lifetime: 0,
            last_move: String::new(),
            score: 0,
        }
    }

    pub fn clone(&self) -> ChessBoard {
        ChessBoard {
            board_vector: self.board_vector.clone(),
            last_move: self.last_move.clone(),
            ..*self
        }
    }

    pub fn setup_initial_position(&mut self) {
        self.build_board_from_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    }

    pub fn setup_position_2(&mut self) {
        self.build_board_from_fen_string("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -".to_string());
    }

    pub fn setup_position_3(&mut self) {
        self.build_board_from_fen_string("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -".to_string());
    }

    pub fn setup_position_4(&mut self) {
        self.build_board_from_fen_string("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string());
    }

    pub fn setup_position_5(&mut self) {
        self.build_board_from_fen_string("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".to_string());
    }

    pub fn setup_position_6(&mut self) {
        self.build_board_from_fen_string("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10".to_string())
    }

    pub fn build_board_from_fen_string(&mut self, fen_string: String) {
        self.board_vector.clear();

        let filler_segment = vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '\n'];

        self.board_vector.append(&mut filler_segment.clone());
        self.board_vector.append(&mut filler_segment.clone());

        let split_fen_string: Vec<&str> = fen_string.split(' ').collect();

        let board_string: Vec<&str> = split_fen_string[0].split('/').collect();

        for split in board_string {
            let mut board_segment = self.build_board_segment(split);
            self.board_vector.append(&mut board_segment);
        }

        self.board_vector.append(&mut filler_segment.clone());
        self.board_vector.append(&mut filler_segment.clone());

        self.set_current_color_from_fen_field(split_fen_string[1]);
        self.set_castling_from_fen_field(split_fen_string[2]);
        self.set_en_passant_square_from_fen_field(split_fen_string[3]);
    }

    fn build_board_segment(&mut self, partial_fen_string: &str) -> Vec<char> {
        let mut board_segment: Vec<char> = Vec::new();
        board_segment.push(' ');

        for square in partial_fen_string.chars() {
            if square.is_alphabetic() {
                board_segment.push(square);
            } else if square.is_alphanumeric() {
                let square_as_digit = square.to_digit(10).unwrap();

                for _ in 0..square_as_digit {
                    board_segment.push('.');
                }
            }
        }

        board_segment.push('\n');
        board_segment
    }

    fn set_current_color_from_fen_field(&mut self, current_color_fen_field: &str) {
        match current_color_fen_field {
            "w" => self.current_color = true,
            "b" => {
                self.current_color = false;
                self.board_vector = self.return_rotated_board_vector_with_changed_letter_case();
            },
            _   => panic!("Error: Can't read active colour information from fen string!!!"),
        }
    }

    fn set_castling_from_fen_field(&mut self, castling_fen_field: &str) {
        if castling_fen_field.contains('K') {
            self.white_castling_rights.1 = true;
        } else {
            self.white_castling_rights.1 = false;
        }

        if castling_fen_field.contains('Q') {
            self.white_castling_rights.0 = true;
        } else {
            self.white_castling_rights.0 = false;
        }

        if castling_fen_field.contains('k') {
            self.black_castling_rights.0 = true;
        } else {
            self.black_castling_rights.0 = false;
        }

        if castling_fen_field.contains('q') {
            self.black_castling_rights.1 = true;
        } else {
            self.black_castling_rights.1 = false;
        }
    }

    fn set_en_passant_square_from_fen_field(&mut self, en_passant_fen_field: &str) {
        if en_passant_fen_field.contains('-') {
            return
        }

        let en_passant_square_index = self.get_piece_index_from_chess_notation(en_passant_fen_field.to_string());

        self.set_en_passant_square(en_passant_square_index as i64);
    }

    pub fn get_square(&self, target_square: i64) -> char {
        if target_square < 0 && target_square > 119 {
            panic!("Error: ChessBoard out of bounds!!!");
        }

        let target_square = target_square as usize;

        self.board_vector[target_square]
    }

    fn match_letter_to_unicode_chess_symbol(&self, letter: char, perspective_color: bool) -> char {
        let matched_letter = if perspective_color {
            let new_letter = match letter {
                'K' => '♔',
                'Q' => '♕',
                'R' => '♖',
                'B' => '♗',
                'N' => '♘',
                'P' => '♙',
                'k' => '♚',
                'q' => '♛',
                'r' => '♜',
                'b' => '♝',
                'n' => '♞',
                'p' => '♟',
                _   => letter,
            };

            new_letter
        } else {
            let new_letter = match letter {
                'K' => '♚',
                'Q' => '♛',
                'R' => '♜',
                'B' => '♝',
                'N' => '♞',
                'P' => '♟',
                'k' => '♔',
                'q' => '♕',
                'r' => '♖',
                'b' => '♗',
                'n' => '♘',
                'p' => '♙',
                _   => letter,
            };

            new_letter
        };

        matched_letter
    }

    fn print_board(&self, white: bool, unicode: bool) {
        let files = if white {
            "   a b c d e f g h \n "
        } else {
            "   h g f e d c b a \n "
        };

        let ranks = if white {
            [" ", " ", "8", "7", "6", "5", "4", "3", "2", "1", " ", " "]
        } else {
            [" ", " ", "1", "2", "3", "4", "5", "6", "7", "8", " ", " "]
        };

        let print_vector = if white == self.current_color {
            self.board_vector.clone()
        } else {
            self.return_rotated_board_vector_with_changed_letter_case()
        };

        for (index, i) in print_vector.iter().enumerate() {
            if index == 20 || index == 100 {
                print!("{}", files);
            }

            if index % 10 == 0  {
                print!("{}", ranks[index / 10]);
            }

            if unicode {
                print!("{} ", self.match_letter_to_unicode_chess_symbol(*i, white));
            } else {
                print!("{} ", i);
            }

        }
    }

    pub fn print_board_from_current_color_perspective(&self, unicode: bool) {
        self.print_board(self.current_color, unicode);
    }

    pub fn print_board_from_white_perspective(&self, unicode: bool) {
        self.print_board(true, unicode);
    }

    pub fn print_board_from_black_perspective(&self, unicode: bool) {
        self.print_board(false, unicode);
    }

    fn return_rotated_board_vector_with_changed_letter_case(&self) -> Vec<char> {
        let mut rotated_board: Vec<char> = Vec::new();
        let mut board_vector_clone = self.board_vector.clone();

        while let Some(square) = board_vector_clone.pop() {
            if square == '\n' {
                rotated_board.push(' ');
            } else if square == ' ' {
                rotated_board.push('\n');
            } else {
                rotated_board.push(square);
            }
        }

        let mut rotated_board_piece_changed = rotated_board.clone();

        for (index, piece) in rotated_board.iter().enumerate() {
            if piece.is_uppercase() {
                rotated_board_piece_changed[index] = piece.to_lowercase().to_string().chars().nth(0).unwrap();
            } else if piece.is_lowercase() {
                rotated_board_piece_changed[index] = piece.to_uppercase().to_string().chars().nth(0).unwrap();
            }
        }

        rotated_board_piece_changed
    }

    pub fn clone_and_rotate(&self) -> ChessBoard {
        ChessBoard {
            board_vector: self.return_rotated_board_vector_with_changed_letter_case(),
            current_color: !self.current_color,
            white_castling_rights: self.white_castling_rights,
            black_castling_rights: self.black_castling_rights,
            en_passant_square: self.en_passant_square,
            en_passant_lifetime: self.en_passant_lifetime,
            last_move: self.last_move.clone(),
            score: -self.score,
        }
    }

    fn get_piece_index_from_chess_notation(&self, location: String) -> usize {
        if location.len() != 2 {
            panic!("Error: Location in chess notation is wrong!!!");
        }

        let mut piece_index = String::new();

        if self.current_color {
            let second_digit = match location.chars().nth(0).unwrap() {
                'h' => '8',
                'g' => '7',
                'f' => '6',
                'e' => '5',
                'd' => '4',
                'c' => '3',
                'b' => '2',
                'a' => '1',
                _   => panic!("Error: Can't match file to index!!!"),
            };

            let first_digit = match location.chars().nth(1).unwrap() {
                '1' => '9',
                '2' => '8',
                '3' => '7',
                '4' => '6',
                '5' => '5',
                '6' => '4',
                '7' => '3',
                '8' => '2',
                _   => panic!("Error: Can't match rank to index!!!"),
            };

            piece_index.push(first_digit);
            piece_index.push(second_digit);

            piece_index.parse().unwrap()
        } else {
            let second_digit = match location.chars().nth(0).unwrap() {
                'h' => '1',
                'g' => '2',
                'f' => '3',
                'e' => '4',
                'd' => '5',
                'c' => '6',
                'b' => '7',
                'a' => '8',
                _   => panic!("Error: Can't match file to index!!!"),
            };

            let first_digit = match location.chars().nth(1).unwrap() {
                '1' => '2',
                '2' => '3',
                '3' => '4',
                '4' => '5',
                '5' => '6',
                '6' => '7',
                '7' => '8',
                '8' => '9',
                _   => panic!("Error: Can't match rank to index!!!"),
            };

            piece_index.push(first_digit);
            piece_index.push(second_digit);

            piece_index.parse().unwrap()
        }
    }

    fn get_piece_location_in_chess_notation(&self, location: usize) -> String {
        let location_as_string = location.to_string();
        let mut piece_location_notated = String::new();

        if self.current_color {
            let rank = match location_as_string.chars().nth(0).unwrap() {
                '2' => "8",
                '3' => "7",
                '4' => "6",
                '5' => "5",
                '6' => "4",
                '7' => "3",
                '8' => "2",
                '9' => "1",
                _   => panic!("Error: Can't match location to a rank!!!"),
            };

            let file = match location_as_string.chars().nth(1).unwrap() {
                '1' => "a",
                '2' => "b",
                '3' => "c",
                '4' => "d",
                '5' => "e",
                '6' => "f",
                '7' => "g",
                '8' => "h",
                _   => panic!("Error: Can't match location to a file!!!")
            };

            piece_location_notated.push_str(file);
            piece_location_notated.push_str(rank);
        } else {
            let rank = match location_as_string.chars().nth(0).unwrap() {
                '2' => "1",
                '3' => "2",
                '4' => "3",
                '5' => "4",
                '6' => "5",
                '7' => "6",
                '8' => "7",
                '9' => "8",
                _   => panic!("Error: Can't match location to a rank!!!"),
            };

            let file = match location_as_string.chars().nth(1).unwrap() {
                '1' => "h",
                '2' => "g",
                '3' => "f",
                '4' => "e",
                '5' => "d",
                '6' => "c",
                '7' => "b",
                '8' => "a",
                _   => panic!("Error: Can't match location to a file!!!")
            };

            piece_location_notated.push_str(file);
            piece_location_notated.push_str(rank);
        }

        piece_location_notated
    }

    pub fn print_last_move(&self) {
        if !self.last_move.is_empty() {
            println!("{}", self.last_move);
        }
    }

    pub fn print_last_move_with_moving_side_info(&self) {
        if !self.last_move.is_empty() {
            if self.current_color {
                println!("Black moves: {}", self.last_move);
            } else {
                println!("White moves: {}", self.last_move);
            }
        }
    }

    pub fn notate_move(&mut self, location: usize, destination: usize) {
        let mut move_notated = String::new();

        let location_notated = self.get_piece_location_in_chess_notation(location);
        let destination_notated = self.get_piece_location_in_chess_notation(destination);

        move_notated.push_str(&location_notated);
        move_notated.push_str(&destination_notated);

        self.last_move = move_notated;
    }

    pub fn make_move(&mut self, location: i64, destination: i64) {
        if location < 0 && location > 119 {
            panic!("Error: ChessBoard out of bounds!!!");
        }

        if destination < 0 && destination > 119 {
            panic!("Error: ChessBoard out of bounds!!!");
        }

        let location = location as usize;
        let destination = destination as usize;

        self.notate_move(location, destination);
        self.evaluate_move(location, destination);

        self.board_vector[destination] = self.board_vector[location];
        self.board_vector[location] = '.';
    }

    pub fn make_en_passant_move(&mut self, location: i64, destination: i64) {
        if location < 0 && location > 119 {
            panic!("Error: ChessBoard out of bounds!!!");
        }

        if destination < 0 && destination > 119 {
            panic!("Error: ChessBoard out of bounds!!!");
        }

        let location = location as usize;
        let destination = destination as usize;

        self.notate_move(location, destination);
        self.evaluate_en_passant_move(location, destination);

        self.board_vector[destination] = self.board_vector[location];
        self.board_vector[destination + 10] = '.';
        self.board_vector[location] = '.';
    }

    pub fn promote_pawn(&mut self, location: usize, promoted_piece: char) {
        if self.last_move.chars().count() == 5 {
            self.last_move.pop();
        }

        self.evaluate_pawn_promotion(location, promoted_piece);

        self.last_move += &promoted_piece.to_lowercase().to_string();

        self.board_vector[location] = promoted_piece;
    }

    pub fn set_castling_rights(&mut self, a1_castling: bool, h1_castling: bool) {
        if self.current_color {
            self.white_castling_rights = (a1_castling, h1_castling);
        } else {
            self.black_castling_rights = (a1_castling, h1_castling);
        }
    }

    pub fn set_left_castling(&mut self, a1_castling: bool) {
        if self.current_color {
            self.white_castling_rights = (a1_castling, self.white_castling_rights.1);
        } else {
            self.black_castling_rights = (a1_castling, self.black_castling_rights.1);
        }
    }

    pub fn set_right_castling(&mut self, h1_castling: bool) {
        if self.current_color {
            self.white_castling_rights = (self.white_castling_rights.0, h1_castling);
        } else {
            self.black_castling_rights = (self.black_castling_rights.0, h1_castling);
        }
    }

    pub fn get_left_castling(&self) -> bool {
        if self.current_color {
            self.white_castling_rights.0
        } else {
            self.black_castling_rights.0
        }
    }

    pub fn get_right_castling(&self) -> bool {
        if self.current_color {
            self.white_castling_rights.1
        } else {
            self.black_castling_rights.1
        }
    }

    pub fn get_current_color(&self) -> bool {
        self.current_color
    }

    pub fn set_en_passant_square(&mut self, new_en_passant_square: i64) {
        if new_en_passant_square < 0 && new_en_passant_square > 119 {
            panic!("Error: ChessBoard out of bounds!!!");
        }

        let new_en_passant_square = new_en_passant_square as usize;

        self.en_passant_square = new_en_passant_square;
        self.en_passant_lifetime = 2;
    }

    pub fn is_en_passant_active(&self) -> bool {
        if self.en_passant_lifetime > 0 {
            true
        } else {
            false
        }
    }

    pub fn decrease_en_passant_lifetime(&mut self) {
        if self.en_passant_lifetime > 0 {
            self.en_passant_lifetime -= 1;
        }
    }
}
