use bevy::prelude::{Plugin, App, Name, Query, ResMut, Mut};
use bevy_egui::*;
use bevy_mod_picking::Selection;

use crate::{body::Mass, input::BlockInputPlugin};

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
    mut query: Query<(&Name, &Selection, &mut Mass)>
) {
    for (name, selection, mut mass) in query.iter_mut() {
        if selection.selected() {
            egui::SidePanel::right("body_panel")
            .default_width(400.0)
            .resizable(false)
            .show(egui_context.ctx_mut(), | ui| {
                ui.heading(name.as_str());
                ui.label("Mass");
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
                })
            });
        }
    }
}