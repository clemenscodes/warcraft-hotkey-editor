use super::model::UnitDetailRowModel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::presentation::UnitOverrideTarget;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

pub struct UnitDetailRowPresentation {
    pub(super) unit_id: WarcraftObjectId,
    pub(super) command_card_slots: Rc<[GridSlotId]>,
    pub(super) build_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) research_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) override_target: UnitOverrideTarget,
}

impl From<&UnitDetailRowModel> for UnitDetailRowPresentation {
    fn from(model: &UnitDetailRowModel) -> Self {
        let grid_slots = &model.grid_slots;
        let unit_id = grid_slots.unit_id;
        let command_card_slots = grid_slots.command_card_slots.clone();
        let build_menu_slots = grid_slots.build_menu_slots.clone();
        let uprooted_menu_slots = grid_slots.uprooted_menu_slots.clone();
        let research_menu_slots = grid_slots.research_menu_slots.clone();
        let override_target = model.override_target.clone();
        Self {
            unit_id,
            command_card_slots,
            build_menu_slots,
            uprooted_menu_slots,
            research_menu_slots,
            override_target,
        }
    }
}

impl ddd::Presentation for UnitDetailRowPresentation {
    type Model = UnitDetailRowModel;
}
