use bevy::prelude::*;

use common::network::message::ServerToClientEstablishmentMessage;

// use crate::state::ServerResource;

// pub fn react_establishment(
//     trigger: Trigger<EstablishmentMessage>,
//     mut server: ResMut<ServerResource>,
// ) {
//     match &trigger.event().0 {
//         ServerToClientEstablishmentMessage::ServerResume(resume, flag) => {
//             server.resume = Some(resume.clone());
//             server.flag = *flag;
//             info!("{:?}", server);
//         }
//         ServerToClientEstablishmentMessage::TakePlaceRefused(_reason) => {}
//     }
// }
