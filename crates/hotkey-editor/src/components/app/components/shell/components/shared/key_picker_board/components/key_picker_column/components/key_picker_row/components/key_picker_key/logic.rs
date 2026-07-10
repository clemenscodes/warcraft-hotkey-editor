use super::props::KeyPickerKeyProps;
use crate::components::app::components::shell::components::shared::key_picker_board::components::key_picker_column::components::key_picker_row::components::key_picker_key::components::shared::color_key::ColorKeyState;
use crate::components::app::components::shell::components::shared::key_picker_board::{
    KeyCellState, KeyWidth,
};
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use dioxus::prelude::*;

/// A picker key's fully shaped presentation: the width the dispatcher matches on to pick
/// a sizing slot, plus the color state and the button attributes every look renders the
/// same way (the cap label, the disabled flag, the click handler) and the conflict
/// tooltip flattened to its three domain fields. Built by `From` so the body only reads
/// the width and hands the rest down by name. The width is owned by the slot, the color
/// by the leaf; every attribute here is shared by both.
pub(super) struct KeyPickerKeyPresentation {
    pub(super) state: ColorKeyState,
    pub(super) label: String,
    pub(super) disabled: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) tooltip_text: String,
    pub(super) tooltip_placement: TooltipPlacement,
    pub(super) tooltip_anchor: TooltipAnchor,
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
        let tooltip_text;
        let tooltip_placement;
        let tooltip_anchor;
        match cell_state {
            KeyCellState::Conflict {
                tooltip,
                placement,
                anchor,
            } => {
                tooltip_text = tooltip.clone();
                tooltip_placement = *placement;
                tooltip_anchor = *anchor;
            }
            _ => {
                tooltip_text = String::new();
                tooltip_placement = TooltipPlacement::default();
                tooltip_anchor = TooltipAnchor::default();
            }
        }
        let disabled = !cell.pickable();
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            if !disabled {
                on_pick.call(key_code);
            }
        });
        Self {
            state,
            label,
            disabled,
            onclick,
            tooltip_text,
            tooltip_placement,
            tooltip_anchor,
            width,
        }
    }
}
