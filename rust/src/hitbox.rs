use godot::{
    classes::{Area2D, IArea2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct Hitbox {
    default_position: Vector2,

    base: Base<Area2D>,
}

#[godot_api]
impl Hitbox {
    #[func]
    fn on_flip(&mut self) {
        let position = self.base().get_scale();
        self.base_mut()
            .set_scale(Vector2::new(position.x * -1., 1.));
    }
}

#[godot_api]
impl IArea2D for Hitbox {
    fn ready(&mut self) {
        self.default_position = self.base().get_position();
    }
}
