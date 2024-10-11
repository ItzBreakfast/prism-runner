use crate::camera::SideCamera;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Map {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Map {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("실행 가능합니다!");

        Self { base }
    }

    fn draw(&mut self) {
        self.base_mut().draw_rect(
            Rect2::new(Vector2::ZERO, Vector2::new(2000., 500.)),
            Color::from_html("#170f20").unwrap(),
        );
    }

    fn physics_process(&mut self, _delta: f64) {
        let parent = self.base_mut().get_parent().unwrap();
        let camera: Gd<SideCamera> = parent.get_node_as("SideCamera");

        self.base_mut()
            .set_position(Vector2::new(camera.get_position().x - 1000., 325.));

        // TODO: Add more buildings to TileMap if map designer doesn't do something.
    }
}
