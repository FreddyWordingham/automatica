use bevy::prelude::*;

#[derive(Resource)]
pub struct CursorIndex(pub Option<UVec2>);

impl Default for CursorIndex {
    fn default() -> Self {
        Self(None)
    }
}
