use godot::prelude::*;
mod player_example;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
