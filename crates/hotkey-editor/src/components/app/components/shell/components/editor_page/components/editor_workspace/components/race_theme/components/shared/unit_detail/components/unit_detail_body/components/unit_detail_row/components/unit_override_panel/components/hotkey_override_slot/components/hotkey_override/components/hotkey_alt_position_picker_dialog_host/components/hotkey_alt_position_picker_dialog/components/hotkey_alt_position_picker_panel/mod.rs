mod model;
mod view;

pub use view::HotkeyAltPositionPickerPanelView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::hotkey_override_slot::components::hotkey_override::components::shared::hotkey_alt_position_picker_body::HotkeyAltPositionPickerBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
use model::HotkeyAltPositionPickerPanelModel;
use style::CLASS;
use tw_macro::assert_component;

/// The off-state position picker's bordered box: it wraps the library `DialogContent`
/// and styles a real `div` of its own with the box `CLASS`, so no project class ever
/// lands on the library element. Holds the header row above the scrolling grid body.
#[component]
pub fn HotkeyAltPositionPickerPanel(props: HotkeyAltPositionPickerPanelModel) -> Element {
    let HotkeyAltPositionPickerPanelModel {
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
                HotkeyAltPositionPickerBody {
                    explainer_text,
                    grid_config,
                }
            }
        }
    }
}

assert_component!(HotkeyAltPositionPickerPanel);
