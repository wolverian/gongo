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
    bouba_spawn_timer: Timer,
    mob_scale: f32,
    bouba_scale: f32,
}

#[derive(Resource)]
struct Colors {
    red: Handle<ColorMaterial>,
    blue: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct RandomSource(ChaCha8Rng);

#[derive(Resource)]
struct Arena(Rectangle);

struct Settings {
    initial_mob_spawn_timer_interval: f32,
    initial_bouba_spawn_timer_interval: f32,
}

fn main() {
    #[cfg(target_os = "macos")]
    unsafe {
        env::set_var("MTL_HUD_ENABLED", "1");
    }
    let settings = Settings {
        initial_mob_spawn_timer_interval: 0.5,
        initial_bouba_spawn_timer_interval: 1.0,
    };
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .insert_resource(RandomSource(ChaCha8Rng::seed_from_u64(42)))
        .insert_resource(DebugPickingMode::Normal)
        .insert_resource(Arena(Rectangle::new(800., 600.)))
        .insert_resource(State {
            mob_spawn_timer: Timer::from_seconds(
                settings.initial_mob_spawn_timer_interval,
                TimerMode::Repeating,
            ),
            bouba_spawn_timer: Timer::from_seconds(
                settings.initial_bouba_spawn_timer_interval,
                TimerMode::Repeating,
            ),
            mob_scale: 1.,
            bouba_scale: 2.,
        })
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (spawn_mobs, spawn_boubas))
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
    if !state.mob_spawn_timer.tick(time.delta()).just_finished() {
        return;
    }

    let rng = &mut rand_source.0;
    let pos = arena.0.sample_interior(rng);
    commands.spawn((
        Mesh2d(spawn_meshes.circle.clone()),
        MeshMaterial2d(spawn_colors.red.clone()),
        Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0))
            .with_scale(Vec3::splat(state.mob_scale)),
    ));
}

fn spawn_boubas(
    mut commands: Commands,
    spawn_meshes: Res<Meshes>,
    spawn_colors: Res<Colors>,
    mut rand_source: ResMut<RandomSource>,
    arena: Res<Arena>,
    mut state: ResMut<State>,
    time: Res<Time>,
) {
    if !state.bouba_spawn_timer.tick(time.delta()).just_finished() {
        return;
    }

    let rng = &mut rand_source.0;
    let pos = arena.0.sample_interior(rng);
    commands.spawn((
        Mesh2d(spawn_meshes.circle.clone()),
        MeshMaterial2d(spawn_colors.blue.clone()),
        Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0))
            .with_scale(Vec3::splat(state.bouba_scale)),
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // resources
    let mob_size = 16.;
    let spawn_meshes = Meshes {
        circle: meshes.add(Circle::new(mob_size)),
    };
    let spawn_colors = Colors {
        red: materials.add(Color::hsl(0., 0.92, 0.61)),
        blue: materials.add(Color::hsl(240., 0.92, 0.61)),
    };
    commands.insert_resource(spawn_meshes);
    commands.insert_resource(spawn_colors);

    // entities
    commands.spawn(Camera2d);
}
