use bevy::sprite::TextureAtlasLayout;
use glam::UVec2;

pub const SELECT_SIZE: UVec2 = UVec2::new(96, 48);
pub const SELECT_ATLAS_PATH: &str = "img/select.png";
pub const SELECT_ATLAS_COLUMNS: u32 = 4;
pub const SELECT_ATLAS_ROWS: u32 = 1;
pub const SELECT_ATLAS_PADDING: Option<UVec2> = None;
pub const SELECT_ATLAS_OFFSET: Option<UVec2> = None;

pub fn select_texture_atlas_layout() -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(
        SELECT_SIZE,
        SELECT_ATLAS_COLUMNS,
        SELECT_ATLAS_ROWS,
        SELECT_ATLAS_PADDING,
        SELECT_ATLAS_OFFSET,
    )
}
