use bon::Builder;
use common::{
    game::{
        city::{CityExploitation, CityProduct, CityProduction, CityProductionTons},
        slice::ClientCityTasks,
        GameFrame, PRODUCTION_TON_FRAMES,
    },
    geo::{Geo, GeoContext},
    rules::RuleSetBox,
};
use uuid::Uuid;

use crate::{
    game::{city::City, task::production::CityProductionTask},
    runner::RunnerContext,
};

use super::{TaskBox, TaskContext, TaskError};

#[derive(Builder)]
pub struct CityGenerator<'a> {
    context: &'a RunnerContext,
    game_frame: &'a GameFrame,
    from: BuildCityFrom<'a>,
    previous: Option<&'a City>,
}

pub enum BuildCityFrom<'a> {
    Scratch(String, GeoContext),
    Change(&'a City, BuildCityFromChange),
}

pub enum BuildCityFromChange {
    Production(CityProduction),
    Exploitation(CityExploitation),
}

impl BuildCityFrom<'_> {
    pub fn id(&self) -> Option<&Uuid> {
        match self {
            BuildCityFrom::Scratch(_, _) => None,
            BuildCityFrom::Change(city, _) => Some(city.id()),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            BuildCityFrom::Scratch(city_name, _) => city_name,
            BuildCityFrom::Change(city, _) => city.name(),
        }
    }

    pub fn geo(&self) -> &GeoContext {
        match self {
            BuildCityFrom::Scratch(_, geo) => geo,
            BuildCityFrom::Change(city, _) => city.geo(),
        }
    }

    pub fn production(&self) -> Option<&CityProduction> {
        match self {
            BuildCityFrom::Scratch(_, _) => None,
            BuildCityFrom::Change(city, _) => Some(city.production()),
        }
    }
}

impl CityGenerator<'_> {
    pub fn generate(&self) -> Result<City, TaskError> {
        let default_production = CityProduction::default();
        let city_id = self.from.id().map(|i| *i).unwrap_or(Uuid::new_v4());
        let tasks = CityTasks::new(production_task(
            &self.context.context.rules(),
            &self.game_frame,
            &self.from,
            &city_id,
            default_production.current(),
        ));
        let exploitation = CityExploitation::new();

        Ok(City::builder()
            .id(city_id)
            .name(self.from.name().to_string())
            .geo(*self.from.geo())
            .production(
                self.from
                    .production()
                    .unwrap_or(&CityProduction::default())
                    .clone(),
            )
            .tasks(tasks)
            .exploitation(exploitation)
            .build())
    }
}

fn production_task(
    rules: &RuleSetBox,
    game_frame: &GameFrame,
    from: &BuildCityFrom,
    city_id: &Uuid,
    default_product: &CityProduct,
) -> CityProductionTask {
    let (previous, current): (
        Option<(&CityProductionTons, &CityProduct)>,
        (&CityProductionTons, &CityProduct),
    ) = match from {
        BuildCityFrom::Scratch(_, _) => (
            None,
            // FIXME BS NOW: for "current" tons, need to determine "new" city exploitation
            (&CityProductionTons(1), default_product),
        ),
        BuildCityFrom::Change(city, BuildCityFromChange::Production(production)) => (
            Some((
                city.exploitation().production_tons(),
                city.production().current(),
            )),
            (city.exploitation().production_tons(), production.current()),
        ),
        BuildCityFrom::Change(city, BuildCityFromChange::Exploitation(exploitation)) => (
            Some((
                city.exploitation().production_tons(),
                city.production().current(),
            )),
            (exploitation.production_tons(), city.production().current()),
        ),
    };

    let previously_produced_tons = previous
        .and_then(|(t, p)| Some(rules.required_tons(p) - *t))
        .unwrap_or(CityProductionTons(0));
    let current_product_left = rules.required_tons(current.1) - previously_produced_tons;
    let required_frames = (PRODUCTION_TON_FRAMES as f32
        * (current_product_left.0 as f32 / current.0 .0 as f32)) as u64;

    CityProductionTask::builder()
        .context(
            TaskContext::builder()
                .id(Uuid::new_v4())
                .start(*game_frame)
                .end(*game_frame + required_frames)
                .build(),
        )
        .city(*city_id)
        .tons(*current.0)
        .build()
}

#[derive(Debug, Builder, Clone)]
pub struct CityTasks {
    production: CityProductionTask,
}

impl CityTasks {
    pub fn new(production: CityProductionTask) -> Self {
        Self { production }
    }
}

impl From<CityTasks> for Vec<TaskBox> {
    fn from(value: CityTasks) -> Self {
        vec![Box::new(value.production)]
    }
}

impl From<CityTasks> for ClientCityTasks {
    fn from(value: CityTasks) -> Self {
        ClientCityTasks::new(value.production.into())
    }
}

// impl From<CityTasks> for Vec<TaskWrapper> {
//     fn from(value: CityTasks) -> Self {
//         vec![TaskWrapper::City(CityTaskWrapper::Production(
//             value.production,
//         ))]
//     }
// }

// impl From<CityTasks> for Vec<CityTaskWrapper> {
//     fn from(value: CityTasks) -> Self {
//         vec![CityTaskWrapper::Production(value.production)]
//     }
// }

#[cfg(test)]
mod test {
    use common::{
        game::{city::CityProduct, unit::UnitType, PRODUCTION_TON_FRAMES},
        geo::{GeoContext, WorldPoint},
        rules::RuleSet,
    };

    use crate::task::{Concern, Task};

    use super::*;

    #[derive(Clone)]
    struct TestRuleSet;

    impl RuleSet for TestRuleSet {
        fn tasks(&self) -> Vec<TaskType> {
            unreachable!()
        }

        fn unit_can(&self, _: &UnitType) -> Vec<TaskType> {
            unreachable!()
        }

        fn settle_duration(&self, _: &UnitType) -> GameFrame {
            unreachable!()
        }

        fn can_settle(&self, _: &UnitType) -> bool {
            unreachable!()
        }

        fn required_tons(&self, product: &CityProduct) -> CityProductionTons {
            match product {
                CityProduct::Unit(unit_type) => match unit_type {
                    UnitType::Settlers => CityProductionTons(40),
                    UnitType::Warriors => CityProductionTons(8),
                    _ => unreachable!(),
                },
            }
        }
    }

    #[test]
    fn test_production_task_from_start() {
        // GIVEN
        let game_frame = GameFrame(0);
        let rule_set: RuleSetBox = Box::new(TestRuleSet);
        let city_geo = GeoContext::builder().point(WorldPoint::new(0, 0)).build();
        let city_id = Uuid::new_v4();
        let city = City::builder()
            .geo(city_geo)
            .id(city_id)
            .name("CityName".to_string())
            .production(CityProduction::new(
                vec![CityProduct::Unit(UnitType::Settlers)],
                CityProductionTons(1),
            ))
            .build();
        let expected_end = PRODUCTION_TON_FRAMES * 40;

        // WHEN
        let task: CityProduction = production_task(&game_frame, &city, &vec![], &rule_set);

        // THEN
        assert_eq!(task.concern(), Concern::City(city_id));
        assert_eq!(task.context().start(), GameFrame(0));
        assert_eq!(task.context().end(), GameFrame(expected_end));
    }

    #[test]
    fn test_production_task_switch() {
        // GIVEN
        let game_frame = GameFrame(24_000);
        let rule_set: RuleSetBox = Box::new(TestRuleSet);
        let city_geo = GeoContext::builder().point(WorldPoint::new(0, 0)).build();
        let city_id = Uuid::new_v4();
        let city = City::builder()
            .geo(city_geo)
            .id(city_id)
            .name("CityName".to_string())
            .production(CityProduction::new(
                vec![CityProduct::Unit(UnitType::Warriors)],
                CityProductionTons(1),
            ))
            .build();
        let previous_tasks: TaskBox = Box::new(
            CityProduction::builder()
                .context(
                    TaskContext::builder()
                        .id(Uuid::new_v4())
                        .start(GameFrame(0))
                        .end(GameFrame(240_000))
                        .build(),
                )
                .city(city.id())
                .tons(CityProductionTons(1))
                .build(),
        );
        // 4 tons because previous task already done 44 (10% (24_000 of 240_000) of 40)
        let expected_end = 24_000 + PRODUCTION_TON_FRAMES * 4;

        // WHEN
        let task: CityProduction =
            production_task(&game_frame, &city, &vec![&previous_tasks], &rule_set);

        // THEN
        assert_eq!(task.concern(), Concern::City(city_id));
        assert_eq!(task.context().start(), GameFrame(24_000));
        assert_eq!(task.context().end(), GameFrame(expected_end));
    }

    #[test]
    fn test_production_task_change_tons() {
        // GIVEN
        let was_producing_tons = CityProductionTons(1);
        let now_producing_tons = CityProductionTons(2);
        let now_producing = UnitType::Settlers;

        let game_frame = GameFrame(120_000);
        let rule_set: RuleSetBox = Box::new(TestRuleSet);
        let city_geo = GeoContext::builder().point(WorldPoint::new(0, 0)).build();
        let city_id = Uuid::new_v4();
        // FIXME BS NOW: change city prod to 2
        let city = City::builder()
            .geo(city_geo)
            .id(city_id)
            .name("CityName".to_string())
            .production(CityProduction::new(
                vec![CityProduct::Unit(now_producing)],
                now_producing_tons,
            ))
            .build();
        let previous_tasks: TaskBox = Box::new(
            CityProduction::builder()
                .context(
                    TaskContext::builder()
                        .id(Uuid::new_v4())
                        .start(GameFrame(0))
                        .end(GameFrame(240_000))
                        .build(),
                )
                .city(city.id())
                .tons(was_producing_tons)
                .build(),
        );
        // 20 tons / 2 because 120_000 is half of 240_000 (total required frames), 20 is half of 40, and tons is now 2.
        let expected_end = 120_000 + PRODUCTION_TON_FRAMES * (20 / 2);

        // WHEN
        let task: CityProduction =
            production_task(&game_frame, &city, &vec![&previous_tasks], &rule_set);

        // THEN
        assert_eq!(task.concern(), Concern::City(city_id));
        assert_eq!(task.context().start(), GameFrame(120_000));
        assert_eq!(task.context().end(), GameFrame(expected_end));
    }

    // FIXME: test with change to unit (will produce excess)
}
