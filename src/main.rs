pub mod components;
pub mod enums;
pub mod systems;
mod constants;
pub mod entities;
pub mod resources;

use bevy::prelude::*;
use bevy_rand::plugin::EntropyPlugin;
use bevy_prng::WyRand;
use crate::systems::_plugin::SystemsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(SystemsPlugin)
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        .run();
}
