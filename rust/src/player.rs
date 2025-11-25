use godot::classes::{
    CharacterBody2D, GpuParticles2D, ICharacterBody2D, Input, Label, Sprite2D, Timer,
};
use godot::prelude::*;

use crate::main_scene::Main;

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,

    #[init(val = OnReady::manual())]
    main: OnReady<Gd<Main>>,

    #[init(val = OnReady::manual())]
    screen_wrap_stuff: OnReady<Gd<Node2D>>,

    #[init(val = OnReady::manual())]
    opposite_screen_sprite: OnReady<Gd<Sprite2D>>,

    #[init(val = OnReady::manual())]
    other_side_back_particles: OnReady<Gd<GpuParticles2D>>,

    #[init(val = OnReady::manual())]
    game_over_text: OnReady<Gd<Label>>,

    #[init(node = "Sprite2D")]
    sprite: OnReady<Gd<Sprite2D>>,

    #[init(node = "BackFireParticles")]
    back_particles: OnReady<Gd<GpuParticles2D>>,

    #[init(node = "IFrameTimer")]
    i_frame_timer: OnReady<Gd<Timer>>,

    #[init(node = "ShootTimer")]
    pub shoot_timer: OnReady<Gd<Timer>>,

    #[init(val = OnReady::manual())]
    screen_size: OnReady<f32>,

    #[init(val = OnReady::manual())]
    sprite_dimensions: OnReady<Vector2>,

    #[init(val = 100)]
    max_speed: i32,

    #[init(val = 2.5)]
    acceleration: f32,

    #[init(val = 5.5)]
    rotation_speed: f32,

    #[init(val = false)]
    ship_on_screen_border_x: bool,

    #[init(val = false)]
    ship_on_screen_border_y: bool,

    #[init(val = Vector2::ZERO)]
    last_direction_faced: Vector2,

    #[init(val = 3)]
    hp: i32,

    #[init(val = true)]
    can_be_damaged: bool,

    #[init(val = 0)]
    millisecond_even_or_odd: i32,

    #[init(val = true)]
    pub alive: bool,

    // singletons
    #[allow(non_snake_case)]
    #[init(node = "/root/SFXManager")]
    SFXManager: OnReady<Gd<crate::audio::sfx_manager::SFXManagerClass>>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn ready(&mut self) {
        // NOTE: aughhhhhhhhh onready initializations
        {
            self.main.init(
                self.base()
                    .get_tree()
                    .unwrap()
                    .get_current_scene()
                    .unwrap()
                    .cast::<Main>(),
            );

            // nodes
            self.screen_wrap_stuff
                .init(self.main.get_node_as::<Node2D>("PlayerStuff/SWStuff"));

            self.opposite_screen_sprite.init(
                self.main
                    .get_node_as::<Sprite2D>("PlayerStuff/SWStuff/OtherSideSprite"),
            );

            self.other_side_back_particles.init(
                self.main
                    .get_node_as::<GpuParticles2D>("PlayerStuff/SWStuff/OpBackFireParticles"),
            );

            self.game_over_text
                .init(self.main.get_node_as::<Label>("UI/GameOver"));

            // not nodes
            self.screen_size
                .init(self.base().get_viewport_rect().size.x);

            // why do i have to call the function like that lmao
            self.sprite_dimensions
                .init(self::Player::get_visible_sprite_dimensions(
                    self.sprite.clone(),
                ));
        }

        self.signals()
            .damage_taken()
            .connect_self(Self::_on_damage_taken);
    }

    fn process(&mut self, delta: f64) {
        let input = Input::singleton();

        if self.main.bind().is_game_over || input.is_action_pressed("decelerate") {
            self.SFXManager.bind_mut().ship_thruster.call("stop", &[]);
        }
        else {
        }
        // else:
        //     if Input.is_action_just_pressed("up"):
        //         SFXManager.ship_thruster.set_parameter("ShouldLoop", "Yes")
        //         SFXManager.ship_thruster.play()
        //
        //     if Input.is_action_pressed("up"):
        //         SFXManager.ship_thruster.play(false)
        //         if SFXManager.ship_thruster.get_parameter("PitchParam") < 0.20:
        //             SFXManager.ship_thruster.set_parameter("PitchParam", SFXManager.ship_thruster.get_parameter("PitchParam") + (0.10 * delta))
        //
        //     if not Input.is_action_pressed("up") and SFXManager.ship_thruster.get_parameter("ShouldLoop") == "Yes":
        //         SFXManager.ship_thruster.stop()
        //         SFXManager.ship_thruster.set_parameter("PitchParam", 0.0)
    }
}

#[godot_api]
impl Player {
    #[signal]
    pub fn damage_taken();

    #[func]
    fn show_screen_wrapped_ship(&self) {}

    #[func]
    fn get_visible_sprite_dimensions(sprite_2d: Gd<Sprite2D>) -> Vector2 {
        if sprite_2d.get_texture().is_none() {
            return Vector2 { x: 0.0, y: 0.0 };
        }

        let image = sprite_2d.get_texture().unwrap().get_image().unwrap();
        let used_rect = image.get_used_rect();

        Vector2 {
            x: used_rect.size.x as f32,
            y: used_rect.size.y as f32,
        }
    }

    // signal connection
    #[func]
    fn _on_damage_taken(&mut self) {}
}
