use godot::{
    classes::{CollisionShape2D, ICollisionShape2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=CollisionShape2D)]
pub struct FlippableCollider {
    default_position: Vector2,

    #[init(val = 1.)]
    flipped: f32,

    base: Base<CollisionShape2D>,
}

#[godot_api]
impl FlippableCollider {
    #[func]
    fn on_flip(&mut self) {
        self.flipped *= -1.;

        let position = self.default_position;
        let flipped = self.flipped;

        self.base_mut()
            .set_position(Vector2::new(position.x * flipped, position.y));
    }
}

#[godot_api]
impl ICollisionShape2D for FlippableCollider {
    fn ready(&mut self) {
        self.default_position = self.base().get_position();
    }
}
