pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::system_hotkeys_section_intro::{
    SystemHotkeysSectionIntro, SystemHotkeysSectionIntroProps,
};
use dioxus::prelude::*;
pub use props::SystemHotkeysSectionProps;
use style::CLASS;
assert_component!(SystemHotkeysSection);

/// Frames one system-hotkeys category editor: its intro line above the editor.
#[component]
pub fn SystemHotkeysSection(props: SystemHotkeysSectionProps) -> Element {
    let intro = SystemHotkeysSectionIntroProps::from(&props);
    let children = props.children;
    rsx! {
        div {
            class: CLASS,
            SystemHotkeysSectionIntro { ..intro }
            {children}
        }
    }
}
