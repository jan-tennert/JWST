use bevy::{prelude::*, render::view::NoFrustumCulling};
use bevy_mod_picking::PickableBundle;

use crate::{menu::setup, lagrange::{LagrangePoint, calculate_lagrange_points}, bodies::Body, body::{body_focus, Sun, Pause}, SimState, speed::Speed};

pub struct JWSTPlugin;

impl Plugin for JWSTPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(SimState::Simulation).with_system(setup_jwst.after(setup)))
        .add_system_set(SystemSet::on_update(SimState::Simulation).with_system(orbit_around_l2.after(calculate_lagrange_points)));
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
    sun: Query<(&Transform, &Sun, Without<LagrangePoint>, Without<JWST>)>,
    lagrange_points: Query<(&LagrangePoint, &Name, &Transform)>,
    time: Res<Time>,
    speed: Res<Speed>,
    pause: Res<Pause>
) {
    for (_, name, center) in lagrange_points.iter() {
        if name.as_str() == "SE-L2" && !pause.0 {
                if let Ok(result) = jwst.get_single_mut() {
                    if let Ok(sun) = sun.get_single() {
                        let mut jwst = result.0;
                        let sun_location = sun.0.translation;
                        let angle = time.elapsed_seconds() * speed.0;
                        // update the position of the entity based on the orbit radius and angle
                        jwst.translation = center.translation + Vec3::new(
                            0.02 * angle.cos(),
                            0.02 * angle.sin(),
                            0.0,
                        );   
                    }
            }
        }
    }
}