use crate::{game::player::components::Bullet, *};
use bevy::{
    input::mouse::{mouse_button_input_system, MouseButtonInput, MouseMotion},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use self::game::enemy::{self, components::Enemy, ENEMY_SIZE};

use super::{components::Player, BULLET_SPEED, PLAYER_SIZE, PLAYER_SPEED};

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

        let half_player = PLAYER_SIZE / 2.0;
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

pub fn shoot(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows_query.get_single().unwrap();
    let player_transform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => {
            return;
        }
    };
    let translation = player_transform.translation;
    let (camera, camera_transform) = q_camera.single();
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if mouse_button_input.pressed(MouseButton::Left) {
            // let mouse_pos = window.physical_cursor_position().unwrap();
            let direction = (world_position - Vec2::new(translation.x, translation.y)).normalize();
            let box_mesh_handle = meshes.add(Cuboid::new(10.0, 10.0, 0.0));
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: box_mesh_handle.into(),
                    material: materials.add(Color::PURPLE),
                    transform: Transform::from_xyz(translation.x, translation.y, 0.0),
                    ..default()
                },
                Bullet { direction },
            ));
        }
    }
}

pub fn bullet_direction(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in bullet_query.iter_mut() {
        let direction = Vec3::new(bullet.direction.x, bullet.direction.y, 0.0);
        transform.translation += direction * BULLET_SPEED * time.delta_seconds();
    }
}

pub fn bullet_hit_screen(
    mut commands: Commands,
    mut bullet_query: Query<(&mut Transform, Entity), With<Bullet>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let min_x = 0.0;
    let max_x = window.width();
    let min_y = 0.0;
    let max_y = window.height();

    if let Ok((mut btransform, bentity)) = bullet_query.get_single_mut() {
        let btranslation = btransform.translation;
        if btranslation.x < min_x
            || btranslation.x > max_x
            || btranslation.y < min_y
            || btranslation.y > max_y
        {
            commands.entity(bentity).despawn();
        }
    }
}

pub fn bullet_hit_enemy(
    mut commands: Commands,
    bullet_query: Query<&Transform, With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for btransform in bullet_query.iter() {
        let btranslation = btransform.translation;
        for (enemy_entity, enemy_trasform) in enemy_query.iter() {
            let enemy_translation = enemy_trasform.translation;
            let distance = btranslation.distance(enemy_translation);
            if distance < ENEMY_SIZE + 10.0 {
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}

pub fn player_despawn(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for player_entity in query.iter() {
        commands.entity(player_entity).despawn();
    }
}
