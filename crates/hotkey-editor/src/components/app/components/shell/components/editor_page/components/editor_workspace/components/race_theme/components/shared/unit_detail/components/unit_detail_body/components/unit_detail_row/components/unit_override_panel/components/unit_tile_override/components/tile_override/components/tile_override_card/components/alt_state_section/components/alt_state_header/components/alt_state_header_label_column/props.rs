use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_label::AltStateLabelProps;
use dioxus::prelude::*;

/// The label column of the off-state header row.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateHeaderLabelColumnProps {
    pub label: AltStateLabelProps,
}

impl From<&AltStateHeaderLabelColumnProps> for AltStateLabelProps {
    fn from(props: &AltStateHeaderLabelColumnProps) -> Self {
        props.label.clone()
    }
}
