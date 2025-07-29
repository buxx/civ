use std::{fs, io, path::PathBuf};

use async_std::channel::Sender;
use common::world::{Chunk, CtxTile, World};
use thiserror::Error;

use common::{space::window::Window, world::Tile};

use common::utils::Progress;

use crate::game::city::City;
use crate::game::unit::Unit;
use crate::world::WorldItem;

#[derive(Error, Debug, Clone)]
pub enum WorldReaderError {
    #[error("Failed to init world: {0}")]
    InitWorldError(InitWorldError),
    #[error("World not initialized")]
    NotInitialized,
}

#[derive(Error, Debug, Clone)]
pub enum InitWorldError {
    #[error("Disk access error : {0}")]
    Io(io::ErrorKind),
    #[error("World.ron load error : {0}")]
    InvalidWorld(#[from] ron::de::SpannedError),
    #[error("Chunk decoding error : {0}")]
    InvalidChunk(String),
}

// TODO: rename ... ?
pub struct WorldReader {
    // FIXME: not required as attribute ?
    source: PathBuf,
    width: u64,
    height: u64,
    items: Vec<WorldItem>,
}

impl WorldReader {
    pub fn new(source: PathBuf, width: u64, height: u64, items: Vec<WorldItem>) -> Self {
        Self {
            source,
            width,
            height,
            items,
        }
    }

    pub fn from(
        source: PathBuf,
        progress: &Option<Sender<Progress<WorldReaderError>>>,
    ) -> Result<Self, WorldReaderError> {
        let mut self_ = Self {
            source,
            width: 0,
            height: 0,
            items: vec![],
        };

        let world: World = ron::from_str(
            &fs::read_to_string(self_.source.join("world.ron"))
                .map_err(|e| WorldReaderError::InitWorldError(InitWorldError::Io(e.kind())))?,
        )
        .map_err(|e| WorldReaderError::InitWorldError(InitWorldError::InvalidWorld(e)))?;

        let chunked_width = world.width / world.chunk_size;
        let chunked_height = world.height / world.chunk_size;

        self_.items.clear();
        self_.width = world.width;
        self_.height = world.height;
        let mut done = 0;
        let expected = world.width * world.height;

        for chunk_x in 0..chunked_width {
            for chunk_y in 0..chunked_height {
                let file_name = format!("{}_{}.ct", chunk_x, chunk_y);
                let chunk: Chunk<WorldItem> =
                    bincode::deserialize(&fs::read(self_.source.join(file_name)).map_err(|e| {
                        WorldReaderError::InitWorldError(InitWorldError::Io(e.kind()))
                    })?)
                    .map_err(|e| {
                        WorldReaderError::InitWorldError(InitWorldError::InvalidChunk(
                            e.to_string(),
                        ))
                    })?;

                done += chunk.item.len();
                self_.items.extend(chunk.item);

                let progress_ = done as f32 / expected as f32;
                progress
                    .as_ref()
                    .map(|s| s.send_blocking(Progress::InProgress(progress_)));
            }
        }

        progress
            .as_ref()
            .map(|s| s.send_blocking(Progress::Finished));
        Ok(self_)
    }

    pub fn tile(&self, x: u64, y: u64) -> Option<&Tile> {
        let index = y * self.width + x;
        self.items.get(index as usize).map(|i| &i.tile)
    }

    pub fn city(&self, x: u64, y: u64) -> Option<&City> {
        let index = y * self.width + x;
        self.items.get(index as usize).and_then(|i| i.city.as_ref())
    }

    pub fn units(&self, x: u64, y: u64) -> &[Unit] {
        let index = y * self.width + x;
        self.items
            .get(index as usize)
            .map(|i| i.units.as_ref())
            .unwrap_or(&[])
    }

    pub fn shape(&self) -> u64 {
        self.items.len() as u64
    }

    pub fn items(&self, window: &Window) -> Vec<Option<&WorldItem>> {
        window_items(&self.items, window, self.width as i64, self.height as i64)
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn height(&self) -> u64 {
        self.height
    }
}

pub fn window_items<'a>(
    world_tiles: &'a [WorldItem],
    window: &Window,
    world_width: i64,
    world_height: i64,
) -> Vec<Option<&'a WorldItem>> {
    let row_width = (window.end().x - window.start().x + 1) as usize;
    let mut tiles = Vec::with_capacity(window.shape() as usize);

    for y in window.start().y..=window.end().y {
        if y < 0 || y >= world_height {
            tiles.resize(tiles.len() + row_width, None);
            continue;
        }

        let start_x = window.start().x;
        let end_x = window.end().x;

        // Left out-of-bounds padding
        let left_padding = (-start_x).max(0).min(row_width as i64) as usize;
        tiles.resize(tiles.len() + left_padding, None);

        // Visible tiles within world bounds
        if start_x < world_width && end_x >= 0 {
            let clamped_start_x = start_x.max(0).min(world_width - 1);
            let clamped_end_x = end_x.min(world_width - 1);

            if clamped_start_x <= clamped_end_x {
                let line_start_idx = (y * world_width + clamped_start_x) as usize;
                let line_end_idx = (y * world_width + clamped_end_x) as usize;

                tiles.extend(world_tiles[line_start_idx..=line_end_idx].iter().map(Some));
            }
        }

        // Right out-of-bounds padding
        let right_padding = ((end_x + 1) - world_width).max(0).min(row_width as i64) as usize;
        tiles.resize(tiles.len() + right_padding, None);
    }

    tiles
}

// FIXME BS NOW: add tests with CtxTile::Outside
#[cfg(test)]
mod test {
    use common::{geo::ImaginaryWorldPoint, space::window::DisplayStep, world::TerrainType};
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
        let world_items: Vec<WorldItem> = world_tiles
            .into_iter()
            .map(|t| common::world::item::WorldItem::new(t, None, vec![]).into())
            .collect();

        // WHEN
        let window_tiles: Vec<Option<Tile>> =
            window_items(&world_items, &window, world_width, world_height)
                .iter()
                .map(|i| i.map(|i| i.tile.clone()))
                .collect();

        // THEN
        assert_eq!(
            window_tiles,
            vec![
                //
                Some(Tile::new(TerrainType::Plain)),
                Some(Tile::new(TerrainType::Plain)),
                Some(Tile::new(TerrainType::GrassLand)),
                //
                Some(Tile::new(TerrainType::Plain)),
                Some(Tile::new(TerrainType::Plain)),
                Some(Tile::new(TerrainType::GrassLand)),
                //
                Some(Tile::new(TerrainType::GrassLand)),
                Some(Tile::new(TerrainType::GrassLand)),
                Some(Tile::new(TerrainType::GrassLand)),
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
        let window_tiles: Vec<CtxTile<Tile>> =
            window_items(&world_tiles, &window, world_width, world_height)
                .into_iter()
                .map(|t| t.into())
                .collect();

        // THEN
        assert_eq!(
            window_tiles,
            vec![
                //
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                //
                CtxTile::Outside,
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Outside,
                //
                CtxTile::Outside,
                CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
                CtxTile::Visible(Tile::new(TerrainType::Plain)),
                CtxTile::Outside,
                //
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
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
        window_items(&world_tiles, &window, world_width, world_height);
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
        window_items(&world_tiles, &window, world_width, world_height);
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

        // WHEN-THEN
        // WHEN
        let window_tiles: Vec<CtxTile<Tile>> =
            window_items(&world_tiles, &window, world_width, world_height)
                .into_iter()
                .map(|t| t.into())
                .collect();

        // THEN
        assert_eq!(
            window_tiles,
            vec![
                //
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                //
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                //
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                //
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                //
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
                CtxTile::Outside,
            ]
        );
    }
}
