use actors::{
    chicken_ai, henry_ai, player_movement, spawn_henry, spawn_player, spike_system,
    unconscious_henry, Chicken, Farmer, Henry, Player, ScaresChickens, Tasty, Wolf, Spider,
};
use ai::{attacks, chase_after, flee_from, process_actions, ActionRequest};
use assets::GameAssets;
use bevy::prelude::*;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Playing,
    Dead,
    Won,
}

// Bevy has a bug! When you on_update for a specific state AND have a timestep,
// your function runs all the time. Ugh.
pub struct TimeStepResource {
    pub timer: Timer,
}

fn time_step_update(time: Res<Time>, mut timestep: ResMut<TimeStepResource>) {
    timestep.timer.tick(time.delta());
}

#[derive(Component)]
pub struct GameElement;

fn main() {
    // Main Menu
    let setup_menu_step = SystemSet::on_enter(GameState::MainMenu)
        .label("GameSetup")
        .with_system(start_main_menu);

    let exit_menu_step = SystemSet::on_exit(GameState::MainMenu)
        .label("GameSetup")
        .with_system(exit_main_menu);

    let menu_step = SystemSet::on_update(GameState::MainMenu).with_system(main_menu);

    // Dead Menu
    let setup_dead_step = SystemSet::on_enter(GameState::Dead)
        .label("GameSetup")
        .with_system(start_dead_menu);

    let exit_dead_step = SystemSet::on_exit(GameState::Dead)
        .label("GameSetup")
        .with_system(exit_dead_menu);

    let dead_step = SystemSet::on_update(GameState::Dead).with_system(dead_menu);

    // Won Menu
    let setup_won_step = SystemSet::on_enter(GameState::Won)
        .label("GameSetup")
        .with_system(start_won_menu);

    let exit_won_step = SystemSet::on_exit(GameState::Won)
        .label("GameSetup")
        .with_system(exit_won_menu);

    let won_step = SystemSet::on_update(GameState::Won).with_system(won_menu);

    // Step to initialize game resources
    let setup_step = SystemSet::on_enter(GameState::Playing)
        .label("GameSetup")
        .with_system(setup_game);
    let game_over_step = SystemSet::on_exit(GameState::Playing)
        .label("GameOverMan")
        .with_system(game_over);

    // The input step handles all direct player interaction
    let input_step = SystemSet::on_update(GameState::Playing)
        .label("InputStep")
        .with_system(player_movement)
        .with_system(player_interaction);

    // The AI step handles computer-controlled actors' actions
    let ai_step = SystemSet::on_update(GameState::Playing)
        //.with_run_criteria(FixedTimestep::step(1.0 / 30.0))
        .label("AiStep")
        .with_system(time_step_update)
        // Running away
        .with_system(flee_from::<Chicken, ScaresChickens>)
        .with_system(flee_from::<Farmer, Player>)
        // Chasing Targets
        .with_system(chase_after::<Henry, Hostile>)
        .with_system(chase_after::<Wolf, Tasty>)
        .with_system(chase_after::<Spider, Tasty>)
        // Actor-level AI
        .with_system(chicken_ai)
        .with_system(henry_ai)
        .with_system(unconscious_henry)
        // Killing things
        .with_system(spike_system)
        .with_system(attacks::<Wolf, Tasty>)
        .with_system(attacks::<Spider, Tasty>)
        .with_system(attacks::<Henry, Hostile>)
        .with_system(attacks::<Player, Hostile>); // Auto attack mode

    let action_step = SystemSet::on_update(GameState::Playing)
        .label("ActionStep")
        .with_system(process_actions);

    let lerping_step = SystemSet::on_update(GameState::Playing)
        .label("LerpStep")
        //.with_run_criteria(FixedTimestep::step(1.0 / 30.0))
        .with_system(combat_lerp)
        .with_system(tile_lerp)
        .with_system(update_field_of_view);

    let cleanup_step = SystemSet::on_update(GameState::Playing)
        .with_system(tile_location_added)
        .with_system(update_consoles)
        .with_system(update_health_hud)
        .label("Cleanup");

    let migrate_step = SystemSet::on_update(GameState::Playing)
        .label("Migrate")
        .with_system(map_exits);

    App::new()
        .insert_resource(WindowDescriptor {
            width: 1024.0,
            height: 768.0,
            title: "Mega Chicken".to_string(),
            resizable: false,
            ..Default::default()
        })
        .insert_resource(TimeStepResource {
            timer: Timer::from_seconds(1.0 / 30.0, true),
        })
        .add_plugins(DefaultPlugins)
        .add_state(GameState::MainMenu)
        .add_event::<ActionRequest>()
        .add_event::<DamageMessage>()
        .add_startup_system(setup)
        // Main Menu
        .add_system_set(setup_menu_step)
        .add_system_set(exit_menu_step)
        .add_system_set(menu_step)
        // Dead Menu
        .add_system_set(setup_dead_step)
        .add_system_set(exit_dead_step)
        .add_system_set(dead_step)
        // Won Menu
        .add_system_set(setup_won_step)
        .add_system_set(exit_won_step)
        .add_system_set(won_step)
        // Game Initialization
        .add_system_set(setup_step)
        .add_system_set(game_over_step)
        // The decision stage runs player input and game AI
        // It just emits messages, nothing changes
        .add_stage("DecisionStage", SystemStage::parallel())
        .add_system_set(input_step)
        .add_system_set(ai_step)
        // The ActionStage processes requested actions and coalesces them into a final
        // decision
        .add_stage_after("DecisionStage", "ActionStage", SystemStage::parallel())
        .add_system_set(action_step)
        // The LerpStage handles animations
        .add_stage_after("ActionStage", "LerpStage", SystemStage::single_threaded())
        .add_system_set(lerping_step)
        // CleanUp stage runs a miscellany of things that need to happen near the end
        .add_stage_after("LerpStage", "CleanupStage", SystemStage::single_threaded())
        .add_system_set(cleanup_step)
        // The battle system runs next-to-last, since it can delete things
        .add_stage_after(
            CoreStage::PostUpdate,
            "battle",
            SystemStage::single_threaded(),
        )
        .add_system(damage_system)
        // A final stage for migrating between maps
        .add_stage_after("battle", "migration", SystemStage::single_threaded())
        .add_system_set(migrate_step)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Make an RNG
    let rng = Rng::new();

    // 2D games need these
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Setup assets
    let assets = GameAssets::new(&asset_server, &mut materials, &mut texture_atlases);

    // Resources
    commands.insert_resource(assets);
    commands.insert_resource(rng);
}

fn setup_game(
    mut commands: Commands,
    assets: Res<GameAssets>,
    rng: Res<Rng>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Console
    let console = Console::new();
    console_setup(&assets, &mut commands, &console);

    // Spawn a map
    //let mut region_map = RegionMap::new(MapToBuild::FarmerTomCoup, &rng);
    let mut region_map = RegionMap::new(MapToBuild::Forest, &rng);
    region_map.spawn(&assets, &mut meshes, &mut commands);

    // Spawn the player
    spawn_player(&mut commands, &assets, region_map.player_start);
    let mut henry_start = region_map.player_start;
    henry_start.0 -= 1;
    spawn_henry(&mut commands, &assets, henry_start);

    // HUD stuff
    setup_health_hud(&mut commands, &assets);

    // Resources
    commands.insert_resource(console);
    commands.insert_resource(region_map);
}

#[derive(Component)]
struct MainMenu;

fn start_main_menu(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.main_menu.clone(),
            ..default()
        })
        .insert(MainMenu);
}

fn main_menu(keyboard: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard.just_pressed(KeyCode::P) {
        state.set(GameState::Playing).unwrap();
    }
}

fn exit_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    query.iter().for_each(|e| commands.entity(e).despawn());
}

fn game_over(mut commands: Commands, query: Query<Entity, With<GameElement>>) {
    query.iter().for_each(|e| commands.entity(e).despawn());
}

fn start_dead_menu(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.dead_menu.clone(),
            ..default()
        })
        .insert(MainMenu);
}

fn dead_menu(keyboard: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard.just_pressed(KeyCode::P) {
        state.set(GameState::Playing).unwrap();
    }
}

fn exit_dead_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    query.iter().for_each(|e| commands.entity(e).despawn());
}

fn start_won_menu(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.won_menu.clone(),
            ..default()
        })
        .insert(MainMenu);
}

fn won_menu(keyboard: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard.just_pressed(KeyCode::P) {
        state.set(GameState::Playing).unwrap();
    }
}

fn exit_won_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    query.iter().for_each(|e| commands.entity(e).despawn());
}
