use godot::prelude::*;

mod asteroid;
mod asteroid_big;
mod asteroid_medium;
mod asteroid_small;
mod camera_manager;
mod main_scene;
mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
