#![allow(unused)]

use bevy::prelude::*;

// region:

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

// endregion

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Invaders!".into(),
                resolution: (598., 676.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // add rectangle
    commands.spawn(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        ..Default::default()
    });

}
