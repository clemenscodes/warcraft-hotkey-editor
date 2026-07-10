use super::components::narrow_key_slot::NarrowKeySlotProps;
use super::components::wide_key_slot::WideKeySlotProps;
use super::props::KeyPickerKeyProps;
use crate::components::app::components::shell::components::shared::key_picker_board::components::key_picker_column::components::key_picker_row::components::key_picker_key::components::shared::color_key::ColorKeyState;
use crate::components::app::components::shell::components::shared::key_picker_board::{
    KeyCellState, KeyWidth,
};
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement, TooltipProps,
};
use dioxus::prelude::*;

/// A picker key's fully shaped presentation: the width the dispatcher matches on to pick
/// a sizing slot, plus the color state and the button attributes and children every look
/// renders the same way (the cap label, the disabled flag, the click handler, and the
/// conflict tooltip). Built by `From` so the body only
/// reads the width and spreads the rest, never deriving them. The width is owned by the
/// slot, the color by the leaf; every attribute here is shared by both.
pub(super) struct KeyPickerKeyPresentation {
    pub(super) state: ColorKeyState,
    pub(super) label: String,
    pub(super) disabled: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) tooltip: TooltipProps,
    pub(super) width: KeyWidth,
}

impl From<&KeyPickerKeyProps> for KeyPickerKeyPresentation {
    fn from(props: &KeyPickerKeyProps) -> Self {
        let cell = props.cell.clone();
        let on_pick = props.on_pick;
        let key_code = cell.key_code();
        let label = cell.label().to_string();
        let width = cell.width();
        let cell_state = cell.state();
        let state = match cell_state {
            KeyCellState::Available => ColorKeyState::Available,
            KeyCellState::Current => ColorKeyState::Current,
            KeyCellState::Conflict { .. } => ColorKeyState::Conflict,
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
            disabled,
            onclick,
            tooltip,
            width,
        }
    }
}

impl From<&KeyPickerKeyPresentation> for NarrowKeySlotProps {
    fn from(presentation: &KeyPickerKeyPresentation) -> Self {
        let state = presentation.state;
        let label = presentation.label.clone();
        let disabled = presentation.disabled;
        let onclick = presentation.onclick;
        let tooltip = presentation.tooltip.clone();
        Self {
            state,
            label,
            disabled,
            onclick,
            tooltip,
        }
    }
}

impl From<&KeyPickerKeyPresentation> for WideKeySlotProps {
    fn from(presentation: &KeyPickerKeyPresentation) -> Self {
        let state = presentation.state;
        let label = presentation.label.clone();
        let disabled = presentation.disabled;
        let onclick = presentation.onclick;
        let tooltip = presentation.tooltip.clone();
        Self {
            state,
            label,
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
