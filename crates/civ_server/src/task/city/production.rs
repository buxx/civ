use common::{
    game::{
        city::{CityId, CityProduct, CityProductionTons},
        GameFrame, FRAME_PRODUCTION_TONS_RATIO, PRODUCTION_FRAMES_PER_TONS,
    },
    rules::RuleSetBox,
};

use crate::{
    game::task::production::CityProductionTask,
    task::{
        city::generator::{BuildCityFrom, BuildCityFromChange},
        WithContext,
    },
};

use super::{TaskContext, TaskId};

type FromProduction<'a> = (
    Option<(GameFrame, &'a CityProductionTons, &'a CityProduct)>,
    (&'a CityProductionTons, &'a CityProduct),
);

pub fn production_task(
    rules: &RuleSetBox,
    game_frame: &GameFrame,
    from: &BuildCityFrom,
    city_id: &CityId,
    default_product: &CityProduct,
) -> CityProductionTask {
    let (previous, current): FromProduction = match from {
        BuildCityFrom::Scratch(_, _, _) => (
            None,
            // TODO: for "current" tons, need to determine with "new" city exploitation
            (&CityProductionTons(1), default_product),
        ),
        BuildCityFrom::Change(city, BuildCityFromChange::Production(production)) => (
            Some((
                *game_frame - city.tasks().production.context().start(),
                city.exploitation().production_tons(),
                city.production().current(),
            )),
            (city.exploitation().production_tons(), production.current()),
        ),
        BuildCityFrom::Change(city, BuildCityFromChange::Exploitation(exploitation)) => (
            Some((
                *game_frame - city.tasks().production.context().start(),
                city.exploitation().production_tons(),
                city.production().current(),
            )),
            (exploitation.production_tons(), city.production().current()),
        ),
    };

    let previously_produced_tons = previous
        .map(|(f, t, _)| {
            CityProductionTons((t.0 as f64 * f.0 as f64 * FRAME_PRODUCTION_TONS_RATIO) as u64)
        })
        .unwrap_or(CityProductionTons(0));
    let current_product_left = rules.required_tons(current.1) - previously_produced_tons;
    let required_frames = (PRODUCTION_FRAMES_PER_TONS as f32
        * (current_product_left.0 as f32 / current.0 .0 as f32)) as u64;

    CityProductionTask::builder()
        .context(
            TaskContext::builder()
                .id(TaskId::default())
                .start(*game_frame)
                .end(*game_frame + required_frames)
                .build(),
        )
        .city(*city_id)
        .tons(*current.0)
        .build()
}
