use crate::camera::SideCamera;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct Map {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Map {
    fn draw(&mut self) {
        self.base_mut().draw_rect(
            Rect2::new(Vector2::ZERO, Vector2::new(2000., 500.)),
            Color::from_html("#170f20").unwrap(),
        );
    }

    fn physics_process(&mut self, delta: f64) {
        let parent = self.base_mut().get_parent().unwrap();
        let mut camera: Gd<SideCamera> = parent.get_node_as("SideCamera");

        self.base_mut()
            .set_position(Vector2::new(camera.get_position().x - 1000., 650.));
    }
}
