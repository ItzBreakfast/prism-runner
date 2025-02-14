#![allow(overlapping_range_endpoints, unused)]

use chrono::Local;
use godot::{classes::Time, init::EditorRunBehavior, prelude::*};

// TODO: Refactory the code (player.rs above all) with following methods:
//         - Put out some conditions from if statements using variable as category.
// TODO: Add a boss from enemy resources.

mod area;
mod aura;
mod camera;
mod collider;
mod crack;
mod enemy;
mod hitbox;
mod map;
mod player;

struct PrismRunner;

#[gdextension]
unsafe impl ExtensionLibrary for PrismRunner {
    fn min_level() -> InitLevel {
        godot_print!("{}\tbuild successful.", Local::now().format("%H:%M:%S"));

        InitLevel::Scene
    }
}
