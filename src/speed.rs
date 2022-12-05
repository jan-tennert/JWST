use bevy::prelude::*;

pub struct SpeedPlugin;

impl Plugin for SpeedPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Speed>();
    }
    
}

#[derive(Resource, Debug)]
pub struct Speed(pub f32);

impl Default for Speed {
    
    fn default() -> Self {
        Self(1.0)
    }
    
}