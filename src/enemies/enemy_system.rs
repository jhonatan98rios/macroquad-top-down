use macroquad::prelude::*;
use crate::strategies::MovementStrategy;
use crate::constants::{WORLD_WIDTH, WORLD_HEIGHT};
use std::cmp;

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum EnemyStatus {
    Pending,
    Live,
    Dead,
}

#[derive(Clone, Copy)]
pub struct EnemyData {
    pub status: EnemyStatus,
    pub last_movement: Vec2, // Track movement direction for flipping
}

pub struct EnemySystem {
    positions: Vec<Vec2>,
    sizes: Vec<Vec2>,
    data: Vec<EnemyData>,
    strategy: Box<dyn MovementStrategy>,
    time: f32,
    chunk_index: usize,
    max_number_of_chunks: usize,
    texture: Option<Texture2D>, // Single texture for all enemies
}

impl EnemySystem {
    pub async  fn new(count: usize, strategy: Box<dyn MovementStrategy>) -> Self {
        
        let texture = match load_texture("assets/enemy.png").await {
            Ok(t) => Some(t),
            Err(_) => {
                println!("Failed to load enemy texture, falling back to rectangles");
                None
            }
        };

        let positions = (0..count)
            .map(|_| vec2(
                rand::gen_range(0.0, WORLD_WIDTH),
                rand::gen_range(0.0, WORLD_HEIGHT),
            ))
            .collect();
            
        let sizes = vec![vec2(64.0, 64.0); count];
        let data = vec![EnemyData { 
            status: EnemyStatus::Pending,
            last_movement: Vec2::new(1.0, 0.0) 
        }; count];

        EnemySystem {
            positions,
            sizes,
            data,
            strategy,
            time: 0.0,
            chunk_index: 0,
            max_number_of_chunks: 4, // Number of chunks to divide the enemies into for processing (bigger is faster)
            texture,
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

                let prev_pos = self.positions[i];

                strategy.move_enemy(
                    &mut self.positions[i],
                    target_pos,
                    current_time,
                    i,
                    &all_positions,
                );

                // Update movement direction if position changed
                let movement = self.positions[i] - prev_pos;
                if movement.length_squared() > 0.0 {
                    self.data[i].last_movement = movement.normalize();
                }
            }
        }
    }
    
    pub fn draw(&self, target_pos: Vec2) {
        match &self.texture {
            Some(texture) => {

                let mut indices: Vec<usize> = (0..self.positions.len()).collect();
                indices.sort_by(|&a, &b| self.positions[b].y.partial_cmp(&self.positions[a].y).unwrap());
            

                for &i in &indices {
                    if self.data[i].status == EnemyStatus::Live {
                        
                        let flip_x = self.positions[i].x > target_pos.x;
                        
                        let params = DrawTextureParams {
                            dest_size: Some(self.sizes[i]),
                            flip_x,
                            source: Some(Rect {
                                x: 0.0,
                                y: self.texture.as_ref().unwrap().height(),
                                w: self.texture.as_ref().unwrap().width(),
                                h: -self.texture.as_ref().unwrap().height(), // <- h negativo inverte o Y
                            }),
                            ..Default::default()
                        };
                        draw_texture_ex(
                            texture,
                            self.positions[i].x,
                            self.positions[i].y,
                            WHITE,
                            params
                        );
                    }
                }
            }
            None => {
                // Fallback to rectangles if no texture
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
    }
}