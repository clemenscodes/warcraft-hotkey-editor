use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

use super::{KeyPickerCell, KeyPickerCellState};

#[derive(Props, Clone, PartialEq)]
pub(super) struct KeyPickerKeyProps {
    pub(super) cell: KeyPickerCell,
    pub(super) allow_conflict_pick: bool,
    pub(super) on_pick: EventHandler<HotkeyToken>,
}

#[component]
pub(super) fn KeyPickerKey(props: KeyPickerKeyProps) -> Element {
    let cell = props.cell;
    let allow_conflict_pick = props.allow_conflict_pick;
    let on_pick = props.on_pick;
    let token = cell.token();
    let label_text = token.display_label();
    let label_attr = label_text.clone();
    let (state_class, conflict_title) = match cell.state() {
        KeyPickerCellState::Available => ("available", None),
        KeyPickerCellState::Current => ("current", None),
        KeyPickerCellState::Conflict { display_name } => {
            let prefix = if allow_conflict_pick {
                "Pick to swap with"
            } else {
                "Already used by"
            };
            ("conflict", Some(format!("{prefix} {display_name}")))
        }
    };
    let is_conflict = matches!(cell.state(), KeyPickerCellState::Conflict { .. });
    let is_disabled = is_conflict && !allow_conflict_pick;
    let is_special = char::try_from(token).is_err();
    let class_name = format!("key-picker-key {state_class}");
    let title_attribute = conflict_title.unwrap_or_default();
    let special_flag = if is_special { "true" } else { "false" };
    let token_for_click = token;
    let handle_click = move |_| {
        if !is_disabled {
            on_pick.call(token_for_click);
        }
    };
    rsx! {
        button {
            class: class_name,
            r#type: "button",
            disabled: is_disabled,
            title: title_attribute,
            "data-label": label_attr,
            "data-special": special_flag,
            onclick: handle_click,
            {label_text}
        }
    }
}
