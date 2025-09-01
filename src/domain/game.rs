use super::{Board, Position, Color, PieceType, MoveRecord};

#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    current_player: Color,
    move_history: Vec<MoveRecord>,
    history_index: usize,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_player: Color::White,
            move_history: Vec::new(),
            history_index: 0,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn current_player(&self) -> Color {
        self.current_player
    }

    pub fn make_move(&mut self, from: Position, to: Position) -> Result<(), String> {
        let piece = self.board.get_piece(from)
            .ok_or("No piece at source position")?;

        if piece.color != self.current_player {
            return Err("Not your piece".to_string());
        }

        if !self.is_valid_move(from, to) {
            return Err("Invalid move".to_string());
        }

        let captured_piece = self.board.move_piece(from, to);
        
        let move_record = MoveRecord::new(from, to, piece, captured_piece);
        
        self.move_history.truncate(self.history_index);
        self.move_history.push(move_record);
        self.history_index += 1;
        
        self.current_player = match self.current_player {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        Ok(())
    }

    pub fn undo_move(&mut self) -> bool {
        if self.history_index == 0 {
            return false;
        }

        self.history_index -= 1;
        let move_record = &self.move_history[self.history_index];

        self.board.set_piece(move_record.from, Some(move_record.moved_piece));
        self.board.set_piece(move_record.to, move_record.captured_piece);

        self.current_player = match self.current_player {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        true
    }

    pub fn redo_move(&mut self) -> bool {
        if self.history_index >= self.move_history.len() {
            return false;
        }

        let move_record = &self.move_history[self.history_index];
        self.board.move_piece(move_record.from, move_record.to);
        
        self.history_index += 1;
        
        self.current_player = match self.current_player {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        true
    }

    pub fn restart(&mut self) {
        self.board = Board::new();
        self.current_player = Color::White;
        self.move_history.clear();
        self.history_index = 0;
    }

    fn is_valid_move(&self, from: Position, to: Position) -> bool {
        if from == to {
            return false;
        }

        let piece = match self.board.get_piece(from) {
            Some(p) => p,
            None => return false,
        };

        if let Some(target_piece) = self.board.get_piece(to) {
            if target_piece.color == piece.color {
                return false;
            }
        }

        match piece.piece_type {
            PieceType::Pawn => self.is_valid_pawn_move(from, to, piece.color),
            PieceType::Rook => self.is_valid_rook_move(from, to),
            PieceType::Bishop => self.is_valid_bishop_move(from, to),
            PieceType::Queen => self.is_valid_queen_move(from, to),
            PieceType::King => self.is_valid_king_move(from, to),
            PieceType::Knight => self.is_valid_knight_move(from, to),
        }
    }

    fn is_valid_pawn_move(&self, from: Position, to: Position, color: Color) -> bool {
        let direction = match color {
            Color::White => -1,
            Color::Black => 1,
        };

        let row_diff = to.row as i32 - from.row as i32;
        let col_diff = to.col as i32 - from.col as i32;

        if col_diff == 0 {
            if row_diff == direction && self.board.get_piece(to).is_none() {
                return true;
            }
            
            let start_row = match color {
                Color::White => 6,
                Color::Black => 1,
            };
            
            if from.row == start_row && row_diff == direction * 2 && self.board.get_piece(to).is_none() {
                return true;
            }
        } else if col_diff.abs() == 1 && row_diff == direction {
            return self.board.get_piece(to).is_some();
        }

        false
    }

    fn is_valid_rook_move(&self, from: Position, to: Position) -> bool {
        if from.row != to.row && from.col != to.col {
            return false;
        }
        self.is_path_clear(from, to)
    }

    fn is_valid_bishop_move(&self, from: Position, to: Position) -> bool {
        let row_diff = (to.row as i32 - from.row as i32).abs();
        let col_diff = (to.col as i32 - from.col as i32).abs();
        
        if row_diff != col_diff {
            return false;
        }
        
        self.is_path_clear(from, to)
    }

    fn is_valid_queen_move(&self, from: Position, to: Position) -> bool {
        self.is_valid_rook_move(from, to) || self.is_valid_bishop_move(from, to)
    }

    fn is_valid_king_move(&self, from: Position, to: Position) -> bool {
        let row_diff = (to.row as i32 - from.row as i32).abs();
        let col_diff = (to.col as i32 - from.col as i32).abs();
        
        row_diff <= 1 && col_diff <= 1
    }

    fn is_valid_knight_move(&self, from: Position, to: Position) -> bool {
        let row_diff = (to.row as i32 - from.row as i32).abs();
        let col_diff = (to.col as i32 - from.col as i32).abs();
        
        (row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2)
    }

    fn is_path_clear(&self, from: Position, to: Position) -> bool {
        let row_step = match to.row.cmp(&from.row) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        };
        
        let col_step = match to.col.cmp(&from.col) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        };

        let mut current_row = from.row as i32 + row_step;
        let mut current_col = from.col as i32 + col_step;
        
        while current_row != to.row as i32 || current_col != to.col as i32 {
            let pos = Position::new(current_row as usize, current_col as usize).unwrap();
            if self.board.get_piece(pos).is_some() {
                return false;
            }
            
            current_row += row_step;
            current_col += col_step;
        }
        
        true
    }
}