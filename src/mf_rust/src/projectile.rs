use gdnative::prelude::*;
use gdnative::api::RigidBody2D;
use uuid::Uuid;

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Projectile {
    #[property]
    pub damage: i64,
    #[property]
    pub group: i64,
    pub sender: Uuid,
}

#[methods]
impl Projectile {
    fn new(_owner: &RigidBody2D) -> Self {
        Projectile { damage: 0, group: 0, sender: Uuid::new_v4() }
    }

    #[export]
    fn _process(&mut self, owner: &RigidBody2D, _delta: f64) {
        let pos = owner.global_position();
        if pos.x < 0f32 || pos.y < 0f32 || pos.x > SCREEN_WIDTH || pos.y > SCREEN_HEIGHT {
            owner.queue_free();
        }
    }
}