use crate::player::Player;
use godot::{
    classes::{Area2D, IArea2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=Area2D)]
struct ClimbableArea {
    base: Base<Area2D>,
}

#[godot_api]
impl ClimbableArea {
    #[func]
    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        let Ok(mut player) = body.try_cast::<Player>() else {
            return;
        };

        player.bind_mut().set_climbable(true);
    }

    #[func]
    fn on_body_exited(&mut self, body: Gd<Node2D>) {
        let Ok(mut player) = body.try_cast::<Player>() else {
            return;
        };

        player.bind_mut().set_climbable(false);
    }
}
