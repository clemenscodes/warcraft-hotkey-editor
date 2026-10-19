use super::model::KeyPickerKeyModel;
use crate::components::app::components::shell::components::shared::key_picker_board::components::key_picker_column::components::key_picker_row::components::key_picker_key::components::shared::color_key::ColorKeyState;
use crate::components::app::components::shell::components::shared::key_picker_board::{
    KeyCellState, KeyWidth,
};
use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use dioxus::prelude::*;

/// The tooltip a taken key explains its conflict with; empty on any other key.
#[derive(Clone, Debug, PartialEq, Default)]
struct ConflictTooltip {
    text: String,
    placement: TooltipPlacement,
    anchor: TooltipAnchor,
}

impl From<&KeyCellState> for ConflictTooltip {
    fn from(cell_state: &KeyCellState) -> Self {
        match cell_state {
            KeyCellState::Conflict {
                tooltip,
                placement,
                anchor,
            } => Self {
                text: tooltip.clone(),
                placement: *placement,
                anchor: *anchor,
            },
            _ => Self::default(),
        }
    }
}

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

impl From<&KeyPickerKeyModel> for KeyPickerKeyPresentation {
    fn from(props: &KeyPickerKeyModel) -> Self {
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
        let ConflictTooltip {
            text: tooltip_text,
            placement: tooltip_placement,
            anchor: tooltip_anchor,
        } = ConflictTooltip::from(cell_state);
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

impl ddd::Layered for KeyPickerKeyPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for KeyPickerKeyPresentation {
    type Model = KeyPickerKeyModel;
}
