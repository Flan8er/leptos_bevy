use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub fn spawn_camera(mut commands: Commands) {
    let initial = Vec3::new(8.0, 6.0, 8.0);
    let radius = initial.length();
    let yaw = initial.x.atan2(initial.z);
    let pitch = (initial.y / radius).asin();

    commands.spawn((
        Camera3d::default(),
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: Some(radius),
            yaw: Some(yaw),
            pitch: Some(pitch),
            ..default()
        },
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 300.,
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 1_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(Vec3::new(15.0, 15.0, -15.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
