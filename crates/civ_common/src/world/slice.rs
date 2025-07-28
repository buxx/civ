use crate::geo::{ImaginaryWorldPoint, WorldPoint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Slice<T> {
    original: ImaginaryWorldPoint,
    width: u64,
    height: u64,
    items: Vec<T>,
}

impl<T> Slice<T> {
    pub fn new(original: ImaginaryWorldPoint, width: u64, height: u64, items: Vec<T>) -> Self {
        Self {
            original,
            width,
            height,
            items,
        }
    }

    #[cfg(test)]
    pub fn zero() -> Slice<T> {
        Slice {
            original: ImaginaryWorldPoint::new(0, 0),
            width: 0,
            height: 0,
            items: vec![],
        }
    }

    pub fn tiles(&self) -> &[T] {
        &self.items
    }

    pub fn original(&self) -> &ImaginaryWorldPoint {
        &self.original
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn item(&self, point: &WorldPoint) -> Option<&T> {
        let rel_point = self
            .original
            .relative_to((point.x as i32, point.y as i32))?;
        let index = (rel_point.y * self.height as i64) + (rel_point.x % self.width as i64);

        match self.items.get(index as usize) {
            Some(tile) => Some(tile),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::world::{CtxTile, TerrainType, Tile};

    use super::*;
    use rstest::rstest;

    #[test]
    fn test_partial_world_get_tile_minimal() {
        let world = Slice::new(
            ImaginaryWorldPoint::new(5, 5),
            1,
            1,
            vec![
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            ],
        );
        assert_eq!(
            world.item(&WorldPoint::new(6, 6)),
            Some(&CtxTile::Visible(Tile::new(TerrainType::GrassLand)))
        );
    }

    fn partial_world_by_one() -> Slice<CtxTile<Tile>> {
        Slice::new(
            ImaginaryWorldPoint::new(10, 100),
            4,
            4,
            vec![
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            ],
        )
    }

    #[rstest]
    #[case((10, 100), TerrainType::Plain)]
    #[case((11, 100), TerrainType::GrassLand)]
    #[case((12, 100), TerrainType::Plain)]
    #[case((13, 100), TerrainType::GrassLand)]
    #[case((10, 101), TerrainType::Plain)]
    #[case((11, 101), TerrainType::GrassLand)]
    #[case((12, 101), TerrainType::Plain)]
    #[case((13, 101), TerrainType::GrassLand)]
    #[case((10, 102), TerrainType::Plain)]
    #[case((11, 102), TerrainType::GrassLand)]
    #[case((12, 102), TerrainType::Plain)]
    #[case((13, 102), TerrainType::GrassLand)]
    #[case((10, 103), TerrainType::Plain)]
    #[case((11, 103), TerrainType::GrassLand)]
    #[case((12, 103), TerrainType::Plain)]
    #[case((13, 103), TerrainType::GrassLand)]
    fn test_get_tile_by_one(#[case] point: (u64, u64), #[case] expected_terrain: TerrainType) {
        let world = partial_world_by_one();
        assert_eq!(
            world.item(&point.into()),
            Some(&CtxTile::Visible(Tile::new(expected_terrain)))
        );
    }

    fn create_partial_world_various_by_two() -> Slice<CtxTile<Tile>> {
        Slice::new(
            ImaginaryWorldPoint::new(10, 100),
            4,
            4,
            vec![
                // Line 0
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                // Line 1
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                // Line 2
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                // Line 3
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            ],
        )
    }

    #[rstest]
    #[case((10, 100), TerrainType::Plain)]
    #[case((11, 100), TerrainType::Plain)]
    #[case((12, 100), TerrainType::GrassLand)]
    #[case((13, 100), TerrainType::GrassLand)]
    #[case((10, 101), TerrainType::Plain)]
    #[case((11, 101), TerrainType::Plain)]
    #[case((12, 101), TerrainType::GrassLand)]
    #[case((13, 101), TerrainType::GrassLand)]
    #[case((10, 102), TerrainType::Plain)]
    #[case((11, 102), TerrainType::Plain)]
    #[case((12, 102), TerrainType::GrassLand)]
    #[case((13, 102), TerrainType::GrassLand)]
    #[case((10, 103), TerrainType::Plain)]
    #[case((11, 103), TerrainType::Plain)]
    #[case((12, 103), TerrainType::GrassLand)]
    #[case((13, 103), TerrainType::GrassLand)]
    fn test_get_tile_by_two(#[case] point: (u64, u64), #[case] expected_terrain: TerrainType) {
        let world = create_partial_world_various_by_two();
        assert_eq!(
            world.item(&point.into()),
            Some(&CtxTile::Visible(Tile::new(expected_terrain)))
        );
    }
}
