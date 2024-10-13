use crate::{camera::SideCamera, hitbox::Hitbox, player::Player};
use godot::{
    classes::{
        AnimatedSprite2D, CharacterBody2D, CollisionShape2D, ICharacterBody2D, InputEvent,
        ProjectSettings, Timer,
    },
    global::{move_toward, Key},
    prelude::*,
};
use rand::Rng;

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Enemy {
    #[var]
    #[init(val = 100.)]
    hp: f32,
    #[init(val = 250.)]
    speed: f32,
    inconstancy: f32,
    #[var]
    invincible: bool,

    flipped: bool,
    falling: bool,
    #[var]
    hit: bool,
    suffering: bool,
    aggro: bool,
    attacking1: bool,
    attacking2: bool,

    flip_delay: bool,
    attack1_delay: bool,
    attack2_delay: bool,

    attack2_shook: bool,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Enemy {
    #[signal]
    fn flip();

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

        if animation == "hit" {
            self.suffering = false;
        }

        if animation == "attack1" {
            self.attacking1 = false;
        }

        if animation == "attack2" {
            self.invincible = false;
            self.attacking2 = false;
        }
    }

    #[func]
    fn on_attack1_body_entered(&mut self, body: Gd<Node2D>) {
        let Ok(mut body) = body.try_cast::<Player>() else {
            return;
        };

        if !body.get("invincible".into()).to::<bool>() {
            let name: StringName = "hp".into();
            let hp: f32 = body.get(name.clone()).to();

            body.set("hit".into(), &true.to_variant());
            body.set(name, &(hp - 10.).to_variant());

            body.set_velocity(Vector2::new(if self.flipped { -200. } else { 200. }, -400.));
        }
    }

    #[func]
    fn on_attack2_body_entered(&mut self, body: Gd<Node2D>) {
        let Ok(mut body) = body.try_cast::<Player>() else {
            return;
        };

        if !body.get("invincible".into()).to::<bool>() {
            let name: StringName = "hp".into();
            let hp: f32 = body.get(name.clone()).to();

            body.set("hit".into(), &true.to_variant());
            body.set(name, &(hp - 30.).to_variant());

            body.set_velocity(Vector2::new(if self.flipped { -400. } else { 400. }, 800.));
        }
    }

    #[func]
    fn on_flip_timeout(&mut self) {
        self.flip_delay = false;
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

    fn draw(&mut self) {
        if self.hp > 0. {
            let hp = self.hp;
            let invincible = self.invincible;

            self.base_mut().draw_rect(
                Rect2::new(Vector2::new(-52., 103.), Vector2::new(104., 9.)),
                Color::BLACK,
            );

            self.base_mut().draw_rect(
                Rect2::new(Vector2::new(-50., 105.), Vector2::new(hp, 5.)),
                if invincible {
                    Color::INDIAN_RED
                } else {
                    Color::RED
                },
            );
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.base_mut().queue_redraw();

        let gravity = ProjectSettings::singleton()
            .get_setting("physics/2d/default_gravity".into())
            .to::<f32>()
            / 35.;

        let mut velocity = self.base().get_velocity();
        let mut animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

        if self.hp <= 0. {
            self.play_animation("death");
            return;
        } else {
            self.hp = (self.hp + 0.05).min(100.)
        }

        if self.hit {
            self.hit = false;
            self.suffering = true;

            animated.set_frame(0);
            self.play_animation("hit");
        }

        if !self.attacking2 {
            self.invincible = false;
        }

        velocity.y = if !self.base().is_on_floor() {
            (velocity.y + gravity + delta as f32).min(750.)
        } else {
            0.
        };

        // TODO: Add collision mechanism (Hitbox) with already existing Area2D for both hit and
        //       attack.
        // TODO: Finish enemy mechanisms: Hit, Death, Jump (It's probably not gonna be implemented)

        let attack1 = self.base().get_node_as::<Hitbox>("Attack1");
        let mut upper_collision = attack1.get_node_as::<CollisionShape2D>("UpperCollision");
        let mut lower_collision = attack1.get_node_as::<CollisionShape2D>("LowerCollision");
        let mut attack2_collision = self
            .base()
            .get_node_as::<Hitbox>("Attack2")
            .get_node_as::<CollisionShape2D>("Collision");

        let player = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        let animation = animated.get_animation().to_string();
        let frame = animated.get_frame();

        let mut camera = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<SideCamera>("SideCamera");

        match frame {
            3 if animation == "attack1" => {
                lower_collision.set_disabled(false);
            }
            4..=6 if animation == "attack1" => {
                upper_collision.set_disabled(false);
                lower_collision.set_disabled(true);
            }
            4..=5 if animation == "attack2" => {
                attack2_collision.set_disabled(false);

                if !self.attack2_shook {
                    self.attack2_shook = true;
                    camera.call("shake".into(), &[50.to_variant()]);
                }
            }
            _ => {
                self.attack2_shook = false;

                upper_collision.set_disabled(true);
                lower_collision.set_disabled(true);
                attack2_collision.set_disabled(true);
            }
        }

        let magnitude =
            player.get_position() - self.base().get_position() + Vector2::new(self.inconstancy, 0.);

        if magnitude.x.abs() < 600.
            && ((magnitude.x < 0. && self.flipped) || (magnitude.x >= 0. && !self.flipped))
        {
            self.aggro = true;
        }

        if magnitude.x.abs() > 800. && self.aggro {
            self.aggro = false;
        }

        let mut attack1_timer = self.base().get_node_as::<Timer>("Attack1Timer");
        let mut attack2_timer = self.base().get_node_as::<Timer>("Attack2Timer");
        let mut flip_timer = self.base().get_node_as::<Timer>("FlipTimer");

        let attacking = self.attacking1 || self.attacking2;
        let idling = !self.suffering && !self.falling && !attacking;

        let flip_delay = rand::thread_rng().gen_range(10..15) as f64;

        // TODO: Add a projectile to attack2.
        if self.aggro && idling {
            velocity.x = if magnitude.x > 200.0 {
                self.play_animation("run");
                self.speed
            } else if magnitude.x < -200.0 {
                self.play_animation("run");
                -self.speed
            } else if !self.attack2_delay {
                flip_timer.set_wait_time(flip_delay);
                flip_timer.start();
                attack2_timer.start();

                self.invincible = true;
                self.attacking2 = true;
                self.attack2_delay = true;
                self.play_animation("attack2");

                0.
            } else if !self.attack1_delay {
                flip_timer.set_wait_time(flip_delay);
                flip_timer.start();
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
                let flipped = magnitude.x < 0.;

                if self.flipped != flipped {
                    self.base_mut().emit_signal("flip".into(), &[]);
                    self.flipped = flipped;

                    animated.set_flip_h(self.flipped);
                }
            }
        } else if idling {
            self.play_animation("idle");

            if !self.flip_delay {
                flip_timer.set_wait_time(flip_delay);
                flip_timer.start();

                self.base_mut().emit_signal("flip".into(), &[]);
                self.flipped = !self.flipped;
                self.flip_delay = true;

                animated.set_flip_h(self.flipped);
            }

            velocity.x = 0.;
        }

        if velocity.y > 0. {
            self.falling = true;

            if !self.suffering {
                self.play_animation("fall");
            }
        }

        if self.base().is_on_floor() {
            self.falling = false;
        }

        self.base_mut().move_and_slide();
        self.base_mut().set_velocity(velocity);
    }
}
