use crate::*;
use bevy::prelude::*;

use self::score::resources::HighestScore;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2.0, 0.0),
        ..default()
    });
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

pub fn exist_game(key_input: Res<ButtonInput<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if key_input.pressed(KeyCode::KeyQ) {
        event_writer.send(AppExit);
    }
}
