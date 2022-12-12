mod body;
mod camera;
mod ui;
mod input;
mod lagrange;
mod bodies;
mod skybox;
mod speed;
mod fps;
mod menu;
mod jwst;
mod reset;


use std::time::Duration;

use crate::bodies::Body;
use crate::body::{BodyBundle, Sun};
use crate::camera::*;
use bevy::app::{ScheduleRunnerSettings, RunMode};
use bevy::core_pipeline::{clear_color::ClearColorConfig, bloom::BloomSettings};
use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy::window::PresentMode;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle};
use body::{Gravity, BodyPlugin};
use fps::FpsPlugin;
use jwst::JWSTPlugin;
use lagrange::LagrangePlugin;
use menu::MenuPlugin;
use reset::ResetPlugin;
use skybox::SkyboxPlugin;
use speed::SpeedPlugin;
use ui::UIPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum SimState {
    Menu,
    Simulation,
    Reset,
    ExitToMainMenu   
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "James Webb Space Telescope GFS Physik".to_string(),
                resizable: true,
                present_mode: PresentMode::AutoVsync,
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
     //   .add_plugin(EguiPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugin(BodyPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(LagrangePlugin)
        .add_plugin(SkyboxPlugin)
        .add_plugin(SpeedPlugin)
        .add_plugin(FpsPlugin)
        .add_plugin(JWSTPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(ResetPlugin)
      //  .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state(SimState::Menu)
      //  .add_plugin(LinePlugin)
        .add_system_set(SystemSet::on_enter(SimState::Simulation).with_system(sim_setup))
        .run();
}

//mass scaled in 10^24 kg m
fn sim_setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut g: ResMut<Gravity>,
) {
    let bodies = vec![Body::earth(), Body::moon()/*, Body::saturn(), Body::venus(), Body::pluto(), Body::mercury(), Body::jupiter(), Body::mars(), Body::uranus()*/];

    let sun_body = BodyBundle::new(1_988_500.0, Vec3::ZERO, Vec3::ZERO);
    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            0.0, 0.0, 0.0,
        )))
        .insert(Name::new("Sun"))
        .insert(meshes.add(shape::UVSphere { radius: 0.6, ..default() }.into()))
        .insert(NotShadowCaster)
        .insert(PickableBundle::default())
        .insert(sun_body)
        .insert(PointLightBundle {
            point_light: PointLight {
                intensity: 10000.0,
                shadows_enabled: false,
                range: 600.0,
                radius: 0.6,
                ..default()
            },
            ..default()
        })
        .insert(NoFrustumCulling)
        .insert(Sun)
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: assets.load("models/sun.glb#Scene0"),
                transform: Transform::from_scale(Vec3::new(0.001, 0.001, 0.001)),
                ..Default::default()
            });
        });

    for body in bodies {
        commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            4.0, 7.0, 0.0,
        )))
        .insert(Name::new(body.name))
        .insert(meshes.add(shape::UVSphere { radius: body.radius, ..default() }.into()))
        .insert(PickableBundle::default())
        .insert(body.body)
        .with_children(|commands| {
            commands.spawn((
                SceneBundle {
                    scene: assets.load(body.model.as_str()),
                    transform: Transform::from_scale(Vec3::splat(body.model_scale)),
                    ..Default::default()
                },
            ));
        });
    }               
}