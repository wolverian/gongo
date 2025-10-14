use bevy::dev_tools::picking_debug::DebugPickingMode;
use bevy::math::prelude::*;
use bevy::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::env;

#[derive(Resource)]
struct Meshes {
    circle: Handle<Mesh>,
}

#[derive(Resource)]
struct State {
    mob_spawn_timer: Timer,
}

#[derive(Resource)]
struct Colors {
    red: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct RandomSource(ChaCha8Rng);

#[derive(Resource)]
struct Arena(Rectangle);

struct Settings {
    initial_mob_spawn_timer_interval: f32,
}

fn main() {
    #[cfg(target_os = "macos")]
    unsafe {
        env::set_var("MTL_HUD_ENABLED", "1");
    }
    let settings = Settings {
        initial_mob_spawn_timer_interval: 0.5,
    };
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .insert_resource(DebugPickingMode::Normal)
        .insert_resource(State {
            mob_spawn_timer: Timer::from_seconds(
                settings.initial_mob_spawn_timer_interval,
                TimerMode::Repeating,
            ),
        })
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, spawn_mobs)
        .run();
}

fn spawn_mobs(
    mut commands: Commands,
    spawn_meshes: Res<Meshes>,
    spawn_colors: Res<Colors>,
    mut rand_source: ResMut<RandomSource>,
    arena: Res<Arena>,
    mut state: ResMut<State>,
    time: Res<Time>,
) {
    if state.mob_spawn_timer.tick(time.delta()).just_finished() {
        let rng = &mut rand_source.0;
        let pos = arena.0.sample_interior(rng);
        commands.spawn((
            Mesh2d(spawn_meshes.circle.clone()),
            MeshMaterial2d(spawn_colors.red.clone()),
            Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
        ));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // resources
    let seeded_rng = ChaCha8Rng::seed_from_u64(19878367467712);
    commands.insert_resource(RandomSource(seeded_rng));
    let mob_size = 16.;
    let spawn_meshes = Meshes {
        circle: meshes.add(Circle::new(mob_size)),
    };
    let spawn_colors = Colors {
        red: materials.add(Color::hsl(0., 0.92, 0.61)),
    };
    let arena = Arena(Rectangle::new(800., 600.));
    commands.insert_resource(spawn_meshes);
    commands.insert_resource(spawn_colors);
    commands.insert_resource(arena);
    // entities
    commands.spawn(Camera2d);
}
