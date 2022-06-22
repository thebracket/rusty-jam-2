use actors::{
    chicken_ai, farmer_ai, henry_ai, player_movement, spawn_henry, spawn_player, wolf_ai,
};
use assets::GameAssets;
use bevy::{core::FixedTimestep, prelude::*};
use combat::{
    combat_lerp, combat_system, damage_system, setup_health_hud, update_health_hud, AttackMessage,
    DamageMessage,
};
use console::{console_setup, update_consoles, Console};
use fov::update_field_of_view;
use interactions::player_interaction;
use maps::{map_exits, tile_lerp, tile_location_added, MapToBuild, RegionMap};
use random::Rng;
mod actors;
mod assets;
mod combat;
mod console;
mod fov;
mod interactions;
mod maps;
mod random;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1024.0,
            height: 768.0,
            title: "Mega Chicken".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_event::<AttackMessage>()
        .add_event::<DamageMessage>()
        .add_startup_system(setup)
        .add_system(player_movement)
        .add_system(player_interaction)
        .add_system(tile_location_added)
        .add_system(update_consoles)
        .add_system(chicken_ai)
        .add_system(farmer_ai)
        .add_system(update_health_hud)
        .add_system(combat_system)
        .add_system(combat_lerp)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 30.0))
                .with_system(tile_lerp)
                .with_system(henry_ai)
                .with_system(wolf_ai)
                .with_system(update_field_of_view),
        )
        .add_stage_after(
            CoreStage::Update,
            "battle",
            SystemStage::single_threaded(),
        )
        .add_system(damage_system)
        .add_stage_after(
            CoreStage::Update,
            "migration",
            SystemStage::single_threaded(),
        )
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

    // HUD stuff
    setup_health_hud(&mut commands, &assets);

    // Resources
    commands.insert_resource(region_map);
    commands.insert_resource(assets);
    commands.insert_resource(console);
    commands.insert_resource(rng);
}
