use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::super::resources::*;

pub fn change_tile_colour(
    mut commands: Commands,
    cursor_pos: Res<CursorTile>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut tilemap_query: Query<&TileStorage>,
    // mut tile_query: Query<&mut TileTextureIndex>,
) {
    if let Some(cursor_tile) = cursor_pos.0 {
        if mouse_input.pressed(MouseButton::Left) {
            for tile_storage in tilemap_query.iter_mut() {
                if let Some(tile_entity) = tile_storage.get(&cursor_tile) {
                    commands.entity(tile_entity).insert(TileTextureIndex(1));
                }
            }
        } else if mouse_input.pressed(MouseButton::Right) {
            for tile_storage in tilemap_query.iter_mut() {
                if let Some(tile_entity) = tile_storage.get(&cursor_tile) {
                    commands.entity(tile_entity).insert(TileTextureIndex(0));
                }
            }
        }
    }
}
