use assets::GameAssets;
use bevy::{core::FixedTimestep, prelude::*};
use console::{console_setup, update_consoles, Console};
use henry::{henry_ai, spawn_henry};
use interactions::player_interaction;
use player::{player_movement, spawn_player};
use random::Rng;
use region_map::{MapToBuild, RegionMap, map_exits};
use tilemap::{tile_lerp, tile_location_added};
mod assets;
mod console;
mod henry;
mod player;
mod region_map;
mod tilemap;
mod interactions;
mod random;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1024.0,
            height: 768.0,
            title: "Happy Chickens".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player_movement)
        .add_system(player_interaction)
        .add_system(tile_location_added)
        .add_system(update_consoles)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 30.0))
                .with_system(tile_lerp)
                .with_system(henry_ai),
        )
        .add_stage_after(CoreStage::Update, "migration", SystemStage::parallel())
        .add_system(map_exits)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Make an RNG
    let rng = Rng::new();

    // 2D games need these
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Setup assets
    let assets = GameAssets::new(&asset_server, &mut materials, &mut texture_atlases);
    let console = Console::new();
    console_setup(&assets, &mut commands, &console);

    // Spawn a map
    let mut region_map = RegionMap::new(MapToBuild::FarmerTomCoup, &rng);
    region_map.spawn(&assets, &mut meshes, &mut commands);

    // Spawn the player
    spawn_player(&mut commands, &assets, region_map.player_start);
    let mut henry_start = region_map.player_start;
    henry_start.0 -= 1;
    spawn_henry(&mut commands, &assets, henry_start);

    // Resources
    commands.insert_resource(region_map);
    commands.insert_resource(assets);
    commands.insert_resource(console);
    commands.insert_resource(rng);
}
