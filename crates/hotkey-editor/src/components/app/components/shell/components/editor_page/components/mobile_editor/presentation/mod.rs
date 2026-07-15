use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::{GridSlotId, UnitSlotContainers};

pub(super) enum MobileEditorView {
    Empty,
    Loaded(MobileCommandCard),
}

pub(super) struct MobileCommandCard {
    pub(super) unit_id: WarcraftObjectId,
    pub(super) command_card_slots: Rc<[GridSlotId]>,
    pub(super) build_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) research_menu_slots: Option<Rc<[GridSlotId]>>,
}

pub(super) fn use_mobile_editor() -> MobileEditorView {
    let navigation = use_view_navigation();
    let selected_unit_id = navigation.selected_unit_id();
    let slot_data_memo = use_memo(move || {
        let unit_id_option = *selected_unit_id.read();
        let unit_id = unit_id_option.unwrap_or_default();
        UnitSlotContainers::resolve(unit_id)
    });
    let unit_id_option = *selected_unit_id.read();
    let Some(unit_id) = unit_id_option else {
        return MobileEditorView::Empty;
    };
    let slot_containers = slot_data_memo.read();
    let command_card_slots = slot_containers.command_card();
    let build_menu_slots = slot_containers.build_menu();
    let uprooted_menu_slots = slot_containers.uprooted();
    let research_menu_slots = slot_containers.research();
    let card = MobileCommandCard {
        unit_id,
        command_card_slots,
        build_menu_slots,
        uprooted_menu_slots,
        research_menu_slots,
    };
    MobileEditorView::Loaded(card)
}
