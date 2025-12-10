// NOTE: finally done with this file

use crate::asteroids::asteroid::AsteroidSize;
use crate::asteroids::{asteroid_medium::MediumAsteroid, asteroid_small::SmallAsteroid};
use crate::audio::music_manager::MusicManagerClass;
use crate::audio::sfx_manager::SFXManagerClass;
use crate::global_settings::SettingsClass;
use crate::player::Player;
use godot::classes::{
    AnimationPlayer, Area2D, Button, CharacterBody2D, Control, INode2D, Input, Label, Marker2D,
    Node, Node2D, ResourceLoader, Timer,
};
use godot::global::randi_range;
use godot::prelude::*;

// good lord what a mess.
#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct Main {
    base: Base<Node2D>,

    // nodes
    #[init(node = "PlayerStuff/Player")]
    player: OnReady<Gd<Player>>,

    #[init(val = OnReady::manual())]
    scene_root_node: OnReady<Gd<Node>>,

    #[init(node = "UI/GameOver")]
    game_over_text: OnReady<Gd<Label>>,

    #[init(node = "UI/TryAgain")]
    try_again_button: OnReady<Gd<Button>>,

    #[init(node = "UI/MainMenu")]
    main_menu_button: OnReady<Gd<Button>>,

    #[init(node = "UI/UIAnimation")]
    ui_animation: OnReady<Gd<AnimationPlayer>>,

    #[init(node = "UI/GameOverAnimation")]
    game_over_label_animation: OnReady<Gd<AnimationPlayer>>,

    #[init(node = "UI/ButtonAnimation")]
    game_over_buttons_animation: OnReady<Gd<AnimationPlayer>>,

    #[init(node = "UI/GameOverAnimTimer")]
    game_over_animation_timer: OnReady<Gd<Timer>>,

    #[init(node = "UI/ScoreText")]
    score_label: OnReady<Gd<Label>>,

    #[init(node = "CameraManager")]
    camera_manager: OnReady<Gd<crate::camera_manager::CameraManager>>,

    #[init(node = "UI/PauseUI")]
    pause_ui: OnReady<Gd<Control>>,

    #[init(node = "UI/PauseUI/Pause")]
    pub pause_label: OnReady<Gd<Label>>,

    #[init(node = "UI/PauseUI/Resume")]
    pub resume_button: OnReady<Gd<Button>>,

    #[init(node = "UI/PauseUI/Settings")]
    pub pause_settings_button: OnReady<Gd<Button>>,

    #[init(node = "UI/PauseUI/MainMenu")]
    pub pause_main_menu_button: OnReady<Gd<Button>>,

    #[init(node = "AliveToDeadMusicTimer")]
    alive_to_dead_music_timer: OnReady<Gd<Timer>>,

    #[init(node = "AsteroidTimer")]
    asteroid_timer: OnReady<Gd<Timer>>,

    // scenes
    #[init(val = OnReady::manual())]
    main_menu_scene: OnReady<Gd<PackedScene>>,

    #[init(val = OnReady::manual())]
    settings_scene: OnReady<Gd<PackedScene>>,

    #[init(val = OnReady::manual())]
    game_scene: OnReady<Gd<PackedScene>>,

    #[init(val = ResourceLoader::singleton().load("res://scenes/asteroid-small.tscn").unwrap().cast::<PackedScene>())]
    small_ast_scene: Gd<PackedScene>,

    #[init(val = ResourceLoader::singleton().load("res://scenes/asteroid-medium.tscn").unwrap().cast::<PackedScene>())]
    medium_ast_scene: Gd<PackedScene>,

    #[init(val = ResourceLoader::singleton().load("res://scenes/asteroid-big.tscn").unwrap().cast::<PackedScene>())]
    big_ast_scene: Gd<PackedScene>,

    #[init(val = OnReady::manual())]
    laser: OnReady<Gd<PackedScene>>,

    #[init(val = OnReady::manual())]
    asteroids: OnReady<Array<Gd<PackedScene>>>, // array of PackedScenes

    // miscellaneous variables
    #[init(val = false)]
    game_over_anim_skipped: bool,

    #[init(val = false)]
    pub is_game_over: bool,

    #[init(val = false)]
    pub is_paused: bool,

    #[init(val = 0)]
    pub score: i64,

    // singletons
    #[allow(non_snake_case)]
    #[init(node = "/root/Settings")]
    Settings: OnReady<Gd<SettingsClass>>,

    #[allow(non_snake_case)]
    #[init(node = "/root/MusicManager")]
    MusicManager: OnReady<Gd<MusicManagerClass>>,

    #[allow(non_snake_case)]
    #[init(node = "/root/SFXManager")]
    SFXManager: OnReady<Gd<SFXManagerClass>>,
}

#[godot_api]
impl INode2D for Main {
    fn ready(&mut self) {
        // NOTE: initializing PackedScenes suuuuucks lmfao
        {
            self.scene_root_node
                .init(self.base().get_tree().unwrap().get_current_scene().unwrap());

            // initialize all of the PackedScenes
            let mut loader = ResourceLoader::singleton();
            self.main_menu_scene.init(
                loader
                    .load("res://scenes/main_menu.tscn")
                    .unwrap()
                    .cast::<PackedScene>(),
            );

            self.settings_scene.init(
                loader
                    .load("res://scenes/settings.tscn")
                    .unwrap()
                    .cast::<PackedScene>(),
            );

            self.game_scene.init(
                loader
                    .load("res://scenes/main.tscn")
                    .unwrap()
                    .cast::<PackedScene>(),
            );

            self.laser.init(
                loader
                    .load("res://scenes/bullet.tscn")
                    .unwrap()
                    .cast::<PackedScene>(),
            );

            self.asteroids.init(array![
                &self.small_ast_scene,
                &self.medium_ast_scene,
                &self.big_ast_scene,
            ]);
        }
        {
            if self.Settings.bind().hurricane_mode {
                self.MusicManager.bind_mut().gameplay.call(
                    "set_parameter",
                    &["WhichGameplaySong".to_variant(), "Hurricane".to_variant()],
                );
            }
            else {
                self.MusicManager.bind_mut().gameplay.call(
                    "set_parameter",
                    &["WhichGameplaySong".to_variant(), "Normal".to_variant()],
                );
            }

            self.MusicManager.bind_mut().gameplay.call(
                "set_parameter",
                &["MuffledOrNot".to_variant(), 1.0.to_variant()],
            );

            self.MusicManager.bind_mut().gameplay.call(
                "set_parameter",
                &["NormalGameplaySongPitch".to_variant(), 0.0.to_variant()],
            );

            self.score = 0;
            self.is_game_over = false;
            self.game_over_anim_skipped = false;
        }
        // signal stuff
        self.signals()
            .asteroid_hit()
            .connect_self(Self::_spawn_asteroid);

        self.signals().game_over().connect_self(Self::_on_game_over);

        self.asteroid_timer
            .signals()
            .timeout()
            .connect_other(self, Self::_on_asteroid_timer_timeout);

        self.game_over_animation_timer
            .signals()
            .timeout()
            .connect_other(self, Self::_on_game_over_anim_timer_timeout);

        self.alive_to_dead_music_timer
            .signals()
            .timeout()
            .connect_other(self, Self::_on_alive_to_dead_music_timer_timeout);

        self.resume_button
            .signals()
            .pressed()
            .connect_other(self, Self::_on_resume_pressed);

        self.pause_settings_button
            .signals()
            .pressed()
            .connect_other(self, Self::_on_settings_pressed);

        self.try_again_button
            .signals()
            .pressed()
            .connect_other(self, Self::_on_try_again_pressed);

        self.main_menu_button
            .signals()
            .pressed()
            .connect_other(self, Self::_on_main_menu_pressed);

        self.pause_main_menu_button
            .signals()
            .pressed()
            .connect_other(self, Self::_on_main_menu_pressed);
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();
        {
            let gameplay_song = &mut self.MusicManager.bind_mut().gameplay;

            if input.is_action_just_pressed("esc") && !self.is_game_over {
                if !self.is_paused {
                    self.is_paused = true;
                    self.pause_ui.set_visible(true);
                    self.resume_button.set_disabled(false);
                    self.pause_settings_button.set_disabled(false);
                    self.pause_main_menu_button.set_disabled(false);
                    gameplay_song.call(
                        "set_parameter",
                        &["MuffledOrNot".to_variant(), 0.40.to_variant()],
                    );
                }
                else {
                    self.is_paused = false;
                    self.pause_ui.set_visible(false);
                    self.resume_button.set_disabled(true);
                    self.pause_settings_button.set_disabled(true);
                    self.pause_main_menu_button.set_disabled(true);
                    gameplay_song.call(
                        "set_parameter",
                        &["MuffledOrNot".to_variant(), 1.0.to_variant()],
                    );
                }
            }

            if self.is_paused {
                if self.Settings.bind().hurricane_mode {
                    if gameplay_song
                        .call("get_parameter", &["WhichGameplaySong".to_variant()])
                        .try_to::<GString>()
                        .unwrap()
                        != "Hurricane".to_godot()
                    {
                        gameplay_song.call("stop", &[]);
                        gameplay_song.call(
                            "set_parameter",
                            &["WhichGameplaySong".to_variant(), "Hurricane".to_variant()],
                        ); // let's NEVER DO THIS MUCH INDENTATION holy fuck
                        gameplay_song.call("play", &[false.to_variant()]);
                    }
                }
                else if gameplay_song
                    .call("get_parameter", &["WhichGameplaySong".to_variant()])
                    .try_to::<GString>()
                    .unwrap()
                    != "Normal".to_godot()
                {
                    gameplay_song.call("stop", &[]);
                    gameplay_song.call(
                        "set_parameter",
                        &["WhichGameplaySong".to_variant(), "Normal".to_variant()],
                    );
                    gameplay_song.call("play", &[false.to_variant()]);
                }
                return;
            }

            if !self.is_game_over {
                gameplay_song.call("play", &[false.to_variant()]);
            }
        }

        if input.is_action_just_pressed("shoot")
            && self.player.bind().alive
            && self.player.bind().shoot_timer.get_time_left() == 0.0
        {
            let mut new_laser = self.laser.instantiate().unwrap().cast::<CharacterBody2D>();
            new_laser.set_global_position(
                self.player
                    .get_node_as::<Marker2D>("ShootOrigin")
                    .get_global_position(),
            );
            let mut lasers_node = self.base().get_node_as::<Node>("Lasers");
            lasers_node.add_child(&new_laser);
            self.player.bind_mut().shoot_timer.start();
        }

        self.score_label
            .set_text(&format!("Score: {}", self.score).to_godot());

        if self.is_game_over
            && !self.game_over_buttons_animation.is_playing()
            && input.is_action_just_pressed("skip_animation")
        {
            let game_over_text_modulate = self.game_over_text.get_modulate();
            let try_again_button_modulate = self.try_again_button.get_modulate();
            let main_menu_button_modulate = self.main_menu_button.get_modulate();

            self.game_over_anim_skipped = true;
            self.ui_animation.stop();
            self.try_again_button.set_visible(true);
            self.main_menu_button.set_visible(true);
            self.game_over_text.set_modulate(Color {
                r: game_over_text_modulate.r,
                g: game_over_text_modulate.g,
                b: game_over_text_modulate.b,
                a: 1.0,
            });
            self.try_again_button.set_modulate(Color {
                r: try_again_button_modulate.r,
                g: try_again_button_modulate.g,
                b: try_again_button_modulate.b,
                a: 1.0,
            });
            self.main_menu_button.set_modulate(Color {
                r: main_menu_button_modulate.r,
                g: main_menu_button_modulate.g,
                b: main_menu_button_modulate.b,
                a: 1.0,
            });
            self.game_over_label_animation.play_ex().name("idle").done();
            self.game_over_buttons_animation
                .play_ex()
                .name("idle")
                .done();
        }

        if self.Settings.bind().hurricane_mode {
            self.camera_manager.bind_mut().screen_shake(3.5, 0.2);
        }
    }
}

#[godot_api]
impl Main {
    #[signal]
    pub fn game_over();

    #[signal]
    fn asteroid_hit(asteroid_size: AsteroidSize, ast_position: Vector2);

    #[func]
    fn pop_in_buttons(&mut self) {
        if self.game_over_anim_skipped {
            return;
        }

        self.ui_animation
            .play_ex()
            .name("pop_in_game_over_buttons")
            .done();

        // also pay animation for label moving around
        self.game_over_label_animation.play_ex().name("idle").done();
    }

    #[func]
    fn button_idle_animation(&mut self) {
        self.game_over_buttons_animation
            .play_ex()
            .name("idle")
            .done();
    }

    // signal connection
    #[func]
    fn _spawn_asteroid(&mut self, asteroid_size: AsteroidSize, ast_position: Vector2) {
        let mut asteroids_node = self.base_mut().get_node_as::<Node>("Asteroids");
        match asteroid_size {
            AsteroidSize::Big => {
                let mut new_ast1 = self
                    .medium_ast_scene
                    .instantiate()
                    .unwrap()
                    .cast::<MediumAsteroid>();
                let mut new_ast2 = self
                    .medium_ast_scene
                    .instantiate()
                    .unwrap()
                    .cast::<MediumAsteroid>();
                // asteroids_node.call_deferred("add_child", &[new_ast1.to_variant()]);
                // asteroids_node.call_deferred("add_child", &[new_ast2.to_variant()]);
                asteroids_node.add_child(&new_ast1);
                asteroids_node.add_child(&new_ast2);
                new_ast1.bind_mut().use_set_position = true;
                new_ast2.bind_mut().use_set_position = true;
                new_ast1.set_global_position(ast_position);
                new_ast2.set_global_position(ast_position);
            }
            AsteroidSize::Medium => {
                let mut new_ast1 = self
                    .small_ast_scene
                    .instantiate()
                    .unwrap()
                    .cast::<SmallAsteroid>();
                let mut new_ast2 = self
                    .small_ast_scene
                    .instantiate()
                    .unwrap()
                    .cast::<SmallAsteroid>();
                // asteroids_node.call_deferred("add_child", &[new_ast1.to_variant()]);
                // asteroids_node.call_deferred("add_child", &[new_ast2.to_variant()]);
                asteroids_node.add_child(&new_ast1);
                asteroids_node.add_child(&new_ast2);
                new_ast1.bind_mut().use_set_position = true;
                new_ast2.bind_mut().use_set_position = true;
                new_ast1.set_global_position(ast_position);
                new_ast2.set_global_position(ast_position);
            }
            AsteroidSize::Small => {} // nothing should happen
        }
    }

    // signal connection
    #[func]
    pub fn _on_game_over(&mut self) {
        self.is_game_over = true;
        self.game_over_text.set_visible(true);
        self.ui_animation
            .play_ex()
            .name("appear_game_over_text")
            .done();
        self.game_over_animation_timer.start();
        self.MusicManager.bind_mut().gameplay.call(
            "set_parameter",
            &["NormalGameplaySongPitch".to_variant(), (-1.0).to_variant()],
        );
        self.alive_to_dead_music_timer.start();
    }

    // signal connection
    #[func]
    fn _on_asteroid_timer_timeout(&mut self) {
        if !self.is_paused {
            let mut asteroids_node = self.base_mut().get_node_as::<Node>("Asteroids");
            let asteroid_packed_scene = self
                .asteroids
                .get(randi_range(0, 2) as usize)
                .unwrap()
                .instantiate()
                .unwrap()
                .cast::<Area2D>();

            asteroids_node.add_child(&asteroid_packed_scene);
        }
    }

    // signal connection
    #[func]
    fn _on_game_over_anim_timer_timeout(&mut self) {
        self.ui_animation
            .play_ex()
            .name("ascend_game_over_text")
            .done();
    }

    // signal connection
    #[func]
    fn _on_alive_to_dead_music_timer_timeout(&mut self) {
        self.MusicManager.bind_mut().gameplay.call(
            "set_parameter",
            &["WhichGameplaySong".to_variant(), "GameOver".to_variant()],
        );
    }

    // signal connection
    #[func]
    fn _on_resume_pressed(&mut self) {
        self.SFXManager.bind_mut().click.call("play", &[]);

        if self.Settings.bind().hurricane_mode {
            self.MusicManager.bind_mut().gameplay.call(
                "set_parameter",
                &["WhichGameplaySong".to_variant(), "Hurricane".to_variant()],
            );
        }
        else {
            self.MusicManager.bind_mut().gameplay.call(
                "set_parameter",
                &["WhichGameplaySong".to_variant(), "Normal".to_variant()],
            );
        }

        self.MusicManager.bind_mut().gameplay.call(
            "set_parameter",
            &["MuffledOrNot".to_variant(), 1.0.to_variant()],
        );

        self.pause_ui.set_visible(false);
        self.resume_button.set_disabled(true);
        self.pause_settings_button.set_disabled(true);
        self.pause_main_menu_button.set_disabled(true);
        self.is_paused = false;
    }

    // signal connection
    #[func]
    fn _on_settings_pressed(&mut self) {
        self.SFXManager.bind_mut().click.call("play", &[]);
        self.pause_label.set_visible(false);
        self.resume_button.set_visible(false);
        self.pause_settings_button.set_visible(false);
        self.pause_main_menu_button.set_visible(false);

        let buttons = self.base().get_tree().unwrap().get_nodes_in_group("button");

        for button in buttons.iter_shared() {
            let mut button = button.cast::<Button>();
            button.set_disabled(true);
        }

        self.base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .add_child(&self.settings_scene.instantiate().unwrap().cast::<Control>());
    }

    // signal connection
    #[func]
    fn _on_try_again_pressed(&mut self) {
        self.SFXManager.bind_mut().click.call("play", &[]);
        let game_scene = self.game_scene.clone();
        self.base_mut()
            .get_tree()
            .unwrap()
            .change_scene_to_packed(&game_scene);
    }

    // signal connection
    #[func]
    fn _on_main_menu_pressed(&mut self) {
        self.MusicManager.bind_mut().gameplay.call("stop", &[]);
        self.SFXManager.bind_mut().click.call("play", &[]);
        let main_menu_scene = self.main_menu_scene.clone();
        self.base_mut()
            .get_tree()
            .unwrap()
            .change_scene_to_packed(&main_menu_scene);
    }
}
