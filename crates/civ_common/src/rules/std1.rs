use crate::game::{unit::UnitType, GameFrame, GAME_FRAMES_PER_SECOND};

use super::RuleSet;

pub struct Std1RuleSet;
impl RuleSet for Std1RuleSet {
    fn settle_duration(&self, unit_type: &UnitType) -> GameFrame {
        GameFrame(match unit_type {
            UnitType::Settlers => GAME_FRAMES_PER_SECOND * 10,
        })
    }

    fn can_settle(&self, unit_type: &UnitType) -> bool {
        match unit_type {
            UnitType::Settlers => true,
        }
    }
}
