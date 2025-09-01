#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Option<Self> {
        if row < 8 && col < 8 {
            Some(Position { row, col })
        } else {
            None
        }
    }

    pub fn from_chess_notation(notation: &str) -> Option<Self> {
        if notation.len() != 2 {
            return None;
        }
        
        let chars: Vec<char> = notation.chars().collect();
        let col = match chars[0] {
            'a' => 0, 'b' => 1, 'c' => 2, 'd' => 3,
            'e' => 4, 'f' => 5, 'g' => 6, 'h' => 7,
            _ => return None,
        };
        
        let row = match chars[1] {
            '1' => 7, '2' => 6, '3' => 5, '4' => 4,
            '5' => 3, '6' => 2, '7' => 1, '8' => 0,
            _ => return None,
        };
        
        Some(Position { row, col })
    }

    pub fn to_chess_notation(&self) -> String {
        let col_char = match self.col {
            0 => 'a', 1 => 'b', 2 => 'c', 3 => 'd',
            4 => 'e', 5 => 'f', 6 => 'g', 7 => 'h',
            _ => '?',
        };
        
        let row_char = match self.row {
            0 => '8', 1 => '7', 2 => '6', 3 => '5',
            4 => '4', 5 => '3', 6 => '2', 7 => '1',
            _ => '?',
        };
        
        format!("{}{}", col_char, row_char)
    }

    pub fn is_valid(&self) -> bool {
        self.row < 8 && self.col < 8
    }
}