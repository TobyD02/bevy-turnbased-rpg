use bevy::prelude::*;
use crate::components::game_log_text_component::GameLogTextComponent;
use crate::resources::game_log_resource::GameLogResource;

pub fn update_draw_game_log_system(
    mut log: ResMut<GameLogResource>,
    mut query: Query<&mut Text, With<GameLogTextComponent>>,
) {
    let Ok(mut text) = query.single_mut() else {
        return;
    };
    // Replace the text content
    **text = log.get_logs_display_string();
}