use fmod_gdrust_bindings::fmod_event_emitter_2d::FmodEventEmitter2DRust;
use godot::classes::Node2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct SFXManagerClass {
    base: Base<Node2D>,

    #[init(val = OnReady::manual())]
    pub click: OnReady<FmodEventEmitter2DRust>,

    #[init(val = OnReady::manual())]
    pub explosion: OnReady<FmodEventEmitter2DRust>,

    #[init(val = OnReady::manual())]
    pub ship_thruster: OnReady<FmodEventEmitter2DRust>,

    #[init(val = OnReady::manual())]
    pub laser_shot: OnReady<FmodEventEmitter2DRust>,

    #[init(val = OnReady::manual())]
    pub player_hurt: OnReady<FmodEventEmitter2DRust>,

    #[init(val = OnReady::manual())]
    pub splash_screen: OnReady<FmodEventEmitter2DRust>,
}

#[godot_api]
impl INode2D for SFXManagerClass {
    fn ready(&mut self) {
        let click_node = self.base().get_node_as::<Node2D>("Click");
        let explosion_node = self.base().get_node_as::<Node2D>("Explosion");
        let ship_thruster_node = self.base().get_node_as::<Node2D>("ShipThruster");
        let laser_shot_node = self.base().get_node_as::<Node2D>("LaserShot");
        let player_hurt_node = self.base().get_node_as::<Node2D>("PlayerHurt");
        let splash_screen_node = self.base().get_node_as::<Node2D>("SplashScreen");

        let click = FmodEventEmitter2DRust::from_node(click_node);
        let explosion = FmodEventEmitter2DRust::from_node(explosion_node);
        let ship_thruster = FmodEventEmitter2DRust::from_node(ship_thruster_node);
        let laser_shot = FmodEventEmitter2DRust::from_node(laser_shot_node);
        let player_hurt = FmodEventEmitter2DRust::from_node(player_hurt_node);
        let splash_screen = FmodEventEmitter2DRust::from_node(splash_screen_node);

        self.click.init(click);
        self.explosion.init(explosion);
        self.ship_thruster.init(ship_thruster);
        self.laser_shot.init(laser_shot);
        self.player_hurt.init(player_hurt);
        self.splash_screen.init(splash_screen);
    }
}
