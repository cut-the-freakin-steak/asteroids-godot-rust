// NOTE: done with this file

use godot::classes::{
    Button, CanvasItem, CheckButton, Control, DisplayServer, Engine, HSlider, IControl, Input,
    display_server,
};
use godot::prelude::*;

use crate::audio::sfx_manager;
use crate::global_settings::SettingsClass;
use crate::main_menu::MainMenu;
use crate::main_scene::Main;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct SettingsScene {
    base: Base<Control>,

    #[init(val = OnReady::manual())]
    main_menu_node_opt: OnReady<Option<Gd<MainMenu>>>,

    #[init(val = OnReady::manual())]
    game_scene_node_opt: OnReady<Option<Gd<Main>>>,

    #[init(val = OnReady::manual())]
    scene_root_node: OnReady<Gd<CanvasItem>>,

    #[init(node = "VSync/VSyncToggle")]
    vsync_toggle: OnReady<Gd<CheckButton>>,

    #[init(node = "ScreenShake/ScreenShakeToggle")]
    screen_shake_toggle: OnReady<Gd<CheckButton>>,

    #[init(node = "HurricaneMode/HurricaneModeToggle")]
    hurricane_mode_toggle: OnReady<Gd<CheckButton>>,

    #[init(node = "Audio/Master/MasterSlider")]
    master_volume_slider: OnReady<Gd<HSlider>>,

    #[init(node = "Audio/Music/MusicSlider")]
    music_volume_slider: OnReady<Gd<HSlider>>,

    #[init(node = "Audio/SFX/SFXSlider")]
    sfx_volume_slider: OnReady<Gd<HSlider>>,

    #[init(node = "Return")]
    return_button: OnReady<Gd<Button>>,

    #[init(val = OnReady::manual())]
    master_bus: OnReady<Gd<RefCounted>>,

    #[init(val = OnReady::manual())]
    music_bus: OnReady<Gd<RefCounted>>,

    #[init(val = OnReady::manual())]
    sfx_bus: OnReady<Gd<RefCounted>>,

    // singletons
    #[allow(non_snake_case)]
    #[init(node = "/root/Settings")]
    Settings: OnReady<Gd<SettingsClass>>,

    #[allow(non_snake_case)]
    #[init(node = "/root/SFXManager")]
    SFXManager: OnReady<Gd<sfx_manager::SFXManagerClass>>,
}

#[godot_api]
impl IControl for SettingsScene {
    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();
        if input.is_action_just_pressed("esc") {
            self.exit_settings();
        }
    }

    fn ready(&mut self) {
        let engine = Engine::singleton();

        // get FmodServer singleton
        if engine.has_singleton("FmodServer") {
            #[allow(non_snake_case)]
            let mut FmodServer = engine.get_singleton("FmodServer").unwrap();

            self.master_bus.init(
                FmodServer
                    .call("get_bus", &["bus:/".to_variant()])
                    .try_to()
                    .unwrap(),
            );

            self.music_bus.init(
                FmodServer
                    .call("get_bus", &["bus:/Music".to_variant()])
                    .try_to()
                    .unwrap(),
            );

            self.sfx_bus.init(
                FmodServer
                    .call("get_bus", &["bus:/SFX".to_variant()])
                    .try_to()
                    .unwrap(),
            );
        }

        self.scene_root_node.init(
            self.base()
                .get_tree()
                .unwrap()
                .get_current_scene()
                .unwrap()
                .cast::<CanvasItem>(),
        );

        let main_menu_node_init = self.base().try_get_node_as::<MainMenu>("/root/MainMenu");

        self.main_menu_node_opt.init(main_menu_node_init);

        let game_scene_node_init = self.base().try_get_node_as::<Main>("/root/Main");
        self.game_scene_node_opt.init(game_scene_node_init);

        use display_server as ds;
        if self.Settings.bind().vsync_on {
            DisplayServer::singleton().window_set_vsync_mode(ds::VSyncMode::ENABLED);
            self.vsync_toggle.set_pressed(true);
        }
        else {
            DisplayServer::singleton().window_set_vsync_mode(ds::VSyncMode::DISABLED);
            self.vsync_toggle.set_pressed(false);
        }

        if self.Settings.bind().screen_shake_on {
            self.screen_shake_toggle.set_pressed(true);
        }
        else {
            self.screen_shake_toggle.set_pressed(false);
        }

        if self.Settings.bind().hurricane_mode {
            self.hurricane_mode_toggle.set_pressed(true);
        }
        else {
            self.hurricane_mode_toggle.set_pressed(false);
        }

        self.master_bus
            .set("volume", &self.Settings.bind().master_volume.to_variant());
        self.master_volume_slider
            .set_value(self.Settings.bind().master_volume);

        self.music_bus
            .set("volume", &self.Settings.bind().music_volume.to_variant());
        self.music_volume_slider
            .set_value(self.Settings.bind().music_volume);

        self.sfx_bus
            .set("volume", &self.Settings.bind().sfx_volume.to_variant());
        self.sfx_volume_slider
            .set_value(self.Settings.bind().sfx_volume);

        // TODO: this
        //
        // signal connections:
        self.return_button
            .signals()
            .pressed()
            .connect_other(self, Self::_on_return_pressed);

        self.vsync_toggle
            .signals()
            .toggled()
            .connect_other(self, Self::_on_v_sync_toggle_toggled);

        self.screen_shake_toggle
            .signals()
            .toggled()
            .connect_other(self, Self::_on_screen_shake_toggle_toggled);

        self.hurricane_mode_toggle
            .signals()
            .toggled()
            .connect_other(self, Self::_on_hurricane_mode_toggle_toggled);

        self.master_volume_slider
            .signals()
            .value_changed()
            .connect_other(self, Self::_on_master_slider_value_changed);

        self.music_volume_slider
            .signals()
            .value_changed()
            .connect_other(self, Self::_on_music_slider_value_changed);

        self.sfx_volume_slider
            .signals()
            .value_changed()
            .connect_other(self, Self::_on_sfx_slider_value_changed);
    }
}

#[godot_api]
impl SettingsScene {
    #[func]
    pub fn exit_settings(&mut self) {
        self.Settings.bind_mut().save_settings();
        self.disappear_node(self.scene_root_node.clone());

        if let Some(main_menu_node) = self.main_menu_node_opt.clone() {
            self.appear_node(main_menu_node.clone().upcast::<CanvasItem>());
        }

        if let Some(mut game_scene_node) = self.game_scene_node_opt.clone() {
            // appear_root_node() wouldnt work here because it doesnt have the following 4 lines of code
            self.appear_node(game_scene_node.clone().upcast::<CanvasItem>());
            game_scene_node.bind_mut().pause_label.set_visible(true);
            game_scene_node.bind_mut().resume_button.set_visible(true);
            game_scene_node
                .bind_mut()
                .pause_settings_button
                .set_visible(true);
            game_scene_node
                .bind_mut()
                .pause_main_menu_button
                .set_visible(true);

            let game_scene_buttons = game_scene_node
                .get_tree()
                .unwrap()
                .get_nodes_in_group("button");
            for button in game_scene_buttons.iter_shared() {
                let mut button = button.cast::<Button>();
                button.set_disabled(false);
            }
        }

        self.base_mut().queue_free();
    }

    #[func]
    fn disappear_node(&self, mut node: Gd<CanvasItem>) {
        node.set_visible(false);

        let buttons = node.get_tree().unwrap().get_nodes_in_group("button");
        for button in buttons.iter_shared() {
            let mut button = button.cast::<Button>();
            button.set_disabled(true);
        }
    }

    #[func]
    fn appear_node(&self, mut node: Gd<CanvasItem>) {
        node.set_visible(true);

        let buttons = node.get_tree().unwrap().get_nodes_in_group("button");
        for button in buttons.iter_shared() {
            let mut button = button.cast::<Button>();
            button.set_disabled(false);
        }
    }

    // NOTE: signal connections
    #[func]
    fn _on_return_pressed(&mut self) {
        self.SFXManager.bind_mut().click.play(None);
        self.exit_settings();
    }

    #[func]
    fn _on_v_sync_toggle_toggled(&mut self, toggled_on: bool) {
        use display_server as ds;

        self.SFXManager.bind_mut().click.play(None);

        if toggled_on {
            DisplayServer::singleton().window_set_vsync_mode(ds::VSyncMode::ENABLED);
            self.Settings.bind_mut().vsync_on = true;
        }
        else {
            DisplayServer::singleton().window_set_vsync_mode(ds::VSyncMode::DISABLED);
            self.Settings.bind_mut().vsync_on = false;
        }

        self.Settings.bind_mut().save_settings();
    }

    #[func]
    fn _on_screen_shake_toggle_toggled(&mut self, toggled_on: bool) {
        self.SFXManager.bind_mut().click.play(None);

        self.Settings.bind_mut().screen_shake_on = toggled_on;

        self.Settings.bind_mut().save_settings();
    }

    #[func]
    fn _on_hurricane_mode_toggle_toggled(&mut self, toggled_on: bool) {
        self.SFXManager.bind_mut().click.play(None);

        self.Settings.bind_mut().hurricane_mode = toggled_on;

        self.Settings.bind_mut().save_settings();
    }

    // NOTE: audio signals
    #[func]
    fn _on_master_slider_value_changed(&mut self, value: f64) {
        self.Settings.bind_mut().master_volume = value;
        self.master_bus
            .set("volume", &self.Settings.bind().master_volume.to_variant());
    }

    #[func]
    fn _on_music_slider_value_changed(&mut self, value: f64) {
        self.Settings.bind_mut().music_volume = value;
        self.music_bus
            .set("volume", &self.Settings.bind().music_volume.to_variant());
    }

    #[func]
    fn _on_sfx_slider_value_changed(&mut self, value: f64) {
        self.Settings.bind_mut().sfx_volume = value;
        self.sfx_bus
            .set("volume", &self.Settings.bind().sfx_volume.to_variant());
    }
}
