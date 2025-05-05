#[macro_use]
mod macros;

mod player;
mod enemies;
mod strategies;
mod constants;
mod menu;
mod pause;
mod state;
mod components;
mod game;
mod event_bus;

use macroquad::prelude::*;
use macroquad::window;

use menu::MenuScreen;
use pause::PauseScreen;

use state::GameState;
use game::Game;
use components::joystick::Joystick;
use components::layout::is_mobile;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Macroquad WASM Game".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Menu;
    let mut menu_screen = MenuScreen::new();
    let mut pause_screen = PauseScreen::new();

    let joystick_size = 200.0;
    let joystick_pos_x = (screen_width() / 2.0) - (joystick_size / 5.0);
    let joystick_pos_y = screen_height() - joystick_size - 20.0;
    let joystick = if is_mobile() {
        Some(Joystick::new(vec2(joystick_pos_x, joystick_pos_y), joystick_size))
    } else {
        None
    };

    let mut game = Game::new(joystick.clone()).await;
    game.init().await;

    let mut previous_state = game_state;

    loop {
        match game_state {
            GameState::Menu => {
                if let Some(next_state) = menu_screen.draw() {
                    game_state = next_state;
                }
            },
            GameState::Playing => {
                if is_key_pressed(KeyCode::Escape) || (is_mobile() && is_key_pressed(KeyCode::Back)) {
                    game_state = GameState::Paused;
                }

                game.update();
            },
            GameState::Paused => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Playing;
                }
    
                if let Some(next_state) = pause_screen.draw() {
                    game_state = next_state;
                }
            }
        }   

        // 💡 Reset the game if returning from Paused to Menu
        if previous_state == GameState::Paused && game_state == GameState::Menu {
            game = Game::new(joystick.clone()).await;
            game.init().await;
        }

        previous_state = game_state;
        next_frame().await;
    }
}