use bevy::{
    app::AppExit, audio::Volume, core::Zeroable, ecs::change_detection, prelude::*,
    window::PrimaryWindow,
};
use rand::seq::SliceRandom;
use rand::{random, Rng};

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 300.0;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_NUMBER: i32 = 10;
pub const ENEMIS_NUMBER: i32 = 4;
pub const ENEMY_SPEED: f32 = 300.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

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
        .init_resource::<HighestScore>()
        .init_resource::<starSpawnTimer>()
        .init_resource::<enemySpawnTimer>()
        .add_event::<GameOver>()
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
                enemy_movement,
                confine_player_movement.before(enemy_movement),
                update_enemy_movement,
                confine_enemy_movement,
                enemy_hit_player,
                player_hit_star,
                update_score,
                spawn_star_overtime,
                tick_star_spawn_timer,
                spawn_enemy_overtime,
                exist_game,
                handle_game_over,
                highest_score_updated,
                enemy_change_direction,
            ),
        )
        .run()
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Default)]
pub struct HighestScore {
    pub value: u32,
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

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<starSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
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
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
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

#[derive(Resource)]
pub struct starSpawnTimer {
    pub timer: Timer,
}
impl Default for starSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct enemySpawnTimer {
    pub timer: Timer,
}

impl Default for enemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
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
            EnemyChangeDirection::default(),
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

#[derive(Component)]
pub struct EnemyChangeDirection {
    pub timer: Timer,
    pub direction: f32,
}

impl Default for EnemyChangeDirection {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let time_to_change: f32 = rng.gen_range(1.0..5.0);
        let choices = vec![1.0, -1.0];
        let rand_direction = choices.choose(&mut rng).unwrap();
        Self {
            timer: Timer::from_seconds(time_to_change, TimerMode::Repeating),
            direction: *rand_direction,
        }
    }
}

pub fn enemy_change_direction(
    time: Res<Time>,
    mut query: Query<&mut EnemyChangeDirection, With<Enemy>>,
) {
    for (mut change_direction) in query.iter_mut() {
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
        let mut direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation +=
            direction * ENEMY_SPEED * time.delta_seconds() * change_direction.direction;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
    music_query: Query<&AudioSink, With<ExplosionClunchSound>>,
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
                commands.spawn((
                    AudioBundle {
                        source: asset_server.load("audio/explosionCrunch_000.ogg"),
                        settings: PlaybackSettings::ONCE.with_volume(Volume::new(0.5)),
                    },
                    ExplosionClunchSound,
                ));
                println!("Game over!");
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
                score.value = 0
            }
        }
    }
}

#[derive(Event)]
pub struct GameOver {
    score: u32,
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut highest_score: ResMut<HighestScore>,
) {
    for event in game_over_event_reader.read() {
        println!("last score: {}", event.score);
        if event.score > highest_score.value {
            highest_score.value = event.score;
        }
    }
}

pub fn highest_score_updated(highest_score: Res<HighestScore>) {
    if highest_score.is_changed() {
        println!("highest score updated! : {}", highest_score.value)
    }
}

pub fn exist_game(key_input: Res<ButtonInput<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if key_input.pressed(KeyCode::KeyQ) {
        event_writer.send(AppExit);
    }
}
