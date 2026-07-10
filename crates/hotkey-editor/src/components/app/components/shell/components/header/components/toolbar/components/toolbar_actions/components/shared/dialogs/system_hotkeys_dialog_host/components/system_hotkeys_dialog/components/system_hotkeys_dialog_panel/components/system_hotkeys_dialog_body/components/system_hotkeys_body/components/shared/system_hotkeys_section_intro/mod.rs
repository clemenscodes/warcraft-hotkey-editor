mod props;
mod style;

use dioxus::prelude::*;
pub use props::SystemHotkeysSectionIntroProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SystemHotkeysSectionIntro);

/// The caption introducing a system-hotkeys category editor.
#[component]
pub fn SystemHotkeysSectionIntro(props: SystemHotkeysSectionIntroProps) -> Element {
    let text = props.text;
    rsx! {
        p { class: CLASS, {text} }
    }
}
