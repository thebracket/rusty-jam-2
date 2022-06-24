use super::{builder, tile_index, MapToBuild, TileMapLayer, TileType, NUM_TILES_X, NUM_TILES_Y};
use crate::{
    actors::{spawn_chicken, spawn_farmer, spawn_mommy_wolf, spawn_spikes, spawn_wolf, spawn_spider},
    assets::GameAssets,
    combat::Health,
    console::Console,
    random::Rng,
    GameElement,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bracket_pathfinding::prelude::{Algorithm2D, BaseMap, DistanceAlg, Point, SmallVec};

#[derive(Component)]
pub struct MapElement;

pub struct RegionMap {
    pub name: String,
    pub base_tiles: Vec<TileType>,
    pub features: Vec<TileType>,
    pub mesh: Option<Handle<Mesh>>,
    pub player_start: (i32, i32),
    pub mesh2: Option<Handle<Mesh>>,
    pub exits: Vec<(usize, usize)>,
    pub spawns: Vec<(String, i32, i32)>,
    pub id: MapToBuild,
}

impl RegionMap {
    pub fn new(map_type: MapToBuild, rng: &Rng) -> Self {
        let map = builder(map_type, rng, None);

        Self {
            name: map.name,
            base_tiles: map.tiles,
            features: map.features,
            player_start: map.player_start,
            exits: map.exits,
            mesh: None,
            mesh2: None,
            spawns: map.spawns,
            id: map_type,
        }
    }

    pub fn spawn(
        &mut self,
        assets: &GameAssets,
        meshes: &mut Assets<Mesh>,
        commands: &mut Commands,
    ) {
        for (tag, x, y) in self.spawns.iter() {
            match tag.as_str() {
                "Chicken" => spawn_chicken(*x, *y, assets, commands),
                "Farmer" => spawn_farmer(*x, *y, assets, commands),
                "WeakWolf" => spawn_wolf(*x, *y, 1, assets, commands),
                "WhiteWolf" => spawn_mommy_wolf(*x, *y, 35, assets, commands),
                "Spikes1" => spawn_spikes(*x, *y, false, assets, commands),
                "Spikes2" => spawn_spikes(*x, *y, true, assets, commands),
                "Spider" => spawn_spider(*x, *y, assets, commands),
                _ => println!("Warning: Don't know how to spawn a [{tag}]"),
            }
        }

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
            .insert(MapElement)
            .insert(GameElement);
    }

    pub fn transition_to(
        &mut self,
        new_map: usize,
        commands: &mut Commands,
        elements: &Query<Entity, With<MapElement>>,
        assets: &GameAssets,
        meshes: &mut Assets<Mesh>,
        rng: &Rng,
    ) -> Point {
        // Remove the old map display
        elements.for_each(|e| commands.entity(e).despawn());

        // Build a map
        let to_build = match new_map {
            1 => MapToBuild::FarmHouse,
            2 => MapToBuild::Cave1,
            3 => MapToBuild::Cave2,
            4 => MapToBuild::Forest,
            _ => MapToBuild::FarmerTomCoup,
        };
        let new_data = builder(to_build, rng, Some(self.id));
        self.base_tiles = new_data.tiles;
        self.exits = new_data.exits;
        self.features = new_data.features;
        self.name = new_data.name;
        self.spawns = new_data.spawns;

        // Spawn the new one
        self.spawn(assets, meshes, commands);
        Point::new(new_data.player_start.0, new_data.player_start.1)
    }

    pub fn can_player_enter(&self, x: i32, y: i32) -> bool {
        if !self.in_bounds(Point::new(x, y)) {
            return false;
        }
        let idx = tile_index(x, y);
        let mut can_go = true;
        if let TileType::ReferTo(refer_idx) = self.base_tiles[idx] {
            if !self.base_tiles[refer_idx].can_player_enter() {
                can_go = false;
            }
        } else if let TileType::ReferTo(refer_idx) = self.features[idx] {
            if !self.features[refer_idx].can_player_enter() {
                can_go = false;
            }
        } else {
            can_go =
                self.base_tiles[idx].can_player_enter() && self.features[idx].can_player_enter();
        }
        can_go
    }

    pub fn interact(&self, x: i32, y: i32, console: &Console, health: &mut Health) {
        let idx = tile_index(x, y);
        self.base_tiles[idx].interact(console, health);
        self.features[idx].interact(console, health);
    }

    fn try_exit(&self, location: Point, delta: Point) -> Option<usize> {
        let destination = location + delta;
        if self.in_bounds(destination) && self.can_player_enter(destination.x, destination.y) {
            Some(tile_index(destination.x, destination.y))
        } else {
            None
        }
    }
}

/// Support for FOV and path-finding via bracket-pathfinding

impl Algorithm2D for RegionMap {
    fn dimensions(&self) -> Point {
        Point::new(NUM_TILES_X, NUM_TILES_Y)
    }

    fn index_to_point2d(&self, idx: usize) -> Point {
        Point::new(idx % NUM_TILES_X, idx / NUM_TILES_X)
    }

    fn point2d_to_index(&self, pt: Point) -> usize {
        tile_index(pt.x, pt.y)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < NUM_TILES_X as i32 && pos.y >= 0 && pos.y < NUM_TILES_Y as i32
    }
}

impl BaseMap for RegionMap {
    fn is_opaque(&self, idx: usize) -> bool {
        let pt = self.index_to_point2d(idx);
        !self.can_player_enter(pt.x, pt.y)
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);
        if let Some(idx) = self.try_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.try_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.try_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.try_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0));
        }
        exits
    }
}
