use crate::player::Player;
use godot::prelude::*;
use rand::Rng;

#[derive(GodotClass)]
#[class(init, base=Camera2D)]
pub struct SideCamera {
    shake: i32,

    base: Base<Camera2D>,
}

#[godot_api]
impl SideCamera {
    #[func]
    fn shake(&mut self, power: i32) {
        self.shake = (self.shake + power).min(200);
    }
}

#[godot_api]
impl ICamera2D for SideCamera {
    fn physics_process(&mut self, _delta: f64) {
        let parent = self.base_mut().get_parent().unwrap();

        let position = self.base().get_position();
        let player: Gd<Player> = parent.get_node_as("Player");

        let power = if self.shake > 2 {
            rand::thread_rng().gen_range(-self.shake..=self.shake)
        } else {
            0
        };

        let player_position = player.get_position() + Vector2::new(0., -200. + power as f32);

        let target = if player.get("dash_attacking".into()).to::<bool>()
            || player.get("fall_attacking".into()).to::<bool>()
        {
            player_position
        } else {
            position.lerp(player_position, 0.1)
        };
        self.base_mut().set_position(target);

        self.shake = (self.shake - 3).max(0);
    }
}
