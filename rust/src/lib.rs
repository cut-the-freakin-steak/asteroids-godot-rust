use godot::prelude::*;
mod asteroid;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
