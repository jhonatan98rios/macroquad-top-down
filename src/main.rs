
mod player;
mod enemies;
mod strategies;
mod constants;
mod menu;
mod state;
mod components;
mod game;

use macroquad::prelude::*;
use menu::MenuScreen;
use state::GameState;
use game::Game;


#[macroquad::main("Macroquad WASM Game")]
async fn main() {
    let mut game_state = GameState::Menu;
    let mut menu = MenuScreen::new();
    let mut game = Game::new().await;

    game.init().await;

    loop {
        match game_state {
            GameState::Menu => {
                if let Some(next_state) = menu.draw() {
                    game_state = next_state;
                }
            },
            GameState::Playing => {
                game.update();
            }
        }   

        next_frame().await;
    }
}