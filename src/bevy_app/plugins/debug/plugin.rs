use bevy::prelude::*;

use crate::bevy_app::plugins::debug::axis::{draw_object_coordinates, draw_world_coordinates};

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (draw_world_coordinates, draw_object_coordinates));
    }
}
