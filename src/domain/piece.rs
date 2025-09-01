#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }

    pub fn unicode_symbol(&self) -> char {
        match (self.color, self.piece_type) {
            (Color::White, PieceType::King) => '♔',
            (Color::White, PieceType::Queen) => '♕',
            (Color::White, PieceType::Rook) => '♖',
            (Color::White, PieceType::Bishop) => '♗',
            (Color::White, PieceType::Knight) => '♘',
            (Color::White, PieceType::Pawn) => '♙',
            (Color::Black, PieceType::King) => '♚',
            (Color::Black, PieceType::Queen) => '♛',
            (Color::Black, PieceType::Rook) => '♜',
            (Color::Black, PieceType::Bishop) => '♝',
            (Color::Black, PieceType::Knight) => '♞',
            (Color::Black, PieceType::Pawn) => '♟',
        }
    }
}