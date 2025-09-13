use bevy::sprite::TextureAtlasLayout;
use glam::{uvec2, UVec2};

pub const TILE_SIZE: UVec2 = uvec2(97, 49);
pub const TILES_ATLAS_PATH: &str = "img/terrain1.png";
pub const TILES_ATLAS_COLUMNS: u32 = 10;
pub const TILES_ATLAS_ROWS: u32 = 16;
pub const TILES_ATLAS_PADDING: Option<UVec2> = Some(uvec2(1, 1));
pub const TILES_ATLAS_OFFSET: Option<UVec2> = None;

pub fn tiles_texture_atlas_layout() -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(
        TILE_SIZE,
        TILES_ATLAS_COLUMNS,
        TILES_ATLAS_ROWS,
        TILES_ATLAS_PADDING,
        TILES_ATLAS_OFFSET,
    )
}
