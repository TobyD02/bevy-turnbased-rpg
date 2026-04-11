use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalRng;
use rand_core::Rng;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::entities::wall_entity::WallEntityBundle;
use crate::enums::tile_sprite_enum::TileSpriteEnum::WallRoundedSolidTop;
use crate::resources::game_log_resource::GameLogResource;
use crate::resources::map_resource::MapResource;
use crate::resources::turn_order_resource::TurnOrderResource;

// @todo: Map Generation
pub fn setup_map_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    mut logger: ResMut<GameLogResource>
) {
    let mut map_resource = MapResource::default();

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {

            if x != 0 && x != MAP_WIDTH - 1 && y != 0 && y != MAP_HEIGHT - 1 {
                if (rng.next_u32() % 100 > 5) {continue}
            }

            let texture = asset_server.load("monochrome_tilemap.png");
            let layout =
                TextureAtlasLayout::from_grid(UVec2::splat(8), 16, 10, Some(UVec2::splat(1)), None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            let wall_entity = commands.spawn((WallEntityBundle{
                sprite: Sprite::from_atlas_image(
                    texture,
                    TextureAtlas {
                        layout: texture_atlas_layout,
                        index: WallRoundedSolidTop.usize(),
                    },
                ),
                ..Default::default()
            })).id();

            map_resource.set_tile(wall_entity, x, y);
        }
    }

     commands.insert_resource(map_resource);
}