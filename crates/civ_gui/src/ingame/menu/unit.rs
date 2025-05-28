use common::game::{
    slice::ClientUnit,
    unit::{UnitCan, UnitId},
};

#[derive(Debug)]
pub struct UnitMenu {
    pub unit_id: UnitId,
    pub can: Vec<UnitCan>,
}

impl UnitMenu {
    pub fn from_unit(unit: &ClientUnit) -> Self {
        Self {
            unit_id: *unit.id(),
            can: unit.can().to_vec(),
        }
    }
}
