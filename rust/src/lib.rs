#![allow(unused)]

use godot::{
    classes::{
        CharacterBody2D, ICharacterBody2D, IRigidBody2D, InputEvent, ProjectSettings, RigidBody2D,
    },
    global::{move_toward, Key},
    obj::WithBaseField,
    prelude::*,
};

struct PrismRunnerExtension;

#[gdextension]
unsafe impl ExtensionLibrary for PrismRunnerExtension {}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f32,
    jump_power: f32,

    left: bool,
    right: bool,
    jump: bool,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello, godot!");

        Player {
            speed: 300.0,
            jump_power: 300.0,
            left: false,
            right: false,
            jump: false,
            base,
        }
    }

    fn input(&mut self, mut event: Gd<InputEvent>) {
        let input = Input::singleton();

        self.left = input.is_key_pressed(Key::A);
        self.right = input.is_key_pressed(Key::D);
        self.jump = input.is_key_pressed(Key::SPACE);
    }

    fn physics_process(&mut self, delta: f64) {
        let gravity = ProjectSettings::singleton()
            .get_setting("physics/2d/default_gravity".into())
            .to::<f32>()
            / 100.0;

        let mut velocity = self.base().get_velocity();

        if !self.base().is_on_floor() {
            velocity.y += gravity + delta as f32;
        }

        if self.jump && self.base().is_on_floor() {
            velocity.y = -self.jump_power;
        }

        if self.left {
            velocity.x = -self.speed;
        }

        if self.right {
            velocity.x = self.speed;
        }

        if !self.left && !self.right {
            velocity.x = move_toward(velocity.x.into(), 0.0, self.speed.into()) as f32;
        }

        self.base_mut().move_and_slide();
        self.base_mut().set_velocity(velocity);
    }
}
