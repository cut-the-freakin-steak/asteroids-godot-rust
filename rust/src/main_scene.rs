use crate::audio::music_manager::MusicManagerClass;
use crate::global_settings::SettingsClass;
use crate::player::Player;
use godot::classes::{
    AnimationPlayer, Button, Control, INode2D, Input, Label, Node, Node2D, ResourceLoader, Timer,
};
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
    camera_manager: OnReady<Gd<Node>>,

    #[init(node = "UI/PauseUI")]
    pause_ui: OnReady<Gd<Control>>,

    #[init(node = "UI/PauseUI/Pause")]
    pause_label: OnReady<Gd<Label>>,

    #[init(node = "UI/PauseUI/Resume")]
    resume_button: OnReady<Gd<Button>>,

    #[init(node = "UI/PauseUI/Settings")]
    pause_settings_button: OnReady<Gd<Button>>,

    #[init(node = "UI/PauseUI/MainMenu")]
    pause_main_menu_button: OnReady<Gd<Button>>,

    #[init(node = "AliveToDeadMusicTimer")]
    alive_to_dead_music_timer: OnReady<Gd<Timer>>,

    // scenes
    #[init(val = OnReady::manual())]
    main_menu_scene: OnReady<Gd<PackedScene>>,

    #[init(val = OnReady::manual())]
    settings_scene: OnReady<Gd<PackedScene>>,

    #[init(val = OnReady::manual())]
    game_scene: OnReady<Gd<PackedScene>>,

    #[init(val = OnReady::manual())]
    small_ast_scene: OnReady<Gd<PackedScene>>,

    #[init(val = OnReady::manual())]
    medium_ast_scene: OnReady<Gd<PackedScene>>,

    #[init(val = OnReady::manual())]
    big_ast_scene: OnReady<Gd<PackedScene>>,

    #[init(val = OnReady::manual())]
    laser: OnReady<Gd<PackedScene>>,

    #[init(val = OnReady::manual())]
    asteroids: OnReady<VariantArray>, // array of PackedScenes

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

            self.small_ast_scene.init(
                loader
                    .load("res://scenes/asteroid-small.tscn")
                    .unwrap()
                    .cast::<PackedScene>(),
            );

            self.medium_ast_scene.init(
                loader
                    .load("res://scenes/asteroid-medium.tscn")
                    .unwrap()
                    .cast::<PackedScene>(),
            );

            self.big_ast_scene.init(
                loader
                    .load("res://scenes/asteroid-big.tscn")
                    .unwrap()
                    .cast::<PackedScene>(),
            );

            self.laser.init(
                loader
                    .load("res://scenes/bullet.tscn")
                    .unwrap()
                    .cast::<PackedScene>(),
            );

            self.asteroids.init(varray![
                self.small_ast_scene,
                self.medium_ast_scene,
                self.big_ast_scene
            ]);
        }

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
        self.signals().game_over().connect_self(Self::_on_game_over);
        self.signals()
            .asteroid_hit()
            .connect_self(Self::_spawn_asteroid);
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();
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
                    );
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

        if input.is_action_just_pressed("shoot") && self.player.bind().alive {}

        // if Input.is_action_just_pressed("shoot") and player.alive and player.shoot_timer.time_left == 0:
        //     var new_laser: CharacterBody2D = laser.instantiate()
        //     new_laser.global_position = player.get_node("ShootOrigin").global_position
        //     $Lasers.add_child(new_laser)
        //     player.shoot_timer.start()
        //
        // score_label.text = "Score: " + str(score)
        //
        // if is_game_over and not game_over_buttons_animation.is_playing() and Input.is_action_just_pressed("skip_animation"):
        //     game_over_anim_skipped = true
        //     ui_animation.stop()
        //     try_again_button.visible = true
        //     main_menu_button.visible = true
        //     game_over_text.modulate.a = 1.0
        //     try_again_button.modulate.a = 1.0
        //     main_menu_button.modulate.a = 1.0
        //     game_over_label_animation.play("idle")
        //     game_over_buttons_animation.play("idle")
        //
        // if Settings.hurricane_mode:
        //     camera_manager.screen_shake(3.5, 0.2)
    }
}

#[godot_api]
impl Main {
    #[signal]
    pub fn game_over();

    #[signal]
    fn asteroid_hit(asteroid_size: GString, position: Vector2);

    // signal connection
    #[func]
    pub fn _on_game_over(&mut self) {}

    // signal connection
    #[func]
    fn _spawn_asteroid(&mut self, asteroid_size: GString, position: Vector2) {}
}
