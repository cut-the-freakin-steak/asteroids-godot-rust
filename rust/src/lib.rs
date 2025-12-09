use godot::prelude::*;

mod asteroids;
mod audio;
mod bullet;
mod camera_manager;
mod credits;
mod fmod_bindings;
mod global_settings;
mod main_menu;
mod main_scene;
mod player;
mod settings_scene;
mod splash_screen;
mod tutorial;
mod window_manager;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
