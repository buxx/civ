use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Window {
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

    pub fn contains(&self, point: &(u64, u64)) -> bool {
        point.0 >= self.start_x
            && point.0 <= self.end_x
            && point.1 >= self.start_y
            && point.1 <= self.end_y
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
        // TODO: Check somewhere start is inferior to end ...
        let width = self.end_y - self.start_y;
        let height = self.end_x - self.start_y;
        width * height
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash)]
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
