#![allow(unused)]

use godot::prelude::*;

// TODO: Refactory the code (player.rs above all) with following methods:
//         - Put out some conditions from if statements using variable as category.
// TODO: Add a boss from enemy resources.

mod area;
mod camera;
mod collider;
mod enemy;
mod hitbox;
mod map;
mod player;

struct PrismRunner;

#[gdextension]
unsafe impl ExtensionLibrary for PrismRunner {}
