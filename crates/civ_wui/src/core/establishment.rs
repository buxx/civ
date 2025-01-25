use bevy::prelude::*;

use common::network::message::ServerToClientEstablishmentMessage;

use crate::{network::EstablishmentMessage, state::Server};

pub fn react_establishment(trigger: Trigger<EstablishmentMessage>, mut server: ResMut<Server>) {
    match &trigger.event().0 {
        ServerToClientEstablishmentMessage::ServerResume(resume, flag) => {
            //
            server.set_resume(Some(resume.clone()));
            server.set_flag(Some(flag.clone()));
            info!("{:?}", server);
        }
        ServerToClientEstablishmentMessage::TakePlaceRefused(reason) => todo!(),
    }
}
