use macroquad::prelude::*;

mod player;
mod enemies;

use player::Player;
use enemies::{EnemySystem, DirectMovement, SinusoidalMovement, BoidsMovement};



#[macroquad::main("Macroquad WASM Game")]
async fn main() {

    // Create player
    let mut player = Player::new(100.0, 100.0);

    #[allow(unused_variables)]
    let direct_enemy_strategy = Box::new(DirectMovement {
        speed: 2.0,
    });

    #[allow(unused_variables)]
    let sinusoidal_movement = Box::new(SinusoidalMovement {
        speed: 1.0, 
        amplitude: 2.0,
        frequency: 0.5,
    });

    #[allow(unused_variables)]
    let boids_movement = Box::new(BoidsMovement {
        max_speed: 2.5,          // Slower movement
        max_force: 0.3,          // More gradual turns
        separation_distance: 10.0, // More personal space
        alignment_distance: 50.0,
        cohesion_distance: 75.0,
        separation_weight: 1.0,   // More avoidance
        alignment_weight: 0.8,
        cohesion_weight: 0.7,
        target_weight: 5.0,       // Stronger follow
    });

    
    let mut enemies = EnemySystem::new(1000, direct_enemy_strategy);
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
            20.0,
            30.0,
            WHITE,
        );

        next_frame().await;
    }
}