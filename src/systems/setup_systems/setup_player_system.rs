use crate::entities::player_entity::PlayerEntityBundle;
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalRng;
use rand_core::Rng;
use crate::components::stats_component::StatsComponent;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::enums::map_layer_enum::MapLayerEnum::MapLayerPlayers;
use crate::enums::tile_sprite_enum::TileSpriteEnum::PlayerIdle;
use crate::enums::turn_group_enum::TurnGroupEnum;
use crate::resources::game_log_resource::GameLogResource;
use crate::resources::map_resource::MapResource;
use crate::resources::turn_order_resource::TurnOrderResource;

pub fn setup_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut turn_order: ResMut<TurnOrderResource>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    mut map_resource: ResMut<MapResource>,
    mut logger: ResMut<GameLogResource>
) {
    let texture = asset_server.load("monochrome_tilemap.png");
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(8), 16, 10, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let stats = StatsComponent::default();
    let initiative = stats.roll_initiative(rng.next_u32());
    let entity = commands.spawn(
        (PlayerEntityBundle {
            transform: Transform {
                translation: Vec3::splat(MapLayerPlayers.float()),
                ..Default::default()
            },
            sprite: Sprite {
                image: texture,
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: PlayerIdle.usize(),
                }),
                color: Color::srgb(1., 1., 0.),
                ..Default::default()
            },
            ..Default::default()
        }),
    ).id();
    turn_order.add_entity(initiative, entity, TurnGroupEnum::Player);
    let player_did_spawn = map_resource.set_tile(entity, MAP_WIDTH / 2, MAP_HEIGHT/ 2);

    logger.log(format!("Spawned Player {:?}", player_did_spawn));
}
