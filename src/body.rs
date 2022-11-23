use bevy::prelude::{Query, Transform, Name};
use bevy_mod_picking::Selection;
use crate::camera::PanOrbitCamera;

pub fn body_focus(
    mut query: Query<&mut PanOrbitCamera>,
    selection: Query<(&Transform, &Selection, &Name)>
) {
    for (transform, selection, name) in &selection {
        if selection.selected() {
            println!("{}", name.as_str());
            for mut camera in query.iter_mut() {
                if camera.focus != transform.translation {
                    camera.set_focus(transform.translation);
                }
            }
        }
    }
}