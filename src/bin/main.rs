use bevy::prelude::*;

use automatica::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(InputPlugin)
        .add_systems(Startup, setup)
        .run();
}

// Setup the initial scene.
fn setup() {
    println!("Init!");
}
