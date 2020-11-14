use gdnative::prelude::*;
use rand::*;
use gdnative::api::Node2D;

#[derive(NativeClass)]
#[inherit(Node)]
#[user_data(user_data::LocalCellData<WorldGenerator>)]
pub struct WorldGenerator {
    #[property]
    enemies_scene: Ref<PackedScene, Shared>,
    level: u32
}

#[methods]
impl WorldGenerator {
    fn new(_owner: &Node) -> Self {
        WorldGenerator {
            enemies_scene: PackedScene::new().into_shared(),
            level: 0
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
    fn start_generation(&self, owner: &Node) {
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

fn instance_scene<Root>(scene: &Ref<PackedScene, Shared>) -> Ref<Root, Unique>
where
    Root: gdnative::GodotObject<RefKind = ManuallyManaged> + SubClass<Node>,
{
    let scene = unsafe { scene.assume_safe() };

    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .expect("should be able to instance scene");

    let instance = unsafe { instance.assume_unique() };

    instance
        .try_cast::<Root>()
        .expect("root node type should be correct")
}