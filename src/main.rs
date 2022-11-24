mod body;
mod camera;

use crate::body::BodyBundle;
use crate::camera::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle};
use body::{Gravity, BodyPlugin};

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
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    //  mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut g: ResMut<Gravity>,
) {
    let sun = assets.load("sun.glb#Scene0");
    let earth: Handle<Scene> = assets.load("earth.glb#Scene0");
    const AU_TO_UNIT_SCALE: f32 = 10.0;
    const DAY: f32 = 86_400.0;
      
    g.0 *= DAY * DAY * 10.0f32.powi(-6) / 1.5f32.powi(3);      
                                    
    let sun_body = BodyBundle::new(1_988_500.0, Vec3::ZERO, Vec3::ZERO);

    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            0.0, 0.0, 0.0,
        )))
        .insert(Name::new("Sun"))
        .insert(meshes.add(shape::UVSphere::default().into()))
        .insert(NotShadowCaster)
        .insert(PickableBundle::default())
        .insert(sun_body)
        .insert(PointLightBundle {
            point_light: PointLight {
                intensity: 10000.0,
                shadows_enabled: false,
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

    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            4.0, 7.0, 0.0,
        )))
        .insert(Name::new("Earth"))
        .insert(meshes.add(shape::UVSphere::default().into()))
        .insert(PickableBundle::default())
        .insert(BodyBundle::new(
            5.97219,
            Vec3::new(
                3.044170697902298E-01, 1.295114876282963E-01, -1.734104195212369E-02
            ) * AU_TO_UNIT_SCALE,
            Vec3::new(
                -1.648628006573339E-02, 2.713585294570181E-02, 3.729745700066048E-03
            ) * AU_TO_UNIT_SCALE
        ))
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: earth,
                transform: Transform::from_scale(Vec3::new(0.0002, 0.0002, 0.0002)),
                ..Default::default()
            });
        });

    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    //camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..Default::default()
        },
        PanOrbitCamera {
            radius,
            ..Default::default()
        },
        PickingCameraBundle::default(),
    ));
}
