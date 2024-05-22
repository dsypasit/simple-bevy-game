use crate::*;
use bevy::prelude::*;

use self::game::score::{events::GameOver, resources::HighestScore};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn transition_to_game_state(
    mut commands: Commands,
    key_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if key_input.just_pressed(KeyCode::KeyG) {
        if *app_state.get() != AppState::Game {
            commands.insert_resource(NextState(Some(AppState::Game)));
            println!("transition to state: game");
        }
    }
}

pub fn transition_to_menu_state(
    mut commands: Commands,
    key_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if key_input.just_pressed(KeyCode::KeyM) {
        if *app_state.get() != AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
            println!("transition to state: main menu");
        }
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut highest_score: ResMut<HighestScore>,
    mut commands: Commands,
) {
    for event in game_over_event_reader.read() {
        println!("last score: {}", event.score);
        if event.score > highest_score.value {
            highest_score.value = event.score;
        }
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}

pub fn exist_game(key_input: Res<ButtonInput<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if key_input.pressed(KeyCode::KeyQ) {
        event_writer.send(AppExit);
    }
}
