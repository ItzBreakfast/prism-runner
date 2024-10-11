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
    #[init(val = 600.)]
    jump_power: f32,

    left: bool,
    right: bool,
    jump: bool,
    slide: bool,
    dash: bool,
    basic_attack: bool,
    dash_attack: bool,
    aura_attack: bool,
    fall_attack: bool,

    flipped: bool,
    jumping: bool,
    falling: bool,
    sliding: bool,
    dashed: bool,
    dashing: bool,
    dash_finishing: bool,
    basic_attacking: bool,
    dash_attacking: bool,
    dash_attack_finishing: bool,
    aura_attacking: bool,
    fall_attacking: bool,

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

        let player = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        velocity.x = if player.get_position().x > self.base().get_position().x {
            animated.set_flip_h(false);
            self.play_animation("run".into());
            self.speed
        } else if player.get_position().x < self.base().get_position().x {
            animated.set_flip_h(true);
            self.play_animation("run".into());
            -self.speed
        } else {
            self.play_animation("idle".into());
            0.
        };

        self.base_mut().move_and_slide();
        self.base_mut().set_velocity(velocity);
    }
}
