pub mod components;
mod logic;
mod props;
mod style;

use components::control_groups_row::{ControlGroupsRow, ControlGroupsRowProps};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_hotkeys_section_intro::{SystemHotkeysSectionIntro, SystemHotkeysSectionIntroProps};
use dioxus::prelude::*;
pub use props::ControlGroupsHotkeysViewProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ControlGroupsHotkeysView);

/// The control-groups (1–10) hotkey editor: the intro caption above a ten-cell
/// strip of editable slots.
#[component]
pub fn ControlGroupsHotkeysView(props: ControlGroupsHotkeysViewProps) -> Element {
    let intro = SystemHotkeysSectionIntroProps::from(&props);
    let row = ControlGroupsRowProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            SystemHotkeysSectionIntro { ..intro }
            ControlGroupsRow { ..row }
        }
    }
}
