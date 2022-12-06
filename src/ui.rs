use bevy::{prelude::{Plugin, App, Name, Query, ResMut, Mut, Entity, Commands, DespawnRecursiveExt, Transform, Vec3, Res, Resource, IntoSystemDescriptor, ParamSet, Visibility, Without, With, MaterialMeshBundle, Camera, SystemSet, PointLight}, time::Time, diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin}, render::FrameCountPlugin};
use bevy_egui::*;
use bevy_inspector_egui::{egui::{TextEdit, RichText}, Inspectable, RegisterInspectable};
use bevy_mod_picking::Selection;
use chrono::{NaiveDate, Days};

use crate::{body::{Mass, Velocity, Acceleration, update_bodies}, input::BlockInputPlugin, lagrange::LagrangePoint, skybox::{CubemapMaterial, Skybox}, speed::Speed, fps::Fps, SimState};

#[derive(Resource, Inspectable, Default)]
pub struct SimTime(f32);

#[derive(Resource, Inspectable, Default)]
pub struct Light {
    
    pub shadows_enabled: bool
    
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
       // .add_plugin(EguiPlugin)
        .register_inspectable::<SimTime>()
        .init_resource::<SimTime>()
        .add_plugin(BlockInputPlugin)
        .add_system_set(SystemSet::on_update(SimState::Simulation).with_system(system_ui.after(time_ui)))
        .add_system_set(SystemSet::on_update(SimState::Simulation).with_system(body_ui.after(update_bodies)))
        .add_system_set(SystemSet::on_update(SimState::Simulation).with_system(time_ui.after(body_ui)));
    }
}

pub fn time_ui(
   time: Res<Time>,
   mut sim_time: ResMut<SimTime>,
   mut egui_context: ResMut<EguiContext>,
   mut speed: ResMut<Speed>,
   fps: Res<Fps>
) {
    sim_time.0 += time.delta_seconds() * speed.0;
    let date = NaiveDate::from_ymd_opt(2022, 11, 25).unwrap().checked_add_days(Days::new(sim_time.0.round() as u64)).unwrap();
    egui::TopBottomPanel::bottom("time_panel")
    .resizable(false)
    .show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.small_button("<<").clicked() {
                speed.0 /= 10.0;
            }
            if ui.small_button("<").clicked() {
                speed.0 /= 2.0;
            }
            let e = if speed.0 == 1.0 {""} else {"e"};
            ui.label(format!("{} ({} Tag{} / s)", date.format("%d.%m.%Y"), speed.0, e));
            if ui.small_button(">").clicked() {
                speed.0 *= 2.0;
            }
            if ui.small_button(">>").clicked() {
                speed.0 *= 10.0;
            }
        });
        ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
            ui.label(format!("{:.0} FPS", fps.0));   
        });
    });
}

pub fn system_ui(
    mut egui_context: ResMut<EguiContext>,
    mut body_query: Query<(&Name, &mut Selection, &mut Visibility, Without<LagrangePoint>)>,
    mut lagrange_point_query: Query<(&Name, &mut Selection, &mut Visibility, With<LagrangePoint>)>,
    mut skybox: Query<(&mut Visibility, &Skybox, Without<LagrangePoint>, Without<Selection>, Without<Name>)>,
    mut camera: Query<&mut Camera>,
    mut light: Query<&mut PointLight>
) {
    let mut points: Vec<(&Name, Mut<Selection>)> = Vec::new();
    let mut selected_body: Option<&str> = None;
    
    egui::SidePanel::left("system_panel")
    .default_width(400.0)
    .resizable(true)
    .show(egui_context.ctx_mut(), | ui| {
        ui.heading("Bodies");
        for (name, mut selected, mut visibility, _) in body_query.iter_mut() {
            ui.horizontal(|ui| {
                ui.checkbox(&mut visibility.is_visible, "");
                if ui.button(name.as_str()).clicked() {
                    selected.set_selected(true);
                    selected_body = Some(name.as_str());
                }
            });
            points.push((name, selected));
        }
        ui.heading("Lagrange Points");
        for (name, mut selected, mut visibility, _) in lagrange_point_query.iter_mut() {
            ui.horizontal(|ui| {
                ui.checkbox(&mut visibility.is_visible, "");
                if ui.button(name.as_str()).clicked() {
                    selected.set_selected(true);
                    selected_body = Some(name.as_str());
                }
            });
            points.push((name, selected))
        }
        ui.heading("Options");
        if let Ok(mut visible) = skybox.get_single_mut() {
           ui.checkbox(&mut visible.0.is_visible, "Milky Way background"); 
        }
        if let Ok(mut camera) = camera.get_single_mut() {
            ui.checkbox(&mut camera.hdr, "HDR/Bloom");
        }
        if let Ok(mut light) = light.get_single_mut() {
            ui.checkbox(&mut light.shadows_enabled, "Shadows");
        }
    });
    if let Some(selected_name) = selected_body {
        for (name, mut selection) in points {
            if selected_name != name.as_str() {
                selection.set_selected(false);
            }
        }
    }
}

fn body_ui(
    mut egui_context: ResMut<EguiContext>,
    mut commands: Commands,
    mut query: Query<(&Name, &Selection, Entity, &Transform, &Velocity, &mut Mass)>
) {
    let sun_pos = Vec3::splat(0.0);
    for (name, selection, entity, transform, velocity, mut mass) in query.iter_mut() {
        if selection.selected() {
            egui::SidePanel::right("body_panel")
            .max_width(250.0)
            .resizable(true)
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
                ui.label(RichText::new("Velocity").size(16.0).underline());
                ui.label(format!("{:.3} km/s", velocity.0.length() / 10.0 * 1731.0));
                // Distance from Sun
                ui.label(RichText::new("Distance from sun").size(16.0).underline());
                let distance_in_au = transform.translation.distance(sun_pos) / 10.0;
                ui.label(format!("{} km", (distance_in_au * 1.496e+8) as f64));
                ui.label(format!("{:.3} au", distance_in_au));
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    if ui.button("Delete").clicked() {
                        commands.entity(entity).despawn_recursive()
                    }    
                });
            });
            
        }
    }
}