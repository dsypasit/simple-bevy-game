use bevy::{audio::Volume, core::Zeroable, prelude::*, window::PrimaryWindow};
use rand::random;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 300.0;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_NUMBER: i32 = 10;
pub const ENEMIS_NUMBER: i32 = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "hello".to_string(),
                // resolution: (800., 600.).into(),
                // resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Score>()
        .add_systems(
            Startup,
            (
                spawn_audio,
                spawn_star,
                spawn_player,
                spawn_enemy,
                spawn_camera,
            ),
        )
        .add_systems(
            Update,
            (
                player_movement,
                confine_player_movement,
                enemy_movement,
                update_enemy_movement,
                confine_enemy_movement,
                enemy_hit_player,
                player_hit_star,
                update_score,
            ),
        )
        .run()
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource, Default)]
pub struct Score {
    value: i32,
}

pub fn spawn_star(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..STAR_NUMBER {
        let rand_x = random::<f32>() * window.width();
        let rand_y = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut star_query: Query<(Entity, &Transform), With<Star>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    for (star_entity, star_transform) in star_query.iter_mut() {
        let star_radius = STAR_SIZE / 2.0;
        let player_radius = PLAYER_SIZE / 2.0;

        if let Ok(player_transform) = player_query.get_single() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            if distance < star_radius + player_radius {
                commands.entity(star_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/laserLarge_000.ogg"),
                    settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(0.5)),
                });
                score.value += 1;
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        eprintln!("Score:{}", score.value)
    }
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
pub struct ExplosionClunchSound;

pub fn spawn_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/pluck_001.ogg"),
            settings: PlaybackSettings::ONCE
                .with_volume(Volume::new(0.5))
                .with_spatial(true),
        },
        ExplosionClunchSound,
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy = ENEMY_SIZE / 2.0;
    let min_x = 0. + half_enemy;
    let min_y = 0. + half_enemy;
    let max_x = window.width() - half_enemy;
    let max_y = window.height() - half_enemy;
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut change_direction = false;
        let translation = transform.translation;

        let mut direction = Vec2::new(enemy.direction.x, enemy.direction.y);

        if translation.x < min_x || translation.x > max_x {
            change_direction = true;
            direction.x *= -1.
        }
        if translation.y < min_y || translation.y > max_y {
            change_direction = true;
            direction.y *= -1.
        }

        if change_direction {
            if random::<f32>() > 0.5 {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_001.ogg"),
                    settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(0.5)),
                });
            } else {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_001.ogg"),
                    settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(0.5)),
                });
            }
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

pub fn enemy_hit_player(
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
    music_query: Query<&AudioSink, With<ExplosionClunchSound>>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                commands.spawn((
                    AudioBundle {
                        source: asset_server.load("audio/explosionCrunch_000.ogg"),
                        settings: PlaybackSettings::ONCE.with_volume(Volume::new(0.5)),
                    },
                    ExplosionClunchSound,
                ));
                println!("Game over!");
                commands.entity(player_entity).despawn();
            }
        }
    }
}
