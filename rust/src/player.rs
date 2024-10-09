use godot::{
    classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, InputEvent, ProjectSettings},
    global::{move_toward, Key},
    prelude::*,
};
use std::sync::{Arc, Mutex};

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

    jumping: bool,
    falling: bool,
    sliding: bool,
    dashing: bool,
    dash_finishing: bool,
    basic_attacking: bool,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Player {
    #[func]
    fn on_animation_finished(&mut self) {
        let mut animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

        let animation = animated.get_animation();

        if animation == "slide".into() {
            self.slide = false;
            self.sliding = false;
        }

        if animation == "dash".into() {
            self.dash = false;
            self.dashing = false;
            self.dash_finishing = true;

            animated.set_animation("dash_finished".into());
        }

        if animation == "dash_finished".into() {
            self.dash_finishing = false;
        }

        if animation == "basic_attack".into() {
            self.basic_attack = false;
            self.basic_attacking = false;
        }
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn input(&mut self, mut event: Gd<InputEvent>) {
        let input = Input::singleton();

        self.left = input.is_key_pressed(Key::A);
        self.right = input.is_key_pressed(Key::D);
        self.jump = input.is_key_pressed(Key::SPACE);

        self.slide = input.is_action_just_pressed("slide".into());
        self.dash = input.is_action_just_pressed("dash".into());

        self.basic_attack = input.is_action_just_pressed("basic_attack".into());

        if self.slide || self.dash || self.basic_attack {
            self.dash_finishing = false;
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let gravity = ProjectSettings::singleton()
            .get_setting("physics/2d/default_gravity".into())
            .to::<f32>()
            / 35.;

        let position = self.base().get_position();
        let mut velocity = self.base().get_velocity();
        let mut animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

        velocity.y = if !self.base().is_on_floor() {
            (velocity.y + gravity + delta as f32).min(750.)
        } else {
            0.
        };

        if self.left && !self.sliding && !self.dashing && !self.basic_attacking {
            self.dash_finishing = false;

            animated.set_flip_h(true);
            velocity.x = -self.speed;

            if !self.jumping && !self.falling {
                animated.set_animation("run".into());
            }
        }

        if self.right && !self.sliding && !self.dashing && !self.basic_attacking {
            self.dash_finishing = false;

            animated.set_flip_h(false);
            velocity.x = self.speed;

            if !self.jumping && !self.falling {
                animated.set_animation("run".into());
            }
        }

        if self.slide
            && !self.sliding
            && !self.dashing
            && !self.falling
            && !self.jumping
            && !self.basic_attacking
            && (self.left || self.right)
        {
            self.sliding = true;

            animated.set_animation("slide".into());

            if self.left {
                velocity.x = self.speed * -1.25;
            }

            if self.right {
                velocity.x = self.speed * 1.25;
            }
        }

        if self.dash
            && !self.dashing
            && !self.sliding
            && !self.basic_attacking
            && (self.left || self.right)
        {
            self.dashing = true;

            animated.set_animation("dash".into());

            if self.left {
                velocity.x = self.speed * -2.;
            }

            if self.right {
                velocity.x = self.speed * 2.;
            }
        }

        if self.basic_attack && !self.sliding && !self.dashing && !self.falling && !self.jumping {
            self.basic_attacking = true;

            animated.set_animation("basic_attack".into());
        }

        if velocity.y > 0. && !self.dashing {
            self.jumping = false;
            self.falling = true;
            self.dash_finishing = false;

            animated.set_animation("fall".into());
        }

        if self.base().is_on_floor() {
            self.jumping = false;
            self.falling = false;
        }

        if self.jump
            && self.base().is_on_floor()
            && !self.sliding
            && !self.dashing
            && !self.basic_attacking
        {
            self.jumping = true;
            self.dash_finishing = false;

            animated.set_animation("jump".into());
            velocity.y = -self.jump_power;
        }

        if ((!self.left && !self.right) || self.basic_attacking) && !self.sliding && !self.dashing {
            velocity.x = move_toward(velocity.x.into(), 0., self.speed.into()) as f32;

            if !self.jumping && !self.falling && !self.dash_finishing && !self.basic_attacking {
                animated.set_animation("idle".into());
            }
        }

        animated.play();

        self.base_mut().move_and_slide();
        self.base_mut().set_velocity(velocity);
    }
}
