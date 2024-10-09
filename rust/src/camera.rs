use crate::player::Player;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Camera2D)]
pub struct SideCamera {
    base: Base<Camera2D>,
}

#[godot_api]
impl ICamera2D for SideCamera {
    fn physics_process(&mut self, delta: f64) {
        let parent = self.base_mut().get_parent().unwrap();

        let position = self.base().get_position();
        let mut player: Gd<Player> = parent.get_node_as("Player");

        let player_position = player.get_position() + Vector2::new(0., -200.);
        self.base_mut()
            .set_position(position.lerp(player_position, 0.1));
    }
}
