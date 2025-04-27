use macroquad::prelude::*;
use crate::strategies::MovementStrategy;

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyStatus {
    Pending,
    Live,
    Dead,
}

#[derive(Clone, Copy)]
pub struct EnemyData {
    pub status: EnemyStatus
}

pub struct EnemySystem {
    positions: Vec<Vec2>,
    sizes: Vec<Vec2>,
    data: Vec<EnemyData>,
    strategy: Box<dyn MovementStrategy>,
    time: f32,
}

impl EnemySystem {
    pub fn new(count: usize, strategy: Box<dyn MovementStrategy>) -> Self {
        let positions = (0..count)
            .map(|_| vec2(
                rand::gen_range(0.0, screen_width()),
                rand::gen_range(0.0, screen_height())
            ))
            .collect();
            
        let sizes = vec![vec2(10.0, 10.0); count];
        let data = vec![EnemyData { status: EnemyStatus::Pending }; count];

        EnemySystem {
            positions,
            sizes,
            data,
            strategy,
            time: 0.0,
        }
    }
    
    pub fn spawn_all(&mut self) {
        for data in &mut self.data {
            data.status = EnemyStatus::Live;
        }
    }
    
    pub fn update(&mut self, target_pos: Vec2) {
        self.time += get_frame_time();
        
        // Store length and time locally to avoid borrowing issues
        let count = self.positions.len();
        let current_time = self.time;
        let strategy = &self.strategy;
        
        // Create a temporary Vec of positions for the strategy
        let all_positions = self.positions.clone();
        
        for i in 0..count {
            if self.data[i].status == EnemyStatus::Live {
                strategy.move_enemy(
                    &mut self.positions[i],
                    target_pos,
                    current_time,
                    i,
                    &all_positions,
                );
            }
        }
    }
    
    pub fn draw(&self) {
        for i in 0..self.positions.len() {
            if self.data[i].status == EnemyStatus::Live {
                draw_rectangle(
                    self.positions[i].x,
                    self.positions[i].y,
                    self.sizes[i].x,
                    self.sizes[i].y,
                    RED
                );
            }
        }
    }
}