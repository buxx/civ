use crate::{
    game::{city::City, task::production::CityProductionTask},
    task::{city::CityTasks, TaskContext, TaskId},
};
use common::{
    game::{
        city::{CityExploitation, CityId, CityProduction, CityProductionTons},
        nation::flag::Flag,
        GameFrame,
    },
    geo::{GeoContext, WorldPoint},
};

pub fn build_city(i: usize) -> City {
    let city_uuid = CityId::default();
    City::builder()
        .id(city_uuid)
        .name("CityName".to_string())
        .geo(
            GeoContext::builder()
                .point(WorldPoint::new(i as u64, i as u64))
                .build(),
        )
        .production(CityProduction::new(vec![]))
        .exploitation(CityExploitation::new(CityProductionTons(1)))
        .tasks(
            CityTasks::builder()
                .production(
                    CityProductionTask::builder()
                        .city(city_uuid)
                        .context(
                            TaskContext::builder()
                                .id(TaskId::default())
                                .start(GameFrame(0))
                                .end(GameFrame(1))
                                .build(),
                        )
                        .tons(CityProductionTons(1))
                        .build(),
                )
                .build(),
        )
        .flag(Flag::Abkhazia)
        .build()
}
