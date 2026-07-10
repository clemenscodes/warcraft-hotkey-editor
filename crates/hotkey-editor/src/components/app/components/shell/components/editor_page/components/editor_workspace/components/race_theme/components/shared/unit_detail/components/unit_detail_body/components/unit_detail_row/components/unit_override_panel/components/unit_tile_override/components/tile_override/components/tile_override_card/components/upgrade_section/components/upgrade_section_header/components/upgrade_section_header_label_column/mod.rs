mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_label::{AltStateLabel, AltStateLabelProps};
use dioxus::prelude::*;
pub use props::UpgradeSectionHeaderLabelColumnProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UpgradeSectionHeaderLabelColumn);

/// The label column of the upgraded-form header row: holds the "Upgraded form" label.
#[component]
pub fn UpgradeSectionHeaderLabelColumn(props: UpgradeSectionHeaderLabelColumnProps) -> Element {
    let label = AltStateLabelProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            AltStateLabel { ..label }
        }
    }
}
