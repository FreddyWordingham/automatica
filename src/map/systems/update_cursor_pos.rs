use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::super::resources::*;

pub fn update_cursor_pos(
    tilemap_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform)>,
    cursor_pos: Res<CursorPos>,
    mut cursor_index: ResMut<CursorIndex>,
) {
    for (map_size, grid_size, map_type, map_transform) in tilemap_q.iter() {
        let cursor_pos: Vec2 = cursor_pos.0;
        let cursor_in_map_pos: Vec2 = {
            let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };

        *cursor_index = if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        {
            CursorIndex(Some(UVec2::new(tile_pos.x, tile_pos.y)))
        } else {
            CursorIndex(None)
        };
    }
}
