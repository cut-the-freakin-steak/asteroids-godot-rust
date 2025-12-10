// NOTE: done with this file

use godot::classes::{
    ConfigFile, DisplayServer, Engine, FileAccess, INode, Node, RefCounted, display_server,
};
use godot::global::Error;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct SettingsClass {
    base: Base<Node>,

    #[init(val = OnReady::manual())]
    pub master_bus: OnReady<Gd<RefCounted>>,

    #[init(val = OnReady::manual())]
    pub music_bus: OnReady<Gd<RefCounted>>,

    #[init(val = OnReady::manual())]
    pub sfx_bus: OnReady<Gd<RefCounted>>,

    #[init(val = "user://settings.cfg".to_godot())]
    pub settings_save_path: GString,

    #[init(val = true)]
    pub vsync_on: bool,

    #[init(val = true)]
    pub screen_shake_on: bool,

    #[init(val = false)]
    pub hurricane_mode: bool,

    #[init(val = 1.0)]
    pub master_volume: f64,

    #[init(val = 1.0)]
    pub music_volume: f64,

    #[init(val = 1.0)]
    pub sfx_volume: f64,

    #[init(val = 0)]
    pub high_score: i64,
}

#[godot_api]
impl INode for SettingsClass {
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
    }
}

impl SettingsClass {
    pub fn save_settings(&self) {
        println!("tit"); // idk

        let mut config_file = ConfigFile::new_gd();

        // settings section
        config_file.set_value("settings", "vsync_on", &self.vsync_on.to_variant());
        config_file.set_value(
            "settings",
            "screen_shake_on",
            &self.screen_shake_on.to_variant(),
        );
        config_file.set_value(
            "settings",
            "hurricane_mode",
            &self.hurricane_mode.to_variant(),
        );
        config_file.set_value(
            "settings",
            "master_volume",
            &self.master_volume.to_variant(),
        );
        config_file.set_value("settings", "music_volume", &self.music_volume.to_variant());
        config_file.set_value("settings", "sfx_volume", &self.sfx_volume.to_variant());

        // game section
        config_file.set_value("game", "high_score", &self.high_score.to_variant());

        let err = config_file.save(&self.settings_save_path);

        if err != Error::OK {
            godot_error!("failed to save settings");
        }
    }

    pub fn load_settings(&mut self) {
        if !FileAccess::file_exists(&self.settings_save_path) {
            self.save_settings();
            return;
        }

        let mut config_file = ConfigFile::new_gd();

        let err = config_file.load(&self.settings_save_path);

        if err != Error::OK {
            godot_error!("failed to load settings");
        }

        // load each setting and update variable values with a fallback value just in case
        self.vsync_on = config_file
            .get_value("settings", "vsync_on")
            .try_to::<bool>()
            .unwrap_or(true);

        self.screen_shake_on = config_file
            .get_value("settings", "screen_shake_on")
            .try_to::<bool>()
            .unwrap_or(true);

        self.hurricane_mode = config_file
            .get_value("settings", "hurricane_mode")
            .try_to::<bool>()
            .unwrap_or(false);

        self.master_volume = config_file
            .get_value("settings", "master_volume")
            .try_to::<f64>()
            .unwrap_or(1.0);

        self.music_volume = config_file
            .get_value("settings", "music_volume")
            .try_to::<f64>()
            .unwrap_or(1.0);

        self.sfx_volume = config_file
            .get_value("settings", "sfx_volume")
            .try_to::<f64>()
            .unwrap_or(1.0);

        // NOTE: this is bullshit, you have to always take it out as a float and then cast it to an
        // int
        self.high_score = config_file
            .get_value("game", "high_score")
            .try_to::<i64>()
            .unwrap_or(0);

        // any settings not seen here handle their behaviour based on a variable or function that isn't the direct variable that we set above
        match self.vsync_on {
            true => {
                DisplayServer::singleton().window_set_vsync_mode(display_server::VSyncMode::ENABLED)
            }
            false => DisplayServer::singleton()
                .window_set_vsync_mode(display_server::VSyncMode::DISABLED),
        };

        self.master_bus
            .set("volume", &self.master_volume.to_variant());

        self.music_bus
            .set("volume", &self.music_volume.to_variant());

        self.sfx_bus.set("volume", &self.sfx_volume.to_variant());
    }
}
