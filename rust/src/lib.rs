#![allow(unused)]

use godot::prelude::*;

mod camera;
mod map;
mod player;

struct PrismRunner;

#[gdextension]
unsafe impl ExtensionLibrary for PrismRunner {}
