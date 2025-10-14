use bevy::dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin};
use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            filter: "bevy_dev_tools=trace".into(),
            ..default()
        }))
        .add_plugins((MeshPickingPlugin, DebugPickingPlugin))
        .insert_resource(DebugPickingMode::Normal)
        .add_systems(Startup, setup)
        .run();
}

const X_EXTENT: f32 = 900.;
const Y_EXTENT: f32 = 900.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let size = Vec2::new(X_EXTENT, Y_EXTENT);
    let rough_size = (X_EXTENT + Y_EXTENT) / 2. / 10.;
    let center = size / 2.;

    let shapes = [
        meshes.add(Rectangle::from_length(rough_size)),
        meshes.add(Circle::new(rough_size)),
        meshes.add(Triangle2d::new(
            Vec2::from_angle(0.) * rough_size,
            Vec2::from_angle(90.0) * rough_size,
            Vec2::from_angle(180.0) * rough_size,
        )),
    ];
    let num_shapes = shapes.len();

    let center_indicator = meshes.add(Circle::new(4.));

    commands.spawn((
        Mesh2d(center_indicator),
        MeshMaterial2d(materials.add(Color::hsl(1., 1., 1.))),
    ));

    for (i, shape) in shapes.into_iter().enumerate() {
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);
        // translate along x axis and then rotate around origin
        let mut transform = Transform::from_translation(Vec3::X * size.x / 2.)
            .with_rotation(Quat::from_rotation_z(PI / 2.));
        transform.translation += transform.forward() * 100.;
        // let transform = Transform::from_rotation(Quat::from_rotation_z(PI / 2.))
        //     .with_translation(Vec3::X * size.x / 2.);
        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            transform,
        ));
    }
}
