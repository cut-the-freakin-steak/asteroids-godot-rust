use godot::classes::Node2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct MusicManagerClass {
    base: Base<Node2D>,

    #[init(node = "Gameplay")]
    pub gameplay: OnReady<Gd<Node2D>>,

    #[init(node = "TitleTheme")]
    pub title_theme: OnReady<Gd<Node2D>>,
}
