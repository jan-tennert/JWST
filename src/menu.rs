use bevy::{prelude::*, core_pipeline::{clear_color::ClearColorConfig, bloom::BloomSettings}};
use bevy_egui::EguiContext;
use bevy_inspector_egui::egui::{Frame, RichText};
use bevy_mod_picking::PickingCameraBundle;
use bevy_egui::*;

use crate::{SimState, camera::PanOrbitCamera};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(SimState::Menu).with_system(setup))
        .add_system_set(SystemSet::on_update(SimState::Menu).with_system(main_menu));
    }
}
pub fn setup(
    camera: Query<&Camera>,
    mut commands: Commands
) {
    if let Err(_) = camera.get_single() {
        let translation = Vec3::new(-2.0, 2.5, 5.0);
        let radius = translation.length();
        commands.spawn((
            Camera3dBundle {
                projection: Projection::Perspective(PerspectiveProjection { near: 0.0001, ..default() }),
                transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(Color::BLACK),
                    ..default()
                },
                camera: Camera {
                    hdr: false,
                    ..default()   
                },
                ..Default::default()
            },
            BloomSettings { threshold: 0.5, ..default() },
            PanOrbitCamera {
                radius,
                ..Default::default()
            },
            PickingCameraBundle::default(),
        ));   
    }
}

fn main_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<SimState>>
) {
    egui::CentralPanel::default().frame(Frame::none()).show(egui_context.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::from_main_dir_and_cross_align(egui::Direction::BottomUp, egui::Align::Center), |ui| {
            if ui.button("Start Simulation").clicked() {
                let _ = state.set(SimState::Simulation);   
            }
        })
    });
}