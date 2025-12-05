// NOTE: done with this file

use godot::classes::{
    AnimationPlayer, Area2D, Button, CanvasItem, Control, IControl, Input, Label, Os,
    ResourceLoader, Timer,
};
use godot::global::randi_range;
use godot::prelude::*;

use crate::audio::music_manager::MusicManagerClass;
use crate::audio::sfx_manager::SFXManagerClass;
use crate::credits::Credits;
use crate::settings_scene::SettingsScene;
use crate::tutorial::Tutorial;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct MainMenu {
    base: Base<Control>,

    #[init(val = OnReady::manual())]
    scene_root_node: OnReady<Gd<CanvasItem>>,

    #[init(node = "UI")]
    ui_animation: OnReady<Gd<AnimationPlayer>>,

    #[init(node = "TitleIdle")]
    title_idle_animation: OnReady<Gd<AnimationPlayer>>,

    #[init(node = "Buttons")]
    button_animation: OnReady<Gd<AnimationPlayer>>,

    #[init(node = "Title")]
    title: OnReady<Gd<Label>>,

    #[init(node = "Play")]
    play_button: OnReady<Gd<Button>>,

    #[init(node = "Settings")]
    settings_button: OnReady<Gd<Button>>,

    #[init(node = "Quit")]
    quit_button: OnReady<Gd<Button>>,

    #[init(node = "Tutorial")]
    tutorial_button: OnReady<Gd<Button>>,

    #[init(node = "Credits")]
    credits_button: OnReady<Gd<Button>>,

    #[init(node = "AreYouSure")]
    are_you_sure: OnReady<Gd<Button>>,

    #[init(node = "NoQuit")]
    no_quit: OnReady<Gd<Button>>,

    #[init(node = "YesQuit")]
    yes_quit: OnReady<Gd<Button>>,

    #[init(node = "AsteroidTimer")]
    asteroid_timer: OnReady<Gd<Timer>>,

    #[allow(dead_code)] // this is literally used????
    #[init(val = ResourceLoader::singleton().load("res://scenes/main.tscn").unwrap().cast::<PackedScene>())]
    game_scene: Gd<PackedScene>,

    #[allow(dead_code)] // this is literally used????
    #[init(val = ResourceLoader::singleton().load("res://scenes/settings.tscn").unwrap().cast::<PackedScene>())]
    settings_scene: Gd<PackedScene>,

    #[allow(dead_code)] // this is literally used????
    #[init(val = ResourceLoader::singleton().load("res://scenes/tutorial.tscn").unwrap().cast::<PackedScene>())]
    tutorial_scene: Gd<PackedScene>,

    #[allow(dead_code)] // this is literally used????
    #[init(val = ResourceLoader::singleton().load("res://scenes/credits.tscn").unwrap().cast::<PackedScene>())]
    credits_scene: Gd<PackedScene>,

    #[init(val = ResourceLoader::singleton().load("res://scenes/asteroid-small.tscn").unwrap().cast::<PackedScene>())]
    small_ast_scene: Gd<PackedScene>,

    #[init(val = ResourceLoader::singleton().load("res://scenes/asteroid-medium.tscn").unwrap().cast::<PackedScene>())]
    medium_ast_scene: Gd<PackedScene>,

    #[init(val = ResourceLoader::singleton().load("res://scenes/asteroid-big.tscn").unwrap().cast::<PackedScene>())]
    big_ast_scene: Gd<PackedScene>,

    #[init(val = OnReady::manual())]
    asteroids: OnReady<Array<Gd<PackedScene>>>, // array of PackedScenes

    // singletons
    #[allow(non_snake_case)]
    #[init(node = "/root/MusicManager")]
    MusicManager: OnReady<Gd<MusicManagerClass>>,

    #[allow(non_snake_case)]
    #[init(node = "/root/SFXManager")]
    SFXManager: OnReady<Gd<SFXManagerClass>>,
}

#[godot_api]
impl IControl for MainMenu {
    fn ready(&mut self) {
        self.asteroids.init(array![
            &self.small_ast_scene,
            &self.medium_ast_scene,
            &self.big_ast_scene,
        ]);

        let current_scene = self
            .base()
            .get_tree()
            .unwrap()
            .get_current_scene()
            .unwrap()
            .cast::<CanvasItem>();
        self.scene_root_node.init(current_scene);

        self.MusicManager.bind_mut().title_theme.call("play", &[]);

        // signal stuff
        self.asteroid_timer
            .signals()
            .timeout()
            .connect_other(self, Self::_on_asteroid_timer_timeout);

        self.play_button
            .signals()
            .pressed()
            .connect_other(self, Self::_on_play_pressed);

        self.settings_button
            .signals()
            .pressed()
            .connect_other(self, Self::_on_settings_pressed);

        self.quit_button
            .signals()
            .pressed()
            .connect_other(self, Self::_on_quit_pressed);

        self.are_you_sure
            .signals()
            .pressed()
            .connect_other(self, Self::_on_are_you_sure_pressed);

        self.no_quit
            .signals()
            .pressed()
            .connect_other(self, Self::_on_no_quit_pressed);

        self.yes_quit
            .signals()
            .pressed()
            .connect_other(self, Self::_on_yes_quit_pressed);

        self.tutorial_button
            .signals()
            .pressed()
            .connect_other(self, Self::_on_tutorial_pressed);

        self.credits_button
            .signals()
            .pressed()
            .connect_other(self, Self::_on_credits_pressed);
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();

        if !self.button_animation.is_playing()
            && (input.is_action_just_pressed("shoot")
                || input.is_action_just_pressed("skip_animation"))
        {
            self.ui_animation.stop();
            self.play_button.set_visible(true);
            self.settings_button.set_visible(true);
            self.quit_button.set_visible(true);
            self.tutorial_button.set_visible(true);
            self.credits_button.set_visible(true);

            let title_modulate = self.title.get_modulate();
            let play_button_modulate = self.play_button.get_modulate();
            let settings_button_modulate = self.settings_button.get_modulate();
            let quit_button_modulate = self.quit_button.get_modulate();
            let tutorial_button_modulate = self.tutorial_button.get_modulate();
            let credits_button_modulate = self.credits_button.get_modulate();

            self.title.set_modulate(Color {
                r: title_modulate.r,
                g: title_modulate.g,
                b: title_modulate.b,
                a: 1.0,
            });
            self.play_button.set_modulate(Color {
                r: play_button_modulate.r,
                g: play_button_modulate.g,
                b: play_button_modulate.b,
                a: 1.0,
            });
            self.settings_button.set_modulate(Color {
                r: settings_button_modulate.r,
                g: settings_button_modulate.g,
                b: settings_button_modulate.b,
                a: 1.0,
            });
            self.quit_button.set_modulate(Color {
                r: quit_button_modulate.r,
                g: quit_button_modulate.g,
                b: quit_button_modulate.b,
                a: 1.0,
            });
            self.tutorial_button.set_modulate(Color {
                r: tutorial_button_modulate.r,
                g: tutorial_button_modulate.g,
                b: tutorial_button_modulate.b,
                a: 1.0,
            });
            self.credits_button.set_modulate(Color {
                r: credits_button_modulate.r,
                g: credits_button_modulate.g,
                b: credits_button_modulate.b,
                a: 1.0,
            });

            self.title_idle_animation.play_ex().name("idle").done();
            self.button_animation.play_ex().name("idle").done();
        }
    }
}

#[godot_api]
impl MainMenu {
    // NOTE: i coded the following 3 functions for the original project but i guess i never used
    // them??? so i guess ill just translate them and never use them here too.

    #[func]
    pub fn start_animations(&mut self) {
        self.ui_animation.play_ex().name("ascend_title").done();
        if !self.play_button.is_visible() {
            self.ui_animation.queue("pop_in_buttons");
        }
    }

    #[func]
    pub fn start_title_idle(&mut self) {
        self.title_idle_animation.play_ex().name("idle").done();
    }

    #[func]
    pub fn start_button_idle(&mut self) {
        self.button_animation.play_ex().name("idle").done();
    }

    #[func]
    fn _on_asteroid_timer_timeout(&mut self) {
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

    #[func]
    fn _on_play_pressed(&mut self) {
        self.MusicManager.bind_mut().title_theme.call("stop", &[]);
        self.SFXManager.bind_mut().click.call("play", &[]);
        let mut scene_tree = self.base_mut().get_tree().unwrap();
        let game_scene = &self.game_scene;
        scene_tree.change_scene_to_packed(game_scene);
    }

    #[func]
    fn _on_settings_pressed(&mut self) {
        self.SFXManager.bind_mut().click.call("play", &[]);

        // FIX: clanker told me to do it like this but idk man sus
        self.disappear_node(self.scene_root_node.clone());

        self.base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .add_child(
                &self
                    .settings_scene
                    .instantiate()
                    .unwrap()
                    .cast::<SettingsScene>(),
            );
    }

    #[func]
    fn _on_quit_pressed(&mut self) {
        self.SFXManager.bind_mut().click.call("play", &[]);
        self.title.set_visible(false);
        self.play_button.set_visible(false);
        self.play_button.set_disabled(true);
        self.settings_button.set_visible(false);
        self.settings_button.set_disabled(true);
        self.quit_button.set_visible(false);
        self.quit_button.set_disabled(true);
        self.tutorial_button.set_visible(false);
        self.tutorial_button.set_disabled(true);
        self.credits_button.set_visible(false);
        self.credits_button.set_disabled(true);

        self.are_you_sure.set_visible(true);
        self.are_you_sure.set_disabled(false);
        self.yes_quit.set_visible(true);
        self.yes_quit.set_disabled(false);
        self.no_quit.set_visible(true);
        self.no_quit.set_disabled(false);
    }

    #[func]
    fn _on_are_you_sure_pressed(&mut self) {
        self.SFXManager.bind_mut().click.call("play", &[]);
        let mut os = Os::singleton();
        os.shell_open("https://i.ytimg.com/vi/YSWMYnuOImg/hqdefault.jpg");
    }

    #[func]
    fn _on_no_quit_pressed(&mut self) {
        self.SFXManager.bind_mut().click.call("play", &[]);
        self.are_you_sure.set_visible(false);
        self.are_you_sure.set_disabled(true);
        self.yes_quit.set_visible(false);
        self.yes_quit.set_disabled(true);
        self.no_quit.set_visible(false);
        self.no_quit.set_disabled(true);

        self.title.set_visible(true);
        self.play_button.set_visible(true);
        self.play_button.set_disabled(false);
        self.settings_button.set_visible(true);
        self.settings_button.set_disabled(false);
        self.quit_button.set_visible(true);
        self.quit_button.set_disabled(false);
        self.tutorial_button.set_visible(true);
        self.tutorial_button.set_disabled(false);
        self.credits_button.set_visible(true);
        self.credits_button.set_disabled(false);
    }

    #[func]
    fn _on_yes_quit_pressed(&mut self) {
        self.SFXManager.bind_mut().click.call("play", &[]);
        self.base_mut().get_tree().unwrap().quit();
    }

    #[func]
    fn _on_tutorial_pressed(&mut self) {
        self.SFXManager.bind_mut().click.call("play", &[]);

        self.disappear_node(self.scene_root_node.clone());

        self.base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .add_child(
                &self
                    .tutorial_scene
                    .instantiate()
                    .unwrap()
                    .cast::<Tutorial>(),
            );
    }

    #[func]
    fn _on_credits_pressed(&mut self) {
        self.SFXManager.bind_mut().click.call("play", &[]);

        self.disappear_node(self.scene_root_node.clone());

        self.base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .add_child(&self.credits_scene.instantiate().unwrap().cast::<Credits>());
    }

    // NOTE: since #[func] makes simple functions aware to godot and gdscript, generics are not
    // supported
    //
    // #[func]
    #[allow(unused_mut)]
    fn disappear_node<T: Inherits<CanvasItem>>(&self, mut node: Gd<T>) {
        let mut node = node.upcast::<CanvasItem>();
        node.set_visible(false);

        let buttons = self.base().get_tree().unwrap().get_nodes_in_group("button");
        for button in buttons.iter_shared() {
            let mut button = button.cast::<Button>();
            button.set_disabled(true);
        }
    }

    #[allow(dead_code)]
    #[allow(unused_mut)]
    fn appear_node<T: Inherits<CanvasItem>>(&self, mut node: Gd<T>) {
        let mut node = node.upcast::<CanvasItem>();
        node.set_visible(true);

        let buttons = self.base().get_tree().unwrap().get_nodes_in_group("button");
        for button in buttons.iter_shared() {
            let mut button = button.cast::<Button>();
            button.set_disabled(false);
        }
    }
}
