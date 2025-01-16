use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::geo::GeoContext;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Window {
    // TODO: WorldPoint instead x, y ...
    start_x: u64,
    start_y: u64,
    end_x: u64,
    end_y: u64,
    step: DisplayStep,
}

impl Window {
    pub fn new(start_x: u64, start_y: u64, end_x: u64, end_y: u64, step: DisplayStep) -> Self {
        Self {
            start_x,
            start_y,
            end_x,
            end_y,
            step,
        }
    }

    pub fn start_x(&self) -> u64 {
        self.start_x
    }

    pub fn start_y(&self) -> u64 {
        self.start_y
    }

    pub fn end_x(&self) -> u64 {
        self.end_x
    }

    pub fn end_y(&self) -> u64 {
        self.end_y
    }

    pub fn shape(&self) -> u64 {
        // TODO: Check somewhere start is inferior to end ...
        let width = self.end_y - self.start_y;
        let height = self.end_x - self.start_y;
        width * height
    }

    pub fn contains(&self, geo: &GeoContext) -> bool {
        let point = geo.point();

        point.x >= self.start_x
            && point.x <= self.end_x
            && point.y >= self.start_y
            && point.y <= self.end_y
    }

    pub fn step(&self) -> &DisplayStep {
        &self.step
    }
}

impl Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}:{} {}:{}",
            self.start_x, self.start_y, self.end_x, self.end_y,
        ))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetWindow {
    start_x: u64,
    start_y: u64,
    end_x: u64,
    end_y: u64,
}

impl SetWindow {
    pub fn new(start_x: u64, start_y: u64, end_x: u64, end_y: u64) -> Self {
        Self {
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }

    pub fn start_x(&self) -> u64 {
        self.start_x
    }

    pub fn start_y(&self) -> u64 {
        self.start_y
    }

    pub fn end_x(&self) -> u64 {
        self.end_x
    }

    pub fn end_y(&self) -> u64 {
        self.end_y
    }

    pub fn shape(&self) -> u64 {
        if self.end_y > self.start_y || self.end_x > self.start_x {
            return 0;
        }

        let width = self.end_y - self.start_y;
        let height = self.end_x - self.start_x;
        width * height
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
