use bevy::sprite::TextureAtlasLayout;
use common::geo::ImaginaryWorldPoint;
use glam::{uvec2, UVec2, Vec2};
use hexx::{hex, HexLayout, HexOrientation};

pub const TILE_SIZE: UVec2 = uvec2(120, 140);
pub const TILES_ATLAS_PATH: &str = "img/tiles.png";
pub const TILES_ATLAS_COLUMNS: u32 = 6;
pub const TILES_ATLAS_ROWS: u32 = 5;
pub const TILES_ATLAS_PADDING: Option<UVec2> = None;
pub const TILES_ATLAS_OFFSET: Option<UVec2> = None;

pub fn layout(origin: &ImaginaryWorldPoint) -> HexLayout {
    let origin = HexLayout::new(HexOrientation::Pointy)
        .with_rect_size(Vec2::new(TILE_SIZE.x as f32, TILE_SIZE.y as f32))
        .hex_to_world_pos(hex(origin.x as i32, origin.y as i32));

    HexLayout::new(HexOrientation::Pointy)
        .with_rect_size(Vec2::new(TILE_SIZE.x as f32, TILE_SIZE.y as f32))
        .with_origin(origin)
}

pub fn texture_atlas_layout() -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(
        TILE_SIZE,
        TILES_ATLAS_COLUMNS,
        TILES_ATLAS_ROWS,
        TILES_ATLAS_PADDING,
        TILES_ATLAS_OFFSET,
    )
}
