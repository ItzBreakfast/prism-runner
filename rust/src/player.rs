use godot::{
    classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, InputEvent, ProjectSettings},
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

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Player {
    fn set_animation(&mut self, name: StringName) {
        let mut animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

        self.on_animation_changed(animated.get_animation(), name.clone());
        animated.set_animation(name);
    }

    fn on_animation_changed(&mut self, old: StringName, _new: StringName) {
        if old == "dash_finished".into() {
            self.dash_finishing = false;
        }
    }

    #[func]
    fn on_animation_finished(&mut self) {
        let animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

        let animation = animated.get_animation();

        if animation == "slide".into() {
            self.slide = false;
            self.sliding = false;
        }

        if animation == "dash".into() {
            self.dash = false;
            self.dashing = false;
            self.dash_finishing = true;

            self.set_animation("dash_finished".into());
        }

        if animation == "dash_finished".into() {
            self.dash_finishing = false;
        }

        if animation == "basic_attack".into() {
            self.basic_attack = false;
            self.basic_attacking = false;
        }

        if animation == "dash_attack".into() {
            self.dash_attack = false;
            self.dash_attacking = false;
            self.dash_attack_finishing = true;

            self.set_animation("dash_attack_finished".into());
        }

        if animation == "dash_attack_finished".into() {
            self.dash_attack_finishing = false;
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

        if self.slide || self.dash || self.basic_attack {
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

        if self.left
            && !self.sliding
            && !self.dashing
            && !self.basic_attacking
            && !self.dash_attacking
            && !self.dash_attack_finishing
        {
            self.flipped = true;
            self.dash_finishing = false;

            animated.set_flip_h(true);
            velocity.x = -self.speed;

            if !self.jumping && !self.falling {
                self.set_animation("run".into());
            }
        }

        if self.right
            && !self.sliding
            && !self.dashing
            && !self.basic_attacking
            && !self.dash_attacking
            && !self.dash_attack_finishing
        {
            self.flipped = false;
            self.dash_finishing = false;

            animated.set_flip_h(false);
            velocity.x = self.speed;

            if !self.jumping && !self.falling {
                self.set_animation("run".into());
            }
        }

        if self.slide
            && !self.sliding
            && !self.dashing
            && !self.falling
            && !self.jumping
            && !self.basic_attacking
            && !self.dash_attacking
            && !self.dash_attack_finishing
            && (self.left || self.right)
        {
            self.sliding = true;

            self.set_animation("slide".into());

            if self.left {
                velocity.x = self.speed * -1.25;
            }

            if self.right {
                velocity.x = self.speed * 1.25;
            }
        }

        if self.dash
            && !self.dashed
            && !self.dashing
            && !self.sliding
            && !self.basic_attacking
            && !self.dash_attacking
            && !self.dash_attack_finishing
            && (self.left || self.right)
        {
            self.dashed = true;
            self.dashing = true;

            self.set_animation("dash".into());

            if self.left {
                velocity.x = self.speed * -2.;
            }

            if self.right {
                velocity.x = self.speed * 2.;
            }
        }

        if self.basic_attack
            && !self.jumping
            && !self.falling
            && !self.sliding
            && !self.dashing
            && !self.dash_attacking
            && !self.dash_attack_finishing
        {
            self.basic_attack = false; // TODO: Remove this or just don't, since i don't have any idea
                                       // that this is working or not.
            self.basic_attacking = true;

            self.set_animation("basic_attack".into());
        }

        if self.dash_attack
            && !self.jumping
            && !self.falling
            && !self.sliding
            && !self.dashing
            && !self.basic_attacking
            && !self.dash_attack_finishing
        {
            self.dash_attacking = true;

            self.set_animation("dash_attack".into());
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

            self.set_animation("fall".into());
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
        {
            self.jumping = true;
            self.dash_finishing = false;

            self.set_animation("jump".into());
            velocity.y = -self.jump_power;
        }

        if ((!self.left && !self.right) || self.basic_attacking || self.dash_attack_finishing)
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
            {
                self.set_animation("idle".into());
            }
        }

        animated.play();

        self.base_mut().move_and_slide();
        self.base_mut().set_velocity(velocity);
    }
}
