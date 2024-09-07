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
            .init_resource::<CursorIndex>()
            .add_systems(Startup, setup)
            .add_systems(First, (update_cursor_pos, update_cursor_index))
        // .add_systems(Update, (report_cursor_pos, report_cursor_index))
        ;
    }
}

#[allow(dead_code)]
fn report_cursor_pos(cursor_pos: Res<CursorPos>) {
    println!("Cursor in tile pos: {:?}", cursor_pos.0);
}

#[allow(dead_code)]
fn report_cursor_index(cursor_index: Res<CursorIndex>) {
    println!("Cursor in tile index: {:?}", cursor_index.0);
}
