use super::{Position, Piece};

#[derive(Debug, Clone)]
pub struct MoveRecord {
    pub from: Position,
    pub to: Position,
    pub moved_piece: Piece,
    pub captured_piece: Option<Piece>,
}

impl MoveRecord {
    pub fn new(from: Position, to: Position, moved_piece: Piece, captured_piece: Option<Piece>) -> Self {
        MoveRecord {
            from,
            to,
            moved_piece,
            captured_piece,
        }
    }
}