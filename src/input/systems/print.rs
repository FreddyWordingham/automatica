use bevy::prelude::*;

use super::super::constants::PING;

pub fn ping(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(PING) {
        println!("Ping!");
    }
}
