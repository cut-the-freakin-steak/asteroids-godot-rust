use godot::classes::{CheckButton, Control, DisplayServer, Engine, HSlider, IControl, Node2D};
use godot::prelude::*;

use crate::global_settings::SettingsClass;
use crate::main_scene::Main;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct SettingsScene {
    base: Base<Control>,

    #[init(val = OnReady::manual())]
    main_menu_node: OnReady<Gd<Control>>,

    #[init(val = OnReady::manual())]
    game_scene_node: OnReady<Gd<Main>>,

    #[init(node = ".")]
    scene_root_node: OnReady<Gd<Control>>,

    #[init(node = "NodePath")]
    vsync_toggle: OnReady<Gd<CheckButton>>,

    #[init(node = "NodePath")]
    screen_shake_toggle: OnReady<Gd<CheckButton>>,

    #[init(node = "NodePath")]
    hurricane_mode_toggle: OnReady<Gd<CheckButton>>,

    #[init(node = "NodePath")]
    master_volume_slider: OnReady<Gd<HSlider>>,

    #[init(node = "NodePath")]
    music_volume_slider: OnReady<Gd<HSlider>>,

    #[init(node = "NodePath")]
    sfx_volume_slider: OnReady<Gd<HSlider>>,

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
}

#[godot_api]
impl IControl for SettingsScene {
    fn process(&mut self, _delta: f64) {
        // if Input.is_action_just_pressed("esc"):
        //     exit_settings()
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

        // FIX: ts is not correct, whenever you make main_scene.rs, change this control node to be
        // a MainScene node
        let main_menu_node_init = self.base().get_node_as::<Control>("/root/MainMenu");
        self.main_menu_node.init(main_menu_node_init);

        let game_scene_node_init = self.base().get_node_as::<Main>("/root/Main");
        self.game_scene_node.init(game_scene_node_init);

        // match Settings.vsync_on:
        //     true:
        //         DisplayServer.window_set_vsync_mode(DisplayServer.VSYNC_ENABLED)
        //         vsync_toggle.button_pressed = true
        //
        //     false:
        //         DisplayServer.window_set_vsync_mode(DisplayServer.VSYNC_DISABLED)
        //         vsync_toggle.button_pressed = false
        //
        // match Settings.screen_shake_on:
        //     true:
        //         screen_shake_toggle.button_pressed = true
        //
        //     false:
        //         screen_shake_toggle.button_pressed = false
        //
        // match Settings.hurricane_mode:
        //     true:
        //         hurricane_mode_toggle.button_pressed = true
        //
        //     false:
        //         hurricane_mode_toggle.button_pressed = false
        //
        // master_bus.volume = Settings.master_volume
        // master_volume_slider.value = Settings.master_volume
        //
        // music_bus.volume = Settings.music_volume
        // music_volume_slider.value = Settings.music_volume
        //
        // sfx_bus.volume = Settings.sfx_volume
        // sfx_volume_slider.value = Settings.sfx_volume
    }
}

#[godot_api]
impl SettingsScene {
    // #[func]
    pub fn exit_settings(&mut self) {
        // Settings.save_settings()
        //
        // dissapear_node(scene_root_node)
        //
        // if main_menu_node != null:
        //     appear_node(main_menu_node)
        //
        // if game_scene_node != null:
        //     # appear_root_node() wouldnt work here because it doesnt have the following 4 lines of code
        //     game_scene_node.pause_label.visible = true
        //     game_scene_node.resume_button.visible = true
        //     game_scene_node.pause_settings_button.visible = true
        //     game_scene_node.pause_main_menu_button.visible = true
        //
        //     var game_scene_buttons = game_scene_node.get_tree().get_nodes_in_group("button")
        //     for button in game_scene_buttons:
        //         button.disabled = false
        //
        // queue_free()
    }

    // NOTE: miscellaneous settings
    // #[func]
    fn _on_return_pressed(&self) {
        // SFXManager.click.play()
        // exit_settings()
    }

    // #[func]
    fn _on_v_sync_toggle_toggled(&self, toggled_on: bool) {
        // SFXManager.click.play()
        //
        // if toggled_on:
        //     DisplayServer.window_set_vsync_mode(DisplayServer.VSYNC_ENABLED)
        //     Settings.vsync_on = true
        //
        // else:
        //     DisplayServer.window_set_vsync_mode(DisplayServer.VSYNC_DISABLED)
        //     Settings.vsync_on = false
        //
        // Settings.save_settings()
    }

    // #[func]
    fn _on_screen_shake_toggle_toggled(&self, toggled_on: bool) {
        // SFXManager.click.play()
        //
        // if toggled_on:
        //     Settings.screen_shake_on = true
        //
        // else:
        //     Settings.screen_shake_on = false
        //
        // Settings.save_settings()
    }

    // #[func]
    fn _on_hurricane_mode_toggle_toggled(&self, toggled_on: bool) {
        // SFXManager.click.play()
        //
        // if toggled_on:
        //     Settings.hurricane_mode = true
        //
        // else:
        //     Settings.hurricane_mode = false
        //
        // Settings.save_settings()
    }

    // NOTE: audio settings
    // #[func]
    fn _on_master_slider_value_changed(&self, value: f32) {
        // Settings.master_volume = value
        // master_bus.volume = Settings.master_volume
    }

    // #[func]
    fn _on_music_slider_value_changed(&self, value: f32) {
        // Settings.music_volume = value
        // music_bus.volume = Settings.music_volume
    }

    // #[func]
    fn _on_sfx_slider_value_changed(&self, value: f32) {
        // Settings.sfx_volume = value
        // sfx_bus.volume = Settings.sfx_volume
    }

    // #[func]
    fn disappear_node(&self, node: Node) {
        // node.visible = false
        //
        // var buttons = node.get_tree().get_nodes_in_group("button")
        // for button in buttons:
        //     button.disabled = true
    }

    // #[func]
    fn appear_node(&self, node: Node) {
        // node.visible = true
        //
        // var buttons = node.get_tree().get_nodes_in_group("button")
        // for button in buttons:
        //     button.disabled = false
    }
}
