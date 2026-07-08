mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_label::{AltStateLabel, AltStateLabelProps};
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_position_button::{AltStatePositionButton, AltStatePositionButtonProps};
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::override_key::{OverrideKey, OverrideKeyProps};
use dioxus::prelude::*;
pub use props::UpgradeSectionProps;
use style::{CONTAINER, HEADER, HEADER_TEXT};
use tw_macro::assert_component;
assert_component!(UpgradeSection);

/// The upgraded-form block of a tile override: its label beside the position button
/// and key cell. It owns its own block, header, and label-column elements directly.
/// Renders nothing when there is no upgrade to show.
#[component]
pub fn UpgradeSection(props: UpgradeSectionProps) -> Element {
    if !props.show {
        return rsx! {};
    }
    let label = AltStateLabelProps::from(&props);
    let position_button = AltStatePositionButtonProps::from(&props);
    let key_cell = OverrideKeyProps::from(&props);
    rsx! {
        div {
            class: CONTAINER,
            div {
                class: HEADER,
                div {
                    class: HEADER_TEXT,
                    AltStateLabel { ..label }
                }
                AltStatePositionButton { ..position_button }
                OverrideKey { ..key_cell }
            }
        }
    }
}
