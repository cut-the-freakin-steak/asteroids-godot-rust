use godot::classes::Node2D;
use godot::prelude::*;

#[allow(dead_code)]
pub struct FmodEventEmitter2D {
    node: Gd<Node2D>,
}

#[allow(dead_code)]
impl FmodEventEmitter2D {
    pub fn play(&mut self) {
        self.node.call("play", &[]);
    }
}
