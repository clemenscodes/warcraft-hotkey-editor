mod hooks;
mod props;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::slot_button::SlotButton;
use dioxus::prelude::*;
use hooks::use_hero_selection_row;
pub use props::HeroSelectionRowProps;
use style::CLASS;
assert_component!(HeroSelectionRow);

/// The three-slot hero-selection row.
#[component]
pub fn HeroSelectionRow(props: HeroSelectionRowProps) -> Element {
    let model = use_hero_selection_row(&props);
    rsx! {
        div {
            class: CLASS,
            style: model.frame,
            for slot in model.slots {
                SlotButton { ..slot }
            }
        }
    }
}
