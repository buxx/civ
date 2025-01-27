use common::{
    network::{
        message::{ClientStateMessage, ServerToClientInGameMessage, ServerToClientMessage},
        Client,
    },
    space::window::{SetWindow, Window},
};

use crate::{
    effect::{ClientEffect, Effect, StateEffect},
    game::extractor::Extractor,
    runner::{RunnerContext, RunnerError},
};

pub struct SetWindowRequestDealer<'a> {
    context: RunnerContext,
    client: &'a Client,
}

impl<'a> SetWindowRequestDealer<'a> {
    pub fn new(context: RunnerContext, client: &'a Client) -> Self {
        Self { context, client }
    }

    pub fn deal(&self, set_window: &SetWindow) -> Result<Vec<Effect>, RunnerError> {
        let window = Window::from(set_window.clone());

        let new_game_slice = Extractor::new(
            self.context.state(),
            self.context
                .world
                .read()
                .expect("Consider world as always readable"),
        )
        .game_slice(self.client, &window);

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
                .send((*self.client.client_id(), message))
                .unwrap();
        }

        Ok(vec![Effect::State(StateEffect::Client(
            *self.client,
            ClientEffect::SetWindow(window),
        ))])
    }
}
