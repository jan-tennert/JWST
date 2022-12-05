use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

use crate::{setup, lagrange::{LagrangePoint, calculate_lagrange_points}, bodies::Body, body::body_focus};

pub struct JWSTPlugin;

impl Plugin for JWSTPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup_jwst.after(setup))
        .add_system(orbit_around_l2.after(calculate_lagrange_points));
    }
    
}

#[derive(Component)]
pub struct JWST;

fn setup_jwst(mut commands: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            4.0, 7.0, 0.0,
        )))
        .insert(Name::new("JWST"))
        .insert(meshes.add(shape::UVSphere { radius: 0.002, ..default() }.into()))
        .insert(PickableBundle::default())
        .insert(JWST)
        .insert(Body::jwst().body)
        .with_children(|commands| {
            commands.spawn((
                SceneBundle {
                    scene: assets.load("models/jwst.glb#Scene0"),
                    transform: Transform::from_scale(Vec3::splat(0.0003)),
                    ..Default::default()
                },
            ));
        });
}

pub fn orbit_around_l2(
    mut jwst: Query<(&mut Transform, &JWST, Without<LagrangePoint>)>,
    lagrange_points: Query<(&LagrangePoint, &Name, &Transform)>,
    time: Res<Time>
) {
    for (_, name, center) in lagrange_points.iter() {
        if name.as_str() == "SE-L2" {
                if let Ok(result) = jwst.get_single_mut() {
                let mut transform = result.0;
                let angle = 0.1 * time.elapsed_seconds() as f32;
                let x = 0.01 * angle.cos();
                let y = 0.01 * angle.sin();
                transform.translation = center.translation + Vec3::new(x, y, 0.0);
               // transform.rotation = Quat::from_rotation_x(angle) * Quat::from_rotation_y(angle);
            }
        }
    }
}