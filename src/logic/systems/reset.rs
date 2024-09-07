use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::super::constants::*;

pub fn reset(mut query: Query<&mut TileTextureIndex>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(RESET) {
        for mut tile in query.iter_mut() {
            tile.0 = 0;
        }
    }
}
