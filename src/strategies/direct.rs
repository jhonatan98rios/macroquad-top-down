use super::{MovementStrategy};
use macroquad::prelude::*;

// Direct movement toward player
pub struct DirectMovement {
    pub speed: f32,
}

impl MovementStrategy for DirectMovement {
    fn move_enemy(&self, x: &mut f32, y: &mut f32, target: (f32, f32), _time: f32, _index: usize, _all_positions: &[(f32, f32)]) {
        let dx = target.0 - *x;
        let dy = target.1 - *y;
        let dist = (dx * dx + dy * dy).sqrt().max(0.0001);
        
        *x += (dx / dist) * self.speed;
        *y += (dy / dist) * self.speed;
    }
}