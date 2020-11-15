use gdnative::prelude::*;

pub fn instance_scene<Root>(scene: &Ref<PackedScene, Shared>) -> Ref<Root, Unique>
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