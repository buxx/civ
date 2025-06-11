use bevy::prelude::*;
use common::{
    game::{slice::ClientUnit, tasks::client::ClientTask, unit::UnitId, GameFrame},
    network::message::{ClientToServerInGameMessage, ClientToServerUnitMessage},
};
use derive_more::Constructor;

use crate::{
    impl_ui_component_resource,
    ingame::{
        interact::{FromUnit, WithUnitId},
        DrawUiComponent, EGUI_DISPLAY_FACTOR,
    },
    send_unit_msg,
    utils::gui::layout::fixed_window,
};

#[derive(Debug, Event, Constructor)]
pub struct SetupUnitInfo(pub UnitId);

#[derive(Debug, Constructor)]
pub struct UnitInfo {
    unit_id: UnitId,
    task: Option<ClientTask>,
}

#[derive(Debug, Resource, Default)]
pub struct UnitInfoResource(pub Option<UnitInfo>);
impl_ui_component_resource!(UnitInfoResource, UnitInfo, SetupUnitInfo);

impl WithUnitId for SetupUnitInfo {
    fn unit_id(&self) -> &UnitId {
        &self.0
    }
}

impl WithUnitId for UnitInfo {
    fn unit_id(&self) -> &UnitId {
        &self.unit_id
    }
}

impl DrawUiComponent for UnitInfo {
    fn draw(
        &mut self,
        ctx: &bevy_egui::egui::Context,
        window: &Window,
        commands: &mut Commands,
        frame: GameFrame,
    ) -> bool {
        let mut close = false;
        let task = &self.task;

        fixed_window()
            .ctx(ctx)
            .window(window)
            .title("Unit info")
            .factor(EGUI_DISPLAY_FACTOR)
            .ui(|ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Current task:");
                        ui.label(
                            task.as_ref()
                                .map(|t| t.to_string(&frame))
                                .unwrap_or("None".to_string()),
                        );

                        if task.is_some() && ui.button("Cancel").clicked() {
                            send_unit_msg!(
                                commands,
                                self.unit_id,
                                ClientToServerUnitMessage::CancelCurrentTask
                            );
                        }
                    });

                    close = ui.button("Close").clicked();
                });
            })
            .call();

        close
    }
}

impl FromUnit for UnitInfo {
    fn from_unit(unit: &ClientUnit) -> Self {
        Self::new(*unit.id(), unit.task().clone())
    }
}
