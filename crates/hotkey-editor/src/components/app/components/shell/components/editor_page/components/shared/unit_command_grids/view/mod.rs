use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

#[derive(Clone, PartialEq)]
pub struct UnitCommandGridsView {
    pub unit_id: WarcraftObjectId,
    pub command_card_slots: Rc<[GridSlotId]>,
    pub build_menu_slots: Option<Rc<[GridSlotId]>>,
    pub uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    pub research_menu_slots: Option<Rc<[GridSlotId]>>,
}

impl ddd::View for UnitCommandGridsView {}
