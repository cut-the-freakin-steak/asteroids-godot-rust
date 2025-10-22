use std::f64;

use godot::classes::{ISprite2D, Sprite2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // prints to godot console

        Self {
            speed: 400.0,
            angular_speed: f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // in gdscript this would be:
        // rotation += angular_speed * delta
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
        // the 'rotate' method requires an f32
        // therefore we convert 'self.angular_speed * delta', which is an f64 by default
    }
}
