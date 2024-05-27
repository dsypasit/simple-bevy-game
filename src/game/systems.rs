use bevy::prelude::*;

use crate::AppState;

use super::SimulationState;

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
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            next_simulation_state.set(SimulationState::Running);
            println!("runing");
        }
    }
}

pub fn game_begin(
    mut commands: Commands,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    commands.insert_resource(NextState(Some(SimulationState::Pause)));
    next_simulation_state.set(SimulationState::Pause);
}
