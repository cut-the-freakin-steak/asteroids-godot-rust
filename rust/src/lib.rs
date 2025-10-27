use godot::prelude::*;

mod asteroid;
mod asteroid_small;
mod main_scene;
mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
