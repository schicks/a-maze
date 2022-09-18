mod domain;
mod input;
mod netcode;
mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera)
        .add_plugin(netcode::OnlinePlugin)
        .add_plugin(player::PlayerPlugin)
        .run();
}

fn camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::new_with_far(1.);
    camera_bundle.projection.scale = 1. / 50.;
    commands.spawn_bundle(camera_bundle);
}
