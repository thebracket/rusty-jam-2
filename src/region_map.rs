use crate::{
    assets::GameAssets,
    console::Console,
    player::{Player, Facing},
    tilemap::{TileMapLayer, TilePosition, TileType, NUM_TILES_X, NUM_TILES_Y, LerpMove}, henry::Henry, random::Rng,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct MapElement;

pub fn map_exits(
    mut map: ResMut<RegionMap>,
    mut queries: ParamSet<(
        Query<&TilePosition, (With<Player>, Changed<TilePosition>)>,
        Query<Entity, With<MapElement>>,
        Query<(&Player, &mut TilePosition)>,
        Query<(Entity, &mut TilePosition), With<Henry>>,
    )>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    rng: Res<Rng>,
) {
    let mut transition = None;
    for player_pos in queries.p0().iter() {
        let player_idx = tile_index(player_pos.x, player_pos.y);
        for (exit, new_map) in map.exits.iter() {
            if *exit == player_idx {
                transition = Some(*new_map);
            }
        }
    }

    if let Some(new_map) = transition {
        map.transition_to(new_map, &mut commands, &queries.p1(), &assets, &mut meshes, &rng);

        // Adjust player position
        let mut player_pos = (0,0);
        for (player, mut ppos) in queries.p2().iter_mut() {
            player_pos = (ppos.x, ppos.y);
            match player.facing {
                Facing::Up => {
                    ppos.y = NUM_TILES_Y as i32 -1;
                    player_pos.1 = NUM_TILES_Y as i32 -1;
                }
                Facing::Down => {
                    ppos.y = 0;
                    player_pos.1 = 0;
                }
                Facing::Left => {
                    ppos.x = NUM_TILES_X as i32 - 1;
                    player_pos.0 = NUM_TILES_X as i32 - 1;
                }
                Facing::Right => {
                    ppos.x = 0;
                    player_pos.0 = 0;
                }
            }
        }
        for (henry, mut henry_pos) in queries.p3().iter_mut() {
            henry_pos.x = player_pos.0 -1;
            henry_pos.y = player_pos.1;
            commands.entity(henry).remove::<LerpMove>();
        }
    }
}

pub struct RegionMap {
    pub name: String,
    pub base_tiles: Vec<TileType>,
    pub features: Vec<TileType>,
    pub mesh: Option<Handle<Mesh>>,
    pub player_start: (i32, i32),
    pub mesh2: Option<Handle<Mesh>>,
    pub exits: Vec<(usize, usize)>,
}

impl RegionMap {
    pub fn new(map: MapToBuild, rng: &Rng) -> Self {
        let map = builder(map, rng);

        Self {
            name: map.name,
            base_tiles: map.tiles,
            features: map.features,
            player_start: map.player_start,
            exits: map.exits,
            mesh: None,
            mesh2: None,
        }
    }

    pub fn spawn(
        &mut self,
        assets: &GameAssets,
        meshes: &mut Assets<Mesh>,
        commands: &mut Commands,
    ) {
        let mesh = TileMapLayer::new(1.0).build_mesh(&self.base_tiles);
        let mesh_handle = meshes.add(mesh);
        self.mesh = Some(mesh_handle.clone());
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                transform: Transform::default(),
                material: assets.tileset.clone().into(),
                ..default()
            })
            .insert(MapElement);

        let mesh = TileMapLayer::new(1.5).build_mesh(&self.features);
        let mesh_handle = meshes.add(mesh);
        self.mesh2 = Some(mesh_handle.clone());
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                transform: Transform::default(),
                material: assets.tileset.clone().into(),
                ..default()
            })
            .insert(MapElement);

        // Label
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        bottom: Val::Px(96.0),
                        right: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    &self.name,
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                    // Note: You can use `Default::default()` in place of the `TextAlignment`
                    TextAlignment {
                        horizontal: HorizontalAlign::Right,
                        ..default()
                    },
                ),
                ..default()
            })
            .insert(MapElement);
    }

    fn transition_to(
        &mut self,
        new_map: usize,
        commands: &mut Commands,
        elements: &Query<Entity, With<MapElement>>,
        assets: &GameAssets,
        meshes: &mut Assets<Mesh>,
        rng: &Rng,
    ) {
        // Remove the old map display
        elements.for_each(|e| commands.entity(e).despawn());

        // Build a map
        let to_build = match new_map {
            1 => MapToBuild::FarmHouse,
            _ => MapToBuild::FarmerTomCoup,
        };
        let new_data = builder(to_build, rng);
        self.base_tiles = new_data.tiles;
        self.exits = new_data.exits;
        self.features = new_data.features;
        self.name = new_data.name;

        // Spawn the new one
        self.spawn(assets, meshes, commands);
    }

    pub fn can_player_enter(&self, x: i32, y: i32) -> bool {
        let idx = tile_index(x, y);
        let mut can_go = true;
        if let TileType::ReferTo(refer_idx) = self.base_tiles[idx] {
            if !self.base_tiles[refer_idx].can_player_enter() { can_go = false; }
        } else if let TileType::ReferTo(refer_idx) = self.features[idx] {
            if !self.features[refer_idx].can_player_enter() { can_go = false; }
        } else {
            can_go = self.base_tiles[idx].can_player_enter() && self.features[idx].can_player_enter();
        }
        can_go
    }

    pub fn interact(&self, x: i32, y: i32, console: &Console) {
        let idx = tile_index(x, y);
        self.base_tiles[idx].interact(console);
        self.features[idx].interact(console);
    }
}

pub enum MapToBuild {
    FarmerTomCoup,
    FarmHouse,
}

struct MapTransfer {
    tiles: Vec<TileType>,
    features: Vec<TileType>,
    name: String,
    player_start: (i32, i32),
    exits: Vec<(usize, usize)>,
}

fn builder(map: MapToBuild, rng: &Rng) -> MapTransfer {
    match map {
        MapToBuild::FarmerTomCoup => build_farmer_tom_coup(rng),
        MapToBuild::FarmHouse => build_toms_house(rng),
    }
}

pub fn tile_index(x: i32, y: i32) -> usize {
    ((NUM_TILES_X as i32 * y) + x) as usize
}

fn build_farmer_tom_coup(rng: &Rng) -> MapTransfer {
    let mut tiles = vec![TileType::Grass; NUM_TILES_X * NUM_TILES_Y];
    let mut features = vec![TileType::None; NUM_TILES_X * NUM_TILES_Y];
    let player_start = (NUM_TILES_X as i32 / 2, NUM_TILES_Y as i32 / 2);
    let mut exits = Vec::new();

    // Coup
    for x in player_start.0 - 5..player_start.0 + 5 {
        for y in player_start.1 - 3..player_start.1 + 3 {
            tiles[tile_index(x, y)] = TileType::Dirt;
            if y == player_start.1 - 3 || y == player_start.1 + 2 {
                features[tile_index(x, y)] = TileType::FenceHorizontal;
            } else if x == player_start.0 - 5 || x == player_start.0 + 4 {
                features[tile_index(x, y)] = TileType::FenceVertical;
            }
        }
    }

    // Cauldron
    features[tile_index(player_start.0-3, player_start.1)] = TileType::Cauldron;

    // Boundaries
    for x in 0..NUM_TILES_X as i32 {
        features[tile_index(x, 0)] = TileType::Bush;
        features[tile_index(x, NUM_TILES_Y as i32 - 1)] = TileType::Bush;
        for y in 0..rng.range(1, 5) {
            features[tile_index(x, NUM_TILES_Y as i32 - 1 - y)] = TileType::Bush;
        }
        for y in 0..rng.range(1, 5) {
            features[tile_index(x, y)] = TileType::Bush;
        }
    }
    for y in 0..NUM_TILES_Y as i32 {
        features[tile_index(0, y)] = TileType::Bush;
        features[tile_index(NUM_TILES_X as i32 - 1, y)] = TileType::Bush;
        for x in 0..rng.range(1, 5) {
            features[tile_index(x, y)] = TileType::Bush;
        }
        for x in 0..rng.range(1, 5) {
            features[tile_index(NUM_TILES_X as i32 - 1 - x, y)] = TileType::Bush;
        }
    }

    // Add some pretty flowers
    tiles.iter_mut().enumerate().for_each(|(idx, t)| {
        if features[idx] == TileType::None && *t == TileType::Grass {
            if rng.range(1, 10) < 2 {
                features[idx] = TileType::Flower;
            }
        }
    });

    // Add a road
    for y in 0..player_start.1 - 3 {
        for x in player_start.0 - 1..player_start.0 + 2 {
            tiles[tile_index(x, y)] = TileType::Road;
            features[tile_index(x, y)] = TileType::None;
            if y == 0 {
                exits.push((tile_index(x, y), 1));
            }
        }
    }

    MapTransfer {
        tiles,
        features,
        name: "Farmer Tom's Coup".to_string(),
        player_start,
        exits,
    }
}

fn build_toms_house(rng: &Rng) -> MapTransfer {
    let mut tiles = vec![TileType::Grass; NUM_TILES_X * NUM_TILES_Y];
    let mut features = vec![TileType::None; NUM_TILES_X * NUM_TILES_Y];
    let player_start = (NUM_TILES_X as i32 / 2, NUM_TILES_Y as i32 / 2);
    let mut exits = Vec::new();

    // Boundaries
    for x in 0..NUM_TILES_X as i32 {
        features[tile_index(x, 0)] = TileType::Bush;
        features[tile_index(x, NUM_TILES_Y as i32 - 1)] = TileType::Bush;
        for y in 0..rng.range(1, 5) {
            features[tile_index(x, NUM_TILES_Y as i32 - 1 - y)] = TileType::Bush;
        }
        for y in 0..rng.range(1, 5) {
            features[tile_index(x, y)] = TileType::Bush;
        }
    }
    for y in 0..NUM_TILES_Y as i32 {
        features[tile_index(0, y)] = TileType::Bush;
        features[tile_index(NUM_TILES_X as i32 - 1, y)] = TileType::Bush;
        for x in 0..rng.range(1, 5) {
            features[tile_index(x, y)] = TileType::Bush;
        }
        for x in 0..rng.range(1, 5) {
            features[tile_index(NUM_TILES_X as i32 - 1 - x, y)] = TileType::Bush;
        }
    }

    // Add a road
    let half_width = NUM_TILES_X as i32 / 2;
    for y in NUM_TILES_Y as i32 -5..NUM_TILES_Y as i32 {
        for x in half_width - 1.. half_width + 2 {
            tiles[tile_index(x, y)] = TileType::Road;
            features[tile_index(x, y)] = TileType::None;
            if y == NUM_TILES_Y as i32 -1 {
                exits.push((tile_index(x, y), 0));
            }
        }
    }

    // Cobbles
    for x in 13..25 {
        for y in 6..15 {
            if y == 6 {
                tiles[tile_index(x, y)] = TileType::CobbleT;
            } else if y == 14 {
                tiles[tile_index(x, y)] = TileType::CobbleB;
            } else if x == 13 {
                tiles[tile_index(x, y)] = TileType::CobbleL;
            } else if x == 24 {
                tiles[tile_index(x, y)] = TileType::CobbleR;
            } else {
                tiles[tile_index(x, y)] = TileType::Cobble;
            }
        }
    }
    tiles[tile_index(13, 6)] = TileType::CobbleTL;
    tiles[tile_index(24, 6)] = TileType::CobbleTR;
    tiles[tile_index(13, 14)] = TileType::CobbleBL;
    tiles[tile_index(24, 14)] = TileType::CobbleBR;

    // Add a haycart
    spawn_big_feature(10, 5, TileType::HayCart, &mut features);

    // Add a barn
    spawn_big_feature(14, 5, TileType::Barn, &mut features);

    // Add a rocky outcropping
    spawn_big_feature(0, 11, TileType::LeftButte, &mut features);

    MapTransfer {
        tiles,
        features,
        name: "Farmer Tom's House".to_string(),
        player_start,
        exits,
    }
}

fn spawn_big_feature(x: i32, y: i32, feature: TileType, features: &mut [TileType]) {
    let (width, height) = match feature {
        TileType::HayCart => (3, 2),
        TileType::Barn => (2, 3),
        TileType::LeftButte => (2, 7),
        _ => (0,0)
    };

    if width == 0 || height == 0 {
        return;
    }

    let base_idx = tile_index(x, y);
    for tx in 0..width {
        for ty in 0..height {
            let idx = tile_index(x+tx, y+ty);
            features[idx] = TileType::ReferTo(base_idx);
        }
    }
    features[base_idx] = feature;
}