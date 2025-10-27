use godot::classes::Area2D;
use godot::prelude::*;

use crate::asteroid::Asteroid;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
struct SmallAsteroid {
    base: Base<Area2D>,

    #[init(val = OnReady::manual())]
    asteroid_base: OnReady<Gd<Asteroid>>,
}
