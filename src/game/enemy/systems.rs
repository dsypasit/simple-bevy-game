use crate::*;
use bevy::prelude::*;

use self::{
    game::player::{components::Player, PLAYER_SIZE},
    game::score::events::GameOver,
    game::score::resources::Score,
};

use super::{
    components::{Enemy, EnemyChangeDirection},
    resources::enemySpawnTimer,
    ENEMIS_NUMBER, ENEMY_SIZE, ENEMY_SPEED,
};

use rand::random;

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
            EnemyChangeDirection::default(),
        ));
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
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

        if translation.x <= min_x {
            translation.x = min_x
        }
        if translation.y <= min_y {
            translation.y = min_y
        }
        if translation.x >= max_x {
            translation.x = max_x
        }
        if translation.y >= max_y {
            translation.y = max_y
        }
        transform.translation = translation;
    });
}

pub fn spawn_enemy_overtime(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut enemy_timer: ResMut<enemySpawnTimer>,
) {
    let window = window_query.get_single().unwrap();
    let rand_x = random::<f32>() * window.width();
    let rand_y = random::<f32>() * window.height();

    enemy_timer.timer.tick(time.delta());
    if enemy_timer.timer.just_finished() {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random(), random()),
            },
            EnemyChangeDirection::default(),
        ));
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

        if translation.x <= min_x || translation.x >= max_x {
            change_direction = true;
            direction.x *= -1.
        }
        if translation.y <= min_y || translation.y >= max_y {
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
                    source: asset_server.load("audio/pluck_002.ogg"),
                    settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(0.5)),
                });
            }
        }
        enemy.direction = direction;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut score: ResMut<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                commands.spawn((AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    settings: PlaybackSettings::ONCE.with_volume(Volume::new(0.5)),
                },));
                println!("Game over!");
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
                score.value = 0
            }
        }
    }
}

pub fn enemy_change_direction(
    time: Res<Time>,
    mut query: Query<&mut EnemyChangeDirection, With<Enemy>>,
) {
    for mut change_direction in query.iter_mut() {
        change_direction.timer.tick(time.delta());
        if change_direction.timer.just_finished() {
            change_direction.direction *= -1.0
        }
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy, &EnemyChangeDirection)>,
    time: Res<Time>,
) {
    for (mut transform, enemy, change_direction) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation +=
            direction * ENEMY_SPEED * time.delta_seconds() * change_direction.direction;
    }
}
