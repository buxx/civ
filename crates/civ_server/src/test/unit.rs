use crate::game::unit::Unit;
use common::{
    game::{
        nation::flag::Flag,
        unit::{UnitId, UnitType},
    },
    geo::{GeoContext, WorldPoint},
};

pub fn build_unit(i: usize) -> Unit {
    Unit::builder()
        .id(UnitId::default())
        .geo(
            GeoContext::builder()
                .point(WorldPoint::new(i as u64, i as u64))
                .build(),
        )
        .type_(UnitType::Warriors)
        .flag(Flag::Abkhazia)
        .can(vec![])
        .build()
}
