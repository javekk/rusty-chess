use super::{Piece, Position, PieceType, Color};

pub type BoardState = [[Option<Piece>; 8]; 8];

#[derive(Debug, Clone)]
pub struct Board {
    state: BoardState,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            state: [[None; 8]; 8],
        };
        board.setup_initial_position();
        board
    }

    pub fn empty() -> Self {
        Board {
            state: [[None; 8]; 8],
        }
    }

    fn setup_initial_position(&mut self) {
        let back_row = [
            PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen,
            PieceType::King, PieceType::Bishop, PieceType::Knight, PieceType::Rook
        ];

        for col in 0..8 {
            self.state[0][col] = Some(Piece::new(back_row[col], Color::Black));
            self.state[1][col] = Some(Piece::new(PieceType::Pawn, Color::Black));
            self.state[6][col] = Some(Piece::new(PieceType::Pawn, Color::White));
            self.state[7][col] = Some(Piece::new(back_row[col], Color::White));
        }
    }

    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        if pos.is_valid() {
            self.state[pos.row][pos.col]
        } else {
            None
        }
    }

    pub fn set_piece(&mut self, pos: Position, piece: Option<Piece>) -> bool {
        if pos.is_valid() {
            self.state[pos.row][pos.col] = piece;
            true
        } else {
            false
        }
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> Option<Piece> {
        if !from.is_valid() || !to.is_valid() {
            return None;
        }

        let piece = self.state[from.row][from.col]?;
        let captured = self.state[to.row][to.col];
        
        self.state[from.row][from.col] = None;
        self.state[to.row][to.col] = Some(piece);
        
        captured
    }

    pub fn get_state(&self) -> &BoardState {
        &self.state
    }
}