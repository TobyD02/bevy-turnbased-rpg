use crate::entities::character_entity::CharacterEntityBundle;
use crate::enums::tile_sprite_enum::TileSpriteEnum;
use bevy::camera::ScalingMode;
use bevy::prelude::*;

pub fn setup_spawn_character_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("monochrome_tilemap.png");
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(8), 16, 10, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 0.2,
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn(
        (CharacterEntityBundle {
            transform: Transform::default(),
            sprite: Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: TileSpriteEnum::PlayerIdle.usize(),
                },
            ),
            stats: Default::default(),
            health: Default::default(),
            map_position: Default::default(),
        }),
    );
}
