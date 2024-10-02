use godot::{
    classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, InputEvent, ProjectSettings},
    global::{move_toward, Key},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f32,
    jump_power: f32,

    left: bool,
    right: bool,
    jump: bool,
    slide: bool,

    jumping: bool,
    falling: bool,
    sliding: bool,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello, godot!");

        Self {
            speed: 300.,
            jump_power: 600.,
            left: false,
            right: false,
            jump: false,
            slide: false,
            jumping: false,
            falling: false,
            sliding: false,
            base,
        }
    }

    fn input(&mut self, mut event: Gd<InputEvent>) {
        let input = Input::singleton();

        self.left = input.is_key_pressed(Key::A);
        self.right = input.is_key_pressed(Key::D);
        self.jump = input.is_key_pressed(Key::SPACE);
        self.slide = input.is_key_pressed(Key::SHIFT);
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

        if self.jump && self.base().is_on_floor() {
            self.jumping = true;

            animated.set_animation("jump".into());
            velocity.y = -self.jump_power;
        }

        // if self.slide {
        //     self.sliding = true;
        //
        //     animated.set_animation("slide".into());
        //     animated.connect(signal, callable);
        //     velocity.x *= 5.;
        // }

        if velocity.y > 0. {
            self.jumping = false;
            self.falling = true;

            animated.set_animation("fall".into());
        }

        if self.base().is_on_floor() {
            self.falling = false;
        }

        if self.left {
            animated.set_flip_h(true);
            velocity.x = -self.speed;

            if !self.jumping && !self.falling {
                animated.set_animation("run".into());
            }
        }

        if self.right {
            animated.set_flip_h(false);
            velocity.x = self.speed;

            if !self.jumping && !self.falling {
                animated.set_animation("run".into());
            }
        }

        if !self.left && !self.right {
            velocity.x = move_toward(velocity.x.into(), 0., self.speed.into()) as f32;

            if !self.jumping && !self.falling {
                animated.set_animation("idle".into());
            }
        }

        animated.play();

        self.base_mut().move_and_slide();
        self.base_mut().set_velocity(velocity);
    }
}
