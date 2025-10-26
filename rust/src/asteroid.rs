// NOTE: done with this file

use godot::classes::{
    Area2D, CollisionPolygon2D, GpuParticles2D, IArea2D, Marker2D, Node, Sprite2D, Timer,
};
use godot::global::{randi, randi_range, randomize};
use godot::prelude::*;

use crate::main_scene;
use crate::player;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
struct Asteroid {
    base: Base<Area2D>,

    // onready
    #[init(val = OnReady::manual())]
    main: OnReady<Gd<main_scene::Main>>,

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
    vertical_speed: f32,

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
                Some(main) => main_scene = main.cast::<main_scene::Main>(),
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
                .get(randi() as usize % asteroid_markers.len())
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
            self.direction.x = ones.get(randi() as usize % ones.len()).unwrap();
        }

        if position.y <= 50.0 {
            self.direction.y = 1.0;
        } else if position.y >= 150.0 {
            self.direction.y = -1.0;
        } else {
            let ones = array![-1.0, 1.0];
            self.direction.y = ones.get(randi() as usize % ones.len()).unwrap();
        }

        let vert_speed_coin_flip = randi_range(0, 1);
        if vert_speed_coin_flip == 0 {
            self.vertical_speed = randi_range(20, 30) as f32 * self.direction.y;
        } else {
            self.vertical_speed = randi_range(35, 45) as f32 * self.direction.y;
        }
        self.base()
            .signals()
            .body_entered()
            .connect_other(self, Self::on_body_entered);
    }

    fn physics_process(&mut self, delta: f64) {
        if self.main.is_in_group("main_scene") && self.main.bind().is_paused {
            return;
        }

        let position = self.base().get_position();

        // use a scope to limit vert_speed since i dont want it cluttering stuff up
        // outside of this operation
        {
            // we have to do this because since we're using base_mut(), we cant access
            // self.vertical_speed while doing set_position() cause borrow checker :3
            let vert_speed = self.vertical_speed;
            self.base_mut().set_position(Vector2 {
                x: position.x,
                y: position.y + vert_speed * delta as f32,
            });
        }

        if position.x > 250.0 || position.x < -50.0 {
            self.base_mut().queue_free();
        }

        if position.y > 250.0 || position.y < -50.0 {
            self.base_mut().queue_free();
        }
    }
}

#[godot_api]
impl Asteroid {
    #[func]
    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        if body.is_in_group("player") {
            // FIX: figure out a way to make ts type safe
            // NOTE: ^ figured it out :3
            // something very interesting about this is that it's actually
            // *safer* than gdscript, since signals are typed in godot-rust

            // invalid state irrepresentable here, so a simple cast is just fine
            let real_body = body.cast::<player::Player>();
            real_body.signals().damage_taken().emit();
        }
    }
}
