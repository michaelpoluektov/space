use gdnative::prelude::*;
use rand::*;
use gdnative::api::Node2D;

use crate::utils::instance_scene;

#[derive(NativeClass)]
#[inherit(Node)]
#[user_data(user_data::LocalCellData<WorldGenerator>)]
pub struct WorldGenerator {
    #[property]
    enemies_scene: Ref<PackedScene, Shared>,
    level: u32,
    current_scene: Option<Ref<Node2D, Unique>>
}

#[methods]
impl WorldGenerator {
    fn new(_owner: &Node) -> Self {
        WorldGenerator {
            enemies_scene: PackedScene::new().into_shared(),
            level: 0,
            current_scene: None
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node) { 
        godot_print!("start gen");
        self.start_generation(owner);
        self.level += 1;
        godot_print!("gen done");
    }

    #[export]
    fn start_generation(&mut self, owner: &Node) {
        let mut rng = rand::thread_rng();
        let enemies_scene: Ref<Node2D, _> = instance_scene(&self.enemies_scene);
        let scene_id = rng.gen_range(0, enemies_scene.get_child_count());
        godot_print!("select {}", scene_id);
        if let Some(scene) = enemies_scene.get_child(scene_id).and_then(|s| unsafe {s.assume_safe() }.duplicate(15)) {
            if let Ok(scene) = unsafe { scene.assume_unique() }.try_cast::<Node2D>() {
                scene.set_position(Vector2::new(0f32, 0f32));
                owner.add_child(scene, true);
            }
        } else {
            godot_error!("Cant get scene {}", scene_id);
        }
    }
}
