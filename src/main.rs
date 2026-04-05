pub mod components;
pub mod enums;
pub mod systems;
mod constants;
pub mod entities;

use bevy::prelude::*;
use crate::systems::setup_spawn_character_system::setup_spawn_character_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup_spawn_character_system)
        .run();
}
