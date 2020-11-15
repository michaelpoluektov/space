use gdnative::prelude::*;
use gdnative::api::Node2D;
use gdnative::api::RigidBody2D;
use uuid::Uuid;

use crate::entity::Entity;
use crate::utils::instance_scene;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct ProjectileEmitter {
    #[property]
    projectile: Ref<PackedScene, Shared>,
    #[property]
    emit_speed: f64,
    #[property]
    projectile_scene: NodePath,
    time_passed: f64,
    sender: Option<Uuid>
}

#[methods]
impl ProjectileEmitter {
    fn new(_owner: &Node2D) -> Self {
        ProjectileEmitter {
            projectile: PackedScene::new().into_shared(),
            emit_speed: 0f64,
            projectile_scene: NodePath::from_str(""),
            time_passed: 0f64,
            sender: None
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        owner.set_physics_process(true);

        if let Some(parent) = owner.get_parent() {
            if let Ok(parent) = unsafe { parent.assume_unique() }.try_cast::<Node2D>() {
                if let Ok(parent) = unsafe { parent.assume_unique() }.try_cast_instance::<Entity>() {
                    parent.map(|p, o| {
                        self.sender = Some(p.id);
                    })
                    .unwrap_or_else(|_| godot_print!("Unable to get access to the parent entity"));
                } else {
                    godot_error!("Parent script is not Entity");
                }
            } else {
                godot_error!("Parent type is not Node2D");
            }
        } else {
            godot_error!("Cannot find parent");
        }
    }

    #[export]
    fn _physics_process(&mut self, owner: &Node2D, delta: f64) {
        self.time_passed += delta;
        if self.time_passed > self.emit_speed {
            self.time_passed = 0f64;
            self.shoot(owner);
        }
    }

    #[export]
    fn shoot(&self, owner: &Node2D) {
        if let Some(sender) = self.sender {
            let projectile_scene: Ref<RigidBody2D, _> = instance_scene(&self.projectile);
            projectile_scene.set_rotation(owner.rotation());
            projectile_scene.set_global_position(owner.global_position());
            projectile_scene.set_angular_velocity(100f64);
            if let Some(root) = owner.get_tree().and_then(|t| unsafe {t.assume_safe() }.root()).and_then(|t| unsafe {t.assume_safe() }.get_child(0)) {
                unsafe { root.assume_safe() }.add_child(projectile_scene, false);
                //projectile_scene_copy.set_owner(unsafe { root.assume_safe() });
                godot_print!("spawning proj {}", unsafe { root.assume_safe() }.name());
            } else {
                godot_error!("cant get root");
            }
            
        } else {
            godot_error!("Sender id is not set");
        }
    }
}
