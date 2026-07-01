mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::SystemHotkeysSectionIntroProps;
use style::CLASS;
assert_component!(SystemHotkeysSectionIntro);

/// The caption introducing a system-hotkeys category editor.
#[component]
pub fn SystemHotkeysSectionIntro(props: SystemHotkeysSectionIntroProps) -> Element {
    let text = props.text;
    rsx! {
        p { class: CLASS, {text} }
    }
}
