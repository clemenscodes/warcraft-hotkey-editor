use super::props::KeyPickerKeyProps;
use super::state::KeyPickerKeyState;
use super::style;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::KeyPickerCellState;
use crate::styling::ClassList;
use dioxus::prelude::*;

/// A picker key's fully shaped presentation: the state class, the letter it shows,
/// the hover title, whether it is disabled, the `data-special` flag that widens
/// multi-character keys, and the click handler. Built by `From` so the body only
/// places these and never derives them.
pub(super) struct KeyPickerKeyPresentation {
    pub(super) class: ClassList,
    pub(super) label: String,
    /// The label again, for the `data-label` selector hook (e2e picks a specific
    /// key by it). Kept separate so the body can place the label as both text and
    /// attribute without cloning in the markup.
    pub(super) data_label: String,
    pub(super) title: String,
    pub(super) disabled: bool,
    pub(super) special: &'static str,
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl From<&KeyPickerKeyProps> for KeyPickerKeyPresentation {
    fn from(props: &KeyPickerKeyProps) -> Self {
        let cell = props.cell.clone();
        let allow_conflict_pick = props.allow_conflict_pick;
        let on_pick = props.on_pick;
        let token = cell.token();
        let label = token.display_label();
        let data_label = label.clone();
        let cell_state = cell.state();
        let state = match cell_state {
            KeyPickerCellState::Available => KeyPickerKeyState::Available,
            KeyPickerCellState::Current => KeyPickerKeyState::Current,
            KeyPickerCellState::Conflict { .. } => KeyPickerKeyState::Conflict,
        };
        let title = match cell_state {
            KeyPickerCellState::Conflict { display_name } => {
                let prefix = if allow_conflict_pick {
                    "Pick to swap with"
                } else {
                    "Already used by"
                };
                format!("{prefix} {display_name}")
            }
            _ => String::new(),
        };
        let is_conflict = matches!(cell_state, KeyPickerCellState::Conflict { .. });
        let disabled = is_conflict && !allow_conflict_pick;
        let single_character = char::try_from(token);
        let special = if single_character.is_err() {
            "true"
        } else {
            "false"
        };
        let class = style::class(state);
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            if !disabled {
                on_pick.call(token);
            }
        });
        Self {
            class,
            label,
            data_label,
            title,
            disabled,
            special,
            onclick,
        }
    }
}
