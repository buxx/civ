use bevy::prelude::*;
use common::network::message::ClientStateMessage;

use common::space::{CityVec2dIndex, UnitVec2dIndex};

use crate::core::{CityRemoved, CityUpdated, UnitRemoved, UnitUpdated};
use crate::ingame::{GameFrameResource, GameFrameUpdated, GameSliceResource, GameWindowResource};

use super::{GameSliceUpdated, GameWindowUpdated};

type TriggerFn = Box<dyn FnOnce(&mut Commands) + Send + Sync + 'static>;

pub fn react_state_message(
    message: &ClientStateMessage,
    slice: &mut GameSliceResource,
    frame: &mut GameFrameResource,
    window: &mut GameWindowResource,
    commands: &mut Commands,
) {
    if let Some(trigger) = react_state_message_(message, slice, frame, window) {
        trigger(commands)
    }
}

pub fn react_state_message_(
    message: &ClientStateMessage,
    slice: &mut GameSliceResource,
    frame: &mut GameFrameResource,
    window: &mut GameWindowResource,
) -> Option<TriggerFn> {
    match message {
        ClientStateMessage::SetGameFrame(frame_) => {
            let frame__ = *frame_;
            frame.0 = Some(frame__);
            Some(Box::new(move |c| c.trigger(GameFrameUpdated(frame__))))
        }
        ClientStateMessage::SetGameSlice(game_slice_) => {
            slice.0 = Some(game_slice_.clone());
            Some(Box::new(|c| c.trigger(GameSliceUpdated)))
        }
        ClientStateMessage::SetWindow(window_) => {
            window.0 = Some(*window_);
            Some(Box::new(|c| c.trigger(GameWindowUpdated)))
        }
        ClientStateMessage::SetCity(city) => {
            if let Some(ref mut slice) = &mut (slice.0) {
                if let Some(index) = slice
                    .cities_mut()
                    .set(city.geo().point(), Some(city.clone()))
                {
                    slice
                        .cities_map_mut()
                        .insert(*city.id(), CityVec2dIndex(index));
                }
            }

            let city = city.clone();
            Some(Box::new(move |c| c.trigger(CityUpdated(city))))
        }
        ClientStateMessage::RemoveCity(point, city_id) => {
            if let Some(ref mut slice) = &mut (slice.0) {
                slice.cities_mut().set(point, None);
                slice.cities_map_mut().remove(city_id);
            }

            let city_id = *city_id;
            let point = *point;
            Some(Box::new(move |c| c.trigger(CityRemoved(city_id, point))))
        }
        ClientStateMessage::SetUnit(unit) => {
            if let Some(ref mut slice) = &mut (slice.0) {
                let mut new_index: Option<UnitVec2dIndex> = None;
                // FIXME BS NOW: this geo is possibly the new one if moved ! Add "previous_point" to SetUnit ?
                if let Some((index1, units)) = slice.units_mut().get_mut(unit.geo().point()) {
                    if let Some(units) = units {
                        if let Some(index2) = units.iter().position(|u| u.id() == unit.id()) {
                            units[index2] = unit.clone();
                            new_index = Some(UnitVec2dIndex(index1, index2));
                        // Its a new unit
                        } else {
                            units.push(unit.clone());
                            new_index = Some(UnitVec2dIndex(index1, 0));
                        }
                    // There is no vector of unit yet here
                    } else {
                        *units = Some(vec![unit.clone()]);
                        new_index = Some(UnitVec2dIndex(index1, 0));
                    }
                    // If None, its outside of the slice
                }

                if let Some(new_index) = new_index {
                    slice.units_map_mut().insert(*unit.id(), new_index);
                }
            }

            let unit = unit.clone();
            Some(Box::new(move |c| c.trigger(UnitUpdated(unit))))
        }
        ClientStateMessage::RemoveUnit(point, unit_id) => {
            // FIXME BS NOW: must update units_map
            if let Some(ref mut slice) = &mut (slice.0) {
                let mut is_empty = false;
                if let Some((_, Some(units))) = slice.units_mut().get_mut(point) {
                    units.retain(|u| u.id() != unit_id);
                    is_empty = units.is_empty();

                    slice.units_map_mut().remove(unit_id);
                }

                if is_empty {
                    slice.units_mut().set(point, None);
                }
            }

            let unit_id = *unit_id;
            let point = *point;
            Some(Box::new(move |c| c.trigger(UnitRemoved(unit_id, point))))
        }
    }
}

#[cfg(test)]
mod test {
    use common::{
        game::{
            city::{CityExploitation, CityId, CityProduction, CityProductionTons},
            nation::flag::Flag,
            slice::{ClientCity, ClientCityTasks, ClientUnit, GameSlice},
            tasks::client::{
                city::production::ClientCityProductionTask, ClientTask, ClientTaskType,
            },
            unit::{UnitId, UnitType},
            GameFrame,
        },
        geo::{GeoContext, WorldPoint},
        space::D2Size,
    };

    use super::*;

    fn build_city(point: WorldPoint) -> ClientCity {
        ClientCity::builder()
            .id(CityId::default())
            .flag(Flag::Abkhazia)
            .name("MyCity".to_string())
            .geo(GeoContext::new(point))
            .production(CityProduction::new(vec![]))
            .exploitation(CityExploitation::new(CityProductionTons(0)))
            .tasks(ClientCityTasks::new(ClientCityProductionTask::new(
                GameFrame(0),
                GameFrame(0),
            )))
            .build()
    }

    fn build_unit(point: WorldPoint) -> ClientUnit {
        ClientUnit::builder()
            .id(UnitId::default())
            .flag(Flag::Abkhazia)
            .type_(UnitType::Warriors)
            .geo(GeoContext::new(point))
            .task(ClientTask::new(
                ClientTaskType::Idle,
                GameFrame(0),
                GameFrame(0),
            ))
            .can(vec![])
            .build()
    }

    #[test]
    fn test_city_update() {
        // Given
        let point: WorldPoint = WorldPoint::new(0, 0);
        let mut slice = GameSliceResource(Some(GameSlice::empty(point.into(), D2Size::new(1, 1))));
        let mut frame = GameFrameResource::default();
        let mut window = GameWindowResource::default();
        let city = build_city(point);

        // When-Then
        let message = ClientStateMessage::SetCity(city.clone());
        react_state_message_(&message, &mut slice, &mut frame, &mut window);

        assert_eq!(
            slice.0.as_ref().map(|s| s.cities().items().to_vec()),
            Some(vec![Some(city.clone())])
        );
        assert_eq!(
            slice.0.as_ref().map(|s| s.cities_map().get(city.id())),
            Some(Some(&CityVec2dIndex(0)))
        );

        // When-Then
        let message = ClientStateMessage::RemoveCity(*city.geo().point(), *city.id());
        react_state_message_(&message, &mut slice, &mut frame, &mut window);

        assert_eq!(
            slice.0.as_ref().map(|s| s.cities().items().to_vec()),
            Some(vec![None])
        );
        assert_eq!(
            slice.0.as_ref().map(|s| s.cities_map().get(city.id())),
            Some(None)
        );
    }

    #[test]
    fn test_unit_update() {
        // Given
        let point: WorldPoint = WorldPoint::new(0, 0);
        let mut slice = GameSliceResource(Some(GameSlice::empty(point.into(), D2Size::new(1, 1))));
        let mut frame = GameFrameResource::default();
        let mut window = GameWindowResource::default();
        let unit = build_unit(point);

        // When-Then
        let message = ClientStateMessage::SetUnit(unit.clone());
        react_state_message_(&message, &mut slice, &mut frame, &mut window);

        assert_eq!(
            slice.0.as_ref().map(|s| s.units().items().to_vec()),
            Some(vec![Some(vec![unit.clone()])])
        );
        assert_eq!(
            slice.0.as_ref().map(|s| s.units_map().get(unit.id())),
            Some(Some(&UnitVec2dIndex(0, 0)))
        );

        // When-Then
        let message = ClientStateMessage::RemoveUnit(*unit.geo().point(), *unit.id());
        react_state_message_(&message, &mut slice, &mut frame, &mut window);

        assert_eq!(
            slice.0.as_ref().map(|s| s.units().items().to_vec()),
            Some(vec![None])
        );
        assert_eq!(
            slice.0.as_ref().map(|s| s.units_map().get(unit.id())),
            Some(None)
        );
    }
}
