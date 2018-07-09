use chess_board::ChessBoard;
use move_generator::MoveGenerator;
use searcher::Searcher;

pub struct Game {
    current_position: ChessBoard,
    legal_moves: Vec<ChessBoard>,
    move_generator: MoveGenerator,
    searcher: Searcher,
}

impl Game {
    pub fn new() -> Game {
        Game {
            current_position: ChessBoard::new(),
            legal_moves: Vec::new(),
            move_generator: MoveGenerator::new(),
            searcher: Searcher::new(),
        }
    }

    pub fn setup_initial_position(&mut self) {
        self.current_position.setup_initial_position();
        self.legal_moves = self.move_generator.generate_next_moves_from_board(&self.current_position);
    }
    
    pub fn build_board_from_fen_string(&mut self, fen_string: String) {
        self.current_position.build_board_from_fen_string(fen_string);
        self.legal_moves = self.move_generator.generate_next_moves_from_board(&self.current_position);
    }

    pub fn accept_move(&mut self, move_notated: String) {
        let mut move_legal = false;

        for legal_move in &self.legal_moves {
            if legal_move.last_move == move_notated {
                move_legal = true;
                self.current_position = legal_move.clone();
                break;
            }
        }

        if !move_legal {
            panic!("Error: Illegal move!!!");
        }

        self.legal_moves = self.move_generator.generate_next_moves_from_board(&self.current_position);
    }

    pub fn find_best_move(&mut self) {
        self.searcher.run_alpha_beta_pruning(&self.current_position, 4);

        println!("bestmove {}", self.searcher.next_move.last_move);
    }
}