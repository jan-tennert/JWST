use bevy::prelude::{Plugin, Query, Name, Transform};

use crate::body::Mass;

pub struct LagrangePlugin;

impl Plugin for LagrangePlugin {
    
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_system(calculate_lagrange_points);
    }
    
}

fn calculate_lagrange_points(
    bodies: Query<(&Mass, &Name, &Transform)>
) {
    let mut sun: Option<(&Mass, &Transform)> = None;
    let mut earth: Option<(&Mass, &Transform)> = None;
    for (mass, name, transform) in bodies.iter() {
        if name.as_str() == "Earth" {
            earth = Some((mass, transform));
        } else if name.as_str() == "Sun" {
            sun = Some((mass, transform));
        }
    }
    if let (Some((smass, spos)), Some((emass, epos))) = (sun, earth) {
        let distance = spos.translation.distance(epos.translation);
    } 
}