use common::{
    game::slice::GameSlice,
    geo::WorldPoint,
    world::{CtxTile, TerrainType, Tile},
};

use crate::map::AtlasIndex;

pub trait AsAtlasIndex {
    fn atlas_index(&self) -> AtlasIndex;
}

impl AsAtlasIndex for Option<&CtxTile<Tile>> {
    fn atlas_index(&self) -> AtlasIndex {
        self.map(|tile| match tile {
            CtxTile::Outside => AtlasIndex(4),
            CtxTile::Visible(tile) => terrain_type_index(&tile.type_()),
        })
        .unwrap_or(AtlasIndex(3))
    }
}

fn terrain_type_index(terrain: &TerrainType) -> AtlasIndex {
    match terrain {
        TerrainType::GrassLand => AtlasIndex(0),
        TerrainType::Plain => AtlasIndex(1),
    }
}

pub trait Displayable {
    fn atlas_index(&self) -> AtlasIndex;
}

pub fn tile_display<'a>(
    game_slice: &'a GameSlice,
    point: &'a WorldPoint,
) -> Option<Box<&'a dyn Displayable>> {
    todo!()
}
