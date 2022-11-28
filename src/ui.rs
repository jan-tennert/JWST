use bevy::prelude::{Plugin, App, Name, Query, ResMut, Mut, Entity, Commands, DespawnRecursiveExt, Transform};
use bevy_egui::*;
use bevy_inspector_egui::egui::{TextEdit, RichText};
use bevy_mod_picking::Selection;

use crate::{body::{Mass, Velocity, Acceleration}, input::BlockInputPlugin};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
       // .add_plugin(EguiPlugin)
        .add_plugin(BlockInputPlugin)
        .add_system(system_ui)
        .add_system(body_ui);
    }
}

pub fn system_ui(
    mut egui_context: ResMut<EguiContext>,
    mut query: Query<(&Name, &mut Selection)>
) {
    let mut bodies: Vec<(&Name, Mut<Selection>)> = Vec::new();
    let mut selected_body: Option<&str> = None;
    egui::SidePanel::left("system_panel")
    .default_width(400.0)
    .resizable(false)
    .show(egui_context.ctx_mut(), | ui| {
        ui.heading("Bodies");
        for (name, mut selected) in query.iter_mut() {
            if ui.button(name.as_str()).clicked() {
                selected.set_selected(true);
                selected_body = Some(name.as_str());
            }
            bodies.push((name, selected));
        }
    });
    if let Some(selected_name) = selected_body {
        for (name, mut selection) in bodies {
            if selected_name != name.as_str() {
                selection.set_selected(false);
            }
        }
    }
}

fn body_ui(
    mut egui_context: ResMut<EguiContext>,
    mut commands: Commands,
    mut query: Query<(&Name, &Selection, Entity, &Transform, &Velocity, &Acceleration, &mut Mass)>
) {
    for (name, selection, entity, transform, velocity, acceleration, mut mass) in query.iter_mut() {
        if selection.selected() {
            
            egui::SidePanel::right("body_panel")
            .max_width(250.0)
            .resizable(false)
            .show(egui_context.ctx_mut(), | ui| {
                ui.heading(name.as_str());
                
                
                //Mass block
                ui.label(RichText::new("Mass").size(16.0).underline());
                ui.horizontal(|ui| {
                    let mut new_mass = mass.0.to_string();
                    if ui.add(TextEdit::singleline(&mut new_mass).desired_width(100.0)).changed() {
                        if let Ok(f_mass) = new_mass.parse::<f32>() {
                            mass.0 = f_mass;
                        }
                    }
                    ui.label("10^24 kg");
                });
                ui.horizontal(|ui| {
                    if ui.button(":10").clicked() {
                        mass.0 /= 10.0;
                    }
                    if ui.button(":2").clicked() {
                        mass.0 /= 2.0;
                    }
                    if ui.button("x10").clicked() {
                        mass.0 *= 10.0;
                    }
                    if ui.button("x2").clicked() {
                        mass.0 *= 2.0;
                    }
                });
                // Position
                ui.label(RichText::new("Vector Position (unit)").size(16.0).underline());
                ui.label(format!("X: {:.2} Y: {:.2} Z: {:.2}", transform.translation.x, transform.translation.y, transform.translation.z));
                // Velocity
                ui.label(RichText::new("Vector Velocity (unit/s)").size(16.0).underline());
                ui.label(format!("X: {:.2} Y: {:.2} Z: {:.2}", velocity.0.x, velocity.0.y, velocity.0.z));
                
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    if ui.button("Delete").clicked() {
                        commands.entity(entity).despawn_recursive()
                    }    
                });
            });
            
        }
    }
}