use common::game::nation::flag::Flag;
use common::game::unit::UnitType;
use common::geo::{GeoContext, GeoVec, WorldPoint};
use common::{geo::ImaginaryWorldPoint, world::TerrainType};
use uuid::Uuid;

use crate::shared::{world::generator::PatternGenerator, Setup};
use civ_server::game::unit::{Unit, UnitCanBuilder};

mod shared;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let world_generator =
        PatternGenerator::new([TerrainType::Plain, TerrainType::GrassLand].to_vec());

    let settler1 = Unit::builder()
        .id(Uuid::new_v4().into())
        .type_(UnitType::Settlers)
        .flag(Flag::Abkhazia)
        .geo(GeoContext::builder().point(WorldPoint::new(2, 2)).build())
        .can(UnitCanBuilder::new().build())
        .build();

    let settler2 = Unit::builder()
        .id(Uuid::new_v4().into())
        .type_(UnitType::Settlers)
        .flag(Flag::Abkhazia)
        .geo(GeoContext::builder().point(WorldPoint::new(5, 5)).build())
        .can(UnitCanBuilder::new().build())
        .build();

    let cities = vec![];
    let units = vec![
        GeoVec::new(settler1.geo, vec![settler1]),
        GeoVec::new(settler2.geo, vec![settler2]),
    ];

    Setup::builder()
        .world_width(1000)
        .world_height(1000)
        .chunk_size(100)
        .world_generator(world_generator)
        .window_start(ImaginaryWorldPoint::new(5, 5))
        .window_end(ImaginaryWorldPoint::new(10, 10))
        .units(units)
        .cities(cities)
        .build()
        .build_app()?
        .run();

    Ok(())
}
