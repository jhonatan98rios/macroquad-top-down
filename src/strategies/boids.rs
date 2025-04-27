use super::{MovementStrategy, EnemyData};
use macroquad::prelude::*;

pub struct BoidsMovement {
    pub visual_range: f32,
    pub separation_dist: f32,
    pub max_speed: f32,
    pub player_weight: f32,
    pub player_distance: f32,
    pub noise_strength: f32,
}

impl MovementStrategy for BoidsMovement {
    fn move_enemy(&self, x: &mut f32, y: &mut f32, _target: (f32, f32), time: f32, _index: usize) {
        // Individual movement fallback
        *x += time.sin() * 0.1;
        *y += time.cos() * 0.1;
    }

    fn batch_update(&self, enemies: &mut [EnemyData], target: (f32, f32), _time: f32) {
        let target_pos = Vec2::new(target.0, target.1);
        let screen_size = Vec2::new(screen_width(), screen_height());

        for i in 0..enemies.len() {
            let position = Vec2::new(enemies[i].x, enemies[i].y);
            let mut velocity = Vec2::new(enemies[i].vel_x, enemies[i].vel_y);

            // Initialize forces
            let mut separation_force = Vec2::ZERO;
            let mut alignment_force = Vec2::ZERO;
            let mut cohesion_force = Vec2::ZERO;
            let mut neighbors = 0;

            // Calculate forces from nearby boids
            for j in 0..enemies.len() {
                if i == j { continue; }

                let other_pos = Vec2::new(enemies[j].x, enemies[j].y);
                let diff = other_pos - position;
                let distance = diff.length();

                if distance < self.visual_range {
                    neighbors += 1;

                    // Separation: Strong repulsion
                    if distance < self.separation_dist {
                        let strength = 2.0 * (1.0 - distance / self.separation_dist);
                        separation_force -= diff.normalize_or_zero() * strength;
                    }

                    // Alignment: Very weak
                    alignment_force += diff.normalize_or_zero() * 0.3;

                    // Cohesion: Only apply if few neighbors
                    if neighbors < 5 {
                        cohesion_force += other_pos;
                    }
                }
            }

            // Normalize forces
            if neighbors > 0 {
                alignment_force /= neighbors as f32;
                if neighbors < 5 {
                    cohesion_force = (cohesion_force / neighbors as f32) - position;
                } else {
                    cohesion_force = Vec2::ZERO;
                }
            }

            // Player attraction
            let to_player = target_pos - position;
            let player_dist = to_player.length();
            let player_dir = to_player.normalize_or_zero();

            let player_force = if player_dist > self.player_distance {
                // Strong chase when far
                player_dir * self.player_weight
            } else {
                // Orbit + outward push
                Vec2::new(-player_dir.y * 0.5 + player_dir.x * 0.2,
                          player_dir.x * 0.5 + player_dir.y * 0.2)
            };

            // Random noise using macroquad's rand
            let noise = Vec2::new(
                (rand::gen_range(-0.5, 0.5)) * self.noise_strength,
                (rand::gen_range(-0.5, 0.5)) * self.noise_strength
            );

            // Combine forces (prioritize separation and player)
            let mut steer = 
                separation_force * 2.5 +  // Strongest force
                player_force * 1.5 +      // Secondary priority
                alignment_force * 0.3 +   // Minimal alignment
                cohesion_force * 0.2 +     // Weakest force
                noise;                    // Chaos factor

            // Limit speed
            if steer.length() > self.max_speed {
                steer = steer.normalize_or_zero() * self.max_speed;
            }

            // Update velocity and position
            velocity = steer;
            enemies[i].x += velocity.x;
            enemies[i].y += velocity.y;
            enemies[i].vel_x = velocity.x;
            enemies[i].vel_y = velocity.y;

            // Soft bounds checking with macroquad's rand
            let margin = 10.0;
            if enemies[i].x < margin {
                enemies[i].x = margin + rand::gen_range(0.0, 5.0);
            }
            if enemies[i].x > screen_size.x - margin {
                enemies[i].x = screen_size.x - margin - rand::gen_range(0.0, 5.0);
            }
            if enemies[i].y < margin {
                enemies[i].y = margin + rand::gen_range(0.0, 5.0);
            }
            if enemies[i].y > screen_size.y - margin {
                enemies[i].y = screen_size.y - margin - rand::gen_range(0.0, 5.0);
            }
        }
    }
}