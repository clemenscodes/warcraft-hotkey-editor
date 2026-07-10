mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::shared::alt_position_picker_body::{AltPositionPickerBody, AltPositionPickerBodyProps};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::{DialogHeader, DialogHeaderProps};
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
pub use props::UpgradePositionPickerPanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The upgraded-form position picker's bordered box: it wraps the library
/// `DialogContent` and styles a real `div` of its own with the box `CLASS`, so no
/// project class ever lands on the library element. Holds the header row above the
/// scrolling grid body.
#[component]
pub fn UpgradePositionPickerPanel(props: UpgradePositionPickerPanelProps) -> Element {
    let header = DialogHeaderProps::from(&props);
    let body = AltPositionPickerBodyProps::from(&props);
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader { ..header }
                AltPositionPickerBody { ..body }
            }
        }
    }
}

assert_component!(UpgradePositionPickerPanel);
