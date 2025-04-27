use super::{MovementStrategy, EnemyData};
use macroquad::prelude::*;

pub struct BoidsMovement {
    pub visual_range: f32,
    pub separation_dist: f32,
    pub max_speed: f32,
    pub player_weight: f32,
    pub player_distance: f32,
    pub noise_strength: f32
}

impl MovementStrategy for BoidsMovement {
    fn move_enemy(
        &self,
        x: &mut f32,
        y: &mut f32,
        target: (f32, f32),
        time: f32,
        index: usize,
        all_positions: &[(f32, f32)],
    ) {
        let visual_range = self.visual_range;
        let separation_dist = self.separation_dist;
        let player_distance = self.player_distance;
        let player_weight = self.player_weight;
        let max_speed = self.max_speed;
        let noise_strength = self.noise_strength;

        let mut sep = Vec2::ZERO;
        let mut ali = Vec2::ZERO;
        let mut coh = Vec2::ZERO;
        let mut neighbors = 0;

        let pos_i = Vec2::new(*x, *y);

        for (j, &(other_x, other_y)) in all_positions.iter().enumerate() {
            if j == index { continue; }

            let pos_j = Vec2::new(other_x, other_y);
            let dist = pos_i.distance(pos_j);

            if dist < visual_range {
                if dist < separation_dist {
                    sep += (pos_i - pos_j) / dist;
                }
                ali += pos_j;
                coh += pos_j;
                neighbors += 1;
            }
        }

        let mut velocity = Vec2::ZERO;

        if neighbors > 0 {
            let n = neighbors as f32;
            velocity += (ali / n - pos_i).normalize_or_zero();
            velocity += (coh / n - pos_i).normalize_or_zero();
            velocity += sep.normalize_or_zero();
            velocity *= max_speed;
        }

        let to_player = Vec2::new(target.0, target.1) - pos_i;
        if to_player.length() < player_distance {
            velocity += to_player.normalize_or_zero() * player_weight;
        }

        let noise = Vec2::new(
            rand::gen_range(-1.0, 1.0),
            rand::gen_range(-1.0, 1.0),
        ) * noise_strength;

        velocity += noise;

        *x += velocity.x;
        *y += velocity.y;
    }
}