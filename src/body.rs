use crate::{camera::PanOrbitCamera, lagrange::calculate_lagrange_points, speed::Speed, jwst::{JWST, orbit_around_l2}, SimState};
use bevy::{
    prelude::{
        App, Bundle, Component, IntoSystemDescriptor, Mut, Name, Plugin, Query,
        Res, Resource, SystemLabel, SystemSet, Transform, Vec3, Deref, Without
    },
    time::Time,
};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_mod_picking::Selection;

pub const G: f32 = 6.67430e-11_f32; //gravitational constant

#[derive(Resource, Inspectable)]
pub struct Gravity(pub f32);

impl Default for Gravity {
    fn default() -> Self {
        Self(G)
    }
}

#[derive(Default, Component, Inspectable)]
pub struct Velocity(pub Vec3);

#[derive(Default, Component, Inspectable)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Inspectable)]
pub struct Mass(pub f32);

#[derive(Component, Inspectable)]
pub struct EnableLines(pub bool);

impl Default for EnableLines {
    
    fn default() -> Self {
        EnableLines(true)
    }
    
}

#[derive(Deref, Component, Inspectable)]
pub struct Lines(Vec<(Vec3, Vec3)>);

impl Default for Lines {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Bundle, Inspectable)]
pub struct BodyBundle {
    mass: Mass,
    transform: Transform,
    vel: Velocity,
    acc: Acceleration,
    lines: Lines,
    enable_lines: EnableLines
}

impl BodyBundle {
    pub fn new(mass: f32, pos: Vec3, vel: Vec3) -> Self {
        Self {
            mass: Mass(mass),
            transform: Transform::from_translation(pos),
            vel: Velocity(vel),
            acc: Acceleration::default(),
            lines: Lines::default(),
            enable_lines: EnableLines::default()
        }
    }
}

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gravity>()
            .register_inspectable::<Gravity>()
            .register_inspectable::<Mass>()
            .register_inspectable::<Velocity>()
            .register_inspectable::<Acceleration>()
            .register_inspectable::<BodyBundle>()
            .register_inspectable::<Lines>()
            .add_system_set(SystemSet::on_update(SimState::Simulation).with_system(body_focus.after(orbit_around_l2)))
            .add_system_set(SystemSet::on_update(SimState::Simulation).with_system(update_bodies));
    }
}

pub fn update_bodies(
    g: Res<Gravity>,
    mut query: Query<(&Name, &Mass, &mut Transform, &mut Acceleration, &mut Velocity, Without<JWST>)>,
    time: Res<Time>,
    speed: Res<Speed>
) {
    let mut bodies: Vec<(&Mass, Mut<Transform>, Mut<Acceleration>)> = Vec::new();
    for (_, mass, transform, mut acc, _, _) in query.iter_mut() {
        acc.0 = Vec3::ZERO;
        for (other_mass, other_pos, other_acc) in bodies.iter_mut() {
            let diff = other_pos.translation - transform.translation;
            if let Some(mut force) = diff.try_normalize() {
                let magnitude = g.0 * mass.0 * other_mass.0 / diff.length_squared();
                force *= magnitude;
                acc.0 += force;
                other_acc.0 -= force;
            }
        }
        bodies.push((mass, transform, acc));
    }
    for (_m, mass, mut transform, mut acc, mut vel, _) in query.iter_mut() {
        acc.0 /= mass.0;
        vel.0 += acc.0 * time.delta_seconds() * speed.0;
        transform.translation += vel.0 * time.delta_seconds() * speed.0;
    }
}

pub fn body_focus(
    mut query: Query<&mut PanOrbitCamera>,
    selection: Query<(&Transform, &Selection, &Name)>,
) {
    for (transform, selection, _) in &selection {
        if selection.selected() {
       // println!("{}", name);               
            for mut camera in query.iter_mut() {
                if camera.focus != transform.translation {
                    camera.set_focus(transform.translation);
                }
            }
        }
    }
}