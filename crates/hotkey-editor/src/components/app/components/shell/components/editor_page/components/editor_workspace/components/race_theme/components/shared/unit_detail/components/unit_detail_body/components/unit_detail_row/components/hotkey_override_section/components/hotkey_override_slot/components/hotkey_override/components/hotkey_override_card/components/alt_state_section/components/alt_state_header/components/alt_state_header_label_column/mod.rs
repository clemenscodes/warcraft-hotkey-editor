mod model;
mod view;

pub use view::AltStateHeaderLabelColumnView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::hotkey_override_section::components::hotkey_override_slot::components::hotkey_override::components::hotkey_override_card::components::shared::alt_state_label::AltStateLabel;
use dioxus::prelude::*;
use model::AltStateHeaderLabelColumnModel;
use style::CLASS;
use tw_macro::assert_component;

/// The label column of the off-state header row: holds the off-state name label.
#[component]
pub fn AltStateHeaderLabelColumn(props: AltStateHeaderLabelColumnModel) -> Element {
    let AltStateHeaderLabelColumnModel { text } = props;
    rsx! {
        div {
            class: CLASS,
            AltStateLabel { text }
        }
    }
}

assert_component!(AltStateHeaderLabelColumn);
