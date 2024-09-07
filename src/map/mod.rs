use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod constants;
mod resources;
mod systems;

pub use resources::*;
pub use systems::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .init_resource::<CursorPos>()
            .init_resource::<CursorTile>()
            .add_systems(Startup, setup)
            .add_systems(First, (update_cursor_pos, update_cursor_tile))
    .add_systems(Update, change_tile_colour)
        // .add_systems(Update, (report_cursor_pos, report_cursor_tile))
        ;
    }
}

#[allow(dead_code)]
fn report_cursor_pos(cursor_pos: Res<CursorPos>) {
    println!("Cursor in tile pos: {:?}", cursor_pos.0);
}

#[allow(dead_code)]
fn report_cursor_tile(cursor_tile: Res<CursorTile>) {
    println!("Cursor in tile index: {:?}", cursor_tile.0);
}
