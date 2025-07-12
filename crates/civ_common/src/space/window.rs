use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    geo::{GeoContext, ImaginaryWorldPoint},
    utils::Rectangle,
};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resolution {
    width: u64,
    height: u64,
}

impl Resolution {
    pub fn new(width: u64, height: u64) -> Self {
        Self { width, height }
    }

    pub fn rectangle(&self) -> Rectangle<i32> {
        let (left, right) = if self.width % 2 == 0 {
            ((self.width / 2) as i32, (self.width / 2) as i32 - 1)
        } else {
            ((self.width / 2) as i32, (self.width / 2) as i32)
        };

        let (top, bottom) = if self.height % 2 == 0 {
            ((self.height / 2) as i32, (self.height / 2) as i32 - 1)
        } else {
            ((self.height / 2) as i32, (self.height / 2) as i32)
        };

        Rectangle::from([left, right, top, bottom])
    }
}

// TODO: only for tests ?
impl Default for Resolution {
    fn default() -> Self {
        Self {
            width: 32,
            height: 32,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Window {
    start: ImaginaryWorldPoint,
    end: ImaginaryWorldPoint,
    step: DisplayStep,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            start: Default::default(),
            end: Default::default(),
            step: DisplayStep::Close,
        }
    }
}

impl Window {
    pub fn new(start: ImaginaryWorldPoint, end: ImaginaryWorldPoint, step: DisplayStep) -> Self {
        // TODO: check start is inferior to end
        Self { start, end, step }
    }

    pub fn from_around(point: &ImaginaryWorldPoint, resolution: &Resolution) -> Self {
        let rectangle = resolution.rectangle();
        let start_x = point.x - rectangle.left as i64;
        let start_y = point.y - rectangle.top as i64;
        let end_x = point.x + rectangle.right as i64;
        let end_y = point.y + rectangle.bottom as i64;
        Self::new(
            ImaginaryWorldPoint::new(start_x, start_y),
            ImaginaryWorldPoint::new(end_x, end_y),
            // FIXME BS NOW
            DisplayStep::Close,
        )
    }

    pub fn start(&self) -> &ImaginaryWorldPoint {
        &self.start
    }

    pub fn end(&self) -> &ImaginaryWorldPoint {
        &self.end
    }

    pub fn center(&self) -> ImaginaryWorldPoint {
        let width = self.end.x - self.start.x;
        let height = self.end.y - self.start.y;
        ImaginaryWorldPoint {
            x: self.start.x + (width / 2),
            y: self.start.y + (height / 2),
        }
    }

    pub fn shape(&self) -> u64 {
        let width = self.end.x - self.start.x;
        let height = self.end.y - self.start.y;
        (width * height) as u64
    }

    pub fn contains(&self, geo: &GeoContext) -> bool {
        let point: ImaginaryWorldPoint = (*geo.point()).into();

        point.x >= self.start.x
            && point.x <= self.end.x
            && point.y >= self.start.y
            && point.y <= self.end.y
    }

    pub fn step(&self) -> &DisplayStep {
        &self.step
    }
}

impl Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{},{}↘{},{}",
            self.start.x, self.start.y, self.end.x, self.end.y,
        ))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq)]
pub enum DisplayStep {
    Close,
    High,
    Map,
}

impl DisplayStep {
    pub fn from_shape(pixel_count: u64) -> Self {
        match pixel_count {
            0..16_384 => Self::Close,
            16_384..524_288 => Self::High,
            _ => Self::Map,
        }
    }

    pub fn include_cities(&self) -> bool {
        match self {
            DisplayStep::Close => true,
            DisplayStep::High => true,
            DisplayStep::Map => false,
        }
    }

    pub fn include_units(&self) -> bool {
        match self {
            DisplayStep::Close => true,
            DisplayStep::High => true,
            DisplayStep::Map => false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::geo::ImaginaryWorldPoint;

    use super::*;

    #[test]
    fn test_set_window_from_around() {
        let (point, resolution) = (ImaginaryWorldPoint::new(4, 4), Resolution::new(9, 9));
        let set_window = Window::from_around(&point, &resolution);
        assert_eq!(
            set_window,
            Window::new(
                ImaginaryWorldPoint::new(0, 0),
                ImaginaryWorldPoint::new(8, 8)
            )
        );

        let (point, resolution) = (ImaginaryWorldPoint::new(100, 100), Resolution::new(100, 10));
        let set_window = Window::from_around(&point, &resolution);
        assert_eq!(
            set_window,
            Window::new(
                ImaginaryWorldPoint::new(50, 95),
                ImaginaryWorldPoint::new(149, 104)
            )
        );

        let (point, resolution) = (ImaginaryWorldPoint::new(0, 0), Resolution::new(9, 9));
        let set_window = Window::from_around(&point, &resolution);
        assert_eq!(
            set_window,
            Window::new(
                ImaginaryWorldPoint::new(-4, -4),
                ImaginaryWorldPoint::new(4, 4)
            )
        );
    }
}
