use std::time::Duration;

use bevy::{prelude::{Component, Plugin, Commands, ResMut, Assets, Query, Transform, Handle, IntoSystemDescriptor, Vec3}, time::{Timer, TimerMode}, math::Vec3A};
use bevy_inspector_egui::{RegisterInspectable, Inspectable};
use bevy_polyline::{prelude::Polyline, PolylinePlugin};
use ringbuffer::{ConstGenericRingBuffer, RingBufferExt, RingBufferWrite};

use crate::body::{movement, EnableLines};

pub const TRAIL_LENGTH: usize = 1024;
const MINIMUM_ANGLE: f32 = 1.48341872;

#[derive(Component, Clone, Default, Debug)]
pub struct Trail(pub ConstGenericRingBuffer<Vec3, TRAIL_LENGTH>);

pub struct LinePlugin;

impl Plugin for LinePlugin {
    
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_plugin(PolylinePlugin)
        .add_system(update_trails.after(movement));
    }
    
}

fn update_trails(
    mut polylines: ResMut<Assets<Polyline>>,
    mut query: Query<(&EnableLines, &Transform, &mut Trail, &Handle<Polyline>)>,
) {
     query.for_each_mut(|(enable_lines, transform, mut trail, polyline)| {
        if let Some(position) = trail.0.back() {
            let last_vec = *position - transform.translation;
            let last_last_vec = if let Some(position) = trail.0.get(-2) {
                *position - transform.translation
            } else {
                last_vec
            };
            let gt_min_angle = last_vec.dot(last_last_vec) > MINIMUM_ANGLE;
            if gt_min_angle {
                trail.0.push(transform.translation);
                polylines.get_mut(polyline).unwrap().vertices =
                    trail.0.iter().map(|v| Vec3::from(*v)).collect()
            } else {
                // If the last point didn't actually add much of a curve, just overwrite it.
                if polylines.get_mut(polyline).unwrap().vertices.len() > 1 {
                    *trail.0.get_mut(-1).unwrap() = transform.translation;
                    *polylines
                        .get_mut(polyline)
                        .unwrap()
                        .vertices
                        .last_mut()
                        .unwrap() = transform.translation.into();
                }
            }
        } else {
            trail.0.push(transform.translation);
            polylines.get_mut(polyline).unwrap().vertices =
                trail.0.iter().map(|v| Vec3::from(*v)).collect()
        }
    });
}