use actors::{
    chicken_ai, henry_ai, player_movement, spawn_henry, spawn_player, unconscious_henry, Chicken,
    Farmer, Henry, Player, ScaresChickens, Tasty, Wolf,
};
use ai::{attacks, chase_after, flee_from, process_actions, ActionRequest};
use assets::GameAssets;
use bevy::{core::FixedTimestep, prelude::*};
use combat::{
    combat_lerp, damage_system, setup_health_hud, update_health_hud, DamageMessage, Hostile,
};
use console::{console_setup, update_consoles, Console};
use fov::update_field_of_view;
use interactions::player_interaction;
use maps::{map_exits, tile_lerp, tile_location_added, MapToBuild, RegionMap};
use random::Rng;
mod actors;
mod ai;
mod assets;
mod combat;
mod console;
mod fov;
mod interactions;
mod maps;
mod random;

fn main() {
    // The input step handles all direct player interaction
    let input_step = SystemSet::new()
        .label("InputStep")
        .with_system(player_movement)
        .with_system(player_interaction);

    // The AI step handles computer-controlled actors' actions
    let ai_step = SystemSet::new()
        .with_run_criteria(FixedTimestep::step(1.0 / 30.0))
        .label("AiStep")
        // Running away
        .with_system(flee_from::<Chicken, ScaresChickens>)
        .with_system(flee_from::<Farmer, Player>)
        // Chasing Targets
        .with_system(chase_after::<Henry, Hostile>)
        .with_system(chase_after::<Wolf, Tasty>)
        // Actor-level AI
        .with_system(chicken_ai)
        .with_system(henry_ai)
        .with_system(unconscious_henry)
        // Killing things
        .with_system(attacks::<Wolf, Tasty>)
        .with_system(attacks::<Henry, Hostile>);

    let action_step = SystemSet::new()
        .label("ActionStep")
        .with_system(process_actions);

    let lerping_step = SystemSet::new()
        .label("LerpStep")
        .with_run_criteria(FixedTimestep::step(1.0 / 30.0))
        .with_system(combat_lerp)
        .with_system(tile_lerp)
        .with_system(update_field_of_view);

    let cleanup_step = SystemSet::new()
        .with_system(tile_location_added)
        .with_system(update_consoles)
        .with_system(update_health_hud)
        .label("Cleanup");

    App::new()
        .insert_resource(WindowDescriptor {
            width: 1024.0,
            height: 768.0,
            title: "Mega Chicken".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_event::<ActionRequest>()
        .add_event::<DamageMessage>()
        .add_startup_system(setup)
        .add_stage("DecisionStage", SystemStage::parallel())
        .add_system_set(input_step)
        .add_system_set(ai_step)
        .add_stage_after(
            "DecisionStage",
            "ActionStage",
            SystemStage::single_threaded(),
        )
        .add_system_set(action_step)
        .add_stage_after("ActionStage", "LerpStage", SystemStage::parallel())
        .add_system_set(lerping_step)
        .add_stage_after("LerpStage", "CleanupStage", SystemStage::single_threaded())
        .add_system_set(cleanup_step)
        .add_stage_after(CoreStage::Update, "battle", SystemStage::single_threaded())
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
    //let mut region_map = RegionMap::new(MapToBuild::FarmHouse, &rng);
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
