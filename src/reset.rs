use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, Selection};

use crate::{SimState, skybox::Skybox, speed::Speed, body::Pause, ui::SimTime};

pub struct ResetPlugin;

impl Plugin for ResetPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_exit(SimState::Simulation).with_system(clean_up))
        .add_system_set(SystemSet::on_update(SimState::ExitToMainMenu).with_system(switch_to_menu))
        .add_system_set(SystemSet::on_update(SimState::Reset).with_system(reset));
    }
    
}

fn clean_up(
    entities: Query<(Entity, &Name, With<Selection>, Without<Camera>, Without<Skybox>)>,
    mut speed: ResMut<Speed>,
    mut pause: ResMut<Pause>,
    mut sim_time: ResMut<SimTime>,
    mut commands: Commands
) {
    for (entity, _, _, _, _) in entities.iter() {
        commands.entity(entity).despawn_recursive()
    }
    speed.0 = 1.0;
    pause.0 = false;
    sim_time.0 = 0.0;
}

fn switch_to_menu(
    mut state: ResMut<State<SimState>>
) {
    println!("hi");
    let _ = state.set(SimState::Menu);
}

fn reset(
    mut state: ResMut<State<SimState>>
) {
    let _ = state.set(SimState::Simulation);
}