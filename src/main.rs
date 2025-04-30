
mod player;
mod enemies;
mod strategies;
mod constants;
mod menu;
mod state;
mod components;

use macroquad::prelude::*;
use menu::MenuScreen;
use player::Player;
use enemies::{EnemySystem, PositionOverlap};
use strategies::{BoidsMovement};
use constants::{WORLD_WIDTH, WORLD_HEIGHT};
use state::GameState;


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

pub struct Game {
    player: Player,
    enemies: EnemySystem,
    camera: Camera2D,
}

impl Game {
    pub async fn new() -> Self {
        Game {
            player: Player::new(100.0, 100.0).await,
            enemies: EnemySystem::new(3000, Box::new(BoidsMovement {
                visual_range: 32.0,
                separation_dist: 40.0,
                max_speed: 3.0,
                player_weight: 0.8,
                player_distance: 2000.0, 
                noise_strength: 0.05,
                separation_weight: 3.2,
                alignment_weight: 1.5,
                cohesion_weight: 0.3,
            })).await,
            camera: Camera2D {
                zoom: vec2(2.0 / screen_width(), -2.0 / screen_height()),
                ..Default::default()
            },
        }
    }

    pub async fn init(&mut self) {
        self.enemies.spawn_all();
    }

    pub fn update(&mut self) {
        clear_background(BLACK);
        
        // Update zoom every frame
        self.camera.zoom = vec2(2.0 / screen_width(), -2.0 / screen_height());

        self.camera.target = clamp_camera_target(self.player.position());
        set_camera(&self.camera);

        draw_rectangle(0.0, 0.0, WORLD_WIDTH, WORLD_HEIGHT, Color::from_rgba(30, 30, 30, 255));

        self.player.update();
        self.enemies.update(self.player.position());

        self.enemies.draw(self.player.position(),PositionOverlap::Behind);
        self.player.draw();
        self.enemies.draw(self.player.position(),PositionOverlap::InFront);

        set_default_camera();
        draw_text(
            &format!("WASD or Arrows to move | FPS: {} | enemies {}", get_fps(), self.enemies.positions.len()),
            20.0,
            30.0,
            30.0,
            WHITE,
        );
    }
}

#[inline]
fn clamp_camera_target(player_position: Vec2) -> Vec2 {
    let half_screen_width = screen_width() / 2.0;
    let half_screen_height = screen_height() / 2.0;

    Vec2::new(
        player_position.x.clamp(half_screen_width, WORLD_WIDTH - half_screen_width),
        player_position.y.clamp(half_screen_height, WORLD_HEIGHT - half_screen_height),
    )
}