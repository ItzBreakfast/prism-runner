use godot::{
    classes::{
        AnimatedSprite2D, CharacterBody2D, CollisionShape2D, ICharacterBody2D, InputEvent,
        ProjectSettings,
    },
    global::{move_toward, Key},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Player {
    #[init(val = 450.)]
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
        if old == "dash_attack" && new == "fall" {
            self.dash_attack = false;
            self.dash_attacking = false;
            self.dash_attack_finishing = false;

            self.base()
                .get_node_as::<CollisionShape2D>("BodyCollision")
                .set_one_way_collision(true);
        }

        if old == "dash_finished" {
            self.dash_finishing = false;
        }
    }

    #[func]
    fn on_animation_finished(&mut self) {
        let animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

        let animation = animated.get_animation().to_string();

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
        }

        if animation == "aura_attack" {
            self.aura_attack = false;
            self.aura_attacking = false;
        }

        if animation == "fall_attack" {
            self.fall_attack = false;
            self.fall_attacking = false;
        }
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
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

        if self.slide || self.dash || self.basic_attack || self.aura_attack {
            self.dash_finishing = false;
        }
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

        if (self.left || self.right)
            && !self.sliding
            && !self.dashing
            && !self.basic_attacking
            && !self.dash_attacking
            && !self.dash_attack_finishing
            && !self.aura_attacking
            && !self.fall_attacking
        {
            if self.left {
                self.dash_finishing = false;

                velocity.x = -self.speed;
                animated.set_flip_h(true);
                if !self.flipped {
                    self.base_mut().emit_signal("flip".into(), &[]);
                    self.flipped = true;
                }

                if !self.jumping && !self.falling {
                    self.play_animation("run");
                }
            }

            if self.right {
                self.dash_finishing = false;

                velocity.x = self.speed;
                animated.set_flip_h(false);
                if self.flipped {
                    self.base_mut().emit_signal("flip".into(), &[]);
                    self.flipped = false;
                }

                if !self.jumping && !self.falling {
                    self.play_animation("run");
                }
            }

            if self.slide && !self.falling && !self.jumping {
                self.sliding = true;

                self.play_animation("slide");

                if self.left {
                    velocity.x = self.speed * -1.25;
                }

                if self.right {
                    velocity.x = self.speed * 1.25;
                }
            }

            if self.dash && !self.dashed {
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
        }

        // TODO: Try add some particle/effects to skill.
        if !self.jumping
            && !self.falling
            && !self.sliding
            && !self.dashing
            && !self.basic_attacking
            && !self.dash_attacking
            && !self.dash_attack_finishing
            && !self.aura_attacking
            && !self.fall_attacking
        {
            if self.basic_attack {
                self.basic_attacking = true;

                self.play_animation("basic_attack");
            }

            if self.dash_attack {
                self.dash_attacking = true;

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

            // TODO: Finish fall attack.
            if self.fall_attack {
                self.fall_attacking = true;

                self.play_animation("fall_attack");
            }
        }

        if self.dash_attacking {
            velocity.x = if self.flipped {
                self.speed * -2.
            } else {
                self.speed * 2.
            }
        }

        if velocity.y > 0. && !self.dashing {
            self.jumping = false;
            self.falling = true;
            self.dash_finishing = false;

            self.play_animation("fall");
        }

        if self.base().is_on_floor() {
            self.jumping = false;
            self.falling = false;
            self.dashed = false;
        }

        if self.jump
            && self.base().is_on_floor()
            && !self.sliding
            && !self.dashing
            && !self.basic_attacking
            && !self.dash_attacking
            && !self.dash_attack_finishing
            && !self.aura_attacking
            && !self.fall_attacking
        {
            self.jumping = true;
            self.dash_finishing = false;

            self.play_animation("jump");
            velocity.y = -self.jump_power;
        }

        if ((!self.left && !self.right)
            || self.basic_attacking
            || self.dash_attack_finishing
            || self.aura_attacking)
            && !self.sliding
            && !self.dashing
            && !self.dash_attacking
            && !self.fall_attacking
        {
            velocity.x = move_toward(velocity.x.into(), 0., self.speed.into()) as f32;

            if !self.jumping
                && !self.falling
                && !self.dash_finishing
                && !self.basic_attacking
                && !self.dash_attack_finishing
                && !self.aura_attacking
            {
                self.play_animation("idle");
            }
        }

        self.base_mut().move_and_slide();
        self.base_mut().set_velocity(velocity);
    }
}
