use serde::{Deserialize, Serialize};

use crate::rules::RuleSetType;

use super::nation::flag::Flag;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ServerResume {
    /// Rules applied on server
    rules: RuleSetType,
    /// Current flags on server
    flags: Vec<Flag>,
}

impl ServerResume {
    pub fn new(rules: RuleSetType, flags: Vec<Flag>) -> Self {
        Self { rules, flags }
    }
}
