use super::model::ModeTabsModel;
use dioxus::prelude::*;
use warcraft_api::UnitMode;

pub(super) struct ModeTabBinding {
    pub(super) label: &'static str,
    pub(super) active: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) onkeydown: EventHandler<KeyboardEvent>,
}

pub(super) struct ModeTabPair {
    pub(super) melee: ModeTabBinding,
    pub(super) campaign: ModeTabBinding,
}

impl ModeTabPair {
    pub(super) fn build(props: &ModeTabsModel) -> Self {
        let melee = mode_tab(props, UnitMode::Melee, "Melee");
        let campaign = mode_tab(props, UnitMode::Campaign, "Campaign");
        Self { melee, campaign }
    }
}

fn mode_tab(props: &ModeTabsModel, mode: UnitMode, label: &'static str) -> ModeTabBinding {
    let unit_mode = props.unit_mode;
    let on_select = props.on_select;
    let active = *unit_mode.read() == mode;
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        on_select.call(mode);
    });
    let onkeydown = EventHandler::new(move |event: KeyboardEvent| {
        let key = event.key();
        let key_value = key.to_string();
        let is_space = key_value == " ";
        let is_enter = key_value == "Enter";
        if is_space || is_enter {
            event.prevent_default();
            on_select.call(mode);
        }
    });
    ModeTabBinding {
        label,
        active,
        onclick,
        onkeydown,
    }
}
