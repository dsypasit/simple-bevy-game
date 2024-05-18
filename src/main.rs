use bevy::{core::Zeroable, prelude::*, window::PrimaryWindow};
use rand::random;

pub const PLAYER_SPEED: f32 = 300.0;
pub const ENEMY_SPEED: f32 = 300.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const ENEMIS_NUMBER: i32 = 4;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_player, spawn_enemy, spawn_camera))
        .add_systems(
            Update,
            (
                player_movement,
                confine_player_movement,
                enemy_movement,
                update_enemy_movement,
                confine_enemy_movement,
            ),
        )
        .run()
}

#[derive(Component)]
pub struct Player {}

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

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..ENEMIS_NUMBER {
        let half_enemy = ENEMY_SIZE / 2.0;
        let rand_x = random::<f32>() * window.width() - half_enemy;
        let rand_y = random::<f32>() * window.height() - half_enemy;
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random(), random()).normalize(),
            },
        ));
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = player_query.get_single_mut().unwrap();

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

pub fn update_enemy_movement(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy = ENEMY_SIZE / 2.0;
    let min_x = 0. + half_enemy;
    let min_y = 0. + half_enemy;
    let max_x = window.width() - half_enemy;
    let max_y = window.height() - half_enemy;
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        let mut direction = Vec2::new(enemy.direction.x, enemy.direction.y);

        if translation.x < min_x || translation.x > max_x {
            direction.x *= -1.
        }
        if translation.y < min_y || translation.y > max_y {
            direction.y *= -1.
        }
        enemy.direction = direction;
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy = ENEMY_SIZE / 2.0;
    let min_x = 0. + half_enemy;
    let min_y = 0. + half_enemy;
    let max_x = window.width() - half_enemy;
    let max_y = window.height() - half_enemy;
    enemy_query.iter_mut().for_each(|mut transform| {
        let mut translation = transform.translation;

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
        transform.translation = translation;
    });
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}
