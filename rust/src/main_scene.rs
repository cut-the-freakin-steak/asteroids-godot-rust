use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct Main {
	base: Base<Node2D>,

	#[init(val = false)]
	pub is_paused: bool,

	#[init(val = 0)]
	pub score: i64,
}

#[godot_api]
impl INode2D for Main {}

#[godot_api]
impl Main {
	#[signal]
	fn game_over();

	#[signal]
	fn asteroid_hit(asteroid_size: GString, position: Vector2);
}
