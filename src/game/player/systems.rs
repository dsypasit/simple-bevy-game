use crate::*;
use bevy::prelude::*;

use super::{components::Player, PLAYER_SPEED};

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        // let half_player = PLAYER_SIZE / 2.0;
        let half_player = 0.0;
        let min_x = 0.0 + half_player;
        let min_y = 0.0 + half_player;
        let max_x = window.width() - half_player;
        let max_y = window.height() - half_player;

        let mut translation = player_transform.translation;

        if translation.x < min_x {
            translation.x = min_x
        }
        if translation.y < min_y {
            translation.y = min_y
        }
        if translation.x > max_x {
            translation.x = max_x
        }
        if translation.y > max_y {
            translation.y = max_y
        }

        player_transform.translation = translation;
    }
}

pub fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::zeroed();

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0., -1., 0.);
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0., 1., 0.);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1., 0., 0.);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}
