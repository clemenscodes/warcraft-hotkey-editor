pub mod components;
mod style;

use components::burger_menu::BurgerMenu;
use components::inline_actions::InlineActions;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The adaptive file-action controls: the inline button row at laptop width and up, and the
/// burger drawer on narrower screens. Pure layout — it threads no data and mounts no dialog;
/// each button and drawer row owns the dialog it opens.
#[component]
pub fn ToolbarActions() -> Element {
    rsx! {
        div {
            class: CLASS,
            InlineActions {}
            BurgerMenu {}
        }
    }
}

assert_component!(ToolbarActions);
