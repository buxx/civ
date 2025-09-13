use common::geo::WorldPoint;
use glam::{UVec2, Vec2};

pub trait Isometric {
    fn iso(&self, size: UVec2) -> Vec2;
}

impl Isometric for WorldPoint {
    fn iso(&self, size: UVec2) -> Vec2 {
        let sx = (self.x as f32 - self.y as f32) * (size.x as f32 / 2.0);
        let sy = (self.x as f32 + self.y as f32) * (size.y as f32 / 2.0);
        Vec2 { x: sx, y: sy }
    }
}
