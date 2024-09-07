use bevy::prelude::*;

use automatica::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Automatica"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(InputPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(LogicPlugin)
        .add_systems(Startup, setup)
        .run();
}

// Setup the initial scene.
fn setup() {
    println!("Init!");
}
