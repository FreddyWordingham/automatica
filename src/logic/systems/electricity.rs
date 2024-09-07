use bevy::prelude::*;
use bevy_ecs_tilemap::helpers::square_grid::neighbors::Neighbors;
use bevy_ecs_tilemap::prelude::*;

use super::super::constants::*;

pub fn electricity(
    mut commands: Commands,
    mut tile_storage_query: Query<(&TileStorage, &TilemapSize)>,
    tile_query: Query<(Entity, &TilePos, &mut TileTextureIndex)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(ELECTRICITY) {
        let (tile_storage, map_size) = tile_storage_query.single_mut();
        for (entity, position, _) in tile_query.iter() {
            let neighbor_count =
                Neighbors::get_square_neighboring_positions(position, map_size, false)
                    .entities(tile_storage)
                    .iter()
                    .filter(|neighbor| {
                        let (_, _, tile_texture) = tile_query.get(**neighbor).unwrap();
                        tile_texture.0 == 1
                    })
                    .count();

            if neighbor_count > 0 {
                commands.entity(entity).insert(TileTextureIndex(1));
            } else {
                commands.entity(entity).insert(TileTextureIndex(0));
            }
        }
    }
}
