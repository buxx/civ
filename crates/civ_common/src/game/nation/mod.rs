pub mod flag;
use bon::Builder;
use flag::Flag;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NationId(pub Uuid);

#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Nation {
    id: NationId,
    flag: Flag,
}
