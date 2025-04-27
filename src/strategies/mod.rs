mod boids;
mod direct;
mod sinusoidal;

pub use boids::BoidsMovement;
pub use direct::DirectMovement;
pub use sinusoidal::SinusoidalMovement;

use macroquad::prelude::*;

#[allow(dead_code)]
pub trait MovementStrategy: Send + Sync {
    fn move_enemy(&self, x: &mut f32, y: &mut f32, target: (f32, f32), time: f32, index: usize, all_positions: &[(f32, f32)]);
}

#[derive(Clone)]
pub struct EnemyData {
    pub x: f32,
    pub y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
}