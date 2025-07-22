use bevy::{
    math::{ops::atan2, FloatPow},
    prelude::*,
};
use bevy_panorbit_camera::PanOrbitCamera;

pub fn spawn_camera(mut commands: Commands) {
    let initial_camera_position = Vec3::new(50.0, 50.0, 50.0);
    let radius = (initial_camera_position.x.squared()
        + initial_camera_position.y.squared()
        + initial_camera_position.z.squared())
    .sqrt();
    let theta = atan2(initial_camera_position.x, initial_camera_position.y);
    let phi = (initial_camera_position.z / radius).acos();

    commands.spawn((
        Camera3d::default(),
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: Some(radius),
            yaw: Some(theta),
            pitch: Some(phi),
            ..default()
        },
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2000.,
    });
}
