use super::props::KeyPickerKeyProps;
use super::state::KeyPickerKeyState;
use super::style;
use crate::components::app::components::shell::components::shared::key_picker_board::{
    KeyCellState, KeyWidth,
};
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement, TooltipProps,
};
use dioxus::prelude::*;
use tw_macro::ClassList;

/// A picker key's fully shaped presentation: the state class, the cap label, the
/// `data-wide` flag that widens oversized caps, the `data-label` selector hook, the
/// disabled flag, and the click handler. Built by `From` so the body only places
/// these and never derives them.
pub(super) struct KeyPickerKeyPresentation {
    pub(super) class: ClassList,
    pub(super) label: String,
    /// The label again, for the `data-label` selector hook (e2e picks a specific key
    /// by it). Kept separate so the body can place the label as both text and
    /// attribute without cloning in the markup.
    pub(super) data_label: String,
    pub(super) data_wide: &'static str,
    pub(super) disabled: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl From<&KeyPickerKeyProps> for KeyPickerKeyPresentation {
    fn from(props: &KeyPickerKeyProps) -> Self {
        let cell = props.cell.clone();
        let on_pick = props.on_pick;
        let key_code = cell.key_code();
        let label = cell.label().to_string();
        let data_label = label.clone();
        let width = cell.width();
        let data_wide = match width {
            KeyWidth::Wide => "true",
            KeyWidth::Standard => "false",
        };
        let cell_state = cell.state();
        let state = match cell_state {
            KeyCellState::Available => KeyPickerKeyState::Available,
            KeyCellState::Current => KeyPickerKeyState::Current,
            KeyCellState::Conflict { .. } => KeyPickerKeyState::Conflict,
        };
        let disabled = !cell.pickable();
        let class = style::class(state);
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            if !disabled {
                on_pick.call(key_code);
            }
        });
        Self {
            class,
            label,
            data_label,
            data_wide,
            disabled,
            onclick,
        }
    }
}

impl From<&KeyPickerKeyProps> for TooltipProps {
    fn from(props: &KeyPickerKeyProps) -> Self {
        let cell_state = props.cell.state();
        match cell_state {
            KeyCellState::Conflict {
                tooltip,
                placement,
                anchor,
            } => {
                let text = tooltip.clone();
                let placement = *placement;
                let anchor = *anchor;
                Self {
                    text,
                    placement,
                    anchor,
                }
            }
            _ => {
                let text = String::new();
                let placement = TooltipPlacement::default();
                let anchor = TooltipAnchor::default();
                Self {
                    text,
                    placement,
                    anchor,
                }
            }
        }
    }
}
