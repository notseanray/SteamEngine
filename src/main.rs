#![feature(trait_alias)]
use bevy::{asset::LoadState, prelude::*};
use std::f64::consts::PI;
mod concepts;
use smooth_bevy_cameras::{LookTransform, LookTransformPlugin, controllers::{fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin}, unreal::{UnrealCameraController, UnrealCameraBundle, UnrealCameraPlugin}}};
use bevy_rapier3d::prelude::*;

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(4.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 16.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

fn rad(theta: f64) -> f64 {
    theta * PI / 180.0
}

fn basketball() {
    println!("Hello, world!");
    let mut tests = 0;
    let h = 1.0;
    // let v_mph = 2.23694;
    // let v_0 = (v_mph * 5280.0 * 12.0) / (60.0 * 60.0);
    let v_0 = 1.0;
    let mut d_theta = 0.1;
    let mut dt = 0.0001;
    let start_theta = 90.0;
    let mut current_theta: f64 = start_theta;
    let mut min_theta = 0.0;
    let mut current_max = 0.0;
    let mut current_angle_max = 0.0;
    let k = 0.54;
    let omega_earth = 7.2921159e-5;
    let lattitude: f64 = 33.927987;
    // kg
    let mass = 0.5903;
    while current_theta > min_theta {
        let mut current_pos_x = 0.0;
        let mut current_pos_y = h;
        let mut current_v_x = v_0 * rad(current_theta).cos();
        let mut current_v_y = v_0 * rad(current_theta).sin();
        let mut t = 0.0;
        while current_pos_y > 0.0 {
            current_pos_y += dt * current_v_y * t - 0.5 * (9.81 * t * t);
            current_pos_x += t * current_v_x * dt;
            current_v_x -= (k * current_v_x * current_v_x) * dt - 2.0 * mass * current_v_x * omega_earth * lattitude.sin() * dt;
            current_v_y -= k * current_v_y * current_v_y;
            t += dt;
        }
        current_theta -= d_theta;
        if current_pos_x > current_max {
            current_max = current_pos_x;
            current_angle_max = current_theta;
        }
        tests += 1;
        println!("{current_pos_x}\t\t\t\t:{current_theta}");
    }
    println!("{current_max} {current_angle_max}");
    panic!();
    println!("second pass");
    current_theta = current_angle_max + d_theta;
    min_theta = current_angle_max - d_theta;
    d_theta = 0.00001;
    dt = 0.00001;
    while current_theta > min_theta {
        let mut current_pos_x = 0.0;
        let mut current_pos_y = h;
        let mut current_v_x = v_0 * rad(current_theta).cos();
        let mut current_v_y = v_0 * rad(current_theta).sin();
        let mut t = 0.0;
        while current_pos_y > 0.0 {
            current_pos_y = h + current_v_y * t - 0.5 * (9.81 * t * t);
            current_pos_x = t * current_v_x;
            current_v_x += -(k * current_v_x * current_v_x) + 2.0 * mass * current_v_x * omega_earth * lattitude.sin() * dt;
            current_v_y -= k * current_v_y * current_v_y;
            t += dt;
        }
        current_theta -= d_theta;
        if current_pos_x > current_max {
            current_max = current_pos_x;
            current_angle_max = current_theta;
        }
        tests += 1;
        println!("{current_pos_x}\t\t\t\t:{current_theta}");
    }
    println!("{current_max}");
    println!("{current_angle_max}");
    println!("iterations: {tests}");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0),
        point_light: PointLight {
            intensity: 600000.,
            range: 100.,
            ..default()
        },
        ..default()
    });
    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(Collider::from_bevy_mesh(
    //
    //             (PbrBundle {
    //                 transform: Transform::from_xyz(17.0, 0.5, 0.0).with_scale(Vec3 { x: 0.1, y: 0.1, z: 0.1 }).with_rotation(Quat::from_rotation_z(8.0)),
    //                 mesh: asset_server.load("Gear.stl"),
    //                 material: materials.add(Color::rgb(0.9, 0.4, 0.3).into()),
    //                 ..Default::default()
    //             }).mesh.to_owned()
    //             ))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 70.0)));
    commands
        .spawn(PbrBundle {
            transform: Transform::from_xyz(5.0, 0.5, 0.0).with_scale(Vec3 { x: 0.1, y: 0.1, z: 0.1 }),
            mesh: asset_server.load("Gear.stl"),
            material: materials.add(Color::rgb(0.9, 0.4, 0.3).into()),
            ..Default::default()
        });
    commands
        .spawn(PbrBundle {
            transform: Transform::from_xyz(17.0, 0.5, 0.0).with_scale(Vec3 { x: 0.1, y: 0.1, z: 0.1 }).with_rotation(Quat::from_rotation_z(8.0)),
            mesh: asset_server.load("Gear.stl"),
            material: materials.add(Color::rgb(0.9, 0.4, 0.3).into()),
            ..Default::default()
        });
    commands
        .spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController {
                mouse_rotate_sensitivity: Vec2 { x: 0.05, y: 0.05 },
                translate_sensitivity: 6.0,
                ..Default::default()
            },
            Vec3::new(20.0, 20.0, 20.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
    // .insert(UnrealCameraBundle::new(
    //         UnrealCameraController::default(),
    //         Vec3::new(-2.0, 5.0, 5.0),
    //         Vec3::new(0., 0., 0.),
    //         Vec3::Y,
    //     ));
}

fn main() {
    App::new()
        // .add_plugin(AssetPlugin {
        //     watch_for_changes: true,
        //     ..Default::default()
        // })
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_stl::StlPlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(FpsCameraPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        // .add_plugin(UnrealCameraPlugin::default())
        .run();
}

struct PIDController {
    kp: f64,
    ki: f64,
    kd: f64,
    bias: f64,
    e_prior: f64,
    i_prior: f64,
}

impl PIDController {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self {
            kp,
            ki,
            kd,
            bias: 0.0,
            e_prior: 0.0,
            i_prior: 0.0,
        }
    }
    pub fn set_bias(&mut self, bias: f64) {
        self.bias = bias;
    }
    // dt in seconds
    pub fn calculate(&mut self, setpoint: f64, value: f64, dt: f64) -> f64 {
        let error = setpoint - value;
        let integral = self.i_prior + error * dt;
        let derivative = (error - self.e_prior) / dt;
        self.e_prior = error;
        self.i_prior = integral;
        self.kp * error * self.ki * integral + self.kd * derivative + self.bias
    }
}
