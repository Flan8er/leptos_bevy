use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub robot: Handle<Scene>,
}

pub fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        robot: asset_server.load("dummy_model_name_replace_me.glb#Scene0"),
    }
}
