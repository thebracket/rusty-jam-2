use bevy::prelude::*;
use bevy::sprite::ColorMaterial;

pub struct GameAssets {
    pub font: Handle<Font>,
    pub tileset: Handle<ColorMaterial>,
    pub player_chicken: Handle<TextureAtlas>,
    pub doggies: Handle<TextureAtlas>,
    pub chick: Handle<TextureAtlas>,
}

impl GameAssets {
    pub fn new(
        asset_server: &AssetServer,
        materials: &mut Assets<ColorMaterial>,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        // Load the map tileset
        let tileset_handle = asset_server.load("tileset.png");
        let material_handle = materials.add(ColorMaterial::from(tileset_handle.clone()));

        // Load the player graphics
        let texture_handle = asset_server.load("player_chicken.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 6, 4);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        // Load the doggies
        let texture_handle = asset_server.load("dog.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 16, 5);
        let dog_atlas_handle = texture_atlases.add(texture_atlas);

        // Load the chicks
        let texture_handle = asset_server.load("chick_24x24.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 4);
        let chick_atlas_handle = texture_atlases.add(texture_atlas);

        // Return the assets
        Self {
            font: asset_server.load("Titania.ttf"),
            tileset: material_handle,
            player_chicken: texture_atlas_handle,
            doggies: dog_atlas_handle,
            chick: chick_atlas_handle,
        }
    }
}
