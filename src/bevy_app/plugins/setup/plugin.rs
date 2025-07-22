use bevy::prelude::*;

use crate::bevy_app::plugins::setup::{
    asset_loader::{load_assets, SceneAssets},
    asset_spawner::spawn_robot,
    camera::spawn_camera,
};

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, (load_assets, spawn_robot, spawn_camera).chain());
    }
}
