mod netcode;
mod player;
mod input;

use bevy::{prelude::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(player::spawn_player)
        .add_startup_system(netcode::start_matchbox_socket)
        .add_system(netcode::wait_for_players)
        .add_system(player::move_player)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::new_with_far(1.);
    camera_bundle.projection.scale = 1. / 50.;
    commands.spawn_bundle(camera_bundle);
}
