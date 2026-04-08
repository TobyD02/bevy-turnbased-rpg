use bevy::prelude::*;
use crate::systems::setup_systems::_plugin::SetupSystemsPlugin;
use crate::systems::update_systems::_plugin::UpdateSystemsPlugin;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SetupSystemsPlugin, UpdateSystemsPlugin));
    }
}
