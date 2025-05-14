use crate::geo::{ImaginaryWorldPoint, WorldPoint};
use serde::{Deserialize, Serialize};

use super::{CtxTile, Tile};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PartialWorld {
    original: ImaginaryWorldPoint,
    width: u64,
    height: u64,
    tiles: Vec<CtxTile<Tile>>,
}

impl PartialWorld {
    pub fn new(
        original: ImaginaryWorldPoint,
        width: u64,
        height: u64,
        tiles: Vec<CtxTile<Tile>>,
    ) -> Self {
        Self {
            original,
            width,
            height,
            tiles,
        }
    }

    pub fn tiles(&self) -> &[CtxTile<Tile>] {
        &self.tiles
    }

    // pub fn rectangle(&self) -> Rectangle<i32> {
    //     let (left, right) = if self.width % 2 == 0 {
    //         ((self.width / 2) as i32, (self.width / 2) as i32 - 1)
    //     } else {
    //         ((self.width / 2) as i32, (self.width / 2) as i32)
    //     };

    //     let (top, bottom) = if self.height % 2 == 0 {
    //         ((self.height / 2) as i32, (self.height / 2) as i32 - 1)
    //     } else {
    //         ((self.height / 2) as i32, (self.height / 2) as i32)
    //     };

    //     Rectangle::from([left, right, top, bottom])
    // }

    pub fn original(&self) -> &ImaginaryWorldPoint {
        &self.original
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn try_world_point_for_center_rel(&self, pos: (isize, isize)) -> Option<WorldPoint> {
        let original_x = self.original.x as isize;
        let original_y = self.original.y as isize;
        let rel_center_x = (self.width / 2) as isize;
        let rel_center_y = (self.height / 2) as isize;
        let rel_x = rel_center_x + pos.0;
        let rel_y = rel_center_y + pos.1;
        let world_x = original_x + rel_x;
        let world_y = original_y + rel_y;

        if world_x < 0
            || world_y < 0
            || world_x > (original_x + (self.width as isize - 1))
            || world_y > (original_y + (self.height as isize - 1))
        {
            return None;
        }

        Some(WorldPoint::new(world_x as u64, world_y as u64))
    }

    pub fn imaginary_world_point_for_center_rel(&self, pos: (isize, isize)) -> ImaginaryWorldPoint {
        let original_x = self.original.x as isize;
        let original_y = self.original.y as isize;
        let rel_center_x = (self.width / 2) as isize;
        let rel_center_y = (self.height / 2) as isize;
        let rel_x = rel_center_x + pos.0;
        let rel_y = rel_center_y + pos.1;
        let world_x = original_x + rel_x;
        let world_y = original_y + rel_y;

        ImaginaryWorldPoint::new(world_x as i64, world_y as i64)
    }

    pub fn tile(&self, point: &WorldPoint) -> &CtxTile<Tile> {
        let rel_point = match self.original.relative_to((point.x as i32, point.y as i32)) {
            Some(rel_point) => rel_point,
            None => return &CtxTile::Outside,
        };
        let index = (rel_point.y * self.height as i64) + (rel_point.x % self.width as i64);

        match self.tiles.get(index as usize) {
            Some(tile) => tile,
            None => &CtxTile::Outside,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::world::TerrainType;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((0, 0), (9, 9), (0, 0), (4, 4))]
    #[case((0, 0), (9, 9), (0, -4), (4, 0))]
    #[case((0, 0), (9, 9), (-4, 0), (0, 4))]
    #[case((0, 0), (9, 9), (-4, -4), (0, 0))]
    #[case((0, 0), (9, 9), (4, 4), (8, 8))]
    #[case((0, 0), (10, 10), (0, 0), (5,5))]
    #[case((0, 0), (10, 10), (0, -4), (5, 1))]
    #[case((0, 0), (10, 10), (-4, 0), (1, 5))]
    #[case((0, 0), (10, 10), (-4, -4), (1, 1))]
    #[case((0, 0), (10, 10), (4, 4), (9, 9))]
    #[case((0, 0), (15, 12), (0, 0), (7, 6))]
    fn test_partial_world_point_for_center_rel(
        #[case] point: (u64, u64),
        #[case] size: (u64, u64),
        #[case] rel: (isize, isize),
        #[case] expected: (u64, u64),
    ) {
        let world = PartialWorld::new(point.into(), size.0, size.1, vec![]);
        assert_eq!(
            world.try_world_point_for_center_rel(rel),
            Some(expected.into())
        );
    }

    #[test]
    fn test_partial_world_get_tile_minimal() {
        let world = PartialWorld::new(
            ImaginaryWorldPoint::new(5, 5),
            1,
            1,
            vec![
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            ],
        );
        assert_eq!(
            world.tile(&WorldPoint::new(6, 6)),
            &CtxTile::Visible(Tile::new(TerrainType::GrassLand))
        );
    }

    fn partial_world_by_one() -> PartialWorld {
        PartialWorld::new(
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
    #[case((-2, -2), (10, 100))]
    #[case((-1, -2), (11, 100))]
    #[case((0, -2), (12, 100))]
    #[case((1, -2), (13, 100))]
    #[case((-2, -1), (10, 101))]
    #[case((-1, -1), (11, 101))]
    #[case((0, -1), (12, 101))]
    #[case((1, -1), (13, 101))]
    #[case((-2, 0), (10, 102))]
    #[case((-1, 0), (11, 102))]
    #[case((0, 0), (12, 102))]
    #[case((1, 0), (13, 102))]
    #[case((-2, 1), (10, 103))]
    #[case((-1, 1), (11, 103))]
    #[case((0, 1), (12, 103))]
    #[case((1, 1), (13, 103))]
    fn test_try_world_point_for_center_rel_by_one(
        #[case] rel: (isize, isize),
        #[case] abs: (u64, u64),
    ) {
        let world = partial_world_by_one();
        assert_eq!(world.try_world_point_for_center_rel(rel), Some(abs.into()));
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
            world.tile(&point.into()),
            &CtxTile::Visible(Tile::new(expected_terrain))
        );
    }

    fn create_partial_world_various_by_two() -> PartialWorld {
        PartialWorld::new(
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
    #[case((-2, -2), (10, 100))]
    #[case((-1, -2), (11, 100))]
    #[case((0, -2), (12, 100))]
    #[case((1, -2), (13, 100))]
    #[case((-2, -1), (10, 101))]
    #[case((-1, -1), (11, 101))]
    #[case((0, -1), (12, 101))]
    #[case((1, -1), (13, 101))]
    #[case((-2, 0), (10, 102))]
    #[case((-1, 0), (11, 102))]
    #[case((0, 0), (12, 102))]
    #[case((1, 0), (13, 102))]
    #[case((-2, 1), (10, 103))]
    #[case((-1, 1), (11, 103))]
    #[case((0, 1), (12, 103))]
    #[case((1, 1), (13, 103))]
    fn test_try_world_point_for_center_rel_by_two(
        #[case] rel: (isize, isize),
        #[case] abs: (u64, u64),
    ) {
        let world = create_partial_world_various_by_two();
        assert_eq!(world.try_world_point_for_center_rel(rel), Some(abs.into()));
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
            world.tile(&point.into()),
            &CtxTile::Visible(Tile::new(expected_terrain))
        );
    }
}
