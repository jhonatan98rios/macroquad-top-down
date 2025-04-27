use super::{MovementStrategy, EnemyData};
use macroquad::prelude::*;

pub struct OrbitMovement {
    #[allow(dead_code)]
    pub speed: f32,
    pub radius: f32,
    pub angular_speed: f32,
}

impl MovementStrategy for OrbitMovement {
    fn move_enemy(&self, x: &mut f32, y: &mut f32, target: (f32, f32), time: f32, index: usize) {
        let angle = time * self.angular_speed + index as f32 * 0.1;
        let offset_x = angle.cos() * self.radius;
        let offset_y = angle.sin() * self.radius;
        
        *x = target.0 + offset_x;
        *y = target.1 + offset_y;
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