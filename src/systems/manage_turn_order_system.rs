use std::cmp::Reverse;
use bevy::prelude::*;
use crate::components::stats_component::StatsComponent;
use crate::components::turn_taker_component::TurnTakerComponent;
use crate::resources::turn_order_resource::TurnOrderResource;

pub fn manage_turn_order_system(
    mut turn_order: ResMut<TurnOrderResource>,
    query: Query< &StatsComponent, With<TurnTakerComponent>>,
) {
    if turn_order.get_entities().len() == 0 {
        return
    }

    if turn_order.is_should_reorder() {
        for (mut order_position, entity) in turn_order.get_entities_mut().iter_mut() {
            if let Ok(stats) = query.get(*entity) {
                order_position = &Reverse(stats.roll_initiative())
            }
        }
        turn_order.set_should_reorder(false);
        println!("New order: {:?}", turn_order.get_entities());
    }

}