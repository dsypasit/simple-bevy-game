use crate::*;
use bevy::prelude::*;

use self::{
    game::player::{components::Player, PLAYER_SIZE},
    game::score::resources::Score,
};

use super::{components::Star, resources::starSpawnTimer, STAR_NUMBER, STAR_SIZE};
use rand::random;

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
                transform: Transform::from_xyz(rand_x, rand_y, -10.0),
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

pub fn spawn_star_overtime(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<starSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let rand_x = random::<f32>() * window.width();
        let rand_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, -10.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<starSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn star_despawn(mut commands: Commands, query: Query<Entity, With<Star>>) {
    for star_entity in query.iter() {
        commands.entity(star_entity).despawn();
    }
}
