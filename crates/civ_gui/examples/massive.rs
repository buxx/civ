use std::thread;
use std::time::Duration;

use async_std::channel::unbounded;
use common::game::nation::flag::Flag;
use common::game::unit::{UnitId, UnitType};
use common::geo::{GeoContext, GeoVec, WorldPoint};
use common::network::message::{ClientToServerInGameMessage, ClientToServerUnitMessage};
use common::{geo::ImaginaryWorldPoint, world::TerrainType};
use rand::seq::IndexedRandom;
use uuid::Uuid;

use crate::shared::{world::generator::PatternGenerator, Setup};
use civ_server::game::unit::{Unit, UnitCanBuilder};

mod shared;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    let world_generator =
        PatternGenerator::new([TerrainType::Plain, TerrainType::GrassLand].to_vec());

    let mut units = vec![];
    for x in 0..200 {
        for y in 0..200 {
            let unit_geo = GeoContext::builder().point(WorldPoint::new(x, y)).build();
            let unit = Unit::builder()
                .id(Uuid::new_v4().into())
                .type_(UnitType::Settlers)
                .flag(Flag::Abkhazia)
                .geo(unit_geo)
                .can(UnitCanBuilder::new().build())
                .build();
            units.push(GeoVec::new(unit_geo, vec![unit]));
        }
    }
    let unit_ids: Vec<UnitId> = units
        .iter()
        .map(|u| *u.items().first().unwrap().id())
        .collect::<Vec<UnitId>>()
        .choose_multiple(&mut rng, units.len())
        .cloned()
        .collect();

    let (client_to_server_sender, client_to_server_receiver) = unbounded();
    let client_to_server_sender_ = client_to_server_sender.clone();

    thread::spawn(move || {
        for unit_id in unit_ids {
            let message = ClientToServerInGameMessage::Unit(
                unit_id,
                ClientToServerUnitMessage::Settle(unit_id.to_string()),
            );
            client_to_server_sender_
                .send_blocking(message.into())
                .unwrap();
            thread::sleep(Duration::from_micros(100));
        }
    });

    Setup::builder()
        .world_width(1000)
        .world_height(1000)
        .chunk_size(100)
        .world_generator(world_generator)
        .window_start(ImaginaryWorldPoint::new(5, 5))
        .window_end(ImaginaryWorldPoint::new(10, 10))
        .units(units)
        .cities(vec![])
        .client_to_server((client_to_server_sender, client_to_server_receiver))
        .build()
        .build_app()?
        .run();

    Ok(())
}
