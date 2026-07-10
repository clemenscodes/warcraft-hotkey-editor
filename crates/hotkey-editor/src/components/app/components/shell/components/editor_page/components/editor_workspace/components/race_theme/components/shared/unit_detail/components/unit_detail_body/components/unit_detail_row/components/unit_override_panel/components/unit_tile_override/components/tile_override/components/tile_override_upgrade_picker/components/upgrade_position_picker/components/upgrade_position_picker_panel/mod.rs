mod props;
mod view;

pub use view::UpgradePositionPickerPanelView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::shared::alt_position_picker_body::AltPositionPickerBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
use props::UpgradePositionPickerPanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The upgraded-form position picker's bordered box: it wraps the library
/// `DialogContent` and styles a real `div` of its own with the box `CLASS`, so no
/// project class ever lands on the library element. Holds the header row above the
/// scrolling grid body.
#[component]
pub fn UpgradePositionPickerPanel(props: UpgradePositionPickerPanelProps) -> Element {
    let UpgradePositionPickerPanelProps {
        title,
        on_close,
        explainer_text,
        grid_config,
    } = props;
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader {
                    title,
                    on_close,
                }
                AltPositionPickerBody {
                    explainer_text,
                    grid_config,
                }
            }
        }
    }
}

assert_component!(UpgradePositionPickerPanel);
