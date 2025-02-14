use godot::{
    classes::{GpuParticles2D, Sprite2D, Timer},
    obj::WithBaseField,
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct GroundCrack {
    #[init(val = true)]
    delay: bool,

    base: Base<Node2D>,
}

#[godot_api]
impl GroundCrack {
    #[func]
    fn on_timeout(&mut self) {
        self.delay = false;
    }
}

#[godot_api]
impl INode2D for GroundCrack {
    fn physics_process(&mut self, _delta: f64) {
        let mut fragment = self
            .base()
            .get_node_as::<GpuParticles2D>("FragmentParticles");
        let timer = self.base().get_node_as::<Timer>("Timer");

        if timer.get_time_left() < 2.9 {
            fragment.set_emitting(false);
        }

        if self.delay {
            return;
        }

        let mut sprite = self.base().get_node_as::<Sprite2D>("GroundCrack");

        let mut modulate = sprite.get_modulate();
        let opacity = modulate.a8();

        if opacity > 0 {
            modulate.set_a8(opacity - 1);
            sprite.set_modulate(modulate);
        } else {
            self.base_mut().call_deferred("queue_free", &[]);
        }
    }
}
