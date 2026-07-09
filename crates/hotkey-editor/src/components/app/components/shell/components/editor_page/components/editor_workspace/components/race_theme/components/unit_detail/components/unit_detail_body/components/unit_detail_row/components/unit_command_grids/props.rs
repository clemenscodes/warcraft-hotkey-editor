use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::{Race, WarcraftObjectId};
use warcraft_keybinds::GridSlotId;

/// The unit's four command grids (command card, build menu, uprooted, research). Only
/// the unit's per-menu identity is a prop; the shared editor signals the grids need are
/// sourced from context by the component's hook.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCommandGridsProps {
    pub unit_id: WarcraftObjectId,
    pub race: Race,
    pub command_card_slots: Rc<[GridSlotId]>,
    pub build_menu_slots: Option<Rc<[GridSlotId]>>,
    pub uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    pub research_menu_slots: Option<Rc<[GridSlotId]>>,
}
