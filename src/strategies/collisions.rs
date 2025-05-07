use macroquad::prelude::*;
use crate::event_bus::{EventBus, EventType, EventPayload};
use crate::enemies::{EnemySystem, EnemyStatus, EnemyData};
use crate::player::Player;
use super::CollisionStrategy;
use std::rc::Rc;
use std::cell::RefCell;


pub struct AABBCollision;

impl CollisionStrategy for AABBCollision {
    fn check_collisions(
        &mut self,
        positions: &mut Vec<Vec2>,
        sizes: &Vec<Vec2>,
        data: &mut Vec<EnemyData>,
        player: &mut Player,
        event_bus: &Rc<RefCell<EventBus>>,
    ) {
        for i in 0..positions.len() {
            if data[i].status != EnemyStatus::Live {
                continue;
            }

            let enemy_pos = positions[i];
            let enemy_size = sizes[i];
            let damage = 1; // Replace with the damage[i]

            let overlap = enemy_pos.x < player.x + player.size &&
                          enemy_pos.x + enemy_size.x > player.x &&
                          enemy_pos.y < player.y + player.size &&
                          enemy_pos.y + enemy_size.y > player.y;

            if overlap {
                event_bus.borrow_mut().emit(
                    &EventType::Damage,
                    player, 
                    &EventPayload::Damage {
                        amount: damage
                    }
                );
            }
        }
    }
}
