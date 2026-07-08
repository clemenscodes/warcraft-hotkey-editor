pub mod components;
mod logic;
mod props;
mod style;

use components::hero_selection_row::{HeroSelectionRow, HeroSelectionRowProps};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_hotkeys_section_intro::{SystemHotkeysSectionIntro, SystemHotkeysSectionIntroProps};
use dioxus::prelude::*;
pub use props::HeroSelectionHotkeysViewProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HeroSelectionHotkeysView);

/// The hero-selection hotkey editor: the intro caption above three big slots for
/// selecting heroes by index.
#[component]
pub fn HeroSelectionHotkeysView(props: HeroSelectionHotkeysViewProps) -> Element {
    let intro = SystemHotkeysSectionIntroProps::from(&props);
    let row = HeroSelectionRowProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            SystemHotkeysSectionIntro { ..intro }
            HeroSelectionRow { ..row }
        }
    }
}
