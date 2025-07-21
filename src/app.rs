use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

#[component]
pub fn DummyPage() -> impl IntoView {
    match window().document().unwrap().body() {
        Some(body) => {
            let _ = body.style().set_property("background", "none transparent");
        }
        None => (),
    };

    view! {
        <BevyCanvas
            init=move || {
                init_bevy_app()
            }
        />
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="w-screen h-screen flex items-center justify-center overflow-hidden">
            <div class="w-full h-full">
                <iframe class="m-0 p-0 w-full h-full" src="/bevy_window"/>
            </div>
        </main>
    }
}

#[derive(Component)]
struct Particle {
    position: Vec3,
}

fn init_bevy_app() -> App {
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
    .init_resource::<SceneAssets>()
    .add_systems(Startup, (setup_ui, spawn_particles))
    .add_systems(Update, animate_sine_wave)
    .add_plugins(PanOrbitCameraPlugin);

    app
}

//
//
//
// THIS IS A DEMO FOR LOADING AND SPAWNING ASSETS
//
//
//
#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub robot: Handle<Scene>,
}

pub fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        robot: asset_server.load("6_axis_industrial_robot_arm.glb#Scene0"),
    }
}

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
//
//
//
// THIS IS A DEMO FOR LOADING AND SPAWNING ASSETS
//
//
//

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, -125., 25.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));

    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2000.,
    });
}

/// Calculates the unit radius for evenly distribued points inside a circle
fn radius(index: u32, total_points: u32, boundary_points: u32) -> f32 {
    if index > total_points - boundary_points {
        1.0
    } else {
        (index as f32 - 0.5).sqrt()
            / ((total_points as f32 - boundary_points as f32 + 1.0) / 2.0).sqrt()
    }
}

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let total_points: u32 = 5_000;
    let distribution: u32 = 1;
    let scale = 80.0;

    let boundary_points = (distribution as f32 * (total_points as f32).sqrt()) as u32;
    let phi = ((5.0_f32).sqrt() + 1.0) / 2.0;
    let golden_angle = std::f32::consts::TAU * (1.0 - 1.0 / phi);

    let mesh = meshes.add(Sphere::default());
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.208, 0.612, 1.),
        ..default()
    });

    for i in 0..total_points {
        let r = radius(i, total_points, boundary_points) * scale;
        let theta = i as f32 * golden_angle;

        let pos = Vec3::new(r * theta.cos(), r * theta.sin(), 0.0);

        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(pos).with_scale(Vec3::splat(0.5)),
            Particle { position: pos },
        ));
    }
}

fn animate_sine_wave(time: Res<Time>, mut query: Query<(&Particle, &mut Transform)>) {
    let t = time.elapsed_secs();

    let amplitude = 2.0; // wave height
    let wavelength = 30.0; // peak-to-peak distance
    let omega = 0.5; // wave propagation speed

    let k = std::f32::consts::TAU / wavelength; // spatial frequency
    for (particle, mut transform) in &mut query {
        let x = particle.position.x;
        let y = particle.position.y;
        let r = (x * x + y * y).sqrt();

        let phase = k * r + omega * t;
        let z = amplitude * phase.sin();

        transform.translation = Vec3::new(x, y, z);
    }
}
