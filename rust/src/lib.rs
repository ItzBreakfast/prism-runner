#![allow(unused)]

use godot::prelude::*;

// TODO: Refactory the code (player.rs above all) with following methods:
//         - Add an animation state variable to avoid .into() abuse.
//         - Put out some conditions from if statements using variable as category.

mod camera;
mod collider;
mod enemy;
mod hitbox;
mod map;
mod player;

struct PrismRunner;

#[gdextension]
unsafe impl ExtensionLibrary for PrismRunner {}
