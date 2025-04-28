use macroquad::prelude::*;
use crate::constants::{ WORLD_HEIGHT, WORLD_WIDTH };

pub struct Player {
    pub x: f32,
    pub y: f32,
    speed: f32,
    size: f32,
    texture: Option<Texture2D>,
    last_movement: Vec2,
    current_frame: usize,
    frame_timer: f32,
    frame_duration: f32,
}

impl Player {
    pub async fn new(x: f32, y: f32) -> Self {
        let texture = match load_texture("assets/player_spritesheet.png").await {
            Ok(t) => Some(t),
            Err(_) => {
                println!("Failed to load player texture, falling back to rectangle");
                None
            }
        };

        Player {
            x,
            y,
            speed: 5.0,
            size: 64.0,
            texture,
            last_movement: Vec2::ZERO,
            current_frame: 0,
            frame_timer: 0.0,
            frame_duration: 0.15, // Cada frame dura 150ms
        }
    }

    pub fn update(&mut self) {
        let mut move_dir = Vec2::ZERO;

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) { move_dir.x += 1.0; }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) { move_dir.x -= 1.0; }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) { move_dir.y += 1.0; }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) { move_dir.y -= 1.0; }

        if move_dir.length_squared() > 0.0 {
            move_dir = move_dir.normalize();
            self.last_movement = move_dir;
        }

        self.x += move_dir.x * self.speed;
        self.y -= move_dir.y * self.speed;

        self.x = self.x.clamp(0.0, WORLD_WIDTH - self.size);
        self.y = self.y.clamp(0.0, WORLD_HEIGHT - self.size);

        // Atualiza animação
        self.frame_timer += get_frame_time();
        if self.frame_timer >= self.frame_duration {
            self.frame_timer = 0.0;
            self.current_frame = (self.current_frame + 1) % 4; // Temos 4 frames (0, 1, 2, 3)
        }
    }

    pub fn draw(&self) {
        match &self.texture {
            Some(texture) => {
                let flip_x = self.last_movement.x < 0.0;
                let frame_width = self.size;
                let frame_height = self.size;

                let params = DrawTextureParams {
                    dest_size: Some(Vec2::new(self.size, self.size)),
                    flip_x,
                    source: Some(Rect {
                        x: self.current_frame as f32 * frame_width,
                        y: texture.height(),  // Começando do topo da imagem
                        w: frame_width,
                        h: -frame_height, // h negativo para inverter
                    }),
                    ..Default::default()
                };
                draw_texture_ex(texture, self.x, self.y, WHITE, params);
            }
            None => {
                draw_rectangle(self.x, self.y, self.size, self.size, BLUE);
            }
        }
    }

    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}
