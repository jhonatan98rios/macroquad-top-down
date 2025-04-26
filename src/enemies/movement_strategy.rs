use macroquad::prelude::*;

pub trait MovementStrategy: Send + Sync {
    fn move_enemy(&self, x: &mut f32, y: &mut f32, target: (f32, f32), time: f32, index: usize);

    #[allow(dead_code)]
    fn batch_update(&self, enemies: &mut [EnemyData], target: (f32, f32), time: f32);
}

pub struct EnemyData {
    pub x: f32,
    pub y: f32,
    pub vel_x: f32,
    pub vel_y: f32,

    #[allow(dead_code)]
    pub max_speed: f32,
}


// Direct movement toward player
pub struct DirectMovement {
    pub speed: f32,
}

impl MovementStrategy for DirectMovement {
    fn move_enemy(&self, x: &mut f32, y: &mut f32, target: (f32, f32), _time: f32, _index: usize) {
        let dx = target.0 - *x;
        let dy = target.1 - *y;
        let dist = (dx * dx + dy * dy).sqrt().max(0.0001);
        
        *x += (dx / dist) * self.speed;
        *y += (dy / dist) * self.speed;
    }

    #[allow(unused_variables)]
    fn batch_update(&self, enemies: &mut [EnemyData], target: (f32, f32), time: f32) {}
}

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

pub struct BoidsMovement {
    pub max_speed: f32,
    pub max_force: f32,
    pub separation_distance: f32,
    pub alignment_distance: f32,
    pub cohesion_distance: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub target_weight: f32,
}

impl MovementStrategy for BoidsMovement {
    fn move_enemy(&self, x: &mut f32, y: &mut f32, _target: (f32, f32), time: f32, _index: usize) {
        // Individual movement fallback
        *x += time.sin() * 0.1;
        *y += time.cos() * 0.1;
    }

    fn batch_update(&self, enemies: &mut [EnemyData], target: (f32, f32), _time: f32) {
        let target_vec = Vec2::new(target.0, target.1);
        
        for i in 0..enemies.len() {
            let position = Vec2::new(enemies[i].x, enemies[i].y);
            let mut velocity = Vec2::new(enemies[i].vel_x, enemies[i].vel_y);
            
            // Initialize forces
            let mut separation = Vec2::ZERO;
            let mut alignment = Vec2::ZERO;
            let mut cohesion = Vec2::ZERO;
            let mut neighbors = 0;

            // Check nearby boids
            for j in 0..enemies.len() {
                if i == j { continue; }

                let other_pos = Vec2::new(enemies[j].x, enemies[j].y);
                let distance = position.distance(other_pos);

                // Separation (avoid crowding)
                if distance < self.separation_distance {
                    let diff = position - other_pos;
                    separation += diff.normalize_or_zero() / distance.max(0.1);
                }

                // Alignment (match velocity)
                if distance < self.alignment_distance {
                    alignment += Vec2::new(enemies[j].vel_x, enemies[j].vel_y);
                    neighbors += 1;
                }

                // Cohesion (stay together)
                if distance < self.cohesion_distance {
                    cohesion += other_pos;
                }
            }

            // Calculate average alignment
            if neighbors > 0 {
                alignment = (alignment / neighbors as f32).normalize_or_zero() * self.max_speed;
                alignment -= velocity;
                alignment = alignment.clamp_length_max(self.max_force);
            }

            // Calculate cohesion force
            if neighbors > 0 {
                cohesion = (cohesion / neighbors as f32) - position;
                cohesion = cohesion.normalize_or_zero() * self.max_speed;
                cohesion -= velocity;
                cohesion = cohesion.clamp_length_max(self.max_force);
            }

            // Target seeking force
            let mut seek_force = target_vec - position;
            seek_force = seek_force.normalize_or_zero() * self.max_speed;
            seek_force -= velocity;
            seek_force = seek_force.clamp_length_max(self.max_force);

            // Apply all forces with weights
            let mut acceleration = Vec2::ZERO;
            acceleration += separation * self.separation_weight;
            acceleration += alignment * self.alignment_weight;
            acceleration += cohesion * self.cohesion_weight;
            acceleration += seek_force * self.target_weight;

            // Update velocity and position
            velocity += acceleration;
            velocity = velocity.clamp_length_max(self.max_speed);
            
            enemies[i].x += velocity.x;
            enemies[i].y += velocity.y;
            enemies[i].vel_x = velocity.x;
            enemies[i].vel_y = velocity.y;
        }
    }
}