use assets::GameAssets;
use bevy::{prelude::*, core::FixedTimestep};
use player::{player_movement, spawn_player};
use region_map::{MapToBuild, RegionMap};
use tilemap::{tile_location_added, tile_lerp};
mod assets;
mod player;
mod region_map;
mod tilemap;

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
        .add_system(tile_location_added)
        .add_system_set(
            SystemSet::new().with_run_criteria(FixedTimestep::step(1.0 / 30.0))
            .with_system(tile_lerp)
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // 2D games need these
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Setup assets
    let assets = GameAssets::new(&asset_server, &mut materials, &mut texture_atlases);

    // Spawn a map
    let mut region_map = RegionMap::new(MapToBuild::FarmerTomCoup);
    region_map.spawn(&assets, &mut meshes, &mut commands);

    // Spawn the player
    spawn_player(&mut commands, &assets, region_map.player_start);

    // Resources
    commands.insert_resource(region_map);
    commands.insert_resource(assets);
}
