mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::SystemHotkeysSectionIntroProps;

assert_component!(SystemHotkeysSectionIntro);

/// The caption introducing a system-hotkeys category editor.
#[component]
pub fn SystemHotkeysSectionIntro(props: SystemHotkeysSectionIntroProps) -> Element {
    let text = props.text;
    rsx! {
        p {
            class: CLASS,
            {text}
        }
    }
}
