// NOTE: done with this file

use godot::classes::{CharacterBody2D, ICharacterBody2D, Node2D};
use godot::global::{randf_range, randomize, snappedf};
use godot::prelude::*;

use crate::audio::sfx_manager;

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
struct Bullet {
    base: Base<CharacterBody2D>,

    #[init(val = OnReady::manual())]
    main: OnReady<Gd<Node2D>>,

    #[init(val = OnReady::manual())]
    player: OnReady<Gd<CharacterBody2D>>,

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
                .cast::<Node2D>();
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

        self.SFXManager.bind_mut().laser_shot.call(
            "set_parameter",
            &[snappedf(randf_range(-0.05, 0.05), 0.01).to_variant()],
        );
        self.SFXManager.bind_mut().laser_shot.call("play", &[]);
    }
}
