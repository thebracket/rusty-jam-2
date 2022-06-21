use crate::{
    assets::GameAssets,
    tilemap::{TileMapLayer, TileType, NUM_TILES_X, NUM_TILES_Y}, console::Console,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bracket_random::prelude::RandomNumberGenerator;

#[derive(Component)]
struct MapElement;

pub struct RegionMap {
    pub name: String,
    pub base_tiles: Vec<TileType>,
    pub features: Vec<TileType>,
    pub mesh: Option<Handle<Mesh>>,
    pub player_start: (i32, i32),
    pub mesh2: Option<Handle<Mesh>>,
}

impl RegionMap {
    pub fn new(map: MapToBuild) -> Self {
        let map = builder(map);

        Self {
            name: map.name,
            base_tiles: map.tiles,
            features: map.features,
            player_start: map.player_start,
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

    pub fn can_player_enter(&self, x: i32, y: i32) -> bool {
        let idx = tile_index(x, y);
        self.base_tiles[idx].can_player_enter() && self.features[idx].can_player_enter()
    }

    pub fn interact(&self, x: i32, y: i32, console: &Console) {
        let idx = tile_index(x, y);
        self.base_tiles[idx].interact(console);
        self.features[idx].interact(console);
    }
}

pub enum MapToBuild {
    FarmerTomCoup,
}

struct MapTransfer {
    tiles: Vec<TileType>,
    features: Vec<TileType>,
    name: String,
    player_start: (i32, i32),
}

fn builder(map: MapToBuild) -> MapTransfer {
    match map {
        MapToBuild::FarmerTomCoup => build_farmer_tom_coup(),
    }
}

pub fn tile_index(x: i32, y: i32) -> usize {
    ((NUM_TILES_X as i32 * y) + x) as usize
}

fn build_farmer_tom_coup() -> MapTransfer {
    let mut rng = RandomNumberGenerator::new();
    let mut tiles = vec![TileType::Grass; NUM_TILES_X * NUM_TILES_Y];
    let mut features = vec![TileType::None; NUM_TILES_X * NUM_TILES_Y];
    let player_start = (NUM_TILES_X as i32 / 2, NUM_TILES_Y as i32 / 2);
    
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

    // Boundaries
    for x in 0..NUM_TILES_X as i32 {
        features[tile_index(x, 0)] = TileType::Bush;
        features[tile_index(x, NUM_TILES_Y as i32 -1)] = TileType::Bush;
        for y in 0..rng.range(1, 5) {
            features[tile_index(x, NUM_TILES_Y as i32 -1 - y)] = TileType::Bush;
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
            features[tile_index(NUM_TILES_X as i32 -1 - x, y)] = TileType::Bush;
        }
    }

    tiles.iter_mut().enumerate().for_each(|(idx, t)| {
        if features[idx] == TileType::None && *t == TileType::Grass {
            if rng.range(1, 10) < 2 {
                features[idx] = TileType::Flower;
            }
        }
    });

    for y in 0..player_start.1 - 3 {
        for x in player_start.0 -1 .. player_start.0 + 2 {
            tiles[tile_index(x, y)] = TileType::Road;
            features[tile_index(x, y)] = TileType::None;
        }
    }

    MapTransfer {
        tiles,
        features,
        name: "Farmer Tom's Coup".to_string(),
        player_start,
    }
}
