use common::geo::WorldPoint;
use hexx::Hex;

pub trait DebugDisplay {
    fn display(&self) -> String;
}

impl DebugDisplay for (Hex, Option<WorldPoint>) {
    fn display(&self) -> String {
        let (hex, world_point) = self;
        format!(
            "{},{} ({},{})",
            hex.x,
            hex.y,
            world_point
                .map(|v| v.x.to_string())
                .unwrap_or("?".to_string()),
            world_point
                .map(|v| v.y.to_string())
                .unwrap_or("?".to_string())
        )
    }
}
