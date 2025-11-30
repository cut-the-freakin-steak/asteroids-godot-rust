// NOTE: done with this file

use godot::classes::{
    DisplayServer, Engine, FileAccess, INode, Json, Node, RefCounted, display_server, file_access,
};
use godot::global::{Error, printerr};
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

    #[init(val = "user://settings.json".to_godot())]
    pub settings_save_path: GString,

    #[init(val = true)]
    pub vsync_on: bool,

    #[init(val = true)]
    pub screen_shake_on: bool,

    #[init(val = false)]
    pub hurricane_mode: bool,

    #[init(val = 1.0)]
    pub master_volume: f32,

    #[init(val = 1.0)]
    pub music_volume: f32,

    #[init(val = 1.0)]
    pub sfx_volume: f32,
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
    pub fn save_settings(&mut self) {
        println!("tit"); // idk

        let settings_dict = vdict! {
            "vsync_on": self.vsync_on,
            "screen_shake_on": self.screen_shake_on,
            "hurricane_mode": self.hurricane_mode,
            "master_volume": self.master_volume,
            "music_volume": self.music_volume,
            "sfx_volume": self.sfx_volume
        };

        let json_string = Json::stringify(&settings_dict.to_variant());
        let file_opt = FileAccess::open(&self.settings_save_path, file_access::ModeFlags::WRITE);
        let mut file;

        if let Some(returned_file) = file_opt {
            file = returned_file
        }
        else {
            printerr(&["file failed to load lmao".to_variant()]);
            return;
        }

        file.store_string(&json_string);
        file.close();
    }

    pub fn load_settings(&mut self) {
        if !FileAccess::file_exists(&self.settings_save_path) {
            return;
        }

        let file_opt = FileAccess::open(&self.settings_save_path, file_access::ModeFlags::READ);
        let mut file;

        if let Some(returned_file) = file_opt {
            file = returned_file
        }
        else {
            printerr(&["file failed to load lmao".to_variant()]);
            return;
        }

        let json_string = file.get_as_text();
        file.close();

        let mut json = Json::new_gd();

        // parse json and check result to make sure settings aren't corrupt
        let parsed_results = json.parse(&json_string);

        if parsed_results != Error::OK {
            printerr(&["failed to parse json".to_variant()]);
            return;
        }

        let settings_dict = json.get_data().try_to::<Dictionary>().unwrap();

        // load each setting and update variable values with a fallback value just in case
        self.vsync_on = settings_dict
            .get("vsync_on")
            .unwrap()
            .try_to::<bool>()
            .unwrap();

        self.screen_shake_on = settings_dict
            .get("screen_shake_on")
            .unwrap()
            .try_to::<bool>()
            .unwrap();

        self.hurricane_mode = settings_dict
            .get("hurricane_mode")
            .unwrap()
            .try_to::<bool>()
            .unwrap();

        self.master_volume = settings_dict
            .get("master_volume")
            .unwrap()
            .try_to::<f32>()
            .unwrap();

        self.music_volume = settings_dict
            .get("music_volume")
            .unwrap()
            .try_to::<f32>()
            .unwrap();

        self.sfx_volume = settings_dict
            .get("sfx_volume")
            .unwrap()
            .try_to::<f32>()
            .unwrap();

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
