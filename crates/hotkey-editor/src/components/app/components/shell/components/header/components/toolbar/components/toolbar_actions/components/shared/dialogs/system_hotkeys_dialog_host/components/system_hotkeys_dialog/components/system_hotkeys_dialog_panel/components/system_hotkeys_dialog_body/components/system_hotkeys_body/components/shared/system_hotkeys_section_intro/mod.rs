mod props;
mod style;

use dioxus::prelude::*;
use props::SystemHotkeysSectionIntroProps;
use style::CLASS;
use tw_macro::assert_component;

/// The caption introducing a system-hotkeys category editor.
#[component]
pub fn SystemHotkeysSectionIntro(props: SystemHotkeysSectionIntroProps) -> Element {
    let text = props.text;
    rsx! {
        p { class: CLASS, {text} }
    }
}

assert_component!(SystemHotkeysSectionIntro);
