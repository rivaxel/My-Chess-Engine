use chess_board::ChessBoard;

static PAWN_SQUARE_TABLE: [i64; 120] = [
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,  50,  50,  50,  50,  50,  50,  50,  50,   0,
    0,  10,  10,  20,  30,  30,  20,  10,  10,   0,
    0,   5,   5,  10,  27,  27,  10,   5,   5,   0,
    0,   0,   0,   0,  25,  25,   0,   0,   0,   0,
    0,   5,  -5, -10,   0,   0, -10,  -5,   5,   0,
    0,   5,  10,  10, -25, -25,  10,  10,   5,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
];

static KNIGHT_SQUARE_TABLE: [i64; 120] = [
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0, -50, -40, -30, -30, -30, -30, -40, -50,   0,
    0, -40, -20,   0,   0,   0,   0, -20, -40,   0,
    0, -30,   0,  10,  15,  15,  10,   0, -30,   0,
    0, -30,   5,  15,  20,  20,  15,   5, -30,   0,
    0, -30,   0,  15,  20,  20,  15,   0, -30,   0,
    0, -30,   5,  10,  15,  15,  10,   5, -30,   0,
    0, -40, -20,   0,   5,   5,   0, -20, -40,   0,
    0, -50, -40, -20, -30, -30, -20, -40, -50,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
];

static BISHOP_SQUARE_TABLE: [i64; 120] = [
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0, -20, -10, -10, -10, -10, -10, -10, -20,   0,
    0, -10,   0,   0,   0,   0,   0,   0, -10,   0,
    0, -10,   0,   5,  10,  10,   5,   0, -10,   0,
    0, -10,   5,   5,  10,  10,   5,   5, -10,   0,
    0, -10,   0,  10,  10,  10,  10,   0, -10,   0,
    0, -10,  10,  10,  10,  10,  10,  10, -10,   0,
    0, -10,   5,   0,   0,   0,   0,   5, -10,   0,
    0, -20, -10, -40, -10, -10, -40, -10, -20,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
];

static ROOK_SQUARE_TABLE: [i64; 120] = [
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,  0 ,   0,   0,   0,
    0,   5,  10,  10,  10,  10,  10,  10,   5,   0,
    0,  -5,   0,   0,   0,   0,   0,   0,  -5,   0,
    0,  -5,   0,   0,   0,   0,   0,   0,  -5,   0,
    0,  -5,   0,   0,   0,   0,   0,   0,  -5,   0,
    0,  -5,   0,   0,   0,   0,   0,   0,  -5,   0,
    0,  -5,   0,   0,   0,   0,   0,   0,  -5,   0,
    0,   0,   0,   0,   5,   5,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
];

static QUEEN_SQUARE_TABLE: [i64; 120] = [
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0, -20, -10, -10,  -5,  -5, -10, -10, -20,   0,
    0, -10,   0,   0,   0,   0,   0,   0, -10,   0,
    0, -10,   0,   5,   5,   5,   5,   0, -10,   0,
    0,  -5,   0,   5,   5,   5,   5,   0,  -5,   0,
    0,   0,   0,   5,   5,   5,   5,   0,  -5,   0,
    0, -10,   5,   5,   5,   5,   5,   0, -10,   0,
    0, -10,   0,   5,   0,   0,   0,   0, -10,   0,
    0, -20, -10, -10,  -5,  -5, -10, -10, -20,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
];

static KING_SQUARE_TABLE: [i64; 120] = [
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0, -30, -40, -40, -50, -50, -40, -40, -30,   0,
    0, -30, -40, -40, -50, -50, -40, -40, -30,   0,
    0, -30, -40, -40, -50, -50, -40, -40, -30,   0,
    0, -30, -40, -40, -50, -50, -40, -40, -30,   0,
    0, -20, -30, -30, -40, -40, -30, -30, -20,   0,
    0, -10, -20, -20, -20, -20, -20, -20, -10,   0,
    0,  20,  20,   0,   0,   0,   0,  20,  20,   0,
    0,  20,  30,  10,   0,   0,  10,  30,  20,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
];

impl ChessBoard {
    pub(super) fn evaluate_move(&mut self, location: usize, destination: usize) {
        let location_piece = self.board_vector[location];
        let destination_piece = self.board_vector[destination];

        let location_score = self.match_square_to_score(location, location_piece);

        let destination_score = self.match_square_to_score(destination, location_piece);

        let mut score = destination_score - location_score;

        if destination_piece.is_lowercase() {
            let capture_bonus = self.evaluate_piece_score(destination_piece);

            score += capture_bonus + self.match_square_to_score(destination, destination_piece);
        }

        self.score += score;
    }

    pub(super) fn evaluate_en_passant_move(&mut self, location: usize, destination: usize) {
        self.evaluate_move(location, destination);

        let en_passant_square_piece = self.board_vector[destination + 10];

        let capture_bonus = self.evaluate_piece_score(en_passant_square_piece);

        self.score += capture_bonus + self.match_square_to_score(destination + 10, en_passant_square_piece);
    }

    pub(super) fn evaluate_pawn_promotion(&mut self, location: usize, promoted_piece: char) {
        let promotion_bonus = self.evaluate_piece_score(promoted_piece);

        self.score += promotion_bonus + self.match_square_to_score(location, promoted_piece);;
    }

    pub fn add_castling_bonus(&mut self) {
        self.score += 110;
    }

    fn match_square_to_score(&self, index: usize, piece: char) -> i64 {
        let destination_score = match piece.to_ascii_uppercase() {
            'P' => PAWN_SQUARE_TABLE[index],
            'N' => KNIGHT_SQUARE_TABLE[index],
            'B' => BISHOP_SQUARE_TABLE[index],
            'R' => ROOK_SQUARE_TABLE[index],
            'Q' => QUEEN_SQUARE_TABLE[index],
            'K' => KING_SQUARE_TABLE[index],
            _   => 0
        };

        destination_score
    }

    fn evaluate_piece_score(&self, piece: char) -> i64 {
        match piece.to_ascii_uppercase() {
            'P' => 100,
            'N' => 310,
            'B' => 370,
            'R' => 500,
            'Q' => 950,
            'K' => 60000,
            _   => 0
        }
    }

    pub fn get_score(&self) -> i64 {
        self.score
    }
}
