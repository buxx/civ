use common::{
    game::slice::{ClientCity, ClientUnit},
    world::{CtxTile, TerrainType, Tile},
};
use dyn_clone::DynClone;

use crate::map::AtlasIndex;

fn terrain_type_index(terrain: &TerrainType) -> AtlasIndex {
    match terrain {
        TerrainType::GrassLand => AtlasIndex(0),
        TerrainType::Plain => AtlasIndex(1),
    }
}

pub trait Displayable: DynClone {
    fn atlas_index(&self) -> AtlasIndex;
}
dyn_clone::clone_trait_object!(Displayable);

impl Displayable for CtxTile<Tile> {
    fn atlas_index(&self) -> AtlasIndex {
        match self {
            CtxTile::Outside => AtlasIndex(4),
            CtxTile::Visible(tile) => terrain_type_index(&tile.type_()),
        }
    }
}

impl Displayable for ClientUnit {
    fn atlas_index(&self) -> AtlasIndex {
        AtlasIndex(5)
    }
}

impl Displayable for ClientCity {
    fn atlas_index(&self) -> AtlasIndex {
        AtlasIndex(4)
    }
}
