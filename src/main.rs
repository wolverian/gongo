use avian2d::prelude::*;
use bevy::math::prelude::*;
use bevy::prelude::*;
use rand::distr::Uniform;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
#[cfg(target_os = "macos")]
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

#[derive(Resource)]
struct MobVelocityDistribution(Uniform<f32>);

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
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default().with_length_unit(20.)) // 1 meter = 20 pixels
        // Resources
        .insert_resource(Gravity::ZERO)
        .insert_resource(RandomSource(ChaCha8Rng::seed_from_u64(42)))
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
        .insert_resource(MobVelocityDistribution(
            Uniform::new(-32., 32.).expect("Failed to create uniform distribution"),
        ))
        // Debug plugins
        .add_plugins(MeshPickingPlugin)
        .add_plugins((
            PhysicsDebugPlugin,
            PhysicsDiagnosticsPlugin,
            PhysicsDiagnosticsUiPlugin,
        ))
        // Systems
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
    mob_velocity_distribution: Res<MobVelocityDistribution>,
) {
    if !state.mob_spawn_timer.tick(time.delta()).just_finished() {
        return;
    }

    let mob_size = 16.; // todo: dry this out
    let circle = Circle::new(mob_size);

    let rng = &mut rand_source.0;
    let pos = arena.0.sample_interior(rng);
    let distr = Uniform::new(-10., 10.).expect("Failed to create uniform distribution");
    let mut collider = circle.collider();
    collider.set_scale(Vec2::splat(state.mob_scale), 32);
    commands.spawn((
        Mesh2d(spawn_meshes.circle.clone()),
        MeshMaterial2d(spawn_colors.red.clone()),
        collider,
        RigidBody::Dynamic,
        Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0))
            .with_scale(Vec3::splat(state.mob_scale)),
        LinearVelocity(Vec2::new(
            rng.sample(mob_velocity_distribution.0),
            rng.sample(mob_velocity_distribution.0),
        )),
        Restitution::new(1.),
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
    let distr = Uniform::new(-10., 10.).expect("Failed to create uniform distribution");
    let mut collider = Circle::new(16.).collider();
    collider.set_scale(Vec2::splat(state.bouba_scale), 32);
    commands.spawn((
        Mesh2d(spawn_meshes.circle.clone()),
        MeshMaterial2d(spawn_colors.blue.clone()),
        collider,
        RigidBody::Dynamic,
        Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0))
            .with_scale(Vec3::splat(state.bouba_scale)),
        LinearVelocity(Vec2::new(rng.sample(distr), rng.sample(distr))),
        Restitution::new(1.),
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
