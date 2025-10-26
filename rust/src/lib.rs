use godot::prelude::*;

mod asteroid;
mod main_scene;
mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
