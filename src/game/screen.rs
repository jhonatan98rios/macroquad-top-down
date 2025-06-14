use macroquad::prelude::*;

use crate::player::Player;
use crate::enemies::{EnemySystem, PositionOverlap};
use crate::constants::{WORLD_WIDTH, WORLD_HEIGHT, virtual_height, virtual_width};
use crate::components::joystick::Joystick;
use crate::components::layout::{is_mobile};
use crate::strategies::{BoidsMovement, AABBCollision};

pub struct Game {
    player: Player,
    enemies: EnemySystem,
    camera: Camera2D,
    pub joystick: Option<Joystick>,
}

impl Game {
    pub async fn new(joystick: Option<Joystick>) -> Self {

        let camera = Camera2D {
            zoom: vec2(2.0 / virtual_width(), -2.0 / virtual_height()),
            target: vec2(virtual_width() / 2.0, virtual_height() / 2.0),
            ..Default::default()
        };

        let movement_strategy = Box::new(BoidsMovement {
            visual_range: 32.0,
            separation_dist: 40.0,
            max_speed: 3.0,
            player_weight: 0.8,
            player_distance: 2000.0, 
            noise_strength: 0.05,
            separation_weight: 3.2,
            alignment_weight: 1.5,
            cohesion_weight: 0.3,
        });

        let collision_strategy = Box::new(AABBCollision {});

        let enemies = EnemySystem::new(
            100, 
            movement_strategy, 
            collision_strategy,
        ).await;

        let player = Player::new(100.0, 100.0).await;

        Game {
            player,
            enemies,
            camera,
            joystick
        }
    }

    pub async fn init(&mut self) {
        self.enemies.spawn_all();
    }

    pub fn update(&mut self) {
        clear_background(BLACK);

        self.camera.zoom = calculate_camera_zoom();
        self.camera.target = clamp_camera_target(self.player.position());
        set_camera(&self.camera);

        draw_rectangle(0.0, 0.0, WORLD_WIDTH, WORLD_HEIGHT, Color::from_rgba(30, 30, 30, 255));

        let joystick_dir = self.joystick.as_mut().map(|joy| {
            joy.update();
            joy.direction()
        });

        self.player.update_with_direction(joystick_dir);
        self.player.update();

        self.enemies.update(self.player.position(), &mut self.player);

        self.enemies.draw(self.player.position(), PositionOverlap::Behind);
        self.player.draw();
        self.enemies.draw(self.player.position(),PositionOverlap::InFront);

        set_default_camera();

        if let Some(joystick) = &self.joystick {
            joystick.draw();
        }

        draw_text(
            &format!("WASD or Arrows to move | FPS: {} | enemies {}", get_fps(), self.enemies.positions.len()),
            20.0,
            30.0,
            30.0,
            WHITE,
        );
    }

    pub fn is_game_over(&self) -> bool {
        self.player.health <= 0.0
    }
}

#[inline]
fn clamp_camera_target(player_position: Vec2) -> Vec2 {
    let half_screen_width = virtual_width() / 2.0;
    let half_screen_height = virtual_height() / 2.0;

    Vec2::new(
        player_position.x.clamp(half_screen_width, WORLD_WIDTH - half_screen_width),
        player_position.y.clamp(half_screen_height, WORLD_HEIGHT - half_screen_height),
    )
}

#[inline]
fn calculate_camera_zoom() -> Vec2 {
    if is_mobile() { 
        return vec2(2.0 / virtual_width(), -2.0 / virtual_height())
    }
    return vec2(
        2.0 / virtual_width() * (virtual_width() / screen_width()),
        -2.0 / virtual_height() * (virtual_height() / screen_height())
    )
}