use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use crate::{console::Console, region_map::tile_index};

const TILE_WIDTH: f32 = 32.0;
const TILE_HEIGHT: f32 = 32.0;
pub const NUM_TILES_X: usize = 32;
pub const NUM_TILES_Y: usize = 20;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TileType {
    None,
    Grass,
    Dirt,
    FenceHorizontal,
    FenceVertical,
    Bush,
    Flower,
    Road,
    HayCart,
    ReferTo(usize),
    Barn,
    LeftButte,
    Cauldron,
    CobbleTL,
    CobbleT,
    CobbleTR,
    CobbleL,
    Cobble,
    CobbleR,
    CobbleBL,
    CobbleB,
    CobbleBR,
}

impl TileType {
    fn index(&self) -> usize {
        match self {
            TileType::None => 0, // Special case
            TileType::Grass => 0,
            TileType::Dirt => 1,
            TileType::FenceHorizontal => 2,
            TileType::FenceVertical => 3,
            TileType::Bush => 4,
            TileType::Flower => 5,
            TileType::Road => 6,
            TileType::Cauldron => 7,
            TileType::CobbleTL => 19,
            TileType::CobbleT => 20,
            TileType::CobbleTR => 21,
            TileType::CobbleL => 35,
            TileType::Cobble => 36,
            TileType::CobbleR => 37,
            TileType::CobbleBL => 51,
            TileType::CobbleB => 52,
            TileType::CobbleBR => 53,
            _ => 0,
        }
    }

    fn should_render(&self) -> bool {
        match self {
            TileType::None => false,
            TileType::ReferTo(..) => false,
            _ => true,
        }
    }

    fn extra_big(&self) -> Option<(i32, i32, Vec<usize>)> {
        match self {
            TileType::HayCart => Some((3, 2, vec![16, 17, 18, 32, 33, 34])),
            TileType::Barn => Some((2, 3, vec![48, 49, 64, 65, 80, 81])),
            TileType::LeftButte => Some((2, 7, vec![112, 113, 128, 129, 144, 145, 160, 161, 176, 177, 192, 193, 208, 209])),
            _ => None,
        }
    }

    pub fn can_player_enter(&self) -> bool {
        match self {
            TileType::FenceHorizontal
            | TileType::FenceVertical
            | TileType::Bush
            | TileType::HayCart
            | TileType::Barn => false,
            | TileType::LeftButte => false,
            _ => true,
        }
    }

    pub fn interact(&self, console: &Console) {
        match self {
            TileType::FenceHorizontal | TileType::FenceVertical => {
                console.write(
                    "There's a fence here. Maybe you can Jump over it?",
                    Color::WHITE,
                );
            }
            TileType::Bush => {
                console.write(
                    "This bush is prickly, but you might be able to jump it.",
                    Color::WHITE,
                );
            }
            TileType::Cauldron => {
                console.write("I guess that explains why you're a 6 foot tall chicken!", Color::YELLOW);
                console.write("Farmer Tom's Magic Miracle Grow", Color::YELLOW);
            }
            _ => {}
        }
    }
}

pub struct TileMapLayer {
    width_tiles: usize,
    height_tiles: usize,
    z: f32,
}

impl TileMapLayer {
    pub fn new(z: f32) -> Self {
        Self {
            width_tiles: NUM_TILES_X,
            height_tiles: NUM_TILES_Y,
            z,
        }
    }

    pub fn build_mesh(&self, tile_indices: &[TileType]) -> Mesh {
        let capacity = self.width_tiles & self.height_tiles;
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(capacity * 4);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(capacity * 4);
        let mut uv: Vec<[f32; 2]> = Vec::with_capacity(capacity * 4);
        let mut indices: Vec<u32> = Vec::with_capacity(capacity * 6);
        let mut index_count = 0;

        for y in 0..self.height_tiles {
            for x in 0..self.width_tiles {
                let idx = tile_index(x as i32, y as i32);
                if tile_indices[idx].should_render() {
                    if let Some((width, height, tiles)) = tile_indices[idx].extra_big() {
                        let mut counter = 0;
                        for ty in 0..height {
                            for tx in 0..width {
                                self.push_tile(
                                    x + tx as usize,
                                    y + ty as usize,
                                    &self.texture_coords_specific(tiles[counter]),
                                    &mut vertices,
                                    &mut normals,
                                    &mut uv,
                                    &mut indices,
                                    &mut index_count,
                                );
                                counter += 1;
                            }
                        }
                    } else {
                        self.push_tile(
                            x,
                            y,
                            &self.texture_coords(idx, tile_indices),
                            &mut vertices,
                            &mut normals,
                            &mut uv,
                            &mut indices,
                            &mut index_count,
                        );
                    }
                }
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }

    fn push_tile(
        &self,
        x: usize,
        y: usize,
        tex: &[f32; 4],
        vertices: &mut Vec<[f32; 3]>,
        normals: &mut Vec<[f32; 3]>,
        uv: &mut Vec<[f32; 2]>,
        indices: &mut Vec<u32>,
        index_count: &mut u32,
    ) {
        let pos = tile_to_screen(x as i32, y as i32);
        let left = pos.0 - (TILE_WIDTH as f32 / 2.0);
        let right = pos.0 + (TILE_WIDTH as f32 / 2.0);
        let top = pos.1 - (TILE_HEIGHT as f32 / 2.0);
        let bottom = pos.1 + (TILE_HEIGHT as f32 / 2.0);

        vertices.push([left, top, self.z]);
        vertices.push([right, top, self.z]);
        vertices.push([left, bottom, self.z]);
        vertices.push([right, bottom, self.z]);

        for _ in 0..4 {
            normals.push([0.0, 1.0, 0.0]);
        }

        //let tex = self.texture_coords(idx, tile_indices);
        uv.push([tex[0], tex[3]]);
        uv.push([tex[2], tex[3]]);
        uv.push([tex[0], tex[1]]);
        uv.push([tex[2], tex[1]]);

        indices.push(*index_count);
        indices.push(*index_count + 1);
        indices.push(*index_count + 2);

        indices.push(*index_count + 3);
        indices.push(*index_count + 2);
        indices.push(*index_count + 1);

        *index_count += 4;
    }

    fn texture_coords(&self, idx: usize, tile_indices: &[TileType]) -> [f32; 4] {
        const SHEET_WIDTH: usize = 16;
        const SHEET_HEIGHT: usize = 16;

        let tile_idx = tile_indices[idx].index();
        let tile_x = tile_idx % SHEET_WIDTH;
        let tile_y = tile_idx / SHEET_WIDTH;

        let width = 1.0 / SHEET_WIDTH as f32;
        let height = 1.0 / SHEET_HEIGHT as f32;

        let left = width * tile_x as f32;
        let right = left + width;
        let top = height * tile_y as f32;
        let bottom = top + height;

        [
            left,   // Left X
            top,    // Top Y
            right,  // Right X
            bottom, // Bottom Y
        ]
    }

    fn texture_coords_specific(&self, idx: usize) -> [f32; 4] {
        const SHEET_WIDTH: usize = 16;
        const SHEET_HEIGHT: usize = 16;

        let tile_idx = idx;
        let tile_x = tile_idx % SHEET_WIDTH;
        let tile_y = tile_idx / SHEET_WIDTH;

        let width = 1.0 / SHEET_WIDTH as f32;
        let height = 1.0 / SHEET_HEIGHT as f32;

        let left = width * tile_x as f32;
        let right = left + width;
        let top = height * tile_y as f32;
        let bottom = top + height;

        [
            left,   // Left X
            top,    // Top Y
            right,  // Right X
            bottom, // Bottom Y
        ]
    }
}

pub fn tile_to_screen(x: i32, y: i32) -> (f32, f32) {
    let scale = (TILE_WIDTH, TILE_HEIGHT);
    let screen_y = (0.0 - (768.0 / 2.0)) + (y as f32 * scale.1) + (4.0 * TILE_HEIGHT as f32); // Bevy centers on (0,0)
    let screen_x = (0.0 - (1024.0 / 2.0)) + (x as f32 * scale.0);
    (
        screen_x + (TILE_WIDTH as f32 / 2.0),
        (0.0 - (screen_y + (TILE_HEIGHT as f32 / 2.0))) + 128.0,
    )
}

#[derive(Component, Clone)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct LerpMove {
    pub start: (i32, i32),
    pub end: (i32, i32),
    pub step: u32,
    pub jumping: bool,
    pub animate: Option<Vec<usize>>,
}

pub fn tile_location_added(
    mut query: Query<
        (&TilePosition, &mut Transform),
        Or<(Added<TilePosition>, Changed<TilePosition>)>,
    >,
) {
    query.for_each_mut(|(tile_pos, mut trans)| {
        let tts = tile_to_screen(tile_pos.x, tile_pos.y);
        trans.translation = Vec3::new(tts.0, tts.1, trans.translation.z);
    });
}

pub fn tile_lerp(
    mut query: Query<(
        Entity,
        &mut LerpMove,
        &mut TilePosition,
        &mut Transform,
        &mut TextureAtlasSprite,
    )>,
    mut commands: Commands,
) {
    for (entity, mut lerp, mut pos, mut trans, mut sprite) in query.iter_mut() {
        lerp.step += 1;

        let start = tile_to_screen(lerp.start.0, lerp.start.1);
        let end = tile_to_screen(lerp.end.0, lerp.end.1);
        let step = ((end.0 - start.0) / 8.0, (end.1 - start.1) / 8.0);

        trans.translation.x = start.0 + (step.0 * lerp.step as f32);
        trans.translation.y = start.1 + (step.1 * lerp.step as f32);

        if let Some(animate) = &lerp.animate {
            let frame = lerp.step % animate.len() as u32;
            sprite.index = animate[frame as usize];
        }

        if lerp.jumping {
            match lerp.step {
                1 => trans.translation.y += 8.0,
                2 => trans.translation.y += 16.0,
                3 => trans.translation.y += 24.0,
                4 => trans.translation.y += 32.0,
                5 => trans.translation.y += 24.0,
                6 => trans.translation.y += 16.0,
                7 => trans.translation.y += 8.0,
                _ => {}
            }
        }

        // Finish the move
        if lerp.step > 8 {
            if let Some(animate) = &lerp.animate {
                sprite.index = animate[0];
            }
            pos.x = lerp.end.0;
            pos.y = lerp.end.1;
            let tts = tile_to_screen(pos.x, pos.y);
            trans.translation = Vec3::new(tts.0, tts.1, trans.translation.z);
            commands.entity(entity).remove::<LerpMove>();
        }
    }
}
