use super::components::available_key::AvailableKeyProps;
use super::components::conflict_key::ConflictKeyProps;
use super::components::current_key::CurrentKeyProps;
use super::props::KeyPickerKeyProps;
use super::state::KeyPickerKeyState;
use crate::components::app::components::shell::components::shared::key_picker_board::{
    KeyCellState, KeyWidth,
};
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement, TooltipProps,
};
use dioxus::prelude::*;

/// A picker key's fully shaped presentation: the visual state the dispatcher matches on
/// to pick a look, plus the button attributes and children every look renders the same
/// way (the cap label, the `data-label` selector hook, the `data-wide` flag that widens
/// oversized caps, the disabled flag, the click handler, and the conflict tooltip).
/// Built by `From` so the body only reads the state and spreads the rest, never deriving
/// them. The three looks differ only in their own color styling; every attribute here is
/// shared.
pub(super) struct KeyPickerKeyPresentation {
    pub(super) state: KeyPickerKeyState,
    pub(super) label: String,
    /// The label again, for the `data-label` selector hook (e2e picks a specific key by
    /// it). Kept separate so a look can place the label as both text and attribute
    /// without cloning in the markup.
    pub(super) data_label: String,
    pub(super) data_wide: &'static str,
    pub(super) disabled: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) tooltip: TooltipProps,
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
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            if !disabled {
                on_pick.call(key_code);
            }
        });
        let tooltip = TooltipProps::from(props);
        Self {
            state,
            label,
            data_label,
            data_wide,
            disabled,
            onclick,
            tooltip,
        }
    }
}

impl From<&KeyPickerKeyPresentation> for AvailableKeyProps {
    fn from(presentation: &KeyPickerKeyPresentation) -> Self {
        let label = presentation.label.clone();
        let data_label = presentation.data_label.clone();
        let data_wide = presentation.data_wide;
        let disabled = presentation.disabled;
        let onclick = presentation.onclick;
        let tooltip = presentation.tooltip.clone();
        Self {
            label,
            data_label,
            data_wide,
            disabled,
            onclick,
            tooltip,
        }
    }
}

impl From<&KeyPickerKeyPresentation> for CurrentKeyProps {
    fn from(presentation: &KeyPickerKeyPresentation) -> Self {
        let label = presentation.label.clone();
        let data_label = presentation.data_label.clone();
        let data_wide = presentation.data_wide;
        let disabled = presentation.disabled;
        let onclick = presentation.onclick;
        let tooltip = presentation.tooltip.clone();
        Self {
            label,
            data_label,
            data_wide,
            disabled,
            onclick,
            tooltip,
        }
    }
}

impl From<&KeyPickerKeyPresentation> for ConflictKeyProps {
    fn from(presentation: &KeyPickerKeyPresentation) -> Self {
        let label = presentation.label.clone();
        let data_label = presentation.data_label.clone();
        let data_wide = presentation.data_wide;
        let disabled = presentation.disabled;
        let onclick = presentation.onclick;
        let tooltip = presentation.tooltip.clone();
        Self {
            label,
            data_label,
            data_wide,
            disabled,
            onclick,
            tooltip,
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
