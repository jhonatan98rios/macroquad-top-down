use super::{MovementStrategy, EnemyData};
use macroquad::prelude::*;

pub struct ZigzagMovement {
    pub speed: f32,
    pub amplitude: f32,
    pub frequency: f32,
}

impl MovementStrategy for ZigzagMovement {
    fn move_enemy(&self, x: &mut f32, y: &mut f32, target: (f32, f32), time: f32, index: usize) {
        let dir_x = target.0 - *x;
        let dir_y = target.1 - *y;
        let dist = (dir_x * dir_x + dir_y * dir_y).sqrt().max(0.1);
        
        let zigzag = (time * self.frequency + index as f32 * 0.3).sin() * self.amplitude;
        
        *x += (dir_x / dist) * self.speed;
        *y += (dir_y / dist) * self.speed + zigzag;
    }

    fn batch_update(&self, enemies: &mut [EnemyData], target: (f32, f32), time: f32) {
        for (i, enemy) in enemies.iter_mut().enumerate() {
            self.move_enemy(
                &mut enemy.x,
                &mut enemy.y,
                target,
                time,
                i,
            );
        }
    }
}