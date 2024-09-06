use bevy::prelude::*;

mod constants;
mod systems;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::ping);
    }
}
