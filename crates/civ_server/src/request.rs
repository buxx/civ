use common::{
    network::message::{ClientStateMessage, ServerToClientInGameMessage, ServerToClientMessage},
    space::window::{DisplayStep, SetWindow, Window},
};
use uuid::Uuid;

use crate::{
    effect::{ClientEffect, Effect, StateEffect},
    game::extractor::Extractor,
    runner::{RunnerContext, RunnerError},
};

pub struct SetWindowRequestDealer {
    context: RunnerContext,
    client_id: Uuid,
}

impl SetWindowRequestDealer {
    pub fn new(context: RunnerContext, client_id: Uuid) -> Self {
        Self { context, client_id }
    }

    pub fn deal(&self, set_window: &SetWindow) -> Result<Vec<Effect>, RunnerError> {
        let window = Window::new(
            set_window.start_x(),
            set_window.start_y(),
            set_window.end_x(),
            set_window.end_y(),
            DisplayStep::from_shape(set_window.shape()),
        );

        let new_game_slice = Extractor::new(
            self.context.state(),
            self.context
                .world
                .read()
                .expect("Consider world as always readable"),
        )
        .game_slice(&self.client_id, &window);

        for message in [
            ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                ClientStateMessage::SetWindow(window.clone()),
            )),
            ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                ClientStateMessage::SetGameSlice(new_game_slice),
            )),
        ] {
            self.context
                .to_client_sender
                .send((self.client_id, message))
                .unwrap();
        }

        Ok(vec![Effect::State(StateEffect::Client(
            self.client_id,
            ClientEffect::SetWindow(window),
        ))])
    }
}
