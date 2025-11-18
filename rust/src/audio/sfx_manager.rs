use godot::classes::Node2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct SFXManagerClass {
    base: Base<Node2D>,

    #[init(node = "Click")]
    pub click: OnReady<Gd<Node2D>>,

    #[init(node = "Explosion")]
    pub explosion: OnReady<Gd<Node2D>>,

    #[init(node = "ShipThruster")]
    pub ship_thruster: OnReady<Gd<Node2D>>,

    #[init(node = "LaserShot")]
    pub laser_shot: OnReady<Gd<Node2D>>,

    #[init(node = "PlayerHurt")]
    pub player_hurt: OnReady<Gd<Node2D>>,

    #[init(node = "SplashScreen")]
    pub splash_screen: OnReady<Gd<Node2D>>,
}
