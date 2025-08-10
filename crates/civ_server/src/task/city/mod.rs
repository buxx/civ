use bon::Builder;
use common::game::slice::ClientCityTasks;
use serde::{Deserialize, Serialize};

use crate::game::task::production::CityProductionTask;

use super::{TaskBox, TaskContext, TaskError, TaskId};

pub mod generator;
pub mod production;

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
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

#[cfg(test)]
mod test {
    use common::{
        game::{
            city::CityProduct,
            unit::{TaskType, UnitType},
            PRODUCTION_FRAMES_PER_TONS,
        },
        geo::{GeoContext, WorldPoint},
        rules::{RuleSet, RuleSetType},
    };
    use common::{
        game::{
            city::{CityExploitation, CityId, CityProduction, CityProductionTons},
            nation::flag::Flag,
            GameFrame,
        },
        rules::RuleSetBox,
    };

    use crate::{
        game::{city::City, task::production::CityProductionTask},
        task::WithContext,
    };

    use crate::task::{
        city::{
            generator::{BuildCityFrom, BuildCityFromChange},
            production::production_task,
        },
        Concern, Task,
    };

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
                },
            }
        }

        fn type_(&self) -> RuleSetType {
            RuleSetType::Testing
        }

        fn can_be_startup(&self, _tile: &common::world::Tile) -> bool {
            true
        }
    }

    #[test]
    fn test_production_task_from_start() {
        // GIVEN
        let game_frame = GameFrame(0);
        let rule_set: RuleSetBox = Box::new(TestRuleSet);
        let city_geo = GeoContext::builder().point(WorldPoint::new(0, 0)).build();
        let city_id = CityId::default();
        let expected_end = PRODUCTION_FRAMES_PER_TONS * 40;

        // WHEN
        let task = production_task(
            &rule_set,
            &game_frame,
            &BuildCityFrom::Scratch("CityName".to_string(), Flag::Abkhazia, city_geo),
            &city_id,
            &CityProduct::Unit(UnitType::Settlers),
        );

        // THEN
        assert_eq!(task.concern(), Concern::City(city_id));
        assert_eq!(task.context().start(), GameFrame(0));
        assert_eq!(task.context().end(), GameFrame(expected_end));
    }

    #[test]
    fn test_production_task_change_product() {
        // GIVEN
        let game_frame = GameFrame(24_000);
        let rule_set: RuleSetBox = Box::new(TestRuleSet);
        let city_geo = GeoContext::builder().point(WorldPoint::new(0, 0)).build();
        let city_id = CityId::default();
        let city = City::builder()
            .geo(city_geo)
            .id(city_id)
            .flag(Flag::Abkhazia)
            .name("CityName".to_string())
            .production(CityProduction::new(vec![CityProduct::Unit(
                UnitType::Settlers,
            )]))
            .tasks(CityTasks::new(
                CityProductionTask::builder()
                    .context(
                        TaskContext::builder()
                            .id(TaskId::default())
                            .start(GameFrame(0))
                            .end(GameFrame(240_000))
                            .build(),
                    )
                    .city(city_id)
                    .tons(CityProductionTons(1))
                    .build(),
            ))
            .exploitation(CityExploitation::new(CityProductionTons(1)))
            .build();

        let new_city_production = CityProduction::new(vec![CityProduct::Unit(UnitType::Warriors)]);
        // 4 tons because previous task already done 44 (10% (24_000 of 240_000) of 40)
        let expected_end = 24_000 + PRODUCTION_FRAMES_PER_TONS * 4;

        // WHEN
        let task = production_task(
            &rule_set,
            &game_frame,
            &BuildCityFrom::Change(&city, BuildCityFromChange::Production(new_city_production)),
            &city_id,
            &CityProduct::Unit(UnitType::Warriors),
        );

        // THEN
        assert_eq!(task.concern(), Concern::City(city_id));
        assert_eq!(task.context().start(), GameFrame(24_000));
        assert_eq!(task.context().end(), GameFrame(expected_end));
    }

    #[test]
    fn test_production_task_change_exploitation() {
        // GIVEN
        let was_producing_tons = CityProductionTons(1);
        let now_producing_tons = CityProductionTons(2);
        let was_producing = UnitType::Settlers;
        let now_producing = UnitType::Settlers;

        let game_frame = GameFrame(120_000);
        let rule_set: RuleSetBox = Box::new(TestRuleSet);
        let city_geo = GeoContext::builder().point(WorldPoint::new(0, 0)).build();
        let city_id = CityId::default();
        let city = City::builder()
            .geo(city_geo)
            .id(city_id)
            .name("CityName".to_string())
            .flag(Flag::Abkhazia)
            .production(CityProduction::new(vec![CityProduct::Unit(was_producing)]))
            .tasks(CityTasks::new(
                CityProductionTask::builder()
                    .context(
                        TaskContext::builder()
                            .id(TaskId::default())
                            .start(GameFrame(0))
                            .end(GameFrame(240_000))
                            .build(),
                    )
                    .city(city_id)
                    .tons(was_producing_tons)
                    .build(),
            ))
            .exploitation(CityExploitation::new(was_producing_tons))
            .build();
        // 20 tons / 2 because 120_000 is half of 240_000 (total required frames), 20 is half of 40, and tons is now 2.
        let expected_end = 120_000 + PRODUCTION_FRAMES_PER_TONS * (20 / 2);

        // WHEN
        let task = production_task(
            &rule_set,
            &game_frame,
            &BuildCityFrom::Change(
                &city,
                BuildCityFromChange::Exploitation(CityExploitation::new(now_producing_tons)),
            ),
            &city_id,
            &CityProduct::Unit(now_producing),
        );

        // THEN
        assert_eq!(task.concern(), Concern::City(city_id));
        assert_eq!(task.context().start(), GameFrame(120_000));
        assert_eq!(task.context().end(), GameFrame(expected_end));
    }

    // TODO: test with change to unit (will produce excess)
}
