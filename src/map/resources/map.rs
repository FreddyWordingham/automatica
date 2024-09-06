use bevy::prelude::*;

#[derive(Resource)]
pub struct Map {}

impl Default for Map {
    fn default() -> Self {
        Self {}
    }
}
