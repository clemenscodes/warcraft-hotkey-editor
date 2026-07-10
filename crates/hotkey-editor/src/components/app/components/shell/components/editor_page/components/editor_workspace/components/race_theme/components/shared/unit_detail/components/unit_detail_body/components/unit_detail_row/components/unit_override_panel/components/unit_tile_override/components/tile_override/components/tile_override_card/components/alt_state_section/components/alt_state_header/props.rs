use super::components::alt_state_controls::AltStateControlsProps;
use super::components::alt_state_header_label_column::AltStateHeaderLabelColumnProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_label::AltStateLabelProps;
use dioxus::prelude::*;

/// The off-state block's top row: the label column beside its editable controls.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateHeaderProps {
    pub label: AltStateLabelProps,
    pub controls: AltStateControlsProps,
}

impl From<&AltStateHeaderProps> for AltStateHeaderLabelColumnProps {
    fn from(props: &AltStateHeaderProps) -> Self {
        let label = props.label.clone();
        Self { label }
    }
}

impl From<&AltStateHeaderProps> for AltStateControlsProps {
    fn from(props: &AltStateHeaderProps) -> Self {
        props.controls.clone()
    }
}
