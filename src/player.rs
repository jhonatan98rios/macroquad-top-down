use macroquad::prelude::*;
use crate::constants::{WORLD_HEIGHT, WORLD_WIDTH};

#[derive(PartialEq, Clone, Copy)]
enum PlayerState {
    Idle,
    Walking,
}

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
    state: PlayerState,
    facing_right: bool,
}

impl Player {
    pub async fn new(x: f32, y: f32) -> Self {
        let texture = match load_texture("images/player_spritesheet.png").await {
            Ok(t) => Some(t),
            Err(_) => {
                println!("Failed to load player texture, falling back to rectangle");
                None
            }
        };

        Player {
            x,
            y,
            speed: 4.0,
            size: 64.0,
            texture,
            last_movement: Vec2::ZERO,
            current_frame: 0,
            frame_timer: 0.0,
            frame_duration: 0.30,
            state: PlayerState::Idle,
            facing_right: true,
        }
    }

    pub fn update(&mut self) {
        let mut move_dir = Vec2::ZERO;

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            move_dir.x += 1.0;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            move_dir.y += 1.0;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            move_dir.y -= 1.0;
        }

        self.move_by_direction(move_dir);
    }

    pub fn draw(&self) {
        match &self.texture {
            Some(texture) => {
                let frame_width = self.size;
                let frame_height = self.size;

                let row = match (self.state, self.facing_right) {
                    (PlayerState::Idle, false) => 0,
                    (PlayerState::Walking, false) => 1,
                    (PlayerState::Idle, true) => 2,
                    (PlayerState::Walking, true) => 3,
                };

                let params = DrawTextureParams {
                    dest_size: Some(Vec2::new(self.size, self.size)),
                    flip_x: false,
                    flip_y: true,
                    source: Some(Rect {
                        x: self.current_frame as f32 * frame_width,
                        y: row as f32 * frame_height,
                        w: frame_width,
                        h: frame_height,
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

    pub fn move_by_direction(&mut self, direction: Vec2) {
        let mut move_dir = direction;
    
        if move_dir.length_squared() > 0.0 {
            move_dir = move_dir.normalize();
            self.last_movement = move_dir;
            self.facing_right = move_dir.x >= 0.0;
            self.state = PlayerState::Walking;
    
            self.x += move_dir.x * self.speed;
            self.y -= move_dir.y * self.speed;
    
            self.x = self.x.clamp(0.0, WORLD_WIDTH - self.size);
            self.y = self.y.clamp(0.0, WORLD_HEIGHT - self.size);
        } else {
            self.state = PlayerState::Idle;
        }
    
        // Atualiza animação
        self.frame_timer += get_frame_time();
        if self.frame_timer >= self.frame_duration {
            self.frame_timer = 0.0;
            self.current_frame = (self.current_frame + 1) % 4;
        }
    }

    pub fn update_with_direction(&mut self, joystick_dir: Option<Vec2>) {
        if let Some(dir) = joystick_dir {
            self.move_by_direction(dir);
        } else {
            self.update(); // fallback para teclado
        }
    }
}
