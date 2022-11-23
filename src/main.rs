mod body;
mod camera;

use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle};
use crate::camera::*;
use crate::body::body_focus;

fn main() {
    App::new()
     .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "James Webb Space Telescope GFS Physik".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..default()
        }))
    .add_plugin(WorldInspectorPlugin::new())
   // .add_startup_system(asset_loading)
    .add_plugins(DefaultPickingPlugins)
    .add_system(pan_orbit_camera)
    .add_system(body_focus)
    .add_startup_system(setup)
    .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
  //  mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let sun = assets.load("sun.glb#Scene0");
    let earth: Handle<Scene> = assets.load("earth.glb#Scene0");
    
    commands.spawn(SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)))
    .insert(Name::new("Sun"))
    .insert(meshes.add(shape::UVSphere::default().into()))
    .insert(NotShadowCaster)
    .insert(PickableBundle::default())
    .insert(PointLightBundle { 
         point_light: PointLight {
            intensity: 10000.0,
            shadows_enabled: false,
            range: 200.0,
            ..default()
        },
        ..default()
        
    })
    .with_children(|commands| {
        commands.spawn(SceneBundle {
            scene: sun,
            transform: Transform::from_scale(Vec3::new(0.002, 0.002, 0.002)),
            ..Default::default()
        });
    });

    
    commands.spawn(SpatialBundle::from_transform(Transform::from_xyz(4.0, 7.0, 0.0)))
    .insert(Name::new("Earth"))
    .insert(meshes.add(shape::UVSphere::default().into()))
    .insert(PickableBundle::default())
    .with_children(|commands| {
        commands.spawn(SceneBundle {
            scene: earth,
            transform: Transform::from_scale(Vec3::new(0.0005, 0.0005, 0.0005)),
            ..Default::default()
        });
    });
    
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    //camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            radius,
            ..Default::default()
        },
        PickingCameraBundle::default()
    ));
}
