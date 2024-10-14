use crate::enemy::Enemy;
use godot::{
    classes::{GpuParticles2D, Sprite2D, Timer},
    obj::WithBaseField,
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct SwordAura {
    #[init(val = true)]
    delay: bool,
    #[var]
    flipped: bool,

    base: Base<Node2D>,
}

#[godot_api]
impl SwordAura {
    #[func]
    fn on_timeout(&mut self) {
        self.delay = false;
    }

    #[func]
    fn on_aura_body_entered(&mut self, body: Gd<Node2D>) {
        let Ok(mut body) = body.try_cast::<Enemy>() else {
            return;
        };

        if !body.get("invincible".into()).to::<bool>() {
            let resistance = body.get("resistance".into()).to::<bool>();
            let name: StringName = "hp".into();
            let hp: f32 = body.get(name.clone()).to();

            body.set(
                name,
                &(hp - if resistance { 15. } else { 30. }).to_variant(),
            );

            if !resistance {
                body.set("hit".into(), &true.to_variant());
            }
        }
    }
}

#[godot_api]
impl INode2D for SwordAura {
    fn physics_process(&mut self, _delta: f64) {
        let mut fragment = self.base().get_node_as::<GpuParticles2D>("AuraParticles");
        let position = self.base().get_position();
        let flipped = self.flipped;

        self.base_mut()
            .set_position(position + Vector2::new(if flipped { -10. } else { 10. }, 0.));

        if self.delay {
            return;
        }

        fragment.set_emitting(false);

        let mut sprite = self.base().get_node_as::<Sprite2D>("SwordAura");

        let mut modulate = sprite.get_modulate();
        let opacity = modulate.a8();

        if (opacity as i32 - 30) < 0 {
            self.base_mut().call_deferred("queue_free".into(), &[]);
        } else if opacity > 0 {
            modulate.set_a8(opacity - 30);
            sprite.set_modulate(modulate);
        }
    }
}
