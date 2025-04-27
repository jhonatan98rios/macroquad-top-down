
use macroquad::prelude::*;
use crate::strategies::MovementStrategy;

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyStatus {
    Pending = 0,
    Live = 1,
    #[allow(dead_code)]
    Dead = 2,
}

pub struct EnemySystem {
    count: usize,
    pos_x: Vec<f32>,
    pos_y: Vec<f32>,
    width: Vec<f32>,
    height: Vec<f32>,
    status: Vec<EnemyStatus>,
    strategy: Box<dyn MovementStrategy>,
    time: f32,
}

impl EnemySystem {
    pub fn new(count: usize, strategy: Box<dyn MovementStrategy>) -> Self {
        let pos_x = (0..count).map(|_| rand::gen_range(0.0, screen_width())).collect();
        let pos_y = (0..count).map(|_| rand::gen_range(0.0, screen_height())).collect();
        
        EnemySystem {
            count,
            pos_x,
            pos_y,
            width: vec![10.0; count],
            height: vec![10.0; count],
            status: vec![EnemyStatus::Pending; count],
            strategy,
            time: 0.0,
        }
    }
    
    pub fn spawn_all(&mut self) {
        for status in &mut self.status {
            *status = EnemyStatus::Live;
        }
    }
    
    pub fn update(&mut self, target_pos: (f32, f32)) {
        self.time += get_frame_time();

        // We dont have enemies, only pos_x: Vec<f32>, pos_y: Vec<f32>, width: Vec<f32>, height: Vec<f32>,
        let positions: Vec<(f32, f32)> = self.pos_x.iter().zip(self.pos_y.iter())
            .map(|(&x, &y)| (x, y)) // Collect positions as tuples
            .collect();
        
        for i in 0..self.count {
            if self.status[i] == EnemyStatus::Live {
                self.strategy.move_enemy(
                    &mut self.pos_x[i],
                    &mut self.pos_y[i],
                    target_pos,
                    self.time,
                    i,
                    &positions,
                );
            }
        }
    }
    
    pub fn draw(&self) {
        for i in 0..self.count {
            if self.status[i] == EnemyStatus::Live {
                draw_rectangle(
                    self.pos_x[i],
                    self.pos_y[i],
                    self.width[i],
                    self.height[i],
                    RED
                );
            }
        }
    }
}