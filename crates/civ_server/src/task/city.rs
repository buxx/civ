use std::sync::RwLockReadGuard;

use bon::Builder;
use common::{
    game::{
        city::CityProductionTons,
        unit::{CityTaskType, TaskType},
        GameFrame, FRAME_PRODUCTION_TONS_RATIO, PRODUCTION_TON_FRAMES,
    },
    rules::RuleSetBox,
};
use uuid::Uuid;

use crate::{
    game::{city::City, task::production::CityProductionTask},
    runner::RunnerContext,
};

use super::{context::TaskContext, TaskBox, TaskError};

/// Produce all tasks for given city. Used when city context change to refill city tasks
#[derive(Builder)]
pub struct CityTasksBuilder<'a> {
    context: &'a RunnerContext,
    city: &'a City,
    previous_tasks: &'a Vec<&'a TaskBox>,
    game_frame: GameFrame,
}

impl CityTasksBuilder<'_> {
    pub fn build(&self) -> Result<Vec<TaskBox>, TaskError> {
        let production_task = self.production_task();

        Ok(vec![Box::new(production_task)])
    }

    fn production_task(&self) -> CityProductionTask {
        production_task(
            &self.game_frame,
            self.city,
            self.previous_tasks,
            self.context.context.rules(),
        )
    }
}

fn production_task(
    game_frame: &GameFrame,
    city: &City,
    previous_tasks: &Vec<&TaskBox>,
    rules: &RuleSetBox,
) -> CityProductionTask {
    let previous_tons = previous_tons(game_frame, previous_tasks);
    let current_product = city.production().current();
    let current_product_left =
        CityProductionTons(rules.required_tons(current_product).0 - previous_tons.0);
    let current_tons = city.production().tons();
    let required_frames = (PRODUCTION_TON_FRAMES as f32
        * (current_product_left.0 as f32 / current_tons.0 as f32)) as u64;

    let task_id = Uuid::new_v4();
    CityProductionTask::builder()
        .context(
            TaskContext::builder()
                .id(task_id)
                .start(*game_frame)
                .end(*game_frame + required_frames)
                .build(),
        )
        .city(city.id())
        .tons(*current_tons)
        .build()
}

fn previous_tons(game_frame: &GameFrame, previous_tasks: &Vec<&TaskBox>) -> CityProductionTons {
    match previous_tasks
        .iter()
        .find(|t| t.type_().is_city_production())
        .map(|t| (t.type_(), t.context()))
    {
        Some((TaskType::City(CityTaskType::Production(tons)), context)) => {
            let elapsed_frames = game_frame.0 - context.start().0;
            CityProductionTons(
                ((elapsed_frames as f64 * tons.0 as f64) * FRAME_PRODUCTION_TONS_RATIO) as u64,
            )
        }
        _ => CityProductionTons(0),
    }
}

#[cfg(test)]
mod test {
    use common::{
        game::{city::CityProduct, unit::UnitType, PRODUCTION_TON_FRAMES},
        geo::{GeoContext, WorldPoint},
        rules::RuleSet,
    };

    use crate::{
        game::city::CityProduction,
        task::{Concern, Task},
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
        let task: CityProductionTask = production_task(&game_frame, &city, &vec![], &rule_set);

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
            CityProductionTask::builder()
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
        let task: CityProductionTask =
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
            CityProductionTask::builder()
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
        let task: CityProductionTask =
            production_task(&game_frame, &city, &vec![&previous_tasks], &rule_set);

        // THEN
        assert_eq!(task.concern(), Concern::City(city_id));
        assert_eq!(task.context().start(), GameFrame(120_000));
        assert_eq!(task.context().end(), GameFrame(expected_end));
    }

    // FIXME: test with change to unit (will produce excess)
}
