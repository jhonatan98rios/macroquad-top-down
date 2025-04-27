mod boids;

pub use boids::BoidsMovement;

use macroquad::prelude::*;

#[allow(dead_code)]
pub trait MovementStrategy: Send + Sync {
    fn move_enemy(
        &self, 
        position: &mut Vec2,
        target: Vec2,
        time: f32,
        index: usize,
        all_positions: &[Vec2]
    );
}