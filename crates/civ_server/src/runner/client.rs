use crate::{
    effect::{self, ClientEffect, ClientsEffect, Effect, StateEffect, UnitEffect},
    game::{
        access::Access,
        task::settle::Settle,
        unit::{Unit, UnitCanBuilder},
    },
    runner::{DealClientRequestError, RunnerContext, RunnerError},
    state::flag::player_flag,
    task::{
        city::generator::{BuildCityFrom, BuildCityFromChange, CityGenerator},
        Concern, TaskId,
    },
};
use common::{
    game::{
        city::CityId,
        nation::flag::Flag,
        unit::{UnitId, UnitType},
    },
    geo::GeoContext,
    network::{
        message::{
            ClientStateMessage, ClientToServerCityMessage, ClientToServerEstablishmentMessage,
            ClientToServerGameMessage, ClientToServerInGameMessage, ClientToServerMessage,
            ClientToServerNetworkMessage, ClientToServerUnitMessage,
            ServerToClientEstablishmentMessage, ServerToClientInGameMessage, ServerToClientMessage,
            TakePlaceRefusedReason,
        },
        Client,
    },
    space::window::{Resolution, Window},
};
use log::debug;

// FIXME: split this module

pub fn deal_client(
    context: &RunnerContext,
    client: &Client,
    message: &ClientToServerMessage,
) -> Result<Vec<Effect>, RunnerError> {
    match message {
        ClientToServerMessage::Network(message) => client_network(context, client, message),
        ClientToServerMessage::Game(message) => client_game(context, client, message),
    }
}

fn client_network(
    context: &RunnerContext,
    _client: &Client,
    message: &ClientToServerNetworkMessage,
) -> Result<Vec<Effect>, RunnerError> {
    match &message {
        ClientToServerNetworkMessage::Hello(client, resolution) => {
            client_hello(context, client, resolution)
        }
        ClientToServerNetworkMessage::Goodbye => Ok(vec![]),
    }
}

fn client_hello(
    context: &RunnerContext,
    client: &Client,
    resolution: &Resolution,
) -> Result<Vec<Effect>, RunnerError> {
    let state = context.state();
    let server_resume = state.server_resume(context.context.rules());
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
                    ClientStateMessage::SetGameFrame(*state.frame()),
                )),
                vec![*client.client_id()],
            ),
        ]);

        let game_slice = context.game_slice(&window);
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
    context: &RunnerContext,
    client: &Client,
    message: &ClientToServerGameMessage,
) -> Result<Vec<Effect>, RunnerError> {
    match message {
        ClientToServerGameMessage::Establishment(message) => {
            client_establishment(context, client, message)
        }
        ClientToServerGameMessage::InGame(message) => client_ingame(context, client, message),
    }
}

fn client_establishment(
    context: &RunnerContext,
    client: &Client,
    message: &ClientToServerEstablishmentMessage,
) -> Result<Vec<Effect>, RunnerError> {
    match message {
        ClientToServerEstablishmentMessage::TakePlace(flag, resolution) => {
            client_take_place(context, client, flag, *resolution)
        }
    }
}

fn client_ingame(
    context: &RunnerContext,
    client: &Client,
    message: &ClientToServerInGameMessage,
) -> Result<Vec<Effect>, RunnerError> {
    let state = context.state();
    let flag = state.client_flag(client)?;
    if !Access::new(context).can(flag, message) {
        return Err(RunnerError::DealClientRequest(
            DealClientRequestError::Unauthorized,
        ));
    };

    match message {
        ClientToServerInGameMessage::SetWindow(window) => {
            let game_slice = context.game_slice(window);

            Ok(vec![
                Effect::State(StateEffect::Client(
                    *client,
                    ClientEffect::SetWindow(*window),
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
            refresh_unit_on(context, unit_id, message)
        }
        ClientToServerInGameMessage::City(city_id, message) => {
            //
            refresh_city_on(context, city_id, message)
        }
    }
}

fn client_take_place(
    context: &RunnerContext,
    client: &Client,
    flag: &Flag,
    resolution: Resolution,
) -> Result<Vec<Effect>, RunnerError> {
    let rules = context.context.rules();
    let world = context.world.read().unwrap();
    let state = context.state();

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

    let point = context.placer.startup(rules, &state, &world).map_err(|e| {
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

    let server_resume = state.server_resume(rules);
    let window = Window::from_around(&point.into(), &resolution);
    let game_slice = context.game_slice(&window);
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
                    ServerToClientEstablishmentMessage::ServerResume(server_resume, Some(*flag)),
                ),
                vec![*client.client_id()],
            ),
        ]),
    ])
}

// TODO: add tests here
pub fn refresh_unit_on(
    context: &RunnerContext,
    unit_id: &UnitId,
    message: &ClientToServerUnitMessage,
) -> Result<Vec<Effect>, RunnerError> {
    debug!("Refresh unit on: {:?}", &message);

    let state = context.state();
    let unit = state.find_unit(unit_id).unwrap(); // TODO: unwrap -> same error management than crate_task
    let old_task = unit.task();

    let new_task = match message {
        ClientToServerUnitMessage::Settle(city_name) => Some(Settle::new(
            TaskId::default(),
            context.context.clone(),
            context.state(),
            unit.clone(),
            city_name.clone(),
        )?),
        ClientToServerUnitMessage::CancelCurrentTask => None,
    };
    let mut unit = unit.clone();
    unit.task = None;

    if let Some(new_task) = &new_task {
        unit.set_task(Some(new_task.clone().into()));
    }

    let mut effects = vec![effect::replace_unit(unit)];

    if let Some(new_task) = new_task {
        effects.push(effect::add_task(Box::new(new_task)));
    }

    if let Some(old_task) = old_task {
        effects.push(effect::remove_task(old_task.clone().into()));
    }

    Ok(effects)
}

pub fn refresh_city_on(
    context: &RunnerContext,
    city_id: &CityId,
    message: &ClientToServerCityMessage,
) -> Result<Vec<Effect>, RunnerError> {
    let state = context.state();
    let city = state.find_city(city_id).unwrap(); // TODO: unwrap -> same error management than crate_task
    let from = match message {
        ClientToServerCityMessage::SetProduction(production) => {
            BuildCityFrom::Change(city, BuildCityFromChange::Production(production.clone()))
        }
        ClientToServerCityMessage::SetExploitation(exploitation) => BuildCityFrom::Change(
            city,
            BuildCityFromChange::Exploitation(exploitation.clone()),
        ),
    };
    let old_tasks = state
        .index()
        .city_tasks(city_id)
        .iter()
        .map(|i| (*i, Concern::City(*city_id)))
        .collect::<Vec<(TaskId, Concern)>>();
    let city = CityGenerator::builder()
        .context(context)
        .game_frame(context.state().frame())
        .from(from)
        .build()
        .generate()
        // TODO: unwrap -> same error management than crate_task
        .unwrap();
    let new_tasks = city.tasks().clone().into();

    Ok(vec![
        effect::replace_city(city),
        effect::remove_tasks(old_tasks),
        effect::add_tasks(new_tasks),
    ])
}
