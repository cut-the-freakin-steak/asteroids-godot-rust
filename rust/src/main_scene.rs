use crate::global_settings::SettingsClass;
use godot::classes::{
    AnimationPlayer, Button, CharacterBody2D, Control, INode2D, Label, Node, Node2D,
    ResourceLoader, Timer,
};
use godot::prelude::*;

// good lord what a mess.
#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct Main {
    base: Base<Node2D>,

    // nodes
    #[init(node = "PlayerStuff/Player")]
    player: OnReady<Gd<CharacterBody2D>>,

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
    is_game_over: bool,

    #[init(val = false)]
    game_over_anim_skipped: bool,

    #[init(val = false)]
    pub is_paused: bool,

    #[init(val = 0)]
    pub score: i64,

    // singletons
    #[allow(non_snake_case)]
    #[init(node = "/root/Settings")]
    Settings: OnReady<Gd<SettingsClass>>,
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

        // if Settings.hurricane_mode:
        //     MusicManager.gameplay.set_parameter("WhichGameplaySong", "Hurricane")
        //
        // else:
        //     MusicManager.gameplay.set_parameter("WhichGameplaySong", "Normal")
        //
        // MusicManager.gameplay.set_parameter("MuffledOrNot", 1.0)
        // MusicManager.gameplay.set_parameter("NormalGameplaySongPitch", 0.0)
        //
        // score = 0
        // is_game_over = false
        // game_over_anim_skipped = false
        // game_over.connect(_on_game_over)
        // asteroid_hit.connect(_spawn_asteroid)
    }

    fn physics_process(&mut self, delta: f64) {}
}

#[godot_api]
impl Main {
    #[signal]
    fn game_over();

    #[signal]
    fn asteroid_hit(asteroid_size: GString, position: Vector2);
}
