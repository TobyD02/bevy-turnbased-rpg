use crate::entities::character_entity::CharacterEntityBundle;
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalRng;
use rand_core::Rng;
use crate::components::stats_component::StatsComponent;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::enums::map_layer_enum::MapLayerEnum::MapLayerCharacters;
use crate::enums::tile_sprite_enum::TileSpriteEnum::CharacterMinotaur;
use crate::enums::turn_group_enum::TurnGroupEnum;
use crate::resources::map_resource::MapResource;
use crate::resources::turn_order_resource::TurnOrderResource;

pub fn setup_character_system(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut turn_order: ResMut<TurnOrderResource>,
    asset_server: Res<AssetServer>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    mut map_resource: ResMut<MapResource>,
) {

    for i in 1..5 {
        let texture = asset_server.load("monochrome_tilemap.png");
        let layout =
            TextureAtlasLayout::from_grid(UVec2::splat(8), 16, 10, Some(UVec2::splat(1)), None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let stats = StatsComponent::default();

        let initiative = stats.roll_initiative(rng.next_u32());
        let entity = commands.spawn(
            (CharacterEntityBundle {
                transform: Transform {
                    translation: Vec3::splat(MapLayerCharacters.float()),
                    ..Default::default()
                },
                sprite: Sprite::from_atlas_image(
                    texture,
                    TextureAtlas {
                        layout: texture_atlas_layout,
                        index: CharacterMinotaur.usize(),
                    },
                ),
                stats,
                name: Name::new(format!("Character {}", i)),
                ..Default::default()
            }),
        ).id();

        turn_order.add_entity(initiative, entity, TurnGroupEnum::Enemy);
        loop {
            let rand_x = rng.next_u32() % MAP_WIDTH as u32;
            let rand_y = rng.next_u32() % MAP_HEIGHT as u32;
            let character_did_spawn = map_resource.set_tile(entity, rand_x as i32, rand_y as i32);
            if character_did_spawn {
                break
            }
        }
    }
}
