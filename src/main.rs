mod domain;
mod presentation;

use macroquad::prelude::*;
use presentation::ChessUI;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Chess".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let chess_ui = ChessUI::new();
    chess_ui.await.run().await;
}
