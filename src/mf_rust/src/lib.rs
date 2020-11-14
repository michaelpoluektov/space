mod world_generator;
mod entity;
mod obstacle;

use gdnative::prelude::*;

use crate::world_generator::WorldGenerator;
use crate::entity::Entity;

#[derive(NativeClass)]
#[inherit(Node)]
struct HelloWorld {
    #[property(path= "i")]
    i: i32
}

#[gdnative::methods]
impl HelloWorld {
    fn new(_owner: &Node) -> Self {
        HelloWorld { i: 0i32 }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("hello, world. {}", self.i)
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
    handle.add_class::<WorldGenerator>();
    handle.add_class::<Entity>();
}

godot_init!(init);