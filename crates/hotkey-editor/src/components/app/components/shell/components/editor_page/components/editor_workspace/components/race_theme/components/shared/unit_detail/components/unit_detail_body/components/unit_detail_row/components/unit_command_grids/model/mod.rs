use super::view::UnitCommandGridsView;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCommandGridsModel {
    pub unit_id: WarcraftObjectId,
    pub command_card_slots: Rc<[GridSlotId]>,
    pub build_menu_slots: Option<Rc<[GridSlotId]>>,
    pub uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    pub research_menu_slots: Option<Rc<[GridSlotId]>>,
}

impl From<&UnitCommandGridsView> for UnitCommandGridsModel {
    fn from(view: &UnitCommandGridsView) -> Self {
        let UnitCommandGridsView {
            unit_id,
            command_card_slots,
            build_menu_slots,
            uprooted_menu_slots,
            research_menu_slots,
        } = view.clone();
        Self {
            unit_id,
            command_card_slots,
            build_menu_slots,
            uprooted_menu_slots,
            research_menu_slots,
        }
    }
}

impl ddd::Model for UnitCommandGridsModel {
    type View = UnitCommandGridsView;
}
