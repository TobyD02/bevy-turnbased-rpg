use crate::entities::player_entity::PlayerEntityBundle;
use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalRng;
use rand_core::Rng;
use crate::components::stats_component::StatsComponent;
use crate::enums::map_layer_enum::MapLayerEnum::MapLayerPlayers;
use crate::enums::tile_sprite_enum::TileSpriteEnum::PlayerIdle;
use crate::resources::turn_order_resource::TurnOrderResource;

pub fn setup_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut turn_order: ResMut<TurnOrderResource>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>
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
            sprite: Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: PlayerIdle.usize(),
                },
            ),
            ..Default::default()
        }),
    ).id();
    turn_order.add_entity(initiative, entity)
}
