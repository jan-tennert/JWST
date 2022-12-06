use bevy::prelude::{Plugin, Query, Name, Transform, Vec3, Commands, Handle, Res, AssetServer, Color, Component, ParamSet, IntoSystemDescriptor, ResMut, Assets, StandardMaterial, AlphaMode, Visibility, SystemSet};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_mod_picking::PickableBundle;
use bevy_text_mesh::{TextMeshFont, TextMeshPlugin, TextMeshBundle, TextMesh, TextMeshStyle, TextMeshSize, SizeUnit};

use crate::{body::update_bodies, camera::PanOrbitCamera, SimState};

pub struct LagrangePlugin;

#[derive(Component, Inspectable)]
pub struct LagrangePoint(pub f32); //distance from earth 10 unit = 1 AU

impl Plugin for LagrangePlugin {
    
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .register_inspectable::<LagrangePoint>()
        .add_plugin(TextMeshPlugin)
        .add_system_set(SystemSet::on_enter(SimState::Simulation).with_system(spawn_lagrange_points))
        .add_system_set(SystemSet::on_update(SimState::Simulation).with_system(calculate_lagrange_points.after(update_bodies)));
    }
    
}

fn spawn_lagrange_points(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let roboto: Handle<TextMeshFont> = assets.load("fonts/Roboto-Regular.ttf#mesh");
    let lagrange_points = vec![
        (Name::new("SE-L1"), LagrangePoint(0.1)), //0.1u = 0.01AU
        (Name::new("SE-L2"), LagrangePoint(-0.1))
    ];
    for (name, point) in lagrange_points {
        commands.spawn(TextMeshBundle {
            text_mesh: TextMesh {
                text: name.to_string(),
                style: TextMeshStyle {
                    color: Color::WHITE,
                    font: roboto.clone(),
                    font_size: SizeUnit::NonStandard(0.5),
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::from_xyz(-1., 1.75, 0.),
            visibility: Visibility::INVISIBLE,
            ..Default::default()
        })
        .insert(point)
        .insert(PickableBundle::default())
        .insert(name);
    }
}

pub fn calculate_lagrange_points(
    mut set: ParamSet<(
        Query<(&LagrangePoint, &mut Transform)>,
        Query<(&Name, &Transform)>,
        Query<(&PanOrbitCamera, &Transform)>
    )>
) {
    let mut sun: Option<Vec3> = None;
    let mut earth: Option<Vec3> = None;
    let camera_rot = set.p2().single().1.rotation;
    for (name, transform) in set.p1().iter() {
        if name.as_str() == "Earth" {
            earth = Some(transform.translation);
        } else if name.as_str() == "Sun" {
            sun = Some(transform.translation);
        }
    }
    if let (Some(spos), Some(epos)) = (sun, earth) {
        for (point, mut transform) in set.p0().iter_mut() {
            let normalized_pos = (spos - epos).normalize_or_zero() * point.0;
            let actual_pos = normalized_pos + epos;
            transform.translation = actual_pos;
            transform.rotation = camera_rot;
        }
    } 
}