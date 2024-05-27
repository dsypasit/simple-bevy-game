use bevy::{ecs::query, prelude::*, utils::info};

use crate::{
    game::components::ResumeButton,
    main_menu::styles::{
        get_button_style, get_text_style, HOVER_BUTTON_COLOR, NORMAL_BUTTON_COLOR,
        PREESED_BUTTON_COLOR,
    },
    AppState,
};

use super::{
    components::{MainMenuButton, PauseWindow},
    SimulationState,
};

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *simulation_state.get() == SimulationState::Running {
            next_simulation_state.set(SimulationState::Pause);
            println!("pause");
        }
        if *simulation_state.get() == SimulationState::Pause {
            next_simulation_state.set(SimulationState::Running);
            println!("runing");
        }
    }
}

pub fn game_begin(
    mut commands: Commands,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    next_simulation_state.set(SimulationState::Pause);
}

pub fn despawn_pause_window(mut commands: Commands, query: Query<Entity, With<PauseWindow>>) {
    info!("On Exit active!");
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive()
    }
}

pub fn spawn_pause_window(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_pause_window(&mut commands, &asset_server)
}

pub fn build_pause_window(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    info!("On Enter active!");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::rgba_u8(150, 150, 150, 200).into(),
                ..default()
            },
            PauseWindow {},
        ))
        .with_children(|p| {
            // resume
            p.spawn((
                ButtonBundle {
                    style: get_button_style(),
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                ResumeButton {},
            ))
            .with_children(|p| {
                p.spawn((TextBundle {
                    text: Text {
                        sections: vec![TextSection::new("Resume", get_text_style(asset_server))],
                        ..default()
                    },
                    ..default()
                },));
            });

            // main menu
            p.spawn((
                ButtonBundle {
                    style: get_button_style(),
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                MainMenuButton {},
            ))
            .with_children(|p| {
                p.spawn((TextBundle {
                    text: Text {
                        sections: vec![TextSection::new("Main Menu", get_text_style(asset_server))],
                        ..default()
                    },
                    ..default()
                },));
            });
        });
}

pub fn interact_with_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PREESED_BUTTON_COLOR.into();
                next_app_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => *background_color = HOVER_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_resume_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
    >,
    mut next_app_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PREESED_BUTTON_COLOR.into();
                next_app_state.set(SimulationState::Running);
            }
            Interaction::Hovered => *background_color = HOVER_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
