use macroquad::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    speed: f32,
    size: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            speed: 5.0,
            size: 10.0,
        }
    }

    pub fn update(&mut self) {
        let mut move_x = 0.0;
        let mut move_y = 0.0;

        // Handle both WASD and Arrow keys
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) { move_x += 1.0; }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) { move_x -= 1.0; }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) { move_y += 1.0; }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) { move_y -= 1.0; }

        // Normalize diagonal movement
        if move_x != 0.0 && move_y != 0.0 {
            let inv_sqrt_2 = 1.0 / std::f32::consts::SQRT_2;
            move_x *= inv_sqrt_2;
            move_y *= inv_sqrt_2;
        }

        self.x += move_x * self.speed;
        self.y += move_y * self.speed;

        // Keep player on screen
        self.x = self.x.clamp(0.0, screen_width() - self.size);
        self.y = self.y.clamp(0.0, screen_height() - self.size);
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.size, self.size, GREEN);
    }

    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}