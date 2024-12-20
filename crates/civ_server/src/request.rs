use common::{
    network::message::{ClientStateMessage, ServerToClientMessage},
    space::window::{DisplayStep, SetWindow, Window},
};
use uuid::Uuid;

use crate::{
    game::extractor::Extractor,
    runner::RunnerContext,
    task::effect::{ClientEffect, Effect, StateEffect},
};

pub struct SetWindowRequestDealer {
    context: RunnerContext,
    client_id: Uuid,
}

impl SetWindowRequestDealer {
    pub fn new(context: RunnerContext, client_id: Uuid) -> Self {
        Self { context, client_id }
    }

    pub fn deal(&self, set_window: &SetWindow) -> Vec<Effect> {
        let window = Window::new(
            set_window.start_x(),
            set_window.start_y(),
            set_window.end_x(),
            set_window.end_y(),
            DisplayStep::from_shape(set_window.shape()),
        );

        let new_game_slice =
            Extractor::new(&self.context.state()).game_slice(&self.client_id, &window);

        for message in [
            ServerToClientMessage::State(ClientStateMessage::SetWindow(window.clone())),
            ServerToClientMessage::State(ClientStateMessage::SetGameSlice(new_game_slice)),
        ] {
            self.context
                .to_client_sender
                .send((self.client_id, message))
                .unwrap();
        }

        vec![Effect::State(StateEffect::Client(
            self.client_id,
            ClientEffect::SetWindow(window),
        ))]
    }
}
