use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Camera2D)]
struct SideCamera {
    base: Base<Camera2D>,
}

#[godot_api]
impl ICamera2D for SideCamera {
    fn init(base: Base<Camera2D>) -> Self {
        Self { base }
    }
}
