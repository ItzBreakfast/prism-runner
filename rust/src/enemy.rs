use crate::player::Player;
use godot::{
    classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, InputEvent, ProjectSettings},
    global::{move_toward, Key},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Enemy {
    #[init(val = 250.)]
    speed: f32,

    falling: bool,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Enemy {
    fn play_animation(&mut self, name: StringName) {
        let mut animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

        self.on_animation_changed(animated.get_animation(), name.clone());

        animated.set_animation(name);
        animated.play();
    }

    fn on_animation_changed(&mut self, old: StringName, _new: StringName) {}

    #[func]
    fn on_animation_finished(&mut self) {
        let animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

        let animation = animated.get_animation();
    }
}

#[godot_api]
impl ICharacterBody2D for Enemy {
    fn physics_process(&mut self, delta: f64) {
        let gravity = ProjectSettings::singleton()
            .get_setting("physics/2d/default_gravity".into())
            .to::<f32>()
            / 35.;

        let mut velocity = self.base().get_velocity();
        let mut animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

        velocity.y = if !self.base().is_on_floor() {
            (velocity.y + gravity + delta as f32).min(750.)
        } else {
            0.
        };

        // TODO: Add collision mechanism (Hitbox) with already existing Area2D for both hit and
        //       attack.

        let player = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        let magnitude = player.get_position() - self.base().get_position();

        // TODO: Add range mechanism: "Monster detects player when player comes in small range.",
        //                            "Monster loses player focus when player goes out of large range."
        // TODO: Make enemy can't rotate while it's attacking.
        // TODO: Make pattern with attack1 and attack2.
        if magnitude.x.abs() < 600. && !self.falling {
            velocity.x = if magnitude.x > 200.0 {
                self.play_animation("run".into());
                self.speed
            } else if magnitude.x < -200.0 {
                self.play_animation("run".into());
                -self.speed
            } else {
                self.play_animation("attack1".into());
                0.
            };

            animated.set_flip_h(magnitude.x < 0.);
        } else if self.falling {
            self.play_animation("idle".into());
            velocity.x = 0.;
        }

        if velocity.y > 0. {
            self.falling = true;

            self.play_animation("fall".into());
        }

        if self.base().is_on_floor() {
            self.falling = false;
        }

        self.base_mut().move_and_slide();
        self.base_mut().set_velocity(velocity);
    }
}
