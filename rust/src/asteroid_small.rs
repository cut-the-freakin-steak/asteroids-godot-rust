use godot::classes::{Area2D, IArea2D};
use godot::global::randi_range;
use godot::prelude::*;

use crate::asteroid::{Asteroid, AsteroidIFunctions};

#[derive(GodotClass)]
#[class(init, base = Area2D)]
struct SmallAsteroid {
    base: Base<Area2D>,

    #[init(node = "Asteroid")]
    ast_base: OnReady<Gd<Asteroid>>,
}

#[godot_api]
impl IArea2D for SmallAsteroid {
    fn ready(&mut self) {
        {
            let mut ast_base = self.ast_base.bind_mut();
            ast_base.asteroid_ready(); // super

            ast_base.horizontal_speed = randi_range(40, 55) as f32 * ast_base.direction.x
        }
        let position = self.ast_base.get_position();
        self.ast_base.set_position(Vector2 { x: self.ast_base.get_position(), y:  });
    }

    fn physics_process(&mut self, delta: f64) {
        let mut ast_base = self.ast_base.bind_mut();
        ast_base.asteroid_physics_process(delta); // super
    }
}
