use godot::prelude::*;

mod asteroids;
mod audio;
mod camera_manager;
mod global_settings;
mod main_scene;
mod player;
mod settings_scene;
mod window_manager;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
