use super::{MovementStrategy, EnemyData};
use macroquad::prelude::*;

// Sinusoidal movement pattern
pub struct SinusoidalMovement {
    pub speed: f32,
    pub amplitude: f32,
    pub frequency: f32,
}

impl MovementStrategy for SinusoidalMovement {
    fn move_enemy(&self, x: &mut f32, y: &mut f32, target: (f32, f32), time: f32, _index: usize) {
        let dx = target.0 - *x;
        let dy = target.1 - *y;
        let dist = (dx * dx + dy * dy).sqrt().max(0.0001);
        
        *x += (dx / dist) * self.speed + time.sin() * self.amplitude * self.frequency;
        *y += (dy / dist) * self.speed + time.cos() * self.amplitude * self.frequency;
    }

    #[allow(unused_variables)]
    fn batch_update(&self, enemies: &mut [EnemyData], target: (f32, f32), time: f32) {}
}