use crate::geo::{ImaginaryWorldPoint, WorldPoint};
use crate::world::{CtxTile, Tile};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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

    pub fn zero() -> Slice<T> {
        Slice {
            original: ImaginaryWorldPoint::new(0, 0),
            width: 0,
            height: 0,
            items: vec![],
        }
    }

    pub fn items(&self) -> &[T] {
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

    pub fn get(&self, point: &WorldPoint) -> Option<&T> {
        if self.width == 0 || self.height == 0 {
            return None;
        }

        let rel_point = self
            .original
            .relative_to((point.x as i32, point.y as i32))?;
        let index = rel_point.y as usize * self.width as usize + rel_point.x as usize;

        self.items.get(index)
    }

    /// Return mutable reference of item for given world point. If any.
    ///
    /// # Returns
    ///
    /// - `Option<(usize, &mut T)>` - Couple of item index and item. If any.
    pub fn get_mut(&mut self, point: &WorldPoint) -> Option<(usize, &mut T)> {
        if self.width == 0 || self.height == 0 {
            return None;
        }

        let rel_point = self
            .original
            .relative_to((point.x as i32, point.y as i32))?;
        let index = rel_point.y as usize * self.width as usize + rel_point.x as usize;

        self.items.get_mut(index).map(|v| (index, v))
    }

    /// Replace items index by given value, at index according to given WorldPoint.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - Item index according to given world point if given point in this Slice.
    pub fn set(&mut self, point: &WorldPoint, value: T) -> Option<usize> {
        if self.width == 0 || self.height == 0 {
            return None;
        }

        let rel_point = self
            .original
            .relative_to((point.x as i32, point.y as i32))?;
        let index = rel_point.y as usize * self.width as usize + rel_point.x as usize;

        self.items[index] = value;

        Some(index)
    }
}

impl Default for Slice<CtxTile<Tile>> {
    fn default() -> Self {
        Self {
            original: Default::default(),
            width: Default::default(),
            height: Default::default(),
            items: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::world::{TerrainType, Tile};

    use super::*;
    use rstest::rstest;

    #[test]
    fn test_partial_world_get_tile_minimal() {
        let world = Slice::new(
            ImaginaryWorldPoint::new(5, 5),
            1,
            1,
            vec![Tile::new(TerrainType::Plain)],
        );
        assert_eq!(
            world.get(&WorldPoint::new(5, 5)),
            Some(&Tile::new(TerrainType::Plain))
        );
    }

    #[test]
    fn test_partial_world_get_tile_outside() {
        let world = Slice::new(
            ImaginaryWorldPoint::new(5, 5),
            1,
            1,
            vec![Tile::new(TerrainType::Plain)],
        );
        assert_eq!(world.get(&WorldPoint::new(6, 6)), None);
    }

    fn partial_world_by_one() -> Slice<Tile> {
        Slice::new(
            ImaginaryWorldPoint::new(10, 100),
            4,
            4,
            vec![
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::GrassLand),
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::GrassLand),
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::GrassLand),
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::GrassLand),
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::GrassLand),
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::GrassLand),
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::GrassLand),
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::GrassLand),
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
        assert_eq!(world.get(&point.into()), Some(&Tile::new(expected_terrain)));
    }

    fn create_partial_world_various_by_two() -> Slice<Tile> {
        Slice::new(
            ImaginaryWorldPoint::new(10, 100),
            4,
            4,
            vec![
                // Line 0
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::GrassLand),
                Tile::new(TerrainType::GrassLand),
                // Line 1
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::GrassLand),
                Tile::new(TerrainType::GrassLand),
                // Line 2
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::GrassLand),
                Tile::new(TerrainType::GrassLand),
                // Line 3
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::Plain),
                Tile::new(TerrainType::GrassLand),
                Tile::new(TerrainType::GrassLand),
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
        assert_eq!(world.get(&point.into()), Some(&Tile::new(expected_terrain)));
    }
}
