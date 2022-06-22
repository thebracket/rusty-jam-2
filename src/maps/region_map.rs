use crate::{
    assets::GameAssets,
    console::Console,
    random::Rng,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use super::{TileType, TileMapLayer, tile_index, builder, MapToBuild};

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

    pub fn transition_to(
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
