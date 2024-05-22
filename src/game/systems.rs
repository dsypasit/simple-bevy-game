use bevy::prelude::*;

use crate::AppState;

use super::SimulationState;

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *simulation_state.get() == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Pause)));
            println!("pause");
        }
        if *simulation_state.get() == SimulationState::Pause {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("runing");
        }
    }
}

pub fn game_begin(mut commands: Commands) {
    commands.insert_resource(NextState(Some(SimulationState::Pause)))
}
