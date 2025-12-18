// NOTE: done with this file

use godot::classes::{
    AnimationPlayer, CharacterBody2D, GpuParticles2D, ICharacterBody2D, Input, Label, Sprite2D,
    Timer,
};
use godot::global::deg_to_rad;
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

    #[init(val = OnReady::manual())]
    screen_size: OnReady<f32>,

    #[init(val = OnReady::manual())]
    sprite_dimensions: OnReady<Vector2>,

    #[init(val = 100.0)]
    max_speed: f32,

    #[init(val = 2.5)]
    acceleration: f32,

    #[init(val = 5.5)]
    rotation_speed: f32,

    #[init(val = false)]
    ship_on_screen_border_x: bool,

    #[init(val = false)]
    ship_on_screen_border_y: bool,

    #[init(val = 3)]
    hp: i32,

    #[init(val = true)]
    can_be_damaged: bool,

    #[init(val = 0)]
    millisecond_even_or_odd: i32,

    #[init(node = "ShootTimer")]
    pub shoot_timer: OnReady<Gd<Timer>>,

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

        self.i_frame_timer
            .signals()
            .timeout()
            .connect_other(self, Self::_on_i_frame_timer_timeout);

        self.signals()
            .damage_taken()
            .connect_self(Self::_on_damage_taken);
    }

    fn process(&mut self, delta: f64) {
        let input = Input::singleton();
        let ship_thruster_sfx = &mut self.SFXManager.bind_mut().ship_thruster;

        if self.main.bind().is_game_over || input.is_action_pressed("decelerate") {
            ship_thruster_sfx.stop();
        }
        else {
            if input.is_action_just_pressed("up") {
                ship_thruster_sfx.set_parameter("ShouldLoop", "Yes".to_variant());

                ship_thruster_sfx.play(None);
            }

            if input.is_action_pressed("up") {
                ship_thruster_sfx.play(Some(false));

                if ship_thruster_sfx
                    .get_parameter("PitchParam")
                    .to::<f32>()
                    .to_godot()
                    < 0.20
                {
                    let modified_pitch_param =
                        ship_thruster_sfx.get_parameter("PitchParam").to::<f32>()
                            + (0.10 * delta as f32);

                    ship_thruster_sfx
                        .set_parameter("PitchParam", modified_pitch_param.to_variant());
                }
            }

            if !input.is_action_pressed("up")
                && ship_thruster_sfx
                    .get_parameter("ShouldLoop")
                    .to::<GString>()
                    == "Yes".to_godot()
            {
                ship_thruster_sfx.stop();
                ship_thruster_sfx.set_parameter("PitchParam", 0.0.to_variant());
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();

        if !self.alive {
            self.back_particles.set_emitting(false);
            let velocity = self.base().get_velocity();
            self.base_mut()
                .set_velocity(velocity.move_toward(Vector2::ZERO, 1.0));

            if velocity == Vector2::ZERO {
                return;
            }
        }

        if self.main.bind().is_paused {
            self.back_particles.set_speed_scale(0.0);
            self.back_particles.set_emitting(false);
            return;
        }
        else {
            self.back_particles.set_speed_scale(1.0);
        }

        // screen wrapping
        let global_position = self.base().get_global_position();
        if global_position.x - 5.0 > *self.screen_size {
            self.base_mut().set_global_position(Vector2 {
                x: 5.0,
                y: global_position.y,
            });
        }

        // hate the borrow checker fr
        let screen_size = *self.screen_size;
        if global_position.x + 5.0 < 0.0 {
            self.base_mut().set_global_position(Vector2 {
                x: screen_size - 5.0,
                y: global_position.y,
            });
        }

        if global_position.y - 5.0 > screen_size {
            self.base_mut().set_global_position(Vector2 {
                x: global_position.x,
                y: 5.0,
            });
        }

        if global_position.y + 5.0 < 0.0 {
            self.base_mut().set_global_position(Vector2 {
                x: global_position.x,
                y: screen_size - 5.0,
            });
        }

        self.show_screen_wrapped_ship();

        // movement
        if self.alive {
            let rotation_direction = input.get_axis("left", "right");
            let mut movement_vector = Vector2 {
                x: 0.0,
                y: input.get_axis("down", "up"),
            };

            if input.is_action_pressed("decelerate") {
                movement_vector.y = -1.0;
            }

            let mut velocity = self.base().get_velocity();
            let rotation = self.base().get_rotation();
            let acceleration = self.acceleration;
            if movement_vector.y == 1.0 {
                self.base_mut().set_velocity(
                    velocity
                        + movement_vector.rotated(rotation - deg_to_rad(90.0) as f32)
                            * acceleration,
                );
                self.back_particles.set_emitting(true);
            }

            if movement_vector.y == 0.0 {
                self.base_mut()
                    .set_velocity(velocity.move_toward(Vector2::ZERO, 1.3));
                self.back_particles.set_emitting(false);
            }

            if movement_vector.y == -1.0 {
                self.base_mut()
                    .set_velocity(velocity.move_toward(Vector2::ZERO, 3.0));
                self.back_particles.set_emitting(false);
            }

            let max_speed = self.max_speed;
            // NOTE: overwrite old velocity value with new, changed velocity value
            velocity = self.base().get_velocity();
            self.base_mut()
                .set_velocity(velocity.limit_length(Some(max_speed)));

            let rotation_speed = self.rotation_speed;
            self.base_mut()
                .rotate(rotation_speed * rotation_direction * delta as f32);

            // i-frame flashing
            let time_left = self.i_frame_timer.get_time_left();
            if time_left > 0.5 {
                self.millisecond_even_or_odd = ((time_left * 10.0) as i32) % 3;

                self.sprite.set_visible(self.millisecond_even_or_odd != 0);
            }
            else if time_left > 0.0 {
                self.millisecond_even_or_odd = ((time_left * 10.0) as i32) % 2;

                self.sprite.set_visible(self.millisecond_even_or_odd != 0);
            }
        }

        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Player {
    #[signal]
    pub fn damage_taken();

    // show ship at other side of screen when player going OOB
    #[func]
    fn show_screen_wrapped_ship(&mut self) {
        // x-axis
        let input = Input::singleton();
        let position = self.base().get_position();
        let global_position = self.base().get_global_position();
        let rotation_degrees = self.base().get_rotation_degrees();

        // right
        if global_position.x
            >= *self.screen_size - (self.sprite_dimensions.x - self.sprite_dimensions.x / 2.0)
        {
            self.ship_on_screen_border_x = true;
            self.screen_wrap_stuff.set_global_position(Vector2 {
                x: position.x - *self.screen_size,
                y: position.y,
            });
            self.opposite_screen_sprite.set_visible(true);

            self.screen_wrap_stuff
                .set_rotation_degrees(rotation_degrees + 90.0);

            if input.is_action_pressed("up") {
                self.other_side_back_particles.set_emitting(true);
            }
            else {
                self.other_side_back_particles.set_emitting(false);
            }
        }
        // left
        else if global_position.x <= 0.0 + (self.sprite_dimensions.x / 2.0) {
            self.ship_on_screen_border_x = true;
            self.screen_wrap_stuff.set_global_position(Vector2 {
                x: position.x + *self.screen_size,
                y: position.y,
            });
            self.opposite_screen_sprite.set_visible(true);

            self.screen_wrap_stuff
                .set_rotation_degrees(rotation_degrees + 90.0);

            if input.is_action_pressed("up") {
                self.other_side_back_particles.set_emitting(true);
            }
            else {
                self.other_side_back_particles.set_emitting(false);
            }
        }
        else {
            self.ship_on_screen_border_x = false;
            self.opposite_screen_sprite.set_visible(false);
            self.other_side_back_particles.set_emitting(false);
        }
        // y-axis
        // bottom
        if global_position.y
            >= *self.screen_size
                - (self.sprite.get_texture().unwrap().get_height() as f32 / 2.0 + 8.0)
        {
            self.ship_on_screen_border_y = true;
            self.screen_wrap_stuff.set_global_position(Vector2 {
                x: position.x,
                y: position.y - *self.screen_size,
            });
            self.opposite_screen_sprite.set_visible(true);
            self.screen_wrap_stuff
                .set_rotation_degrees(rotation_degrees + 90.0);

            if input.is_action_pressed("up") {
                self.other_side_back_particles.set_emitting(true);
            }
            else {
                self.other_side_back_particles.set_emitting(false);
            }
        }
        // top
        else if global_position.y
            <= 0.0 + (self.sprite.get_texture().unwrap().get_height() as f32 / 2.0 - 8.0)
        {
            self.ship_on_screen_border_y = true;
            self.screen_wrap_stuff.set_global_position(Vector2 {
                x: position.x,
                y: position.y + *self.screen_size,
            });
            self.opposite_screen_sprite.set_visible(true);
            self.screen_wrap_stuff
                .set_rotation_degrees(rotation_degrees + 90.0);

            if input.is_action_pressed("up") {
                self.other_side_back_particles.set_emitting(true);
            }
            else {
                self.other_side_back_particles.set_emitting(false);
            }
        }
        else {
            self.ship_on_screen_border_y = false;
            if !self.ship_on_screen_border_x {
                self.opposite_screen_sprite.set_visible(false);
                self.other_side_back_particles.set_emitting(false);
            }
        }

        if !self.alive {
            self.other_side_back_particles.set_emitting(false);
        }
    }

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
    fn _on_damage_taken(&mut self) {
        godot_print!("boing");
        if !self.can_be_damaged || !self.alive {
            return;
        }

        self.hp -= 1;

        {
            let player_hurt_sfx = &mut self.SFXManager.bind_mut().player_hurt;

            match self.hp {
                2 => {
                    player_hurt_sfx.set_parameter("HPRemaining", "Alive".to_variant());
                    player_hurt_sfx.play(None);
                    self.main
                        .get_node_as::<AnimationPlayer>("UI/HealthUI/Health")
                        .play_ex()
                        .name("lose_health_1")
                        .done();
                }
                1 => {
                    player_hurt_sfx.set_parameter("HPRemaining", "Alive".to_variant());
                    player_hurt_sfx.play(None);
                    self.main
                        .get_node_as::<AnimationPlayer>("UI/HealthUI/Health")
                        .play_ex()
                        .name("lose_health_2")
                        .done();
                }
                0 => {
                    player_hurt_sfx.set_parameter("HPRemaining", "Dead".to_variant());
                    player_hurt_sfx.play(None);
                    self.main
                        .get_node_as::<AnimationPlayer>("UI/HealthUI/Health")
                        .play_ex()
                        .name("lose_health_3")
                        .done();
                }
                _ => {} // nothing should happen
            }
        }
        if self.hp > 0 {
            self.can_be_damaged = false;
            self.i_frame_timer.start();

            let velocity = self.base().get_velocity();
            let rotation = self.base().get_rotation();
            self.base_mut().set_velocity(
                velocity
                    + Vector2 { x: 0.0, y: -100.0 }.rotated(rotation - deg_to_rad(90.0) as f32),
            );
        }
        else {
            // you are die lmao
            if self.alive {
                let velocity = self.base().get_velocity();
                let rotation = self.base().get_rotation();
                self.base_mut().set_velocity(
                    velocity
                        + Vector2 { x: 0.0, y: -200.0 }.rotated(rotation - deg_to_rad(90.0) as f32),
                );
                self.main.signals().game_over().emit();
            }

            self.alive = false;
            self.can_be_damaged = false;
        }
    }

    #[func]
    fn _on_i_frame_timer_timeout(&mut self) {
        self.sprite.set_visible(true);
        if self.alive {
            self.can_be_damaged = true
        }
    }
}
