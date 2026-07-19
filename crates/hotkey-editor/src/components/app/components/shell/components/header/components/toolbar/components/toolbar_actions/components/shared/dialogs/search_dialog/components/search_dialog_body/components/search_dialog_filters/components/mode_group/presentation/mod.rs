use super::data;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::search_dialog::components::search_dialog_body::components::search_dialog_filters::components::shared::segmented_control::SegmentChoice;
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_api::{UnitMode, UnitModeSelection};

pub(super) fn use_mode_group() -> Vec<SegmentChoice> {
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let selected_slot = editor.selected_slot();
    let unit_modes = navigation.unit_modes();
    let current = *unit_modes.read();
    let melee = UnitModeSelection::only(UnitMode::Melee);
    let campaign = UnitModeSelection::only(UnitMode::Campaign);
    let both = UnitModeSelection::both();
    let melee_active = current == melee;
    let campaign_active = current == campaign;
    let both_active = current == both;
    let on_melee = EventHandler::new(move |_event: MouseEvent| {
        navigation.set_unit_modes(melee, selected_slot);
    });
    let on_campaign = EventHandler::new(move |_event: MouseEvent| {
        navigation.set_unit_modes(campaign, selected_slot);
    });
    let on_both = EventHandler::new(move |_event: MouseEvent| {
        navigation.set_unit_modes(both, selected_slot);
    });
    let melee_choice = SegmentChoice {
        key: "melee",
        label: data::MELEE,
        is_active: melee_active,
        on_pick: on_melee,
    };
    let campaign_choice = SegmentChoice {
        key: "campaign",
        label: data::CAMPAIGN,
        is_active: campaign_active,
        on_pick: on_campaign,
    };
    let both_choice = SegmentChoice {
        key: "both",
        label: data::BOTH,
        is_active: both_active,
        on_pick: on_both,
    };
    vec![melee_choice, campaign_choice, both_choice]
}
