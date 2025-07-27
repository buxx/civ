use glam::U64Vec2;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::{
    geo::Geo,
    space::{window::Window, D2Size},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rectangle<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

impl Rectangle<i32> {
    pub fn into_pointy_rectangle(self) -> [i32; 4] {
        [-self.left, self.right, -self.top, self.bottom]
    }
}

impl<T: Copy> From<[T; 4]> for Rectangle<T> {
    fn from(value: [T; 4]) -> Self {
        Rectangle {
            left: value[0],
            right: value[1],
            top: value[2],
            bottom: value[3],
        }
    }
}

#[derive(Debug)]
pub enum Progress<E: Error + Clone> {
    InProgress(f32),
    Finished,
    Error(E),
}

// FIXME: write tests (inspire from other flat2d struct)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vec2d<T: Clone> {
    size: D2Size,
    items: Vec<Option<T>>,
}

impl<T: Clone> Vec2d<T> {
    pub fn new(size: D2Size) -> Self {
        let len = size.len();
        let mut items = Vec::with_capacity(len);
        items.resize_with(len, || None);
        Self { size, items }
    }

    pub fn from(size: D2Size, items: Vec<impl Into<T> + Geo>) -> Self {
        let mut self_ = Self::new(size);
        for item in items {
            let item_point = item.geo().point();
            let index = item_point.y as usize * size.width() + item_point.x as usize;
            self_.items[index] = Some(item.into());
        }
        self_
    }

    pub fn index(&self, point: impl Into<U64Vec2>) -> usize {
        let point: U64Vec2 = point.into();
        point.y as usize * self.size.width() + point.x as usize
    }

    pub fn get_by_point(&self, point: impl Into<U64Vec2>) -> &Option<T> {
        let index = self.index(point);
        &self.items[index]
    }

    pub fn get(&self, index: usize) -> &Option<T> {
        &self.items[index]
    }

    pub fn get_by_point_mut(&mut self, point: impl Into<U64Vec2>) -> &mut Option<T> {
        let index = self.index(point);
        &mut self.items[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Option<T> {
        &mut self.items[index]
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Option<T>> {
        self.items.iter()
    }

    pub fn slice(&self, window: &Window) -> Vec<Option<T>> {
        slice(
            &self.items,
            window,
            self.size.width() as i64,
            self.size.height() as i64,
        )
        .iter()
        .map(|item| item.and_then(|x| x.clone()))
        .collect()
    }
}

impl<'a, T: Clone> IntoIterator for &'a Vec2d<T> {
    type Item = &'a Option<T>;
    type IntoIter = std::slice::Iter<'a, Option<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

pub fn slice<'a, T>(
    source: &'a [T],
    window: &Window,
    world_width: i64,
    world_height: i64,
) -> Vec<Option<&'a T>> {
    let row_width = (window.end().x - window.start().x + 1) as usize;
    let mut tiles = Vec::with_capacity(window.shape() as usize);

    for y in window.start().y..=window.end().y {
        if y < 0 || y >= world_height {
            tiles.resize(tiles.len() + row_width, None); // FIXME: why resize ?
            continue;
        }

        let start_x = window.start().x;
        let end_x = window.end().x;

        // Left out-of-bounds padding
        let left_padding = (-start_x).max(0).min(row_width as i64) as usize;
        tiles.resize(tiles.len() + left_padding, None); // FIXME: why resize ?

        // Visible tiles within world bounds
        if start_x < world_width && end_x >= 0 {
            let clamped_start_x = start_x.max(0).min(world_width - 1);
            let clamped_end_x = end_x.min(world_width - 1);

            if clamped_start_x <= clamped_end_x {
                let line_start_idx = (y * world_width + clamped_start_x) as usize;
                let line_end_idx = (y * world_width + clamped_end_x) as usize;

                tiles.extend(source[line_start_idx..=line_end_idx].iter().map(Some));
            }
        }

        // Right out-of-bounds padding
        let right_padding = ((end_x + 1) - world_width).max(0).min(row_width as i64) as usize;
        tiles.resize(tiles.len() + right_padding, None); // FIXME: why resize ?
    }

    tiles
}

#[cfg(test)]
mod test {
    use crate::{
        geo::{GeoContext, ImaginaryWorldPoint, WorldPoint},
        space::window::DisplayStep,
        world::{TerrainType, Tile},
    };
    use derive_more::Constructor;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_tiles_from_window() {
        // GIVEN
        let world_tiles = vec![
            // line 0
            Tile::new(TerrainType::GrassLand),
            Tile::new(TerrainType::GrassLand),
            Tile::new(TerrainType::GrassLand),
            Tile::new(TerrainType::GrassLand),
            // line 1
            Tile::new(TerrainType::GrassLand),
            Tile::new(TerrainType::Plain),
            Tile::new(TerrainType::Plain),
            Tile::new(TerrainType::GrassLand),
            // line 2
            Tile::new(TerrainType::GrassLand),
            Tile::new(TerrainType::Plain),
            Tile::new(TerrainType::Plain),
            Tile::new(TerrainType::GrassLand),
            // line 3
            Tile::new(TerrainType::GrassLand),
            Tile::new(TerrainType::GrassLand),
            Tile::new(TerrainType::GrassLand),
            Tile::new(TerrainType::GrassLand),
        ];
        let world_width = 4;
        let world_height = 4;
        let window = Window::new((1, 1).into(), (3, 3).into(), DisplayStep::Close);

        // WHEN
        let window_tiles: Vec<Option<&Tile>> =
            slice(&world_tiles, &window, world_width, world_height);

        // THEN
        assert_eq!(
            window_tiles,
            vec![
                //
                Some(&Tile::new(TerrainType::Plain)),
                Some(&Tile::new(TerrainType::Plain)),
                Some(&Tile::new(TerrainType::GrassLand)),
                //
                Some(&Tile::new(TerrainType::Plain)),
                Some(&Tile::new(TerrainType::Plain)),
                Some(&Tile::new(TerrainType::GrassLand)),
                //
                Some(&Tile::new(TerrainType::GrassLand)),
                Some(&Tile::new(TerrainType::GrassLand)),
                Some(&Tile::new(TerrainType::GrassLand)),
            ]
        );
    }

    #[rstest]
    fn test_tiles_from_window_outside() {
        // GIVEN
        let world_tiles = vec![
            // line 0
            Tile::new(TerrainType::GrassLand),
            Tile::new(TerrainType::Plain),
            // line 1
            Tile::new(TerrainType::GrassLand),
            Tile::new(TerrainType::Plain),
        ];
        let world_width = 2;
        let world_height = 2;
        let window = Window::new(
            ImaginaryWorldPoint::new(-1, -1),
            ImaginaryWorldPoint::new(2, 2),
            DisplayStep::Close,
        );

        // WHEN
        let window_tiles: Vec<Option<&Tile>> =
            slice(&world_tiles, &window, world_width, world_height);

        // THEN
        assert_eq!(
            window_tiles,
            vec![
                //
                None,
                None,
                None,
                None,
                //
                None,
                Some(&Tile::new(TerrainType::GrassLand)),
                Some(&Tile::new(TerrainType::Plain)),
                None,
                //
                None,
                Some(&Tile::new(TerrainType::GrassLand)),
                Some(&Tile::new(TerrainType::Plain)),
                None,
                //
                None,
                None,
                None,
                None,
            ]
        );
    }

    #[rstest]
    fn test_tiles_from_window_non_reg_slice() {
        // GIVEN
        let world_tiles = vec![Tile::new(TerrainType::GrassLand); 100];
        let world_width = 10;
        let world_height = 10;
        let window = Window::new(
            ImaginaryWorldPoint::new(-6, 3),
            ImaginaryWorldPoint::new(-2, 7),
            DisplayStep::Close,
        );

        // WHEN-THEN
        slice(&world_tiles, &window, world_width, world_height);
    }

    #[rstest]
    fn test_tiles_from_window_non_reg_slice2() {
        // GIVEN
        let world_tiles = vec![Tile::new(TerrainType::GrassLand); 100];
        let world_width = 10;
        let world_height = 10;
        let window = Window::new(
            ImaginaryWorldPoint::new(11, 3),
            ImaginaryWorldPoint::new(15, 7),
            DisplayStep::Close,
        );

        // WHEN-THEN
        slice(&world_tiles, &window, world_width, world_height);
    }

    #[rstest]
    fn test_tiles_from_window_outside2() {
        // GIVEN
        let world_tiles = vec![Tile::new(TerrainType::GrassLand); 100];
        let world_width = 10;
        let world_height = 10;
        let window = Window::new(
            ImaginaryWorldPoint::new(10, 3),
            ImaginaryWorldPoint::new(14, 7),
            DisplayStep::Close,
        );

        // WHEN
        let window_tiles: Vec<Option<&Tile>> =
            slice(&world_tiles, &window, world_width, world_height);

        // THEN
        assert_eq!(
            window_tiles,
            vec![
                None, None, None, None, None, //
                None, None, None, None, None, //
                None, None, None, None, None, //
                None, None, None, None, None, //
                None, None, None, None, None, //
            ]
        );
    }

    #[derive(Debug, Constructor, PartialEq, Eq, Clone, Copy)]
    struct GeoItem(GeoContext, usize);

    impl Geo for GeoItem {
        fn geo(&self) -> &GeoContext {
            &self.0
        }

        fn geo_mut(&mut self) -> &mut GeoContext {
            &mut self.0
        }
    }

    #[test]
    fn test_partial_and_outside() {
        // Given
        let size = D2Size::new(2, 2);
        let x00 = GeoContext::new(WorldPoint::new(0, 0));
        let x11 = GeoContext::new(WorldPoint::new(1, 1));
        let i00 = GeoItem::new(x00, 0);
        let i11 = GeoItem::new(x11, 1);
        let items: Vec2d<GeoItem> = Vec2d::from(size, vec![i00, i11]);

        // When-Then
        let window_start = ImaginaryWorldPoint::new(0, 0);
        let window_end = ImaginaryWorldPoint::new(1, 1);
        let window = Window::new(window_start, window_end, DisplayStep::Close);
        let slice = items.slice(&window);

        assert_eq!(slice, vec![Some(i00), None, None, Some(i11)]);

        // When-Then
        let window_start = ImaginaryWorldPoint::new(1, 1);
        let window_end = ImaginaryWorldPoint::new(2, 2);
        let window = Window::new(window_start, window_end, DisplayStep::Close);
        let slice = items.slice(&window);

        assert_eq!(slice, vec![Some(i11), None, None, None]);
    }
}
