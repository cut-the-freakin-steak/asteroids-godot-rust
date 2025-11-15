// NOTE: done with this file

use godot::classes::{DisplayServer, INode, Node};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct WindowManagerClass {
    base: Base<Node>,

    #[init(val = DisplayServer::singleton())]
    display_server: Gd<DisplayServer>,

    #[init(val = DisplayServer::singleton().window_get_size())]
    window_size: Vector2i,

    #[init(val = 0)]
    window_w: i32,

    #[init(val = 0)]
    window_h: i32,
}

#[godot_api]
impl INode for WindowManagerClass {
    fn process(&mut self, _delta: f64) {
        // check to see if the window size changes
        if self.display_server.window_get_size() != self.window_size {
            self.window_w = self.display_server.window_get_size().x;

            // since the resolution is 1:1 divide the width by 1 and multiply by 1 to get height
            // self.window_h = self.window_w / 1 * 1; // if the aspect ratio wasnt 1:1, this would be a whole lot more useful lmao
            self.window_h = self.window_w;
            // set the window size to the current width and the new height
            self.display_server.window_set_size(Vector2i {
                x: self.window_w,
                y: self.window_h,
            });
            self.window_size = self.display_server.window_get_size();
        }
    }
}
