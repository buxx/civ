use common::game::{slice::ClientUnit, unit::UnitId};

#[derive(Debug)]
pub struct UnitMenu {
    pub unit_id: UnitId,
}

impl UnitMenu {
    pub fn from_unit(unit: &ClientUnit) -> Self {
        Self {
            unit_id: *unit.id(),
        }
    }
}
