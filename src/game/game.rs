use macroquad::prelude::*;

use crate::player::Player;
use crate::enemies::{EnemySystem, PositionOverlap};
use crate::constants::{WORLD_WIDTH, WORLD_HEIGHT, virtual_height, virtual_width};
use crate::components::joystick::Joystick;
use crate::components::layout::{is_mobile};
use crate::strategies::{BoidsMovement, AABBCollision};
use crate::state::GameState;

use crate::skills::skills_system::SkillsSystem;
use crate::skills::skills_factory::SkillsFactory;

use crate::experience::experience_system::ExperienceSystem;

pub struct Game {
    player: Player,
    enemies: EnemySystem,
    camera: Camera2D,
    skills_system: SkillsSystem,
    experience_system: ExperienceSystem,
    pub joystick: Option<Joystick>,
    joystick_dir: Option<Vec2>,
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
            50, 
            movement_strategy, 
            collision_strategy,
        ).await;

        let player = Player::new(
            WORLD_WIDTH / 2.0,
            WORLD_HEIGHT / 2.0,
        ).await;

        let mut skills_system = SkillsSystem::new();

        skills_system.add_skill(Box::new(
            SkillsFactory::create_simple_projectile_manager()
        ));

        let experience_system = ExperienceSystem::new();

        Game {
            player,
            enemies,
            camera,
            skills_system,
            experience_system,
            joystick,
            joystick_dir: None,
        }
    }

    pub async fn init(&mut self) {
        self.enemies.spawn_all();
    }

    pub fn update(&mut self) -> Option<GameState> {

        self.joystick_dir = self.joystick.as_mut().map(|joy| {
            joy.update();
            joy.direction()
        });

        self.player.update_with_direction(self.joystick_dir); //TODO: Refactore these two methods
        self.player.update();

        self.enemies.update(self.player.position(), &mut self.player);
        

        let enemy_views = self.enemies.to_views();

        self.skills_system.spawn(&self.player, &enemy_views);
        self.skills_system.update(get_frame_time(), &enemy_views, &mut |_, damage, enemy_index| {
            self.enemies.take_damage(enemy_index, damage, &mut |positions, value| {
                self.experience_system.spawn_experience_orb(positions, value);
            });
        });

        if let Some(next_state) = self.experience_system.update(&mut self.player) {
            return Some(next_state);
        }

        if self.is_game_over() {
            return Some(GameState::GameOver);
        }

        if is_key_pressed(KeyCode::Escape) || (is_mobile() && is_key_pressed(KeyCode::Back)) {
            return Some(GameState::Paused);
        }

        return None
    }

    pub fn draw_scene(&mut self) {
        clear_background(BLACK);

        self.camera.zoom = calculate_camera_zoom();
        self.camera.target = clamp_camera_target(self.player.position());
        set_camera(&self.camera);
        draw_rectangle(0.0, 0.0, WORLD_WIDTH, WORLD_HEIGHT, Color::from_rgba(30, 30, 30, 255));

        self.enemies.draw(self.player.position(), PositionOverlap::Behind);
        self.player.draw();
        self.enemies.draw(self.player.position(),PositionOverlap::InFront);
        self.experience_system.draw();
        self.skills_system.draw();
    }

    pub fn draw_hub(&mut self) {
        set_default_camera();

        if let Some(joystick) = &self.joystick {
            joystick.draw();
        }

        draw_text(
            &format!("WASD or Arrows to move | FPS: {} | enemies {}", get_fps(), self.enemies.positions.len()),
            if is_mobile() { 40.0 } else { 20.0 },
            30.0,
            30.0, 
            WHITE,
        );

        draw_text(
            &format!("Level {} | Experience {}/{} ", self.player.level, self.player.current_experience, self.player.experience_to_next_level),
            if is_mobile() { 40.0 } else { 20.0 },
            60.0,
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