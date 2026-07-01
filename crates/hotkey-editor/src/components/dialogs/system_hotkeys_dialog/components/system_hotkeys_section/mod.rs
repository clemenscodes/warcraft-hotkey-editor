pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::system_hotkeys_section_intro::SystemHotkeysSectionIntro;
use dioxus::prelude::*;
pub use props::SystemHotkeysSectionProps;
use style::CLASS;
assert_component!(SystemHotkeysSection);

/// Frames one system-hotkeys category editor: its intro line above the editor.
#[component]
pub fn SystemHotkeysSection(props: SystemHotkeysSectionProps) -> Element {
    let intro = props.intro.clone();
    let children = props.children;
    rsx! {
        div { class: CLASS,
            SystemHotkeysSectionIntro { text: intro }
            {children}
        }
    }
}
