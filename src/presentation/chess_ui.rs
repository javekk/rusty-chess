use crate::domain::{Color, Game, Piece, PieceType, Position};
use macroquad::prelude::*;

const BOARD_SIZE: f32 = 640.0;
const SQUARE_SIZE: f32 = BOARD_SIZE / 8.0;
const BOARD_OFFSET_X: f32 = 50.0;
const BOARD_OFFSET_Y: f32 = 50.0;

pub struct ChessUI {
    game: Game,
    selected_square: Option<Position>,
    dragging_piece: Option<Position>,
    drag_offset: (f32, f32),
    pieces_texture: Texture2D,
}

impl ChessUI {
    pub async fn new() -> Self {
        let _pieces_texture = load_texture("src/presentation/assests/pieces/chess_pieces_humans_vs_zombies.png")
            .await
            .unwrap();

        // Set texture filtering to Nearest for pixel-perfect scaling
        _pieces_texture.set_filter(FilterMode::Nearest);

        ChessUI {
            game: Game::new(),
            selected_square: None,
            dragging_piece: None,
            drag_offset: (0.0, 0.0),
            pieces_texture: _pieces_texture,
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.handle_input().await;
            self.draw();
            next_frame().await;
        }
    }

    async fn handle_input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            if let Some(pos) = self.screen_to_board_position(mouse_x, mouse_y) {
                if let Some(piece) = self.game.board().get_piece(pos) {
                    if piece.color == self.game.current_player() {
                        self.dragging_piece = Some(pos);
                        self.selected_square = Some(pos);
                        let (board_x, board_y) = self.board_to_screen_position(pos);
                        self.drag_offset = (mouse_x - board_x, mouse_y - board_y);
                    }
                } else if let Some(selected) = self.selected_square {
                    if let Err(error) = self.game.make_move(selected, pos) {
                        println!("Invalid move: {}", error);
                    }
                    self.selected_square = None;
                    self.dragging_piece = None;
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(drag_pos) = self.dragging_piece {
                let (mouse_x, mouse_y) = mouse_position();

                if let Some(target_pos) = self.screen_to_board_position(mouse_x, mouse_y) {
                    if let Err(error) = self.game.make_move(drag_pos, target_pos) {
                        println!("Invalid move: {}", error);
                    }
                }

                self.dragging_piece = None;
                self.selected_square = None;
            }
        }

        if is_key_pressed(KeyCode::U) {
            if !self.game.undo_move() {
                println!("Nothing to undo");
            }
        }

        if is_key_pressed(KeyCode::R) && is_key_down(KeyCode::LeftControl) {
            if !self.game.redo_move() {
                println!("Nothing to redo");
            }
        }

        if is_key_pressed(KeyCode::N) && is_key_down(KeyCode::LeftControl) {
            self.game.restart();
            self.selected_square = None;
            self.dragging_piece = None;
        }
    }

    fn draw(&self) {
        clear_background(WHITE);

        self.draw_board();
        self.draw_pieces();
        self.draw_ui_info();
    }

    fn draw_board(&self) {
        for row in 0..8 {
            for col in 0..8 {
                let x = BOARD_OFFSET_X + col as f32 * SQUARE_SIZE;
                let y = BOARD_OFFSET_Y + row as f32 * SQUARE_SIZE;

                let is_light_square = (row + col) % 2 == 0;
                let mut color = if is_light_square {
                    macroquad::color::Color::from_rgba(100, 64, 37, 255)
                } else {
                    macroquad::color::Color::from_rgba(76, 57, 59, 255)
                };

                if let Some(selected) = self.selected_square {
                    if selected.row == row && selected.col == col {
                        color = macroquad::color::Color::from_rgba(255, 255, 0, 128);
                    }
                }

                draw_rectangle(x, y, SQUARE_SIZE, SQUARE_SIZE, color);
            }
        }

        draw_rectangle_lines(
            BOARD_OFFSET_X,
            BOARD_OFFSET_Y,
            BOARD_SIZE,
            BOARD_SIZE,
            2.0,
            BLACK,
        );
    }

    fn draw_pieces(&self) {
        let board_state = self.game.board().get_state();

        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = board_state[row][col] {
                    let pos = Position::new(row, col).unwrap();

                    if Some(pos) == self.dragging_piece {
                        let (mouse_x, mouse_y) = mouse_position();
                        let piece_x = mouse_x - self.drag_offset.0;
                        let piece_y = mouse_y - self.drag_offset.1;
                        self.draw_piece_at(piece, piece_x, piece_y);
                    } else {
                        let (piece_x, piece_y) = self.board_to_screen_position(pos);
                        self.draw_piece_at(piece, piece_x, piece_y);
                    }
                }
            }
        }
    }

    fn get_piece_sprite_coords(&self, piece: Piece) -> (f32, f32) {
        const SPRITE_SIZE: f32 = 16.0;
        
        let (row, col) = match (piece.color, piece.piece_type) {
            // White pieces (top row)
            (Color::White, PieceType::Pawn) => (0, 0),
            (Color::White, PieceType::Knight) => (0, 1),
            (Color::White, PieceType::Bishop) => (0, 2),
            (Color::White, PieceType::Rook) => (1, 0),
            (Color::White, PieceType::Queen) => (1, 1),
            (Color::White, PieceType::King) => (1, 2),
            
            // Black pieces (second row)
            (Color::Black, PieceType::Pawn) => (2, 0),
            (Color::Black, PieceType::Knight) => (2, 1),
            (Color::Black, PieceType::Bishop) => (2, 2),
            (Color::Black, PieceType::Rook) => (3, 0),
            (Color::Black, PieceType::Queen) => (3, 1),
            (Color::Black, PieceType::King) => (3, 2),
        };
        
        (col as f32 * SPRITE_SIZE, row as f32 * SPRITE_SIZE)
    }

    fn draw_piece_at(&self, piece: crate::domain::Piece, x: f32, y: f32) {
        let (sprite_x, sprite_y) = self.get_piece_sprite_coords(piece);
        const SPRITE_SIZE: f32 = 16.0;
        
        // Calculate piece size (smaller than square to avoid distortion)
        let piece_size = SQUARE_SIZE * 0.8;
        let offset = (SQUARE_SIZE - piece_size) / 2.0;

        draw_texture_ex(
            &self.pieces_texture,
            x + offset,
            y + offset,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(sprite_x, sprite_y, SPRITE_SIZE, SPRITE_SIZE)),
                dest_size: Some(Vec2::new(piece_size, piece_size)),
                ..Default::default()
            }
        );
    }

    fn draw_ui_info(&self) {
        let current_player = match self.game.current_player() {
            Color::White => "White",
            Color::Black => "Black",
        };

        draw_text(
            &format!("Current player: {}", current_player),
            BOARD_OFFSET_X,
            BOARD_OFFSET_Y + BOARD_SIZE + 30.0,
            24.0,
            BLACK,
        );

        draw_text(
            "Controls: U - Undo, Ctrl+R - Redo, Ctrl+N - New Game",
            BOARD_OFFSET_X,
            BOARD_OFFSET_Y + BOARD_SIZE + 60.0,
            20.0,
            GRAY,
        );
    }

    fn screen_to_board_position(&self, screen_x: f32, screen_y: f32) -> Option<Position> {
        let board_x = screen_x - BOARD_OFFSET_X;
        let board_y = screen_y - BOARD_OFFSET_Y;

        if board_x >= 0.0 && board_x < BOARD_SIZE && board_y >= 0.0 && board_y < BOARD_SIZE {
            let col = (board_x / SQUARE_SIZE) as usize;
            let row = (board_y / SQUARE_SIZE) as usize;
            Position::new(row, col)
        } else {
            None
        }
    }

    fn board_to_screen_position(&self, pos: Position) -> (f32, f32) {
        let x = BOARD_OFFSET_X + pos.col as f32 * SQUARE_SIZE;
        let y = BOARD_OFFSET_Y + pos.row as f32 * SQUARE_SIZE;
        (x, y)
    }
}
