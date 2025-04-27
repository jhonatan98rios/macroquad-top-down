
mod player;
mod enemies;
mod strategies;

use player::Player;
use enemies::EnemySystem;
use strategies::{BoidsMovement, DirectMovement, SinusoidalMovement, ZigzagMovement, OrbitMovement};
use macroquad::prelude::*;


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
        visual_range: 50.0,
        separation_dist: 20.0,
        max_speed: 2.0,
        player_weight: 0.7,
        player_distance: 25.0,
        noise_strength: 0.4,
    });

    #[allow(unused_variables)]
    let zigzag_movement = Box::new(ZigzagMovement {
        speed: 2.0,
        amplitude: 20.0,
        frequency: 0.1,
    });

    #[allow(unused_variables)]
    let orbit_movement = Box::new(OrbitMovement {
        speed: 5.0,
        radius: 50.0,
        angular_speed: 0.5,
    });

    
    let mut enemies = EnemySystem::new(10, boids_movement);
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