use bevy::{
    prelude::*,
    sprite::{Sprite, SpriteBundle},
};
use bevy_ggrs::Rollback;
use ggrs::InputStatus;

use crate::{domain::AppState, input::InputSet};

#[derive(Component)]
pub struct Player {
    handle: usize,
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { handle: 0 });
}

// rollback enabled systems need to be set up by netcode.
pub fn move_player(
    inputs: Res<Vec<(InputSet, InputStatus)>>,
    mut query: Query<(&mut Transform, &Player), With<Rollback>>,
) {
    for (mut transform, player) in query.iter_mut() {
        let input = inputs[player.handle as usize].0;
        let mut delta = Vec2::ZERO;
        if input.contains(InputSet::UP) {
            delta.y += 1.;
        }
        if input.contains(InputSet::DOWN) {
            delta.y -= 1.;
        }
        if input.contains(InputSet::DOWN_RIGHT) {
            delta.x += 1.;
        }
        if input.contains(InputSet::DOWN_LEFT) {
            delta.x -= 1.;
        }
        if delta == Vec2::ZERO {
            return;
        }

        let move_speed = 0.13;
        transform.translation += (delta * move_speed).extend(0.);
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_player));
    }
}
