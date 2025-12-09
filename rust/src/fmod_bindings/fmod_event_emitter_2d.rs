use godot::classes::Node2D;
use godot::prelude::*;

pub struct FmodEventEmitter2D {
    node: Gd<Node2D>,
}

impl FmodEventEmitter2D {
    pub fn play(&mut self) {
        self.node.call("play", &[]);
    }
}
