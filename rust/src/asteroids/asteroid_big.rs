// NOTE: done with this file

use godot::classes::{Area2D, IArea2D};
use godot::global::randi_range;
use godot::prelude::*;

use crate::{
    asteroids::asteroid::{Asteroid, AsteroidIFunctions},
    camera_manager::CameraManager,
};

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct BigAsteroid {
    base: Base<Area2D>,

    #[init(node = "Asteroid")]
    ast_base: OnReady<Gd<Asteroid>>,

    #[init(val = OnReady::manual())]
    camera_manager: OnReady<Gd<CameraManager>>,
}

#[godot_api]
impl IArea2D for BigAsteroid {
    fn ready(&mut self) {
        {
            let mut ast_bind = self.ast_base.bind_mut();
            ast_bind.asteroid_ready(); // super()

            ast_bind.rotation_speed = randi_range(1, 2) as i32;
            ast_bind.horizontal_speed = randi_range(15, 30) as f32 * ast_bind.direction.x;
        }
        self.ast_base
            .bind()
            .explosion_to_queue_free
            .signals()
            .timeout()
            .connect_other(self, Self::on_explosion_to_queue_free_timeout);

        // big asteroid shakes camera, so we need to have the camera manager node in main
        let ast_bind = self.ast_base.bind();
        let ast_base = self.base();
        if ast_base.get_name() == "Main".into() {
            self.camera_manager
                .init(ast_bind.main.get_node_as::<CameraManager>("CameraManager"));
        }
    }

    fn physics_process(&mut self, delta: f64) {
        {
            let mut ast_base_bind = self.ast_base.bind_mut();
            ast_base_bind.asteroid_physics_process(delta); // super
        }
        {
            let rotation = self.ast_base.get_rotation();
            let rotation_speed = self.ast_base.bind().rotation_speed;
            self.ast_base
                .set_rotation(rotation + rotation_speed as f32 * delta as f32);

            let position = self.ast_base.get_position();
            let horizontal_speed = self.ast_base.bind().horizontal_speed;
            self.ast_base.set_position(Vector2 {
                x: position.x + horizontal_speed * delta as f32,
                y: position.y,
            });
        }
    }
}

#[godot_api]
impl BigAsteroid {
    #[func]
    fn split_in_two(&mut self) {
        let global_position = self.base().get_global_position();
        let mut ast_bind = self.ast_base.bind_mut();
        ast_bind.main.bind_mut().score += 1;
        ast_bind.main.emit_signal(
            "asteroid_hit",
            &["big".to_variant(), global_position.to_variant()],
        );
        ast_bind.explosion_parts.set_emitting(true);
        ast_bind.sprite.set_visible(false);
        ast_bind.collision.set_disabled(true);
        ast_bind.explosion_to_queue_free.start();

        // it took me way too much time to learn about Object::call
        let explosion_sfx = &mut ast_bind.SFXManager.bind_mut().explosion;
        explosion_sfx.call(
            "set_parameter",
            &["WhichSound".to_variant(), "Big".to_variant()],
        );
        explosion_sfx.call("play", &[]);

        self.camera_manager.bind_mut().screen_shake(2.5, 0.3);
    }

    // signal connection
    #[func]
    fn on_explosion_to_queue_free_timeout(&mut self) {
        self.base_mut().queue_free();
    }
}
