mod body;
mod camera;
mod ui;
mod input;

use crate::body::BodyBundle;
use crate::camera::*;
use bevy::core_pipeline::{clear_color::ClearColorConfig, bloom::BloomSettings};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle};
use body::{Gravity, BodyPlugin};
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
    let sun: Handle<Scene> = assets.load("sun.glb#Scene0");
    let earth: Handle<Scene> = assets.load("earth.glb#Scene0");
//    let saturn: Handle<Scene> = assets.load("saturn.glb#Scene0");
    let moon = assets.load("moon.glb#Scene0");
 //   let jwst: Handle<Scene> = assets.load("jwst.glb#Scene0");
    const AU_TO_UNIT_SCALE: f32 = 10.0;
    const DAY: f32 = 86_400.0;
      
    g.0 *= DAY * DAY * 10.0f32.powi(-6) / 1.5f32.powi(3);      
                                    
    let sun_body = BodyBundle::new(1_988_500.0, Vec3::ZERO, Vec3::ZERO);
   //let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());

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

            

    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            4.0, 7.0, 0.0,
        )))
        .insert(Name::new("Earth"))
        .insert(meshes.add(shape::UVSphere { radius: 0.005, ..default() }.into()))
        .insert(PickableBundle::default())
        .insert(BodyBundle::new(
            5.97219,
            Vec3::new(
                4.487758087146768E-01, 8.751235324844499E-01, 1.618817013329493E-04
            ) * AU_TO_UNIT_SCALE,
            Vec3::new(
                -1.552868871220300E-02, 7.906229533085379E-03, 3.064648367334892E-07
            ) * AU_TO_UNIT_SCALE
        ))
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: earth,
                transform: Transform::from_scale(Vec3::new(0.00001, 0.00001, 0.00001)),
                ..Default::default()
            });
        });
            
   /* commands.spawn(SpatialBundle::from_transform(Transform::from_xyz(
            4.0, 7.0, 0.0,
        )))
        .insert(Name::new("Saturn"))
        .insert(meshes.add(shape::UVSphere::default().into()))
        .insert(PickableBundle::default())
        .insert(BodyBundle::new(
            568.34 ,
            Vec3::new(
                8.032503665636328E+00, -5.674419409731062E+00, -2.211472254846864E-01
            ) * AU_TO_UNIT_SCALE,
            Vec3::new(
                2.906095271828988E-03, 4.545286691593917E-03, -1.944528757086951E-04
            ) * AU_TO_UNIT_SCALE
        ))
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: saturn,
                transform: Transform::from_scale(Vec3::new(0.0009, 0.0009, 0.0009)),
                ..Default::default()
            });
        });        */
    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            4.0, 7.0, 0.0,
        )))
        .insert(Name::new("Moon"))
        .insert(meshes.add(shape::UVSphere { radius: 0.002, ..default() }.into()))
        .insert(PickableBundle::default())
        .insert(BodyBundle::new(
            0.0734767,
            Vec3::new(
               4.482115265952957E-01, 8.727621196450731E-01, 3.888179917645140E-05
            ) * AU_TO_UNIT_SCALE,
            Vec3::new(
              -1.491883668334010E-02, 7.773993419863166E-03, -4.679176055656679E-05
            ) * AU_TO_UNIT_SCALE
        ))
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: moon,
                transform: Transform::from_scale(Vec3::new(0.000003, 0.000003, 0.000003)),
                ..Default::default()
            });
        }); 
        
    /*commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            4.0, 7.0, 0.0,
        )))
        .insert(Name::new("JWST"))
        .insert(meshes.add(shape::UVSphere::default().into()))
        .insert(PickableBundle::default())
        .insert(BodyBundle::new(
            6200.0 * f32::powf(10.0, -22.0),
            Vec3::new(
               4.488948840878112E-01, 8.856860339483225E-01, -7.512566561474845E-04
            ) * AU_TO_UNIT_SCALE,
            Vec3::new(
              -1.564139006806661E-02, 7.940335006606503E-03, -9.026475694712961E-05
            ) * AU_TO_UNIT_SCALE
        ))
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: jwst,
                transform: Transform::from_scale(Vec3::new(0.03, 0.03, 0.03)),
                ..Default::default()
            });
        });           
*/
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