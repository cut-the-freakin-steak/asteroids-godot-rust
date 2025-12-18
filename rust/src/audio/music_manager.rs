use godot::classes::Node2D;
use godot::prelude::*;

use fmod_gdrust_bindings::fmod_event_emitter_2d::FmodEventEmitter2DRust;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct MusicManagerClass {
    base: Base<Node2D>,

    #[init(val = OnReady::manual())]
    pub gameplay: OnReady<FmodEventEmitter2DRust>,

    #[init(val = OnReady::manual())]
    pub title_theme: OnReady<FmodEventEmitter2DRust>,
}

#[godot_api]
impl INode2D for MusicManagerClass {
    fn ready(&mut self) {
        // #[init(node = "Gameplay")]
        // #[init(node = "TitleTheme")]
        let gameplay_node = self.base().get_node_as::<Node2D>("Gameplay");
        let title_theme_node = self.base().get_node_as::<Node2D>("TitleTheme");

        let gameplay = FmodEventEmitter2DRust::from_node(gameplay_node);
        self.gameplay.init(gameplay);

        let title_theme = FmodEventEmitter2DRust::from_node(title_theme_node);
        self.title_theme.init(title_theme);
    }
}
