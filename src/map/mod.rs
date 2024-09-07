use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod resources;
mod systems;

pub use resources::*;
pub use systems::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .init_resource::<CursorPos>()
            .add_systems(Startup, setup)
            .add_systems(First, update_cursor_pos)
            .add_systems(Update, report_cursor_position);
    }
}

// SYSTEMS

fn report_cursor_position(
    cursor_pos: Res<CursorPos>,
    tilemap_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform)>,
) {
    for (map_size, grid_size, map_type, map_transform) in tilemap_q.iter() {
        let cursor_pos: Vec2 = cursor_pos.0;
        let cursor_in_map_pos: Vec2 = {
            let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };

        if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        {
            println!("Cursor in tile pos: {:?}", tile_pos);
        }
    }
}
