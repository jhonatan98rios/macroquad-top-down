use macroquad::prelude::*;
use crate::strategies::MovementStrategy;
use std::cmp;

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
    chunk_index: usize,
    max_number_of_chunks: usize,
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
            chunk_index: 0,
            max_number_of_chunks: 4, // Number of chunks to divide the enemies into for processing (bigger is faster)
        }
    }
    
    pub fn spawn_all(&mut self) {
        for data in &mut self.data {
            data.status = EnemyStatus::Live;
        }
    }
    
    pub fn update(&mut self, target_pos: Vec2) {
        self.time += get_frame_time();
        self.chunk_index = if self.chunk_index < self.max_number_of_chunks - 1 { self.chunk_index + 1 } else { 0 };
        
        let chunk_size = self.positions.len() / self.max_number_of_chunks;
        let start = self.chunk_index * chunk_size;
        let end = cmp::min(start + chunk_size, self.positions.len());
        
        let current_time = self.time;
        let strategy = &self.strategy;
        let all_positions = self.positions.clone();
        
        for i in start..end {
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