use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_panorbit_camera::PanOrbitCameraPlugin;

use crate::bevy_app::plugins::{
    debug::plugin::DebugPlugin, particle::plugin::ParticlePlugin, setup::plugin::SetupPlugin,
};

pub fn init_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#bevy_canvas".into()),
                    transparent: true,
                    decorations: false,
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
    )
    .insert_resource(ClearColor(Color::NONE))
    .add_plugins(PanOrbitCameraPlugin)
    .add_plugins(DebugPlugin)
    .add_plugins(SetupPlugin)
    .add_plugins(ParticlePlugin);

    app
}
