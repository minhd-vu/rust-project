use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_system(keyboard_input_system.system())
        .run();
}

/// This system prints keyboard key state
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        println!("up just pressed");
    }

    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        println!("down just pressed");
    }

    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        println!("left just pressed");
    }

    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        println!("right just pressed");
    }
}
