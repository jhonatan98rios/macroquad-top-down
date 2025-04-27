
mod player;
mod enemies;
mod strategies;
mod constants;

use player::Player;
use enemies::EnemySystem;
use strategies::{BoidsMovement};
use constants::{WORLD_WIDTH, WORLD_HEIGHT};


use macroquad::prelude::*;


#[macroquad::main("Macroquad WASM Game")]
async fn main() {

    // Create player
    let mut player = Player::new(100.0, 100.0).await;

    #[allow(unused_variables)]
    let boids_movement = Box::new(BoidsMovement {
        visual_range: 18.0,
        separation_dist: 25.0,
        max_speed: 2.0,
        player_weight: 0.8,
        player_distance: 2000.0, 
        noise_strength: 0.05,
        separation_weight: 3.2,
        alignment_weight: 1.5,
        cohesion_weight: 0.3,
    });
    
    let mut enemies = EnemySystem::new(3000, boids_movement).await;
    enemies.spawn_all();

    let mut camera = Camera2D {
        zoom: vec2(2.0 / screen_width(), -2.0 / screen_height()), // Deixa 1:1 normal
        ..Default::default()
    };

    loop {
        clear_background(BLACK);
        camera.target = clamp_camera_target(player.position());
        set_camera(&camera);

        // Draw the floor
        draw_rectangle(0.0, 0.0, WORLD_WIDTH, WORLD_HEIGHT, Color::from_rgba(30, 30, 30, 255));

        // Update and draw player
        player.update();
        player.draw();

        // Update and draw enemies
        enemies.update(player.position());
        enemies.draw();

        // Display debug info
        set_default_camera();
        draw_text(
            &format!("WASD or Arrows to move | FPS: {}", get_fps()),
            20.0,
            30.0,
            30.0,
            WHITE,
        );

        next_frame().await;
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