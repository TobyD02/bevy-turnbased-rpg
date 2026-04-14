use bevy::prelude::*;
use crate::components::movement_trail_component::MovementTrailComponent;
use crate::resources::turn_order_resource::TurnOrderResource;

pub fn update_movement_trail_system (
    mut query: Query<(Entity, &mut MovementTrailComponent, &mut Sprite)>,
    turn_order_resource: Res<TurnOrderResource>,
    mut commands: Commands,
) {
    for (entity, mut trail, mut sprite) in query.iter_mut() {
        trail.decay = turn_order_resource.get_turn_iteration() - trail.start_turn_iter;
        if trail.decay >= trail.decay_max {
            commands.entity(entity).despawn();
            continue
        }
        let col = 1.0 - trail.decay as f32 / trail.decay_max as f32;

        sprite.color.set_alpha(col);
    }
}