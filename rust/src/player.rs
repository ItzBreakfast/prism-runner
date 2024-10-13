use godot::{
    classes::{
        AnimatedSprite2D, CharacterBody2D, CollisionShape2D, ICharacterBody2D, InputEvent,
        ProjectSettings,
    },
    global::{move_toward, Key},
    prelude::*,
};

use crate::{camera::SideCamera, enemy::Enemy, hitbox::Hitbox};

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Player {
    #[var]
    #[init(val = 100.)]
    hp: f32,
    #[init(val = 450.)]
    speed: f32,
    #[init(val = 600.)]
    jump_power: f32,
    #[var]
    invincible: bool,
    #[var]
    climbable: bool,

    left: bool,
    right: bool,
    jump: bool,
    slide: bool,
    dash: bool,
    basic_attack: bool,
    dash_attack: bool,
    aura_attack: bool,
    fall_attack: bool,
    up: bool,
    climb: bool,

    flipped: bool,
    jumping: bool,
    falling: bool,
    #[var]
    hit: bool,
    suffering: bool,
    sliding: bool,
    dashed: bool,
    dashing: bool,
    dash_finishing: bool,
    basic_attacking: bool,
    dash_attacking: bool,
    dash_attack_finishing: bool,
    aura_attacking: bool,
    #[var]
    fall_attacking: bool,
    fall_attack_finishing: bool,
    climbing: bool,

    aura_attack_shook: bool,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Player {
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

    fn on_animation_changed(&mut self, old: String, new: String) {
        let mut animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

        if old == "slide" {
            self.slide = false;
            self.sliding = false;
        }

        if old == "dash" {
            self.dash = false;
            self.dashing = false;
        }

        if old == "dash_finished" {
            self.dash_finishing = false;
        }

        if old == "dash_attack" && new == "fall" {
            self.dash_attack = false;
            self.dash_attacking = false;
            self.dash_attack_finishing = false;
            self.invincible = false;

            self.base()
                .get_node_as::<CollisionShape2D>("BodyCollision")
                .set_one_way_collision(true);
        }

        if old == "basic_attack" {
            self.basic_attack = false;
            self.basic_attacking = false;
        }

        if old == "aura_attack" {
            self.aura_attack = false;
            self.aura_attacking = false;
        }

        if old == "climb" {
            self.climbing = false;

            animated.play();
        }
    }

    #[func]
    fn on_animation_finished(&mut self) {
        let animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

        let animation = animated.get_animation().to_string();

        if animation == "hit" {
            self.suffering = false;
        }

        if animation == "slide" {
            self.slide = false;
            self.sliding = false;
        }

        if animation == "dash" {
            self.dash = false;
            self.dashing = false;
            self.dash_finishing = true;

            self.play_animation("dash_finished");
        }

        if animation == "dash_finished" {
            self.dash_finishing = false;
        }

        if animation == "basic_attack" {
            self.basic_attack = false;
            self.basic_attacking = false;
        }

        if animation == "dash_attack" {
            self.dash_attack = false;
            self.dash_attacking = false;
            self.dash_attack_finishing = true;

            self.base()
                .get_node_as::<CollisionShape2D>("BodyCollision")
                .set_one_way_collision(true);
            self.play_animation("dash_attack_finished");
        }

        if animation == "dash_attack_finished" {
            self.dash_attack_finishing = false;
            self.invincible = false;
        }

        if animation == "aura_attack" {
            self.aura_attack = false;
            self.aura_attacking = false;
        }

        if animation == "fall_attack_finished" {
            self.invincible = false;
            self.fall_attack_finishing = false;
        }
    }

    #[func]
    fn on_basic_body_entered(&mut self, body: Gd<Node2D>) {
        let Ok(mut body) = body.try_cast::<Enemy>() else {
            return;
        };

        if !body.get("invincible".into()).to::<bool>() {
            let name: StringName = "hp".into();
            let hp: f32 = body.get(name.clone()).to();

            body.set("hit".into(), &true.to_variant());
            body.set(name, &(hp - 10.).to_variant());

            body.set_velocity(Vector2::new(0., -400.));
        }
    }

    #[func]
    fn on_strong_body_entered(&mut self, body: Gd<Node2D>) {
        let Ok(mut body) = body.try_cast::<Enemy>() else {
            return;
        };

        if !body.get("invincible".into()).to::<bool>() {
            let name: StringName = "hp".into();
            let hp: f32 = body.get(name.clone()).to();

            body.set("hit".into(), &true.to_variant());
            body.set(name, &(hp - 30.).to_variant());

            body.set_velocity(Vector2::new(0., 400.));
        }
    }

    #[func]
    fn on_fall_body_entered(&mut self, body: Gd<Node2D>) {
        let Ok(mut body) = body.try_cast::<Enemy>() else {
            return;
        };

        if !body.get("invincible".into()).to::<bool>() {
            let name: StringName = "hp".into();
            let hp: f32 = body.get(name.clone()).to();

            body.set("hit".into(), &true.to_variant());
            body.set(name, &(hp - 30.).to_variant());

            body.set_velocity(Vector2::new(0., 400.));
        }
    }

    #[func]
    fn on_earthquake_body_entered(&mut self, body: Gd<Node2D>) {
        let Ok(mut body) = body.try_cast::<Enemy>() else {
            return;
        };

        if !body.get("invincible".into()).to::<bool>() {
            let name: StringName = "hp".into();
            let hp: f32 = body.get(name.clone()).to();

            body.set("hit".into(), &true.to_variant());
            body.set(name, &(hp - 50.).to_variant());

            let velocity = if self.base().get_position().x - body.get_position().x < 0. {
                500.
            } else {
                -500.
            };

            body.set_velocity(Vector2::new(velocity, -500.));
        }
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn draw(&mut self) {
        if self.hp > 0. {
            let hp = self.hp;
            let invincible = self.invincible;

            self.base_mut().draw_rect(
                Rect2::new(Vector2::new(-52., 73.), Vector2::new(104., 9.)),
                Color::BLACK,
            );

            self.base_mut().draw_rect(
                Rect2::new(Vector2::new(-50., 75.), Vector2::new(hp, 5.)),
                if invincible {
                    Color::LIGHT_GREEN
                } else {
                    Color::GREEN
                },
            );
        }
    }

    fn input(&mut self, _event: Gd<InputEvent>) {
        let input = Input::singleton();

        self.left = input.is_key_pressed(Key::LEFT);
        self.right = input.is_key_pressed(Key::RIGHT);
        self.jump = input.is_key_pressed(Key::SPACE);

        self.slide = input.is_action_just_pressed("slide".into());
        self.dash = input.is_action_just_pressed("dash".into());

        self.basic_attack = input.is_action_just_pressed("basic_attack".into());
        self.dash_attack = input.is_action_just_pressed("dash_attack".into());
        self.aura_attack = input.is_action_just_pressed("aura_attack".into());
        self.fall_attack = input.is_action_just_pressed("fall_attack".into());

        self.up = input.is_key_pressed(Key::UP);
        self.climb = input.is_action_just_pressed("climb".into());

        if self.slide || self.dash || self.basic_attack || self.aura_attack || self.climb {
            self.dash_finishing = false;
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
            self.hp = (self.hp + 0.1).min(100.)
        }

        velocity.y = if self.fall_attacking {
            (velocity.y + 300. + gravity * 1.5 + delta as f32).min(1200.)
        } else if !self.base().is_on_floor() && !self.climbing {
            (velocity.y + gravity + delta as f32).min(750.)
        } else {
            0.
        };

        // TODO: Add collision mechanism (Hitbox) with already existing Area2D for both hit and
        //       attack.
        if self.hit {
            self.hit = false;
            self.suffering = true;

            animated.set_frame(0);
            self.play_animation("hit");
        }

        let mut basic_collision = self
            .base()
            .get_node_as::<Hitbox>("BasicAttack")
            .get_node_as::<CollisionShape2D>("Collision");
        let mut strong_collision = self
            .base()
            .get_node_as::<Hitbox>("StrongAttack")
            .get_node_as::<CollisionShape2D>("Collision");
        let mut fall_collision = self
            .base()
            .get_node_as::<Hitbox>("FallAttack")
            .get_node_as::<CollisionShape2D>("Collision");
        let mut earthquake_collision = self
            .base()
            .get_node_as::<Hitbox>("Earthquake")
            .get_node_as::<CollisionShape2D>("Collision");

        let animation = animated.get_animation().to_string();
        let frame = animated.get_frame();

        let mut camera = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<SideCamera>("SideCamera");

        match frame {
            5..=6 | 9..=10 if animation == "basic_attack" => {
                basic_collision.set_disabled(false);
            }
            3..=4 if animation == "aura_attack" || animation == "dash_attack_finished" => {
                strong_collision.set_disabled(false);

                if !self.aura_attack_shook {
                    self.aura_attack_shook = true;
                    camera.call("shake".into(), &[30.to_variant()]);
                }
            }
            _ if animation == "fall_attack" => {
                fall_collision.set_disabled(false);
            }
            1..=2 if animation == "fall_attack_finished" => {
                fall_collision.set_disabled(true);
                earthquake_collision.set_disabled(false);
            }
            _ => {
                self.aura_attack_shook = false;

                basic_collision.set_disabled(true);
                strong_collision.set_disabled(true);
                fall_collision.set_disabled(true);
                earthquake_collision.set_disabled(true);
            }
        }

        if (self.left || self.right || self.fall_attack || (self.up && self.climbing))
            && !self.suffering
            && !self.sliding
            && !self.dashing
            && !self.basic_attacking
            && !self.dash_attacking
            && !self.dash_attack_finishing
            && !self.aura_attacking
            && !self.fall_attacking
            && !self.fall_attack_finishing
        {
            if self.left && !self.up {
                self.dash_finishing = false;

                velocity.x = -self.speed * if self.climbing { 0.5 } else { 1. };
                animated.set_flip_h(true);
                if !self.flipped {
                    self.base_mut().emit_signal("flip".into(), &[]);
                    self.flipped = true;
                }

                if !self.jumping && !self.falling && !self.climbing {
                    self.play_animation("run");
                } else if self.climbing {
                    animated.play();
                }
            }

            if self.right && !self.up {
                self.dash_finishing = false;

                velocity.x = self.speed * if self.climbing { 0.5 } else { 1. };
                animated.set_flip_h(false);
                if self.flipped {
                    self.base_mut().emit_signal("flip".into(), &[]);
                    self.flipped = false;
                }

                if !self.jumping && !self.falling && !self.climbing {
                    self.play_animation("run");
                } else if self.climbing {
                    animated.play();
                }
            }

            if self.up && self.climbing && velocity.x == 0. {
                self.dash_finishing = false;

                velocity.y = -self.speed * 0.5;

                animated.play();
            }

            if self.slide && !self.falling && !self.jumping && !self.climbing {
                self.sliding = true;

                self.play_animation("slide");

                if self.left {
                    velocity.x = self.speed * -1.25;
                }

                if self.right {
                    velocity.x = self.speed * 1.25;
                }
            }

            if self.dash && !self.dashed && !self.climbing {
                self.dashed = true;
                self.dashing = true;

                self.play_animation("dash");

                if self.left {
                    velocity.x = self.speed * -2.;
                }

                if self.right {
                    velocity.x = self.speed * 2.;
                }
            }

            if self.fall_attack && !self.fall_attacking {
                self.invincible = true;
                self.fall_attacking = true;

                self.play_animation("fall_attack");
            }
        }

        // TODO: Try add some particle/effects to skill.
        if !self.jumping
            && !self.falling
            && !self.suffering
            && !self.sliding
            && !self.dashing
            && !self.basic_attacking
            && !self.dash_attacking
            && !self.dash_attack_finishing
            && !self.aura_attacking
            && !self.fall_attacking
            && !self.fall_attack_finishing
            && !self.climbing
        {
            if self.basic_attack {
                self.basic_attacking = true;

                self.play_animation("basic_attack");
            }

            if self.dash_attack {
                self.dash_attacking = true;
                self.invincible = true;

                self.play_animation("dash_attack");
                self.base()
                    .get_node_as::<CollisionShape2D>("BodyCollision")
                    .set_one_way_collision(false);
            }

            // TODO: Finish aura_attack by adding sword aura.
            if self.aura_attack {
                self.aura_attacking = true;

                self.play_animation("aura_attack");
            }
        }

        if self.dash_attacking {
            velocity.x = if self.flipped {
                self.speed * -2.
            } else {
                self.speed * 2.
            }
        }

        if velocity.y > 0. && !self.dashing && !self.fall_attacking {
            self.jumping = false;
            self.falling = true;
            self.dash_finishing = false;

            if !self.suffering {
                self.play_animation("fall");
            }
        }

        if self.base().is_on_floor() {
            self.jumping = false;
            self.falling = false;
            self.dashed = false;

            if self.fall_attacking {
                self.fall_attacking = false;
                self.fall_attack = false;
                self.fall_attacking = false;
                self.fall_attack_finishing = true;

                camera.call("shake".into(), &[150.to_variant()]);
                self.play_animation("fall_attack_finished");
            }
        }

        if !self.climbable {
            self.climbing = false;
        }

        if self.climbing && !self.left && !self.right && !self.up {
            animated.pause();
        }

        if self.climb && self.climbable && !self.climbing {
            self.climbing = true;

            self.play_animation("climb");
        }

        if ((!self.left && !self.right)
            || self.basic_attacking
            || self.dash_attack_finishing
            || self.aura_attacking
            || self.fall_attacking
            || self.fall_attack_finishing)
            && !self.suffering
            && !self.sliding
            && !self.dashing
            && !self.dash_attacking
        {
            velocity.x = move_toward(velocity.x.into(), 0., self.speed.into()) as f32;

            if !self.jumping
                && !self.falling
                && !self.dash_finishing
                && !self.basic_attacking
                && !self.dash_attack_finishing
                && !self.aura_attacking
                && !self.fall_attacking
                && !self.fall_attack_finishing
                && !self.climbing
            {
                self.play_animation("idle");
            }
        }

        if self.jump
            && (self.base().is_on_floor() || self.climbing)
            && !self.suffering
            && !self.sliding
            && !self.dashing
            && !self.basic_attacking
            && !self.dash_attacking
            && !self.dash_attack_finishing
            && !self.aura_attacking
            && !self.fall_attacking
            && !self.fall_attack_finishing
        {
            self.jumping = true;
            self.dash_finishing = false;
            self.climbing = false;

            self.play_animation("jump");
            velocity.y = -self.jump_power;
        }

        if self.suffering {
            velocity.x = velocity.x.lerp(0., 0.1);
        }

        self.base_mut().move_and_slide();
        self.base_mut().set_velocity(velocity);
    }
}
