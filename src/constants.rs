use once_cell::sync::Lazy;
use macroquad::prelude::*;
use crate::components::layout::{is_mobile};

pub const WORLD_WIDTH: f32 = 4000.0;
pub const WORLD_HEIGHT: f32 = 4000.0;


pub static VIRTUAL_HEIGHT: Lazy<f32> = Lazy::new(|| {
    if is_mobile() { 854.0 } else { 1080.0 }
});

pub static VIRTUAL_WIDTH: Lazy<f32> = Lazy::new(|| {
    if is_mobile() { 480.0 } else { 1920.0 }
});

pub fn virtual_height() -> f32 {
    *VIRTUAL_HEIGHT
}

pub fn virtual_width() -> f32 {
    *VIRTUAL_WIDTH
}



// 1280 == 720
// 854 == 480