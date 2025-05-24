use common::game::unit::UnitId;

#[derive(Debug)]
pub struct UnitMenu {
    unit_id: UnitId,
}

impl UnitMenu {
    pub fn new(unit_id: UnitId) -> Self {
        Self { unit_id }
    }
}
