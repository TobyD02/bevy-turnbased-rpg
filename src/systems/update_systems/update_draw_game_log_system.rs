use crate::components::game_log_text_component::GameLogTextComponent;
use crate::resources::game_log_resource::GameLogResource;
use bevy::prelude::*;

pub fn update_draw_game_log_system(
    mut logger: ResMut<GameLogResource>,
    mut text_query: Query<(&mut Text), With<GameLogTextComponent>>,
) {
    let Ok((mut text)) = text_query.single_mut() else {
        return;
    };

    // Replace the text content
    **text = logger.get_logs_display_string();
}
