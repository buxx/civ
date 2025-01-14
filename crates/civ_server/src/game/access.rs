use common::{
    game::nation::flag::Flag,
    network::message::{
        ClientToServerCityMessage, ClientToServerInGameMessage, ClientToServerUnitMessage,
    },
};
use uuid::Uuid;

use crate::runner::RunnerContext;

pub struct Access<'a> {
    context: &'a RunnerContext,
}

impl<'a> Access<'a> {
    pub fn new(context: &'a RunnerContext) -> Self {
        Self { context }
    }

    pub fn can(&self, flag: &Flag, message: &ClientToServerInGameMessage) -> bool {
        match message {
            ClientToServerInGameMessage::SetWindow(_) => true,
            ClientToServerInGameMessage::Unit(uuid, message) => match message {
                ClientToServerUnitMessage::Settle(_) => self.unit_is_owned_by_client(uuid, flag),
            },
            ClientToServerInGameMessage::City(uuid, message) => match message {
                ClientToServerCityMessage::SetProduction(_)
                | ClientToServerCityMessage::SetExploitation(_) => {
                    self.city_is_owned_by_client(uuid, flag)
                }
            },
        }
    }

    fn unit_is_owned_by_client(&self, uuid: &Uuid, flag: &Flag) -> bool {
        if let Ok(unit) = self.context.state().find_unit(uuid) {
            unit.flag() == flag
        } else {
            false
        }
    }

    fn city_is_owned_by_client(&self, uuid: &Uuid, flag: &Flag) -> bool {
        if let Ok(unit) = self.context.state().find_unit(uuid) {
            unit.flag() == flag
        } else {
            false
        }
    }
}
