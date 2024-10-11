#![allow(unused)]

use godot::prelude::*;

mod camera;
mod collider;
mod enemy;
mod hitbox;
mod map;
mod player;

struct PrismRunner;

#[gdextension]
unsafe impl ExtensionLibrary for PrismRunner {}
