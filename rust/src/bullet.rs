use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
struct Bullet {
    base: Base<CharacterBody2D>,
}
