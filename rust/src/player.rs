use crate::{
    aura::SwordAura, camera::SideCamera, crack::GroundCrack, enemy::Enemy, hitbox::Hitbox,
};
use godot::{
    classes::{
        AnimatedSprite2D, CharacterBody2D, CollisionShape2D, ICharacterBody2D, InputEvent,
        ProjectSettings, Timer,
    },
    global::{move_toward, Key},
    prelude::*,
};

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
    // TODO: Implement resistance mechanism for player.
    #[var]
    resistance: bool,
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
    #[var]
    dash_attacking: bool,
    dash_attack_finishing: bool,
    aura_attacking: bool,
    #[var]
    fall_attacking: bool,
    fall_attack_finishing: bool,
    climbing: bool,

    dash_attack_delay: bool,
    aura_attack_delay: bool,
    fall_attack_delay: bool,
    climb_delay: bool,

    strong_attack_shook: bool,
    sword_aura_spawned: bool,

    #[init(val=load("scene/sword_aura.tscn"))]
    sword_aura: Gd<PackedScene>,
    #[init(val=load("scene/ground_crack.tscn"))]
    ground_crack: Gd<PackedScene>,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Player {
    #[signal]
    fn flip();

    fn play_animation(&mut self, new: &str) {
        let mut animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

        let old = animated.get_animation().to_string();

        if old != new {
            self.on_animation_changed(&old, new);

            animated.set_animation(new);
            animated.play();
        }
    }

    fn on_animation_changed(&mut self, old: &str, new: &str) {
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

        let hp: f32 = body.bind().get_hp();

        if !body.bind().get_invincible() && hp > 0. {
            let resistance = body.bind().get_resistance();

            body.bind_mut()
                .set_hp(hp - if resistance { 7.5 } else { 15. });

            if !resistance {
                body.bind_mut().set_hit(true);
                body.set_velocity(Vector2::new(0., -400.));
            }
        }
    }

    #[func]
    fn on_strong_body_entered(&mut self, body: Gd<Node2D>) {
        let Ok(mut body) = body.try_cast::<Enemy>() else {
            return;
        };

        let hp: f32 = body.bind().get_hp();

        if !body.bind().get_invincible() && hp > 0. {
            let resistance = body.bind().get_resistance();

            body.bind_mut()
                .set_hp(hp - if resistance { 25. } else { 35. });

            if !resistance {
                body.bind_mut().set_hit(true);
                body.set_velocity(Vector2::new(0., 400.));
            }
        }
    }

    #[func]
    fn on_fall_body_entered(&mut self, body: Gd<Node2D>) {
        let Ok(mut body) = body.try_cast::<Enemy>() else {
            return;
        };

        let hp: f32 = body.bind().get_hp();

        if !body.bind().get_invincible() && hp > 0. {
            let resistance = body.bind().get_resistance();

            body.bind_mut()
                .set_hp(hp - if resistance { 25. } else { 35. });

            if !resistance {
                body.bind_mut().set_hit(true);
                body.set_velocity(Vector2::new(0., 400.));
            }
        }
    }

    #[func]
    fn on_earthquake_body_entered(&mut self, body: Gd<Node2D>) {
        let Ok(mut body) = body.try_cast::<Enemy>() else {
            return;
        };

        let hp: f32 = body.bind().get_hp();

        if !body.bind().get_invincible() && hp > 0. {
            let resistance = body.bind().get_resistance();

            body.bind_mut()
                .set_hp(hp - if resistance { 30. } else { 50. });

            if !resistance {
                let velocity = if self.base().get_position().x - body.get_position().x < 0. {
                    1000.
                } else {
                    -1000.
                };

                body.bind_mut().set_hit(true);
                body.set_velocity(Vector2::new(velocity, -1500.));
            }
        }
    }

    #[func]
    fn on_dash_attack_timeout(&mut self) {
        self.dash_attack_delay = false;
    }

    #[func]
    fn on_aura_attack_timeout(&mut self) {
        self.aura_attack_delay = false;
    }

    #[func]
    fn on_fall_attack_timeout(&mut self) {
        self.fall_attack_delay = false;
    }

    #[func]
    fn on_climb_timeout(&mut self) {
        self.climb_delay = false;
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
                    Color::WHITE
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

        self.slide = input.is_action_just_pressed("slide");
        self.dash = input.is_action_just_pressed("dash");

        self.basic_attack = input.is_action_just_pressed("basic_attack");

        self.dash_attack = input.is_action_just_pressed("dash_attack");
        self.aura_attack = input.is_action_just_pressed("aura_attack");
        self.fall_attack = input.is_action_just_pressed("fall_attack");

        self.up = input.is_key_pressed(Key::UP);
        self.climb = input.is_action_just_pressed("climb");

        if self.slide || self.dash || self.basic_attack || self.aura_attack || self.climb {
            self.dash_finishing = false;
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.base_mut().queue_redraw();

        let gravity = ProjectSettings::singleton()
            .get_setting("physics/2d/default_gravity")
            .to::<f32>()
            / 35.;

        let mut velocity = self.base().get_velocity();
        let mut animated = self.base().get_node_as::<AnimatedSprite2D>("Animation");

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

        velocity.y = if self.fall_attacking {
            (velocity.y + 300. + gravity * 1.5 + delta as f32).min(1200.)
        } else if !self.base().is_on_floor() && !self.climbing {
            (velocity.y + gravity + delta as f32).min(750.)
        } else {
            0.
        };

        if self.hp <= 0. {
            velocity.x = velocity.x.lerp(0., 0.1);

            basic_collision.set_disabled(true);
            strong_collision.set_disabled(true);
            fall_collision.set_disabled(true);
            earthquake_collision.set_disabled(true);

            self.base_mut().move_and_slide();
            self.base_mut().set_velocity(velocity);
            self.play_animation("death");

            return;
        } else {
            self.hp = (self.hp + 0.1).min(100.)
        }

        if self.hit {
            self.hit = false;
            self.suffering = true;

            animated.set_frame(0);
            self.play_animation("hit");
        }

        let animation = animated.get_animation().to_string();
        let frame = animated.get_frame();

        let mut camera = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<SideCamera>("SideCamera");

        match frame {
            0..=5 if animation == "slide" => {
                basic_collision.set_disabled(true);
                strong_collision.set_disabled(true);
                fall_collision.set_disabled(true);
                earthquake_collision.set_disabled(true);

                self.invincible = true;
            }
            6 if animation == "slide" => {
                self.invincible = false;
            }
            5..=6 | 9..=10 if animation == "basic_attack" => {
                basic_collision.set_disabled(false);
                strong_collision.set_disabled(true);
                fall_collision.set_disabled(true);
                earthquake_collision.set_disabled(true);
            }
            3..=4 if animation == "aura_attack" || animation == "dash_attack_finished" => {
                basic_collision.set_disabled(true);
                strong_collision.set_disabled(false);
                fall_collision.set_disabled(true);
                earthquake_collision.set_disabled(true);

                if !self.strong_attack_shook {
                    self.strong_attack_shook = true;
                    camera.bind_mut().shake(30);
                }

                if !self.sword_aura_spawned && animation == "aura_attack" {
                    let mut sword_aura = self.sword_aura.instantiate().unwrap().cast::<SwordAura>();

                    self.sword_aura_spawned = true;

                    self.base().get_parent().unwrap().add_child(&sword_aura);

                    sword_aura.bind_mut().set_flipped(self.flipped);
                    sword_aura.set_scale(Vector2::new(if self.flipped { -1. } else { 1. }, 1.));

                    sword_aura.set_position(self.base().get_position() + Vector2::new(50., 0.));
                    sword_aura.set_physics_process(true);
                }
            }
            _ if animation == "fall_attack" => {
                basic_collision.set_disabled(true);
                strong_collision.set_disabled(true);
                fall_collision.set_disabled(false);
                earthquake_collision.set_disabled(true);
            }
            1 if animation == "fall_attack_finished" => {
                basic_collision.set_disabled(true);
                strong_collision.set_disabled(true);
                fall_collision.set_disabled(true);
                earthquake_collision.set_disabled(false);
            }
            _ => {
                self.strong_attack_shook = false;
                self.sword_aura_spawned = false;

                basic_collision.set_disabled(true);
                strong_collision.set_disabled(true);
                fall_collision.set_disabled(true);
                earthquake_collision.set_disabled(true);
            }
        }

        let mut dash_attack_timer = self.base().get_node_as::<Timer>("DashAttackTimer");
        let mut aura_attack_timer = self.base().get_node_as::<Timer>("AuraAttackTimer");
        let mut fall_attack_timer = self.base().get_node_as::<Timer>("FallAttackTimer");
        let mut climb_timer = self.base().get_node_as::<Timer>("ClimbTimer");

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
                    self.base_mut().emit_signal("flip", &[]);
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
                    self.base_mut().emit_signal("flip", &[]);
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

            if self.fall_attack
                && !self.fall_attacking
                && !self.fall_attack_delay
                && !self.base().is_on_floor()
            {
                fall_attack_timer.start();

                self.invincible = true;
                self.fall_attacking = true;
                self.fall_attack_delay = true;
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

            if self.dash_attack && !self.dash_attack_delay {
                dash_attack_timer.start();

                self.dash_attacking = true;
                self.dash_attack_delay = true;
                self.invincible = true;
                self.play_animation("dash_attack");
                self.base()
                    .get_node_as::<CollisionShape2D>("BodyCollision")
                    .set_one_way_collision(false);
            }

            if self.aura_attack && !self.aura_attack_delay {
                aura_attack_timer.start();

                self.aura_attacking = true;
                self.aura_attack_delay = true;
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
                let mut ground_crack = self
                    .ground_crack
                    .instantiate()
                    .unwrap()
                    .cast::<GroundCrack>();

                self.fall_attacking = false;
                self.fall_attack = false;
                self.fall_attacking = false;
                self.fall_attack_finishing = true;

                camera.bind_mut().shake(75);
                self.play_animation("fall_attack_finished");

                self.base().get_parent().unwrap().add_child(&ground_crack);

                ground_crack.set_position(self.base().get_position() + Vector2::new(0., 55.));
                ground_crack.set_physics_process(true);
            }
        }

        if !self.climbable {
            self.climbing = false;
        }

        if self.climbing && !self.left && !self.right && !self.up {
            animated.pause();
        }

        if self.climb
            && self.climbable
            && !self.climbing
            && !self.climb_delay
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
            climb_timer.start();

            self.climbing = true;
            self.climb_delay = true;
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
