use crate::{
    effect::{ClientEffect, ClientsEffect, Effect, StateEffect, UnitEffect},
    game::{
        access::Access,
        extractor::game_slice,
        unit::{Unit, UnitCanBuilder},
    },
    runner::{DealClientRequestError, Runner, RunnerError},
    state::flag::player_flag,
};
use common::{
    game::{
        nation::flag::Flag,
        unit::{UnitId, UnitType},
    },
    geo::GeoContext,
    network::{
        message::{
            ClientStateMessage, ClientToServerEstablishmentMessage, ClientToServerGameMessage,
            ClientToServerInGameMessage, ClientToServerMessage, ClientToServerNetworkMessage,
            ServerToClientEstablishmentMessage, ServerToClientInGameMessage, ServerToClientMessage,
            TakePlaceRefusedReason,
        },
        Client,
    },
    space::window::{Resolution, Window},
};
use log::debug;

impl Runner {
    pub fn client(
        &self,
        client: &Client,
        message: ClientToServerMessage,
    ) -> Result<Vec<Effect>, RunnerError> {
        match message {
            ClientToServerMessage::Network(message) => self.client_network(client, message),
            ClientToServerMessage::Game(message) => self.client_game(client, message),
        }
    }

    fn client_network(
        &self,
        _client: &Client,
        message: ClientToServerNetworkMessage,
    ) -> Result<Vec<Effect>, RunnerError> {
        match &message {
            ClientToServerNetworkMessage::Hello(client, resolution) => {
                self.client_hello(client, resolution)
            }
            ClientToServerNetworkMessage::Goodbye => Ok(vec![]),
        }
    }

    fn client_hello(
        &self,
        client: &Client,
        resolution: &Resolution,
    ) -> Result<Vec<Effect>, RunnerError> {
        let state = self.state();
        let server_resume = state.server_resume(self.context.context.rules());
        let player_flag = state.player_flag(client.player_id());
        let mut shines = vec![(
            ServerToClientMessage::Establishment(ServerToClientEstablishmentMessage::ServerResume(
                server_resume,
                player_flag,
            )),
            vec![*client.client_id()],
        )];
        if let Some(window) = state
            .clients()
            .states()
            .get(client.player_id())
            .map(|state| Window::from_around(&state.window().center(), resolution))
        {
            shines.extend(vec![
                (
                    ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                        ClientStateMessage::SetWindow(window),
                    )),
                    vec![*client.client_id()],
                ),
                (
                    ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                        ClientStateMessage::SetGameFrame(*self.state().frame()),
                    )),
                    vec![*client.client_id()],
                ),
            ]);
            // FIXME BS NOW: c'est le bazar entre take place et hello !
            let game_slice = self.slice(&window);

            shines.push((
                ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                    ClientStateMessage::SetGameSlice(game_slice),
                )),
                vec![*client.client_id()],
            ));
        }

        Ok(vec![
            Effect::State(StateEffect::Clients(ClientsEffect::Insert(
                *client.client_id(),
                *client.player_id(),
            ))),
            Effect::Shines(shines),
        ])
    }

    fn client_game(
        &self,
        client: &Client,
        message: ClientToServerGameMessage,
    ) -> Result<Vec<Effect>, RunnerError> {
        match message {
            ClientToServerGameMessage::Establishment(message) => {
                self.client_establishment(client, message)
            }
            ClientToServerGameMessage::InGame(message) => self.client_ingame(client, message),
        }
    }

    fn client_establishment(
        &self,
        client: &Client,
        message: ClientToServerEstablishmentMessage,
    ) -> Result<Vec<Effect>, RunnerError> {
        match message {
            ClientToServerEstablishmentMessage::TakePlace(flag, resolution) => {
                self.client_take_place(client, &flag, resolution)
            }
        }
    }

    fn client_ingame(
        &self,
        client: &Client,
        message: ClientToServerInGameMessage,
    ) -> Result<Vec<Effect>, RunnerError> {
        let state = self.state();
        let flag = state.client_flag(client)?;
        if !Access::new(&self.context).can(flag, &message) {
            return Err(RunnerError::DealClientRequest(
                DealClientRequestError::Unauthorized,
            ));
        };

        match message {
            ClientToServerInGameMessage::SetWindow(window) => {
                let game_slice = self.slice(&window);

                Ok(vec![
                    Effect::State(StateEffect::Client(
                        *client,
                        ClientEffect::SetWindow(window),
                    )),
                    Effect::Shines(vec![(
                        ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                            ClientStateMessage::SetGameSlice(game_slice),
                        )),
                        vec![*client.client_id()],
                    )]),
                ])
            }
            ClientToServerInGameMessage::Unit(unit_id, message) => {
                //
                self.refresh_unit_on(&unit_id, message)
            }
            ClientToServerInGameMessage::City(city_id, message) => {
                //
                self.refresh_city_on(&city_id, message)
            }
        }
    }

    fn client_take_place(
        &self,
        client: &Client,
        flag: &Flag,
        resolution: Resolution,
    ) -> Result<Vec<Effect>, RunnerError> {
        let rules = self.context.context.rules();
        let world = self.world();
        let state = self.state();

        if state
            .clients()
            .states()
            .values()
            .map(|s| s.flag())
            .any(|s| s == flag)
        {
            debug!("Client {}: establishment refused", client.client_id());
            return Ok(vec![Effect::Shines(vec![(
                ServerToClientMessage::Establishment(
                    ServerToClientEstablishmentMessage::TakePlaceRefused(
                        TakePlaceRefusedReason::FlagAlreadyTaken(*flag),
                    ),
                ),
                vec![*client.client_id()],
            )])]);
        }

        let point = self.placer.startup(rules, &state, &world).map_err(|e| {
            RunnerError::DealClientRequest(DealClientRequestError::Unfeasible(e.to_string()))
        })?;

        // TODO: move code of unit generation and make it depend on ruleset
        let settler_id = UnitId::default();
        let settler = Unit::builder()
            .id(settler_id)
            .type_(UnitType::Settlers)
            .geo(GeoContext::builder().point(point).build())
            .flag(*flag)
            .can(UnitCanBuilder::new().build())
            .build();

        let server_resume = self.state().server_resume(rules);
        let window = Window::from_around(&point.into(), &resolution);
        let game_slice = self.slice(&window);
        Ok(vec![
            Effect::State(StateEffect::Unit(settler_id, UnitEffect::New(settler))),
            Effect::State(StateEffect::Client(
                *client,
                ClientEffect::PlayerTookPlace(*flag, window),
            )),
            Effect::State(StateEffect::Client(
                *client,
                ClientEffect::SetWindow(window),
            )),
            Effect::Shines(vec![
                // Need to send window to client as he took place and is not the origin of this window
                (
                    ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                        ClientStateMessage::SetGameSlice(game_slice),
                    )),
                    vec![*client.client_id()],
                ),
                (
                    ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                        ClientStateMessage::SetWindow(window),
                    )),
                    vec![*client.client_id()],
                ),
                (
                    ServerToClientMessage::Establishment(
                        ServerToClientEstablishmentMessage::ServerResume(
                            server_resume,
                            Some(*flag),
                        ),
                    ),
                    vec![*client.client_id()],
                ),
            ]),
        ])
    }
}
