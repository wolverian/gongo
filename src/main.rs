use bevy::dev_tools::picking_debug::DebugPickingMode;
use bevy::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::env;

#[derive(Resource)]
struct Meshes {
    circle: Handle<Mesh>,
}

#[derive(Resource)]
struct Colors {
    red: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct RandomSource(ChaCha8Rng);

fn main() {
    #[cfg(target_os = "macos")]
    unsafe {
        env::set_var("MTL_HUD_ENABLED", "1");
    }
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .insert_resource(DebugPickingMode::Normal)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, spawn_mobs)
        .run();
}

fn spawn_mobs(mut commands: Commands, spawn_meshes: Res<Meshes>, spawn_colors: Res<Colors>) {
    commands.spawn((
        Mesh2d(spawn_meshes.circle.clone()),
        MeshMaterial2d(spawn_colors.red.clone()),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let seeded_rng = ChaCha8Rng::seed_from_u64(19878367467712);
    commands.insert_resource(RandomSource(seeded_rng));
    commands.spawn(Camera2d);
    let mob_size = 16.;
    let spawn_meshes = Meshes {
        circle: meshes.add(Circle::new(mob_size)),
    };
    let spawn_colors = Colors {
        red: materials.add(Color::hsl(0., 0.92, 0.61)),
    };
    commands.insert_resource(spawn_meshes);
    commands.insert_resource(spawn_colors);
    // let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);
}
