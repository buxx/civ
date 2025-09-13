use bon::Builder;
use derive_more::Constructor;
use glam::{U64Vec2, UVec2, Vec2};
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default,
)]
pub struct WorldPoint {
    pub x: u64,
    pub y: u64,
}

impl WorldPoint {
    pub fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }

    pub fn from_iso(size: UVec2, point: Vec2) -> Self {
        let sx = (point.x - point.y) * (size.x as f32 * 0.5);
        let sy = (point.x + point.y) * (size.y as f32 * 0.5);

        Self {
            x: sx as u64,
            y: sy as u64,
        }
    }

    pub fn apply(&self, pos: (i32, i32)) -> Self {
        let x = self.x as isize;
        let y = self.y as isize;
        let new_x = pos.0 as isize - x;
        let new_y = pos.1 as isize - y;
        Self {
            x: new_x.max(0) as u64,
            y: new_y.max(0) as u64,
        }
    }

    pub fn relative_to(&self, pos: (i32, i32)) -> Option<Self> {
        let x = self.x as isize;
        let y = self.y as isize;
        let new_x = pos.0 as isize - x;
        let new_y = pos.1 as isize - y;

        if new_x < 0 || new_y < 0 {
            None
        } else {
            Some(Self {
                x: new_x as u64,
                y: new_y as u64,
            })
        }
    }
}

impl From<(u64, u64)> for WorldPoint {
    fn from(value: (u64, u64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<WorldPoint> for U64Vec2 {
    fn from(value: WorldPoint) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<WorldPoint> for Vec2 {
    fn from(value: WorldPoint) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default)]
pub struct ImaginaryWorldPoint {
    pub x: i64,
    pub y: i64,
}

impl ImaginaryWorldPoint {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn relative_to(&self, pos: (i32, i32)) -> Option<Self> {
        let x = self.x as isize;
        let y = self.y as isize;
        let new_x = pos.0 as isize - x;
        let new_y = pos.1 as isize - y;

        if new_x < 0 || new_y < 0 {
            None
        } else {
            Some(Self {
                x: new_x as i64,
                y: new_y as i64,
            })
        }
    }
}

impl From<WorldPoint> for ImaginaryWorldPoint {
    fn from(value: WorldPoint) -> Self {
        ImaginaryWorldPoint::new(value.x as i64, value.y as i64)
    }
}

impl From<(u64, u64)> for ImaginaryWorldPoint {
    fn from(value: (u64, u64)) -> Self {
        ImaginaryWorldPoint::new(value.0 as i64, value.1 as i64)
    }
}

impl From<ImaginaryWorldPoint> for Vec2 {
    fn from(value: ImaginaryWorldPoint) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

pub trait Geo {
    fn geo(&self) -> &GeoContext;
    fn geo_mut(&mut self) -> &mut GeoContext;
}

#[derive(
    Builder,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Constructor,
)]
pub struct GeoContext {
    point: WorldPoint,
}

impl GeoContext {
    pub fn point(&self) -> &WorldPoint {
        &self.point
    }

    pub fn set_point(&mut self, value: WorldPoint) {
        self.point = value
    }
}

#[derive(Debug, Constructor)]
pub struct GeoVec<T> {
    geo: GeoContext,
    items: Vec<T>,
}

impl<T> GeoVec<T> {
    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut Vec<T> {
        &mut self.items
    }
}

impl<T> Geo for GeoVec<T> {
    fn geo(&self) -> &GeoContext {
        &self.geo
    }

    fn geo_mut(&mut self) -> &mut GeoContext {
        &mut self.geo
    }
}

impl<T> From<(GeoContext, Vec<T>)> for GeoVec<T> {
    fn from(value: (GeoContext, Vec<T>)) -> Self {
        Self {
            geo: value.0,
            items: value.1,
        }
    }
}

impl<T> From<GeoVec<T>> for Vec<T> {
    fn from(val: GeoVec<T>) -> Self {
        val.items
    }
}
