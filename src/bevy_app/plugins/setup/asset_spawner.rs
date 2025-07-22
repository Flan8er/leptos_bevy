use bevy::prelude::*;

use crate::bevy_app::plugins::setup::asset_loader::SceneAssets;

#[derive(Component)]
pub struct Robot;

pub fn spawn_robot(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        Robot,
        SceneRoot(scene_assets.robot.clone()),
        Transform {
            ..Default::default()
        },
    ));
}
