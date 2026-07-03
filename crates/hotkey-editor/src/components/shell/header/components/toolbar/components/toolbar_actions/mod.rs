pub mod components;
mod style;

use crate::assert_component;
use components::burger_menu::BurgerMenu;
use components::inline_actions::InlineActions;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(ToolbarActions);

/// The adaptive file-action controls: the inline button row at laptop width and
/// up, collapsing into the burger drawer on narrower screens. A layout-neutral
/// grouping wrapper — it owns no box and threads no data; each child self-sources.
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
