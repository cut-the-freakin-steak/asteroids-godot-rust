use godot::classes::{
    Area2D, CollisionPolygon2D, GpuParticles2D, IArea2D, Marker2D, Node, Sprite2D, Timer,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Asteroid {
    // base
    base: Base<Area2D>,

    // onready
    main: OnReady<Node>,

    sprite: OnReady<Sprite2D>,
    collision: OnReady<CollisionPolygon2D>,
    explosion_parts: OnReady<GpuParticles2D>,
    explosion_to_queue_free: OnReady<Timer>,

    // normal vars
    direction: Vector2,
    vertical_speed: f64,
    horizontal_speed: f64,
    use_set_position: bool,
}

#[godot_api]
impl IArea2D for Asteroid {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,

            main: OnReady::manual(),

            sprite: OnReady::manual(),
            collision: OnReady::manual(),
            explosion_parts: OnReady::manual(),
            explosion_to_queue_free: OnReady::manual(),

            direction: Vector2 { x: 0.0, y: 0.0 },
            vertical_speed: 0.0,
            horizontal_speed: 0.0,
            use_set_position: false,
        }
    }

    fn ready(&mut self) {
        // onready node declarations
        self.manual.init(self.base.get_node_as())
    }

    fn physics_process(&mut self, delta: f64) {}
}
