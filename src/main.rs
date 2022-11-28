mod body;
mod camera;
mod ui;
mod input;
mod lagrange;
mod lines;
mod bodies;

use std::str::FromStr;

use crate::bodies::Body;
use crate::body::BodyBundle;
use crate::lines::{Trail, TRAIL_LENGTH};
use crate::camera::*;
use bevy::core_pipeline::{clear_color::ClearColorConfig, bloom::BloomSettings};
use bevy::math::Vec3A;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle};
use bevy_polyline::prelude::{PolylineBundle, PolylineMaterial, Polyline};
use body::{Gravity, BodyPlugin};
use lagrange::LagrangePlugin;
use lines::LinePlugin;
use ringbuffer::ConstGenericRingBuffer;
use ui::UIPlugin;




fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "James Webb Space Telescope GFS Physik".to_string(),
                resizable: true,
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugin(BodyPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(LagrangePlugin)
     //   .add_plugin(LinePlugin)
        .add_startup_system(setup)
        .run();
}

//mass scaled in 10^24 kg m
fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut g: ResMut<Gravity>,
) {
    let sun: Handle<Scene> = assets.load("sun.glb#Scene0");
    let earth: Handle<Scene> = assets.load("earth.glb#Scene0");
    let saturn: Handle<Scene> = assets.load("saturn.glb#Scene0");
    let venus: Handle<Scene> = assets.load("venus.glb#Scene0");
    let pluto: Handle<Scene> = assets.load("pluto.glb#Scene0");
    let jupiter: Handle<Scene> = assets.load("jupiter.glb#Scene0");
    let mercury: Handle<Scene> = assets.load("mercury.glb#Scene0");
    let mars: Handle<Scene> = assets.load("mars.glb#Scene0");
    let uranus: Handle<Scene> = assets.load("uranus.glb#Scene0");
    let moon = assets.load("moon.glb#Scene0");
//    let jwst: Handle<Scene> = assets.load("jwst.glb#Scene0");
  //  let hubble = assets.load("hubble.glb#Scene0");
    let bodies = vec![Body::earth(earth), Body::moon(moon), Body::saturn(saturn), Body::venus(venus), Body::pluto(pluto), Body::mercury(mercury), Body::jupiter(jupiter), Body::mars(mars), Body::uranus(uranus)];
    
    const DAY: f32 = 86_400.0;
      
    g.0 *= DAY * DAY * 10.0f32.powi(-6) / 1.5f32.powi(3);      
                                    
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
                intensity: 100000.0,
                shadows_enabled: true,
                range: 200.0,
                ..default()
            },
            ..default()
        })
        .insert(NoFrustumCulling)
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: sun,
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
                    scene: body.model,
                    transform: Transform::from_scale(Vec3::splat(body.model_scale)),
                    ..Default::default()
                },
            ));
        });
    }               

    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    //camera
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection { near: 0.0001, ..default() }),
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..Default::default()
        },
        BloomSettings::default(),
        PanOrbitCamera {
            radius,
            ..Default::default()
        },
        PickingCameraBundle::default(),
    ));
}