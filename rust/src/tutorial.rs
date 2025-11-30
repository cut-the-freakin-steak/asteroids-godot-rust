// NOTE: done with this file

use godot::classes::{Button, Control, IControl, Input};
use godot::prelude::*;

use crate::audio::sfx_manager::SFXManagerClass;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct Tutorial {
    base: Base<Control>,

    #[init(val = OnReady::manual())]
    scene_root_node: OnReady<Gd<Control>>,

    #[init(node = "/root/MainMenu")]
    main_menu_node: OnReady<Gd<Control>>,

    // singletons
    #[allow(non_snake_case)]
    #[init(node = "/root/SFXManager")]
    SFXManager: OnReady<Gd<SFXManagerClass>>,
}

#[godot_api]
impl IControl for Tutorial {
    fn ready(&mut self) {
        let current_scene = self
            .base()
            .get_tree()
            .unwrap()
            .get_current_scene()
            .unwrap()
            .cast::<Control>();
        self.scene_root_node.init(current_scene);
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();

        if input.is_action_just_pressed("esc") {
            self.disappear_node(self.scene_root_node.clone());

            self.appear_node(self.main_menu_node.clone());

            self.base_mut().queue_free();
        }
    }
}

#[godot_api]
impl Tutorial {
    #[func]
    fn disappear_node(&self, mut node: Gd<Control>) {
        node.set_visible(false);

        let buttons = self.base().get_tree().unwrap().get_nodes_in_group("button");
        for button in buttons.iter_shared() {
            let mut button = button.cast::<Button>();
            button.set_disabled(true);
        }
    }

    #[func]
    fn appear_node(&self, mut node: Gd<Control>) {
        node.set_visible(true);

        let buttons = self.base().get_tree().unwrap().get_nodes_in_group("button");
        for button in buttons.iter_shared() {
            let mut button = button.cast::<Button>();
            button.set_disabled(false);
        }
    }

    #[func]
    fn _on_return_pressed(&mut self) {
        self.SFXManager.bind_mut().click.call("play", &[]);

        self.disappear_node(self.scene_root_node.clone());

        self.appear_node(self.main_menu_node.clone());

        self.base_mut().queue_free();
    }
}
