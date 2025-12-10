// NOTE: done with this file

use godot::global::{randi, randi_range, randomize};
use godot::prelude::*;

use crate::asteroids::{
    asteroid_big::BigAsteroid, asteroid_medium::MediumAsteroid, asteroid_small::SmallAsteroid,
};
use crate::main_scene;

pub trait AsteroidIFunctions {
    fn asteroid_ready(&mut self);
    fn asteroid_physics_process(&mut self, _delta: f64);
}

impl AsteroidIFunctions for BigAsteroid {
    fn asteroid_ready(&mut self) {
        randomize();

        let position = self.base().get_position();
        if position.x <= 50.0 {
            self.direction.x = 1.0;
        }
        else if position.x >= 150.0 {
            self.direction.x = -1.0;
        }
        else {
            godot_print!("why x");
            let ones = array![-1.0, 1.0];
            self.direction.x = ones.get(randi() as usize % ones.len()).unwrap();
        }

        if position.y <= 50.0 {
            self.direction.y = 1.0;
        }
        else if position.y >= 150.0 {
            self.direction.y = -1.0;
        }
        else {
            godot_print!("why y");
            let ones = array![-1.0, 1.0];
            self.direction.y = ones.get(randi() as usize % ones.len()).unwrap();
        }

        let vert_speed_coin_flip = randi_range(0, 1);
        if vert_speed_coin_flip == 0 {
            self.vertical_speed = randi_range(20, 30) as f32 * self.direction.y;
        }
        else {
            self.vertical_speed = randi_range(35, 45) as f32 * self.direction.y;
        }
    }

    fn asteroid_physics_process(&mut self, _delta: f64) {
        if self.main.is_in_group("main_scene") {
            let main = self.main.clone().cast::<main_scene::Main>();
            if main.bind().is_paused {
                return;
            }
        }

        let position = self.base().get_position();

        if position.x > 250.0 || position.x < -50.0 {
            self.base_mut().queue_free();
        }

        if position.y > 250.0 || position.y < -50.0 {
            self.base_mut().queue_free();
        }
    }
}

impl AsteroidIFunctions for MediumAsteroid {
    fn asteroid_ready(&mut self) {
        // rest of the ready function
        randomize();

        let position = self.base().get_position();
        if position.x <= 50.0 {
            self.direction.x = 1.0;
        }
        else if position.x >= 150.0 {
            self.direction.x = -1.0;
        }
        else {
            godot_print!("why x");
            let ones = array![-1.0, 1.0];
            self.direction.x = ones.get(randi() as usize % ones.len()).unwrap();
        }

        if position.y <= 50.0 {
            self.direction.y = 1.0;
        }
        else if position.y >= 150.0 {
            self.direction.y = -1.0;
        }
        else {
            godot_print!("why y");
            let ones = array![-1.0, 1.0];
            self.direction.y = ones.get(randi() as usize % ones.len()).unwrap();
        }

        let vert_speed_coin_flip = randi_range(0, 1);
        if vert_speed_coin_flip == 0 {
            self.vertical_speed = randi_range(20, 30) as f32 * self.direction.y;
        }
        else {
            self.vertical_speed = randi_range(35, 45) as f32 * self.direction.y;
        }
    }

    fn asteroid_physics_process(&mut self, _delta: f64) {
        if self.main.is_in_group("main_scene") {
            let main = self.main.clone().cast::<main_scene::Main>();
            if main.bind().is_paused {
                return;
            }
        }

        let position = self.base().get_position();

        if position.x > 250.0 || position.x < -50.0 {
            self.base_mut().queue_free();
        }

        if position.y > 250.0 || position.y < -50.0 {
            self.base_mut().queue_free();
        }
    }
}

impl AsteroidIFunctions for SmallAsteroid {
    fn asteroid_ready(&mut self) {
        // rest of the ready function
        randomize();

        let position = self.base().get_position();
        if position.x <= 50.0 {
            self.direction.x = 1.0;
        }
        else if position.x >= 150.0 {
            self.direction.x = -1.0;
        }
        else {
            godot_print!("why x");
            let ones = array![-1.0, 1.0];
            self.direction.x = ones.get(randi() as usize % ones.len()).unwrap();
        }

        if position.y <= 50.0 {
            self.direction.y = 1.0;
        }
        else if position.y >= 150.0 {
            self.direction.y = -1.0;
        }
        else {
            godot_print!("why y");
            let ones = array![-1.0, 1.0];
            self.direction.y = ones.get(randi() as usize % ones.len()).unwrap();
        }

        let vert_speed_coin_flip = randi_range(0, 1);
        if vert_speed_coin_flip == 0 {
            self.vertical_speed = randi_range(20, 30) as f32 * self.direction.y;
        }
        else {
            self.vertical_speed = randi_range(35, 45) as f32 * self.direction.y;
        }
    }

    fn asteroid_physics_process(&mut self, _delta: f64) {
        if self.main.is_in_group("main_scene") {
            let main = self.main.clone().cast::<main_scene::Main>();
            if main.bind().is_paused {
                return;
            }
        }

        let position = self.base().get_position();

        if position.x > 250.0 || position.x < -50.0 {
            self.base_mut().queue_free();
        }

        if position.y > 250.0 || position.y < -50.0 {
            self.base_mut().queue_free();
        }
    }
}

#[derive(PartialEq, GodotConvert, Debug, Clone)]
#[godot(via = i32)]
pub enum AsteroidSize {
    Small,
    Medium,
    Big,
}
