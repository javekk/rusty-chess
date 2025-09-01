use crate::domain::{Color, Game, Position};
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
    font: Font,
}

impl ChessUI {
    pub async fn new() -> Self {
        let _font = load_ttf_font("src/presentation/assests/fonts/0xProtoNerdFontMono-Bold.ttf")
            .await
            .unwrap();

        ChessUI {
            game: Game::new(),
            selected_square: None,
            dragging_piece: None,
            drag_offset: (0.0, 0.0),
            font: _font,
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
                    macroquad::color::Color::from_rgba(240, 217, 181, 255)
                } else {
                    macroquad::color::Color::from_rgba(181, 136, 99, 255)
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

    fn draw_piece_at(&self, piece: crate::domain::Piece, x: f32, y: f32) {
        let symbol = piece.unicode_symbol();
        let font_size = SQUARE_SIZE * 0.8;
        draw_text_ex(
            &symbol.to_string(),
            x + SQUARE_SIZE * 0.1,
            y + SQUARE_SIZE * 0.8,
            TextParams {
                font_size: font_size as u16,
                font_scale: 1.0,
                color: BLACK,
                font: Some(&self.font),
                ..Default::default()
            },
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
