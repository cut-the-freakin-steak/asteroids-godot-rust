// NOTE: done with this file

use godot::classes::{Area2D, CharacterBody2D, ICharacterBody2D};
use godot::global::{deg_to_rad, randf_range, randomize, snappedf};
use godot::prelude::*;

use crate::audio::sfx_manager;
use crate::main_scene::Main;

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
struct Bullet {
    base: Base<CharacterBody2D>,

    #[init(val = OnReady::manual())]
    main: OnReady<Gd<Main>>,

    #[init(val = OnReady::manual())]
    player: OnReady<Gd<CharacterBody2D>>,

    #[init(node = "Area2D")]
    area_2d: OnReady<Gd<Area2D>>,

    #[init(val = 200.0)]
    max_speed: f32,

    #[init(val = 30.0)]
    acceleration: f32,

    #[allow(non_snake_case)]
    #[init(node = "/root/SFXManager")]
    SFXManager: OnReady<Gd<sfx_manager::SFXManagerClass>>,
}

#[godot_api]
impl ICharacterBody2D for Bullet {
    fn ready(&mut self) {
        {
            let current_scene = self
                .base()
                .get_tree()
                .unwrap()
                .get_current_scene()
                .unwrap()
                .cast::<Main>();
            self.main.init(current_scene);

            let player_node = self
                .main
                .get_node_as::<CharacterBody2D>("PlayerStuff/Player");
            self.player.init(player_node);
        }
        randomize();
        let player_rotation_degs = self.player.get_rotation_degrees();
        self.base_mut()
            .set_rotation_degrees(player_rotation_degs + 90.0);

        self.SFXManager.bind_mut().laser_shot.set_parameter(
            "PitchParam",
            snappedf(randf_range(-0.05, 0.05), 0.01).to_variant(),
        );
        self.SFXManager.bind_mut().laser_shot.play(None);

        // signal stuff
        self.area_2d
            .signals()
            .area_entered()
            .connect_other(self, Self::_on_area_2d_area_entered);
    }

    fn physics_process(&mut self, _delta: f64) {
        if self.main.bind().is_paused {
            return;
        }

        let mut velocity = self.base().get_velocity();
        let rotation = self.base().get_rotation();
        let acceleration = self.acceleration;
        self.base_mut().set_velocity(
            velocity
                + Vector2 { x: 0.0, y: 1.0 }.rotated(rotation - deg_to_rad(180.0) as f32)
                    * acceleration,
        );
        velocity = self.base().get_velocity();
        let max_speed = self.max_speed;
        self.base_mut()
            .set_velocity(velocity.limit_length(Some(max_speed)));
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Bullet {
    // signal connection
    #[func]
    fn _on_area_2d_area_entered(&mut self, mut area: Gd<Area2D>) {
        if area.is_in_group("asteroid") {
            area.call_deferred("split_in_two", &[]);
            self.base_mut().queue_free();
        }
    }
}
