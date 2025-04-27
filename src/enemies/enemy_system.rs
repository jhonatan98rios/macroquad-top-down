use macroquad::prelude::*;
use crate::strategies::MovementStrategy;

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyStatus {
    Pending,
    Live,
    Dead,
}

pub struct Enemy {
    pub position: Vec2,
    pub size: Vec2,
    pub status: EnemyStatus,
}

pub struct EnemySystem {
    enemies: Vec<Enemy>,
    strategy: Box<dyn MovementStrategy>,
    time: f32,
}

impl EnemySystem {
    pub fn new(count: usize, strategy: Box<dyn MovementStrategy>) -> Self {
        let enemies = (0..count)
            .map(|_| Enemy {
                position: vec2(
                    rand::gen_range(0.0, screen_width()),
                    rand::gen_range(0.0, screen_height()),
                ),
                size: vec2(10.0, 10.0),
                status: EnemyStatus::Pending,
            })
            .collect();

        EnemySystem {
            enemies,
            strategy,
            time: 0.0,
        }
    }
    
    pub fn spawn_all(&mut self) {
        for enemy in &mut self.enemies {
            enemy.status = EnemyStatus::Live;
        }
    }
    
    pub fn update(&mut self, target_pos: Vec2) {
        self.time += get_frame_time();
        
        let live_enemies: Vec<&Enemy> = self.enemies.iter()
            .filter(|e| e.status == EnemyStatus::Live)
            .collect();
            
        let positions: Vec<Vec2> = live_enemies.iter()
            .map(|e| e.position)
            .collect();

        for (i, enemy) in self.enemies.iter_mut().enumerate() {
            if enemy.status == EnemyStatus::Live {
                self.strategy.move_enemy(
                    &mut enemy.position,
                    target_pos,
                    self.time,
                    i,
                    &positions,
                );
            }
        }
    }
    
    pub fn draw(&self) {
        for enemy in &self.enemies {
            if enemy.status == EnemyStatus::Live {
                draw_rectangle(
                    enemy.position.x,
                    enemy.position.y,
                    enemy.size.x,
                    enemy.size.y,
                    RED
                );
            }
        }
    }
}