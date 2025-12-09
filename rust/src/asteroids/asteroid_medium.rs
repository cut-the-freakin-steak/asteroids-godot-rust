// NOTE: done with this file

use godot::classes::{
    Area2D, CollisionPolygon2D, GpuParticles2D, IArea2D, Marker2D, Sprite2D, Timer,
};
use godot::global::{randi, randi_range};
use godot::prelude::*;

use crate::asteroids::asteroid::{Asteroid, AsteroidIFunctions, AsteroidSize};
use crate::audio::sfx_manager;
use crate::main_scene::Main;
use crate::player;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct MediumAsteroid {
    base: Base<Area2D>,

    // NOTE: new stuff
    #[init(val = OnReady::manual())]
    pub main: OnReady<Gd<Node>>,

    #[init(val = Vector2::ZERO)]
    pub direction: Vector2,

    #[init(val = 0.0)]
    pub vertical_speed: f32,

    #[init(val = 0)]
    pub rotation_speed: i32,

    #[init(val = 0.0)]
    pub horizontal_speed: f32,

    #[init(val = false)]
    pub use_set_position: bool,

    #[init(node = "ExplosionToQueueFree")]
    pub explosion_to_queue_free: OnReady<Gd<Timer>>,

    #[init(node = "Sprite2D")]
    pub sprite: OnReady<Gd<Sprite2D>>,

    #[init(node = "CollisionPolygon2D")]
    pub collision: OnReady<Gd<CollisionPolygon2D>>,

    #[init(node = "AsteroidExplosion")]
    pub explosion_parts: OnReady<Gd<GpuParticles2D>>,
    // NOTE: end of new stuff
    #[init(node = "Asteroid")]
    pub ast_base: OnReady<Gd<Asteroid>>,

    #[allow(dead_code)]
    #[init(val = AsteroidSize::Medium)]
    asteroid_size: AsteroidSize,

    // singletons
    #[allow(non_snake_case)]
    #[init(node = "/root/SFXManager")]
    pub SFXManager: OnReady<Gd<sfx_manager::SFXManagerClass>>,
}

#[godot_api]
impl IArea2D for MediumAsteroid {
    fn ready(&mut self) {
        // NOTE: getting the scene tree and then current scene is a bit more
        // complicated in rust than in gdscript lmao
        let main_scene = self.base().get_tree().unwrap().get_current_scene().unwrap();

        self.main.init(main_scene);

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

        AsteroidIFunctions::asteroid_ready(self);

        self.rotation_speed = randi_range(2, 3) as i32;
        self.horizontal_speed = randi_range(30, 40) as f32 * self.direction.x;
        self.explosion_to_queue_free
            .signals()
            .timeout()
            .connect_other(self, Self::on_explosion_to_queue_free_timeout);

        self.base()
            .signals()
            .body_entered()
            .connect_other(self, Self::_on_body_entered);
    }

    fn physics_process(&mut self, delta: f64) {
        if self.main.get_name() == "Main".into() {
            let main = self.main.clone().cast::<Main>();

            if main.bind().is_paused {
                return;
            }
        }

        AsteroidIFunctions::asteroid_physics_process(self, delta);
        {
            if !self.ast_base.is_instance_valid() {
                godot_warn!("ast_base was freed, removing self");
                self.base_mut().queue_free();
                return;
            }
            // let mut ast_base_bind = self.ast_base.bind_mut();
            // ast_base_bind.asteroid_physics_process(delta); // super
        }
        {
            let rotation = self.base().get_rotation();
            let rotation_speed = self.rotation_speed;
            self.base_mut()
                .set_rotation(rotation + rotation_speed as f32 * delta as f32);

            let position = self.base().get_position();
            let horizontal_speed = self.horizontal_speed;
            let vertical_speed = self.vertical_speed;
            self.base_mut().set_position(Vector2 {
                x: position.x + horizontal_speed * delta as f32,
                y: position.y + vertical_speed * delta as f32,
            });
        }
    }
}

#[godot_api]
impl MediumAsteroid {
    #[func]
    fn split_in_two(&mut self) {
        let global_position = self.base().get_global_position();
        let mut main = self.main.clone().cast::<Main>();
        main.bind_mut().score += 2;
        main.emit_signal(
            "asteroid_hit",
            &[
                AsteroidSize::Medium.to_variant(),
                global_position.to_variant(),
            ],
        );
        self.explosion_parts.set_emitting(true);
        self.sprite.set_visible(false);
        self.collision.set_disabled(true);
        self.explosion_to_queue_free.start();

        // it took me way too much time to learn about Object::call
        let explosion_sfx = &mut self.SFXManager.bind_mut().explosion;
        explosion_sfx.call(
            "set_parameter",
            &["WhichSound".to_variant(), "SmallMed".to_variant()],
        );
        explosion_sfx.call("play", &[]);
    }

    // signal connection
    #[func]
    pub fn _on_body_entered(&mut self, body: Gd<Node2D>) {
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

    // signal connection
    #[func]
    fn on_explosion_to_queue_free_timeout(&mut self) {
        self.base_mut().queue_free();
    }
}
