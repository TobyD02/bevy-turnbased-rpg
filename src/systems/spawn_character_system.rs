use crate::entities::character_entity::CharacterEntityBundle;
use bevy::prelude::*;
use crate::components::map_position_component::MapPositionComponent;
use crate::components::stats_component::StatsComponent;
use crate::enums::map_layer_enum::MapLayerEnum::MapLayerCharacters;
use crate::enums::tile_sprite_enum::TileSpriteEnum::CharacterMinotaur;
use crate::resources::turn_order_resource::TurnOrderResource;

pub fn spawn_character_system(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut turn_order: ResMut<TurnOrderResource>,
    asset_server: Res<AssetServer>,

) {

    for i in 1..10 {
        let texture = asset_server.load("monochrome_tilemap.png");
        let layout =
            TextureAtlasLayout::from_grid(UVec2::splat(8), 16, 10, Some(UVec2::splat(1)), None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let stats = StatsComponent::default();
        let map_position = MapPositionComponent::new(i, i);
        let initiative = stats.roll_initiative() - 1;
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
                map_position,
                ..Default::default()
            }),
        ).id();

        turn_order.add_entity(initiative, entity)
    }
}
