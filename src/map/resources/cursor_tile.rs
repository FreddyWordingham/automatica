use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Resource)]
pub struct CursorTile(pub Option<TilePos>);

impl Default for CursorTile {
    fn default() -> Self {
        Self(None)
    }
}
