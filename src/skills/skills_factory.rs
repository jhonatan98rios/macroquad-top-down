use crate::skills::simple_projectile::SimpleProjectileManager;
use std::collections::HashMap;

pub struct SkillsFactory {}

impl SkillsFactory {

    pub fn create_simple_projectile_manager() -> SimpleProjectileManager {
        
        return SimpleProjectileManager {
            damage: 1.0,
            speed: 300.0,
            radius: 5.0,
            life: 2.0,
            cooldown: 0.5,
            is_active: true,
            timer: 0.0,
            projectiles: HashMap::new(),
        };
    }
}