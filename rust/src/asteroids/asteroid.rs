// NOTE: done with this file

use godot::classes::{
    Area2D, CollisionPolygon2D, GpuParticles2D, IArea2D, Node, Node2D, Sprite2D, Timer,
};
use godot::global::{randi, randi_range, randomize};
use godot::prelude::*;

use crate::{audio::sfx_manager, main_scene, player};

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct Asteroid {
    pub base: Base<Area2D>,

    // singletons
    #[allow(non_snake_case)]
    #[init(node = "/root/SFXManager")]
    pub SFXManager: OnReady<Gd<sfx_manager::SFXManagerClass>>,

    // onreadys
    #[init(val = OnReady::manual())]
    pub main: OnReady<Gd<Node>>,

    #[init(node = "../Sprite2D")]
    pub sprite: OnReady<Gd<Sprite2D>>,

    #[init(node = "../CollisionPolygon2D")]
    pub collision: OnReady<Gd<CollisionPolygon2D>>,

    #[init(node = "../AsteroidExplosion")]
    pub explosion_parts: OnReady<Gd<GpuParticles2D>>,

    #[init(node = "../ExplosionToQueueFree")]
    pub explosion_to_queue_free: OnReady<Gd<Timer>>,

    // normal vars
    #[init(val = Vector2::ZERO)]
    pub direction: Vector2,

    #[init(val = 0.0)]
    pub vertical_speed: f32,

    #[init(val = 0.0)]
    pub horizontal_speed: f32,

    #[init(val = false)]
    pub use_set_position: bool,

    #[init(val = 0)]
    pub rotation_speed: i32,
}

pub trait AsteroidIFunctions {
    fn asteroid_ready(&mut self);
    fn asteroid_physics_process(&mut self, _delta: f64);
}

impl AsteroidIFunctions for Asteroid {
    fn asteroid_ready(&mut self) {
        // NOTE: getting the scene tree and then current scene is a bit more
        // complicated in rust than in gdscript lmao
        let main_scene = self.base().get_tree().unwrap().get_current_scene().unwrap();

        self.main.init(main_scene);

        // rest of the ready function
        randomize();

        let position = self.base().get_position();
        if position.x <= 50.0 {
            self.direction.x = 1.0;
        }
        else if position.x >= 150.0 {
            self.direction.x = -1.0;
        }
        else {
            godot_print!("why x");
            let ones = array![-1.0, 1.0];
            self.direction.x = ones.get(randi() as usize % ones.len()).unwrap();
        }

        if position.y <= 50.0 {
            self.direction.y = 1.0;
        }
        else if position.y >= 150.0 {
            self.direction.y = -1.0;
        }
        else {
            godot_print!("why y");
            let ones = array![-1.0, 1.0];
            self.direction.y = ones.get(randi() as usize % ones.len()).unwrap();
        }

        let vert_speed_coin_flip = randi_range(0, 1);
        if vert_speed_coin_flip == 0 {
            self.vertical_speed = randi_range(20, 30) as f32 * self.direction.y;
        }
        else {
            self.vertical_speed = randi_range(35, 45) as f32 * self.direction.y;
        }
        self.base()
            .signals()
            .body_entered()
            .connect_other(self, Self::_on_body_entered);
    }

    fn asteroid_physics_process(&mut self, _delta: f64) {
        if self.main.is_in_group("main_scene") {
            let main = self.main.clone().cast::<main_scene::Main>();
            if main.bind().is_paused {
                return;
            }
        }

        let position = self.base().get_position();

        if position.x > 250.0 || position.x < -50.0 {
            self.base_mut().queue_free();
        }

        if position.y > 250.0 || position.y < -50.0 {
            self.base_mut().queue_free();
        }
    }
}

#[godot_api]
impl IArea2D for Asteroid {
    fn ready(&mut self) {
        self.asteroid_ready();
    }

    fn physics_process(&mut self, delta: f64) {
        self.asteroid_physics_process(delta);
    }
}

#[godot_api]
impl Asteroid {
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
}

#[derive(PartialEq, GodotConvert, Debug, Clone)]
#[godot(via = i32)]
pub enum AsteroidSize {
    Small,
    Medium,
    Big,
}
