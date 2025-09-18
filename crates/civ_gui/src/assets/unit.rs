use bevy::sprite::TextureAtlasLayout;
use glam::UVec2;

pub const UNIT_SIZE: UVec2 = UVec2::new(63, 48);
pub const UNITS_ATLAS_PATH: &str = "img/units.png";
pub const UNITS_ATLAS_COLUMNS: u32 = 20;
pub const UNITS_ATLAS_ROWS: u32 = 3;
pub const UNITS_ATLAS_PADDING: Option<UVec2> = Some(UVec2::new(2, 1));
pub const UNITS_ATLAS_OFFSET: Option<UVec2> = Some(UVec2::new(2, 1));

pub fn units_texture_atlas_layout() -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(
        UNIT_SIZE,
        UNITS_ATLAS_COLUMNS,
        UNITS_ATLAS_ROWS,
        UNITS_ATLAS_PADDING,
        UNITS_ATLAS_OFFSET,
    )
}
