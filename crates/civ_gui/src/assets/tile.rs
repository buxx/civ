use bevy::image::TextureAtlasLayout;
use glam::UVec2;

pub const TILE_SIZE: UVec2 = UVec2::new(93, 48);
pub const TILES_ATLAS_PATH: &str = "img/terrain1.png";
pub const TILES_ATLAS_COLUMNS: u32 = 10;
pub const TILES_ATLAS_ROWS: u32 = 16;
pub const TILES_ATLAS_PADDING: Option<UVec2> = Some(UVec2::new(4, 2));
pub const TILES_ATLAS_OFFSET: Option<UVec2> = Some(UVec2::new(2, 1));

pub fn tiles_texture_atlas_layout() -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(
        TILE_SIZE,
        TILES_ATLAS_COLUMNS,
        TILES_ATLAS_ROWS,
        TILES_ATLAS_PADDING,
        TILES_ATLAS_OFFSET,
    )
}
