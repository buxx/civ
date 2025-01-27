use common::world::{CtxTile, TerrainType, Tile};

use crate::ingame::AtlasIndex;

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
