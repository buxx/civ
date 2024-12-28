use common::{
    network::message::{ClientStateMessage, ServerToClientMessage},
    space::window::{DisplayStep, SetWindow, Window},
    world::reader::WorldReader,
};
use uuid::Uuid;

use crate::{
    game::extractor::Extractor,
    runner::RunnerContext,
    task::effect::{ClientEffect, Effect, StateEffect},
};

pub struct SetWindowRequestDealer<W: WorldReader + Sync + Send> {
    context: RunnerContext<W>,
    client_id: Uuid,
}

impl<W: WorldReader + Sync + Send> SetWindowRequestDealer<W> {
    pub fn new(context: RunnerContext<W>, client_id: Uuid) -> Self {
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

        let new_game_slice = Extractor::new(
            &self.context.state(),
            &self
                .context
                .world
                .read()
                .expect("Consider world as always readable"),
        )
        .game_slice(&self.client_id, &window);

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
