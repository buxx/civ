use crate::{
    effect::{Action, ClientEffect, ClientsEffect, Effect, StateEffect, UnitEffect},
    game::{
        access::Access,
        extractor::Extractor,
        unit::{Unit, UnitCanBuilder},
    },
    runner::{DealClientRequestError, Runner, RunnerError},
};
use common::{
    game::{
        city::{CityId, CityProduct, CityProduction},
        nation::flag::Flag,
        unit::{UnitId, UnitType},
        GameFrame, GAME_FRAMES_PER_SECOND,
    },
    geo::GeoContext,
    network::{
        message::{
            ClientStateMessage, ClientToServerCityMessage, ClientToServerEstablishmentMessage,
            ClientToServerGameMessage, ClientToServerInGameMessage, ClientToServerMessage,
            ClientToServerNetworkMessage, ClientToServerUnitMessage, NotificationLevel,
            ServerToClientEstablishmentMessage, ServerToClientInGameMessage, ServerToClientMessage,
            TakePlaceRefusedReason,
        },
        Client, ClientId,
    },
    space::window::{Resolution, SetWindow, Window},
    task::{CreateTaskError, GamePlayReason},
    world::reader::WorldReader,
};

impl Runner {
    pub fn client(
        &self,
        client: &Client,
        message: ClientToServerMessage,
    ) -> Result<Vec<Effect>, RunnerError> {
        let state = self.state();
        match message {
            ClientToServerMessage::Network(message) => match &message {
                ClientToServerNetworkMessage::Hello(client, resolution) => {
                    let state = self.state();
                    let server_resume = state.server_resume(self.context.context.rules());
                    let player_flag = state
                        .clients()
                        .player_state(client.player_id())
                        .map(|s| s.flag())
                        .cloned();
                    let mut shines = vec![(
                        ServerToClientMessage::Establishment(
                            ServerToClientEstablishmentMessage::ServerResume(
                                server_resume,
                                player_flag,
                            ),
                        ),
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
                        let game_slice = Extractor::new(
                            self.context.state(),
                            self.context
                                .world
                                .read()
                                .expect("Consider world as always readable"),
                        )
                        .game_slice(client, &window);

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
                ClientToServerNetworkMessage::Goodbye => Ok(vec![]),
            },
            ClientToServerMessage::Game(message) => {
                match message {
                    ClientToServerGameMessage::Establishment(message) => match message {
                        ClientToServerEstablishmentMessage::TakePlace(flag, resolution) => {
                            self.player_take_place(client, &flag, resolution)
                        }
                    },
                    ClientToServerGameMessage::InGame(message) => {
                        let flag = state.client_flag(client)?;
                        if !Access::new(&self.context).can(flag, &message) {
                            return Err(RunnerError::DealClientRequest(
                                DealClientRequestError::Unauthorized,
                            ));
                        };

                        match message {
                            ClientToServerInGameMessage::SetWindow(window) => {
                                Ok(vec![Effect::Action(Action::UpdateClientWindow(
                                    *client, window,
                                ))])
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
                }
            }
        }
    }

    pub fn player_take_place(
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
        // FIXME BS NOW: on a vraiment besoin de ce SetWindow là ?
        let client_window = SetWindow::from_around(&point.into(), &resolution);
        let window = Window::from(client_window.clone());
        Ok(vec![
            Effect::State(StateEffect::Unit(settler_id, UnitEffect::New(settler))),
            Effect::State(StateEffect::Client(
                *client,
                ClientEffect::PlayerTookPlace(*flag, window),
            )),
            Effect::Action(Action::UpdateClientWindow(*client, client_window.clone())),
            Effect::Shines(vec![
                (
                    ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                        ClientStateMessage::SetWindow(client_window.into()),
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
