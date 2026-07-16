use super::model::UnitCardModel;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

pub(super) struct UnitCardPresentation {
    pub(super) icon_path: Option<IconUrl>,
    pub(super) display_name: String,
    pub(super) unit_id: WarcraftObjectId,
    pub(super) is_selected: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) onkeydown: EventHandler<KeyboardEvent>,
}

pub(super) fn use_unit_card(props: &UnitCardModel) -> UnitCardPresentation {
    let unit_id = props.unit_id;
    let unit_kind = props.unit_kind;
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let selected_unit_id = navigation.selected_unit_id();
    let mut selected_slot = editor.selected_slot();
    let mut active_category = editor.active_category();
    let is_selected = *selected_unit_id.read() == Some(unit_id);
    // Searching drops the race filter, so a card in this list can belong to any
    // race. `open_unit` derives the race and the mode from the unit itself, so
    // picking one always lands on that unit's own theme.
    let on_click = EventHandler::new(move |_event: MouseEvent| {
        navigation.open_unit(unit_id);
        selected_slot.set(None);
        active_category.set(unit_kind);
    });
    let on_keydown = EventHandler::new(move |event: KeyboardEvent| {
        let key_value = event.data().key().to_string();
        if key_value == " " || key_value == "Enter" {
            event.prevent_default();
            navigation.open_unit(unit_id);
            selected_slot.set(None);
            active_category.set(unit_kind);
        }
    });
    let icon_path = props.icon_path.clone();
    let display_name = props.display_name.clone();
    UnitCardPresentation {
        icon_path,
        display_name,
        unit_id,
        is_selected,
        onclick: on_click,
        onkeydown: on_keydown,
    }
}

impl ddd::Presentation for UnitCardPresentation {
    type Model = UnitCardModel;
}
