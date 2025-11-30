// NOTE: done with this file

use godot::classes::{Control, IControl, ResourceLoader, TextureRect};
use godot::prelude::*;

use crate::audio::sfx_manager;
use crate::global_settings::SettingsClass;

#[derive(GodotClass)]
#[class(init, base = Control)]
struct SplashScreen {
    base: Base<Control>,

    #[init(node = "SplashScreenTexture")]
    splash_screen: OnReady<Gd<TextureRect>>,

    #[init(val = ResourceLoader::singleton().load("res://scenes/main_menu.tscn").unwrap().cast::<PackedScene>())]
    main_menu_scene: Gd<PackedScene>,

    #[init(val = 0.5)]
    in_time: f64,

    #[init(val = 1.5)]
    fade_in_time: f64,

    #[init(val = 1.5)]
    fade_out_time: f64,

    #[init(val = 1.5)]
    pause_time: f64,

    #[init(val = 1.25)]
    out_time: f64,

    // singletons
    #[allow(non_snake_case)]
    #[init(node = "/root/Settings")]
    Settings: OnReady<Gd<SettingsClass>>,

    #[allow(non_snake_case)]
    #[init(node = "/root/SFXManager")]
    SFXManager: OnReady<Gd<sfx_manager::SFXManagerClass>>,
}

#[godot_api]
impl IControl for SplashScreen {
    fn ready(&mut self) {
        self.Settings.bind_mut().load_settings();
        self.fade();
        self.SFXManager.bind_mut().splash_screen.call("play", &[]);
    }
}

#[godot_api]
impl SplashScreen {
    #[func]
    fn fade(&mut self) {
        let splash_screen_modulate = self.splash_screen.get_modulate();
        self.splash_screen.set_modulate(Color {
            r: splash_screen_modulate.r,
            g: splash_screen_modulate.g,
            b: splash_screen_modulate.b,
            a: 0.0,
        });

        let mut tween = self.base_mut().create_tween().unwrap();
        tween.tween_interval(self.in_time);
        tween.tween_property(
            &*self.splash_screen,
            "modulate:a",
            &1.0.to_variant(),
            self.fade_in_time,
        );
        tween.tween_interval(self.pause_time);
        tween.tween_property(
            &*self.splash_screen,
            "modulate:a",
            &0.0.to_variant(),
            self.fade_out_time,
        );
        tween.tween_interval(self.out_time);
        // NOTE: cant use await easily in rust with godot so we gotta do this bs
        tween
            .signals()
            .finished()
            .connect_other(self, Self::_on_tween_finished);
    }

    #[func]
    fn _on_tween_finished(&mut self) {
        self.base()
            .get_tree()
            .unwrap()
            .change_scene_to_packed(&self.main_menu_scene);
    }
}
