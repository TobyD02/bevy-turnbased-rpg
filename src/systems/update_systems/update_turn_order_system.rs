use std::cmp::Reverse;
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::__private228::Content::String;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalRng;
use rand_core::Rng;
use crate::components::stats_component::StatsComponent;
use crate::components::turn_taker_component::TurnTakerComponent;
use crate::resources::game_log_resource::GameLogResource;
use crate::resources::turn_order_resource::TurnOrderResource;

pub fn update_turn_order_system(
    mut turn_order: ResMut<TurnOrderResource>,
    query: Query< (&StatsComponent, &Name), With<TurnTakerComponent>>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    mut logger: ResMut<GameLogResource>
) {
    if turn_order.get_entities().len() == 0 {
        return
    }

    if turn_order.is_should_reorder() {
        for (order_position, entity) in turn_order.get_entities_mut().iter_mut() {
            if let Ok((stats, name)) = query.get(*entity) {
                *order_position = stats.roll_initiative(rng.next_u32());
            }
        }

        turn_order.sort();

        turn_order.set_should_reorder(false);

        for (initiative, e) in turn_order.get_entities() {
            if let Ok((stats, name)) = query.get(*e) {
                logger.log(format!("{:?}: {:?}", initiative, name))
            }
        }
    }
}