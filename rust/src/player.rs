use godot::classes::CharacterBody2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Player {
    #[signal]
    pub fn damage_taken();
}
