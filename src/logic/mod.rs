use bevy::prelude::*;

mod constants;
mod systems;

pub use systems::*;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, reset)
            .add_systems(Update, electricity);
    }
}
