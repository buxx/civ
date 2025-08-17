use bevy::sprite::TextureAtlasLayout;
use common::geo::ImaginaryWorldPoint;
use glam::{uvec2, UVec2, Vec2};
use hexx::{hex, HexLayout, HexOrientation};

pub const TILE_SIZE: UVec2 = uvec2(105, 120);
pub const TILES_ATLAS_PATH: &str = "img/tiles.png";
pub const TILES_ATLAS_COLUMNS: u32 = 10;
pub const TILES_ATLAS_ROWS: u32 = 1;
pub const TILES_ATLAS_PADDING: Option<UVec2> = None;
pub const TILES_ATLAS_OFFSET: Option<UVec2> = None;

pub fn relative_layout(origin: &ImaginaryWorldPoint) -> HexLayout {
    let origin = absolute_layout().hex_to_world_pos(hex(origin.x as i32, origin.y as i32));

    absolute_layout().with_origin(origin)
}

pub fn absolute_layout() -> HexLayout {
    zero_layout().with_rect_size(Vec2::new(TILE_SIZE.x as f32, TILE_SIZE.y as f32))
}

pub fn zero_layout() -> HexLayout {
    HexLayout::new(HexOrientation::Pointy)
}

pub fn tiles_texture_atlas_layout() -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(
        TILE_SIZE,
        TILES_ATLAS_COLUMNS,
        TILES_ATLAS_ROWS,
        TILES_ATLAS_PADDING,
        TILES_ATLAS_OFFSET,
    )
}
