use godot::classes::{
    Area2D, CollisionPolygon2D, GpuParticles2D, IArea2D, Marker2D, Node, Sprite2D, Timer,
};
use godot::global::{randi, randomize};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
struct Asteroid {
    // base
    base: Base<Area2D>,

    // onready
    #[init(val = OnReady::manual())]
    main: OnReady<Gd<Node>>,

    #[init(node = "Sprite2D")]
    sprite: OnReady<Gd<Sprite2D>>,

    #[init(node = "CollisionPolygon2D")]
    collision: OnReady<Gd<CollisionPolygon2D>>,

    #[init(node = "AsteroidExplosion")]
    explosion_parts: OnReady<Gd<GpuParticles2D>>,

    #[init(node = "ExplosionToQueueFree")]
    explosion_to_queue_free: OnReady<Gd<Timer>>,

    // normal vars
    #[init(val = Vector2::ZERO)]
    direction: Vector2,

    #[init(val = 0.0)]
    vertical_speed: f64,

    #[init(val = 0.0)]
    horizontal_speed: f64,

    #[init(val = false)]
    use_set_position: bool,
}

#[godot_api]
impl IArea2D for Asteroid {
    fn ready(&mut self) {
        // NOTE: getting the scene tree and then current scene is a bit more
        // complicated in rust than in gdscript lmao
        let main_scene;
        match self.base().get_tree() {
            Some(tree) => match tree.get_current_scene() {
                Some(main) => main_scene = main,
                None => panic!(),
            },
            None => panic!(),
        };

        self.main.init(main_scene);

        // rest of the ready function
        randomize();

        if !self.use_set_position {
            let asteroid_markers = self
                .main
                .get_node_as::<Node>("AsteroidMarkers")
                .get_children();

            let selected_asteroid_spawn = asteroid_markers
                .get((randi() as usize) % asteroid_markers.len())
                .unwrap() // an invalid state here is irrepresentable.
                .cast::<Marker2D>();

            self.base_mut()
                .set_position(selected_asteroid_spawn.get_position());
        }

        let position = self.base().get_position();
        if position.x <= 50.0 {
            self.direction.x = 1.0;
        } else if position.x >= 150.0 {
            self.direction.x = -1.0;
        } else {
            let ones = array![-1.0, 1.0];
            self.direction.x = ones.get((randi() as usize) % ones.len()).unwrap();
        }
    }

    fn physics_process(&mut self, delta: f64) {}
}
