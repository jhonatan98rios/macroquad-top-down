
mod player;
mod enemies;
mod strategies;

use player::Player;
use enemies::EnemySystem;
use strategies::{BoidsMovement};
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

    loop {
        clear_background(BLACK);

        // Update and draw player
        player.update();
        player.draw();

        // Update and draw enemies
        enemies.update(player.position());
        enemies.draw();

        // Display debug info
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