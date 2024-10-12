use crate::player::Player;
use godot::{
    classes::{
        AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, InputEvent, ProjectSettings, Timer,
    },
    global::{move_toward, Key},
    prelude::*,
};
use rand::Rng;

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Enemy {
    #[init(val = 250.)]
    speed: f32,
    inconstancy: f32,

    falling: bool,
    aggro: bool,
    attacking1: bool,
    attacking2: bool,

    attack1_delay: bool,
    attack2_delay: bool,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Enemy {
    fn play_animation(&mut self, name: impl Into<String>) {
        let mut animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

        let old = animated.get_animation().to_string();
        let new: String = name.into();

        if old != new {
            self.on_animation_changed(old, new.clone());

            animated.set_animation(new.into());
            animated.play();
        }
    }

    fn on_animation_changed(&mut self, old: String, _new: String) {
        if old == "attack1" {
            self.attacking1 = false;
        }

        if old == "attack2" {
            self.attacking2 = false;
        }
    }

    #[func]
    fn on_animation_finished(&mut self) {
        let animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

        let animation = animated.get_animation().to_string();

        if animation == "attack1" {
            self.attacking1 = false;
        }

        if animation == "attack2" {
            self.attacking2 = false;
        }
    }

    #[func]
    fn on_attack1_timeout(&mut self) {
        self.attack1_delay = false;
    }

    #[func]
    fn on_attack2_timeout(&mut self) {
        self.attack2_delay = false;
    }
}

#[godot_api]
impl ICharacterBody2D for Enemy {
    fn ready(&mut self) {
        let mut rng = rand::thread_rng();

        self.inconstancy = rng.gen_range(-50..50) as f32;
    }

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
        // TODO: Finish enemy mechanisms: Hit, Death, Jump (It's probably not gonna be implemented)

        let player = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        let magnitude =
            player.get_position() - self.base().get_position() + Vector2::new(self.inconstancy, 0.);

        if magnitude.x.abs() < 600. {
            self.aggro = true;
        }

        if magnitude.x.abs() > 800. && self.aggro {
            self.aggro = false;
        }

        let mut attack1_timer = self.base().get_node_as::<Timer>("Attack1Timer");
        let mut attack2_timer = self.base().get_node_as::<Timer>("Attack2Timer");

        let attacking = self.attacking1 || self.attacking2;
        let idling = !self.falling && !attacking;

        // TODO: Add a projectile to attack2.
        if self.aggro && idling {
            velocity.x = if magnitude.x > 200.0 {
                self.play_animation("run");
                self.speed
            } else if magnitude.x < -200.0 {
                self.play_animation("run");
                -self.speed
            } else if !self.attack2_delay {
                attack2_timer.start();

                self.attacking2 = true;
                self.attack2_delay = true;
                self.play_animation("attack2");

                0.
            } else if !self.attack1_delay {
                attack1_timer.start();

                self.attacking1 = true;
                self.attack1_delay = true;
                self.play_animation("attack1");

                0.
            } else {
                self.play_animation("idle");
                0.
            };

            if !attacking {
                animated.set_flip_h(magnitude.x < 0.);
            }
        } else if idling {
            self.play_animation("idle");
            velocity.x = 0.;
        }

        if velocity.y > 0. {
            self.falling = true;
            self.play_animation("fall");
        }

        if self.base().is_on_floor() {
            self.falling = false;
        }

        self.base_mut().move_and_slide();
        self.base_mut().set_velocity(velocity);
    }
}
