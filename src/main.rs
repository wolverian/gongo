use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

const X_EXTENT: f32 = 900.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let rough_size = X_EXTENT / 10.;

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

    for (i, shape) in shapes.into_iter().enumerate() {
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);
        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
        ));
    }
}
