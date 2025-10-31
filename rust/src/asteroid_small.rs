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

            ast_base.horizontal_speed = randi_range(40, 55) as f32 * ast_base.direction.x;
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
impl SmallAsteroid {
    #[func]
    fn split_in_two(&mut self) {
        let mut ast_bind = self.ast_base.bind_mut();
        ast_bind.main.bind_mut().score += 3;
        ast_bind.explosion_parts.set_emitting(true);
        ast_bind.sprite.set_visible(false);
        ast_bind.collision.set_disabled(true);
        ast_bind.explosion_to_queue_free.start();

        // TODO: find out how to translate this stuff to rust (it uses a singleton)
        // SFXManager.explosion.set_parameter("WhichSound", "SmallMed")
        // SFXManager.explosion.play()
    }

    // signal connection
    #[func]
    fn on_explosion_to_queue_free_timeout(&mut self) {
        self.base_mut().queue_free();
    }
}
