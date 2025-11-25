// NOTE: done with this file

use godot::classes::{Area2D, IArea2D};
use godot::global::randi_range;
use godot::prelude::*;

use crate::asteroids::asteroid::{Asteroid, AsteroidIFunctions, AsteroidSize};

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct MediumAsteroid {
    base: Base<Area2D>,

    #[init(node = "Asteroid")]
    pub ast_base: OnReady<Gd<Asteroid>>,

    #[init(val = AsteroidSize::Medium)]
    asteroid_size: AsteroidSize,
}

#[godot_api]
impl IArea2D for MediumAsteroid {
    fn ready(&mut self) {
        {
            let mut ast_bind = self.ast_base.bind_mut();
            ast_bind.asteroid_ready(); // super()

            ast_bind.rotation_speed = randi_range(2, 3) as i32;
            ast_bind.horizontal_speed = randi_range(30, 40) as f32 * ast_bind.direction.x;
        }
        self.ast_base
            .bind()
            .explosion_to_queue_free
            .signals()
            .timeout()
            .connect_other(self, Self::on_explosion_to_queue_free_timeout);
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
impl MediumAsteroid {
    #[func]
    fn split_in_two(&mut self) {
        let global_position = self.base().get_global_position();
        let mut ast_bind = self.ast_base.bind_mut();
        ast_bind.main.bind_mut().score += 2;
        ast_bind.main.emit_signal(
            "asteroid_hit",
            &["medium".to_variant(), global_position.to_variant()],
        );
        ast_bind.explosion_parts.set_emitting(true);
        ast_bind.sprite.set_visible(false);
        ast_bind.collision.set_disabled(true);
        ast_bind.explosion_to_queue_free.start();

        // it took me way too much time to learn about Object::call
        let explosion_sfx = &mut ast_bind.SFXManager.bind_mut().explosion;
        explosion_sfx.call(
            "set_parameter",
            &["WhichSound".to_variant(), "SmallMed".to_variant()],
        );
        explosion_sfx.call("play", &[]);
    }

    // signal connection
    #[func]
    fn on_explosion_to_queue_free_timeout(&mut self) {
        self.base_mut().queue_free();
    }
}
