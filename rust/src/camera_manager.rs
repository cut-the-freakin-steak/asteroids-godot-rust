// NOTE: done with this file

use godot::classes::{FastNoiseLite, INode, Node, Node2D};
use godot::global::{maxf, randi, randomize};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct CameraManager {
    base: Base<Node>,

    #[init(node = "PhantomCamera2D")]
    camera: OnReady<Gd<Node2D>>,

    #[allow(non_snake_case)]
    #[init(node = "/root/Settings")]
    Settings: OnReady<Gd<Node>>,

    #[init(val = 0.0)]
    shake_intensity: f32,

    #[init(val = 0.0)]
    active_shake_time: f32,

    #[init(val = 5.0)]
    shake_decay: f32,

    #[init(val = 0.0)]
    shake_time: f32,

    #[init(val = 20.0)]
    shake_time_speed: f32,

    #[init(val = FastNoiseLite::new_gd())]
    noise: Gd<FastNoiseLite>,
}

#[godot_api]
impl INode for CameraManager {
    fn physics_process(&mut self, delta: f64) {
        if self.active_shake_time > 0.0 {
            self.shake_time += self.shake_time_speed * delta as f32;
            self.active_shake_time -= delta as f32;

            self.camera.set_position(Vector2 {
                x: self.noise.get_noise_2d(self.shake_time, 0.0) * self.shake_intensity + 100.0,

                y: self.noise.get_noise_2d(0.0, self.shake_time) * self.shake_intensity + 100.0,
            });

            self.shake_intensity = maxf(
                self.shake_intensity as f64 - self.shake_decay as f64 * delta,
                0.0,
            ) as f32;
        }
        else {
            let camera_position = self.camera.get_position();
            self.camera.set_position(Vector2::lerp(
                camera_position,
                Vector2 { x: 100.0, y: 100.0 },
                10.5 * delta as f32,
            ));
        }
    }
}

#[godot_api]
impl CameraManager {
    pub fn screen_shake(&mut self, intensity: f32, time: f32) {
        let screen_shake_setting = self.Settings.get("screen_shake_on").to::<bool>();

        if screen_shake_setting {
            randomize();
            self.noise.set_seed(randi() as i32);
            self.noise.set_frequency(2.0);

            self.shake_intensity = intensity;
            self.active_shake_time = time;
            self.shake_time = 0.0;
        }
    }
}
