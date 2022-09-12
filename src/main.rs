use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use debug::DebugPlugin;
use iyes_loopless::prelude::*;
use map::Map;
use std::collections::VecDeque;
use ui::{draw_ui, Config};

mod algorithms;
mod debug;
mod map;
mod map_builders;
mod ui;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum States {
    Menu,
    Running,
}

#[derive(Component)]
struct MapComponent;

struct Snapshots(VecDeque<Map>);
struct CurrentMap(Vec<Entity>);
pub struct TextureMap(Handle<TextureAtlas>);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {
            title: "Procedural dungeon demo".to_string(),
            fit_canvas_to_parent: true,
            ..default()
        })
        .init_resource::<Config>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(DebugPlugin)
        .add_loopless_state(States::Menu)
        .add_startup_system(spawn_camera)
        .add_startup_system(load_assets)
        .add_system(draw_ui)
        .add_system(draw_map.run_in_state(States::Running))
        .add_enter_system(
            States::Running,
            despawn_with::<MapComponent>.chain(setup_map),
        )
        .run();
}

fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    info!("Despawning entities with {:?}", std::any::type_name::<T>());
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn load_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture = server.load("texture_map.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        texture,
        Vec2::splat(9.0),
        16,
        16,
        Vec2::splat(2.0),
        Vec2::splat(0.),
    );
    let handle = texture_atlases.add(atlas);
    commands.insert_resource(TextureMap(handle));
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());
}

fn setup_map(cfg: Res<Config>, mut cmd: Commands) {
    // Spawn parent map component for easy positioning
    let parent = cmd
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(cfg.offset / 2., 0., 0.),
            ..default()
        })
        .insert(MapComponent)
        .id();

    // Generate map with chosen algorithm
    let mut builder = cfg.algorithm.get();
    builder.build_map(cfg.width, cfg.height, &cfg.options);

    // Store snapshots for later use
    let snaphots = builder.get_snapshot_history();
    println!("Snapshots: {}", snaphots.len());
    cmd.insert_resource(Snapshots(snaphots));

    // Spawn tile entities
    let mut entities = vec![Entity::from_raw(0); builder.get_map().tiles.len()];
    let map = builder.get_map();
    for (idx, _) in map.tiles.iter().enumerate() {
        let entity = cmd.spawn().id();
        cmd.entity(parent).add_child(entity);
        entities[idx] = entity;
    }
    cmd.insert_resource(CurrentMap(entities));
}

fn draw_map(
    mut cmd: Commands,
    mut snaps: ResMut<Snapshots>,
    current_map: Res<CurrentMap>,
    texture: Res<TextureMap>,
    mut cfg: ResMut<Config>,
    time: Res<Time>,
) {
    // tick the timer
    cfg.speed_timer.tick(time.delta());
    if cfg.speed_timer.finished() {
        if let Some(snap) = snaps.0.pop_front() {
            snap.draw(cmd, texture, &current_map.0)
        } else {
            println!("No more snapshots");
            cmd.insert_resource(NextState(States::Menu));
        }
    }
}
